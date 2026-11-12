import { execFileSync } from "node:child_process";
import { appendFileSync, existsSync, readFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { GitHub, Manifest } from "release-please";

export function buildReleaseGraph(root = process.cwd()) {
  const readJson = path => JSON.parse(readFileSync(resolve(root, path), "utf8"));
  const config = readJson("release-please-config.json");
  const versions = readJson(".release-please-manifest.json");
  const metadata = JSON.parse(execFileSync("cargo", ["metadata", "--no-deps", "--locked", "--format-version", "1"], {
    cwd: root,
    encoding: "utf8",
  }));

  const dependencies = new Map();
  const dependents = new Map();
  const link = (dependency, consumer) => {
    if (!dependencies.has(consumer)) dependencies.set(consumer, new Set());
    if (!dependents.has(dependency)) dependents.set(dependency, new Set());

    dependencies.get(consumer).add(dependency);
    dependents.get(dependency).add(consumer);
  };

  const cargoPaths = new Map(
    metadata.packages.map(pkg => [pkg.name, relative(root, dirname(pkg.manifest_path)).replaceAll("\\", "/")]),
  );

  for (const pkg of metadata.packages) {
    for (const dependency of pkg.dependencies) {
      const path = cargoPaths.get(dependency.name);

      if (path) link(path, cargoPaths.get(pkg.name));
    }
  }

  const npmPackages = Object.keys(config.packages)
    .filter(path => existsSync(resolve(root, path, "package.json")))
    .map(path => ({ path, ...readJson(`${path}/package.json`) }));
  const npmPaths = new Map(npmPackages.map(pkg => [pkg.name, pkg.path]));

  for (const pkg of npmPackages) {
    for (const name of Object.keys({ ...pkg.dependencies, ...pkg.devDependencies, ...pkg.optionalDependencies })) {
      const path = npmPaths.get(name);

      if (path) link(path, pkg.path);
    }
  }

  // Generated assets have build-time inputs that are not package-manager dependencies.
  for (
    const [source, outputs] of [
      ["crates/comfit_grammars", ["npm/wasm-grammars"]],
      ["crates/comfit_themes", ["npm/css"]],
      ["xtask/grammars", ["crates/comfit_grammars"]],
      ["xtask/theme", ["crates/comfit_themes"]],
      ["xtask/css", ["npm/css"]],
      ["xtask/wasm", ["npm/wasm", "npm/wasm-grammars"]],
      ["xtask/notices", Object.keys(config.packages)],
    ]
  ) {
    for (const output of outputs) link(source, output);
  }

  return {
    config,
    versions,
    cargoPaths: [...cargoPaths.values()],
    npmPaths: [...npmPaths.values()],
    dependencies,
    dependents,
  };
}

export function affectedPackages(files, graph) {
  const contains = (path, file) => file === path || file.startsWith(`${path}/`);
  const pending = new Set([...graph.cargoPaths, ...Object.keys(graph.config.packages)]
    .filter(path => files.some(file => contains(path, file))));

  for (const file of files) {
    if (["Cargo.toml", "Cargo.lock"].includes(file)) {
      for (const path of graph.cargoPaths) pending.add(path);
    }

    if (["package.json", "pnpm-workspace.yaml", "pnpm-lock.yaml"].includes(file)) {
      for (const path of graph.npmPaths) pending.add(path);
    }

    if (["pyproject.toml", "uv.lock"].includes(file)) pending.add("python/comfit");

    if (["queries", "registry/languages.toml", "registry/patches"].some(path => contains(path, file))) {
      pending.add("crates/comfit_grammars");
    }

    if (file === "registry/themes.toml") pending.add("crates/comfit_themes");

    if (["LICENSE", "registry/notices.toml", "registry/licenses"].some(path => contains(path, file))) {
      for (const path of Object.keys(graph.config.packages)) pending.add(path);
    }
  }

  const affected = new Set();

  for (const path of pending) {
    if (affected.has(path)) continue;

    affected.add(path);
    for (const consumer of graph.dependents.get(path) ?? []) pending.add(consumer);

    // A first publication must also include any unpublished local dependencies.
    for (const dependency of graph.dependencies.get(path) ?? []) {
      if (graph.versions[dependency] === "0.0.0") pending.add(dependency);
    }
  }

  return [...affected].filter(path => Object.hasOwn(graph.config.packages, path)).sort();
}

async function main() {
  const token = process.env.RELEASE_PLEASE_TOKEN;

  if (!token) throw new Error("Set the RELEASE_PLEASE_TOKEN repository secret");

  const [owner, repo] = (process.env.GITHUB_REPOSITORY ?? "").split("/");

  if (!owner || !repo) throw new Error("GITHUB_REPOSITORY must identify the release repository");

  const graph = buildReleaseGraph();
  const github = await GitHub.create({ owner, repo, token });
  const commits = github.mergeCommitIterator.bind(github);

  // Route shared inputs before splitting commits, preserving each package's release boundary.
  github.mergeCommitIterator = async function*(...args) {
    for await (const commit of commits(...args)) {
      const files = commit.files ?? [];

      yield {
        ...commit,
        files: [...files, ...affectedPackages(files, graph).map(path => `${path}/.release-please`)],
      };
    }
  };

  const manifest = await Manifest.fromManifest(github, "main");
  await manifest.createReleases();

  const prs = (await manifest.createPullRequests()).filter(Boolean);

  if (prs.length > 1) throw new Error("Keep related package updates in a single release PR");
  if (prs[0] && process.env.GITHUB_OUTPUT) {
    appendFileSync(process.env.GITHUB_OUTPUT, `pr=${JSON.stringify(prs[0])}\n`);
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) await main();
