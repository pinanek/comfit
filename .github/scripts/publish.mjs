import { execFileSync } from "node:child_process";
import {
  appendFileSync,
  cpSync,
  mkdirSync,
  mkdtempSync,
  readdirSync,
  readFileSync,
  rmSync,
  statSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { basename, resolve } from "node:path";
import { setTimeout } from "node:timers/promises";

const root = process.cwd();
const readJson = path => JSON.parse(readFileSync(path, "utf8"));
const run = (command, args, cwd = root) => execFileSync(command, args, { cwd, stdio: "inherit" });
const cargo = () =>
  JSON.parse(execFileSync("cargo", ["metadata", "--locked", "--no-deps", "--format-version", "1"], {
    encoding: "utf8",
  })).packages;

const output = (name, value) => {
  console.log(`${name}=${value}`);

  if (process.env.GITHUB_OUTPUT) appendFileSync(process.env.GITHUB_OUTPUT, `${name}=${value}\n`);
};

function release(ecosystem) {
  const tag = process.env.RELEASE_TAG;
  const config = readJson("release-please-config.json");
  const entry = Object.entries(config.packages).find(([, pkg]) => tag?.startsWith(`${pkg.component}-v`));

  if (!entry || entry[1]["release-type"] !== ecosystem) throw new Error(`Unexpected ${ecosystem} release: ${tag}`);

  const [path, pkg] = entry;
  const version = tag.slice(`${pkg.component}-v`.length);

  if (!/^\d+\.\d+\.\d+(?:-[\da-zA-Z.-]+)?$/.test(version)) throw new Error(`Invalid version: ${version}`);

  let name;
  let actual;

  if (ecosystem === "rust") {
    const metadata = cargo().find(pkg => pkg.manifest_path === resolve(path, "Cargo.toml"));

    name = metadata.name;
    actual = metadata.version;
  } else if (ecosystem === "node") {
    ({ name, version: actual } = readJson(`${path}/package.json`));
  } else {
    const project = readFileSync(`${path}/pyproject.toml`, "utf8").split("[project]")[1]?.split(/\n\[/)[0];
    name = project?.match(/^name = "([^"]+)"$/m)?.[1];
    actual = project?.match(/^version = "([^"]+)"$/m)?.[1];
  }

  if (!name || actual !== version) throw new Error(`${tag} does not match ${path}'s version (${actual})`);

  if (readJson(".release-please-manifest.json")[path] !== version) {
    throw new Error(`${tag} does not match the release manifest`);
  }

  return { path, name, version, component: pkg.component };
}

async function published(ecosystem, name, version) {
  const index = name.length < 3
    ? `${name.length}/${name}`
    : name.length === 3
    ? `3/${name[0]}/${name}`
    : `${name.slice(0, 2)}/${name.slice(2, 4)}/${name}`;
  const url = ecosystem === "rust"
    ? `https://index.crates.io/${index}`
    : ecosystem === "python"
    ? `https://pypi.org/pypi/${encodeURIComponent(name)}/${version}/json`
    : `https://registry.npmjs.org/${encodeURIComponent(name)}/${version}`;

  const response = await fetch(url, {
    headers: { "User-Agent": "comfit-release (https://github.com/pinanek/comfit)" },
    signal: AbortSignal.timeout(30_000),
  });

  if (response.status === 404) return false;
  if (!response.ok) throw new Error(`${url}: HTTP ${response.status}`);

  if (ecosystem === "rust") {
    const entry = (await response.text()).trim().split("\n").map(line => JSON.parse(line)).find(entry =>
      entry.vers === version
    );

    if (entry?.yanked) throw new Error(`${name}@${version} is yanked`);

    return Boolean(entry);
  }

  const entry = await response.json();

  if (ecosystem === "python") {
    // A partial upload must remain retryable until the sdist and all seven wheel targets exist.
    const files = entry.urls.filter(file => !file.yanked);
    const platforms = [
      "manylinux_2_28_x86_64",
      "manylinux_2_28_aarch64",
      "musllinux_1_2_x86_64",
      "musllinux_1_2_aarch64",
      "macosx_\\d+_\\d+_x86_64",
      "macosx_\\d+_\\d+_arm64",
      "win_amd64",
    ];

    return files.some(file => file.packagetype === "sdist")
      && platforms.every(platform =>
        files.some(file => new RegExp(`-cp311-abi3-[\\w.]*${platform}[\\w.]*\\.whl$`).test(file.filename))
      );
  }

  return entry.name === name && entry.version === version;
}

async function waitFor(ecosystem, dependencies) {
  const deadline = ecosystem === "node" ? Infinity : Date.now() + 20 * 60_000;

  for (const { name, version } of dependencies) {
    while (!await published(ecosystem, name, version)) {
      if (Date.now() >= deadline) {
        throw new Error(`Timed out waiting for ${name}@${version}; rerun after its publish succeeds`);
      }

      console.log(`Waiting for ${name}@${version}`);
      await setTimeout(15_000);
    }
  }
}

function nodePlatforms() {
  return readJson("npm/node/package.json").napi.targets.map(target => {
    const cpu = target.startsWith("aarch64-") ? "arm64" : target.startsWith("x86_64-") ? "x64" : undefined;
    const os = target.endsWith("-apple-darwin")
      ? "darwin"
      : target.endsWith("-pc-windows-msvc")
      ? "win32"
      : /-unknown-linux-(gnu|musl)$/.test(target)
      ? "linux"
      : undefined;

    if (!cpu || !os) throw new Error(`Unsupported Node target: ${target}`);

    const abi = os === "linux" ? target.split("-").at(-1) : os === "win32" ? "msvc" : undefined;
    const libc = os === "linux" ? abi === "musl" ? "musl" : "glibc" : undefined;
    const suffix = [os, cpu, abi].filter(Boolean).join("-");

    return { suffix, os, cpu, libc };
  });
}

function stageNpm(selected) {
  const destination = resolve("target/npm-release");

  rmSync(destination, { recursive: true, force: true });
  mkdirSync(destination, { recursive: true });

  const packages = [];

  if (selected.component === "node") {
    const manifest = readJson(`${selected.path}/package.json`);
    const stage = resolve("target/npm-stage");

    rmSync(stage, { recursive: true, force: true });
    mkdirSync(stage, { recursive: true });

    manifest.optionalDependencies = {};

    for (const { suffix, os, cpu, libc } of nodePlatforms()) {
      const name = `${manifest.name}-${suffix}`;
      const directory = `${stage}/${suffix}`;
      const binary = `${manifest.napi.binaryName}.${suffix}.node`;

      if (!statSync(`${selected.path}/dist/${binary}`).size) throw new Error(`Empty binary: ${binary}`);

      mkdirSync(directory);

      for (const file of ["LICENSE", "THIRD_PARTY_NOTICES.txt"]) {
        cpSync(`${selected.path}/${file}`, `${directory}/${file}`);
      }

      cpSync(`${selected.path}/dist/${binary}`, `${directory}/${binary}`);
      writeFileSync(
        `${directory}/package.json`,
        JSON.stringify(
          {
            name,
            version: selected.version,
            description: `${manifest.description} (${suffix})`,
            repository: manifest.repository,
            license: manifest.license,
            engines: manifest.engines,
            main: binary,
            files: [binary, "LICENSE", "THIRD_PARTY_NOTICES.txt"],
            os: [os],
            cpu: [cpu],
            ...(libc ? { libc: [libc] } : {}),
          },
          null,
          2,
        ),
      );

      const archive = `${name.replace("@", "").replace("/", "-")}-${selected.version}.tgz`;

      run("npm", ["pack", "--ignore-scripts", "--pack-destination", destination], directory);
      packages.push({ name, version: selected.version, archive, os, cpu, libc });
      manifest.optionalDependencies[name] = selected.version;
    }

    const directory = `${stage}/main`;
    mkdirSync(directory);

    for (const file of ["README.md", "LICENSE", "THIRD_PARTY_NOTICES.txt", "dist"]) {
      cpSync(`${selected.path}/${file}`, `${directory}/${file}`, { recursive: true });
    }

    // Only the optional platform packages ship binaries; keep the generated NAPI loader and types.
    for (const file of readdirSync(`${directory}/dist`)) {
      if (file.endsWith(".node")) rmSync(`${directory}/dist/${file}`);
    }

    delete manifest.scripts;
    delete manifest.devDependencies;
    writeFileSync(`${directory}/package.json`, JSON.stringify(manifest, null, 2));

    run("npm", ["pack", "--ignore-scripts", "--pack-destination", destination], directory);
    packages.push({ name: manifest.name, version: selected.version, archive: `comfit-node-${selected.version}.tgz` });
  } else {
    const archive = `${selected.component}.tgz`;

    run("pnpm", ["--dir", selected.path, "pack", "--out", `${destination}/${archive}`]);
    packages.push({ name: selected.name, version: selected.version, archive });
  }

  writeFileSync(`${destination}/packages.json`, JSON.stringify(packages, null, 2));
}

const [command, ecosystem] = process.argv.slice(2);

if (command === "select") {
  for (const [name, value] of Object.entries(release(ecosystem))) output(name, value);
} else if (command === "status") {
  const selected = release(ecosystem);

  const names = [selected.name];

  if (ecosystem === "node" && selected.component === "node") {
    names.push(...nodePlatforms().map(platform => `${selected.name}-${platform.suffix}`));
  }

  const statuses = await Promise.all(names.map(name => published(ecosystem, name, selected.version)));
  output("published", statuses.every(Boolean));
} else if (command === "wait") {
  const selected = release(ecosystem);
  const dependencies = [];

  if (ecosystem === "rust") {
    const packages = cargo();

    for (const dependency of packages.find(pkg => pkg.name === selected.name).dependencies) {
      if (!dependency.path) continue;

      const pkg = packages.find(pkg => pkg.manifest_path === resolve(dependency.path, "Cargo.toml"));

      if (!pkg || pkg.publish?.length === 0) throw new Error(`Unpublishable dependency: ${dependency.name}`);

      dependencies.push({ name: pkg.name, version: pkg.version });
    }
  } else {
    const manifest = readJson(`${selected.path}/package.json`);
    const config = readJson("release-please-config.json");
    const locals = Object.keys(config.packages).filter(path => config.packages[path]["release-type"] === "node")
      .map(path => readJson(`${path}/package.json`));

    for (const name of Object.keys({ ...manifest.dependencies, ...manifest.optionalDependencies })) {
      const pkg = locals.find(pkg => pkg.name === name);

      if (pkg) dependencies.push({ name, version: pkg.version });
    }
  }

  await waitFor(ecosystem, dependencies);
} else if (command === "stage-npm") {
  stageNpm(release("node"));
} else if (command === "smoke-node") {
  const packages = readJson("target/npm-release/packages.json");
  const libc = process.platform === "linux"
    ? process.report.getReport().header.glibcVersionRuntime ? "glibc" : "musl"
    : undefined;
  const native = packages.find(pkg => pkg.os === process.platform && pkg.cpu === process.arch && pkg.libc === libc);
  const main = packages.find(pkg => pkg.name === "@comfit/node");

  if (!native || !main) throw new Error("Missing host platform package");

  const temporary = mkdtempSync(resolve(tmpdir(), "comfit-node-"));

  try {
    writeFileSync(`${temporary}/package.json`, JSON.stringify({ private: true, type: "module" }));
    run("npm", [
      "install",
      "--omit=optional",
      "--no-audit",
      "--no-fund",
      "--package-lock=false",
      ...[native, main].map(pkg => resolve("target/npm-release", pkg.archive)),
    ], temporary);

    mkdirSync(`${temporary}/npm/node/test`, { recursive: true });
    cpSync("npm/node/test/contracts.test.mjs", `${temporary}/npm/node/test/contracts.test.mjs`);
    cpSync("crates/comfit/tests/fixtures", `${temporary}/crates/comfit/tests/fixtures`, { recursive: true });

    process.env.NAPI_RS_ENFORCE_VERSION_CHECK = "1";
    run(process.execPath, ["--test", "npm/node/test/contracts.test.mjs"], temporary);
  } finally {
    rmSync(temporary, { recursive: true, force: true });
  }
} else if (command === "publish-npm") {
  const selected = release("node");

  for (const pkg of readJson("target/npm-release/packages.json")) {
    if (pkg.version !== selected.version || basename(pkg.archive) !== pkg.archive) {
      throw new Error("Invalid npm artifact manifest");
    }

    if (await published("node", pkg.name, pkg.version)) {
      console.log(`Already published: ${pkg.name}@${pkg.version}`);
      continue;
    }

    run("npm", [
      "publish",
      resolve("target/npm-release", pkg.archive),
      "--access",
      "public",
      "--provenance",
      "--tag",
      selected.version.includes("-") ? "next" : "latest",
    ]);

    await waitFor("node", [pkg]);
  }
} else {
  throw new Error(`Unknown publishing command: ${command}`);
}
