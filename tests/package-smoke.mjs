import { execFileSync } from "node:child_process";
import { cp, mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("../", import.meta.url));
const target = process.argv[2] ?? "all";
if (!["all", "npm", "python"].includes(target)) {
  throw new Error(`Unknown package target: ${target}`);
}

const temporary = await mkdtemp(join(tmpdir(), "comfit-packages-"));

function run(command, args, cwd = root) {
  execFileSync(command, args, { cwd, stdio: "inherit" });
}

try {
  if (target !== "python") {
    const archives = [];
    for (const name of ["node", "wasm", "wasm-grammars", "rehype", "markdown-it", "css"]) {
      const archive = join(temporary, `${name}.tgz`);
      run("pnpm", ["--silent", "--dir", `npm/${name}`, "pack", "--out", archive]);
      archives.push(archive);
    }

    await writeFile(join(temporary, "package.json"), JSON.stringify({ private: true, type: "module" }));
    run("npm", ["install", "--no-audit", "--no-fund", "--package-lock=false", ...archives], temporary);
    await cp(join(root, "tests/consumer"), join(temporary, "consumer"), { recursive: true });
    run(process.execPath, ["consumer/npm.mjs"], temporary);
    await writeFile(
      join(temporary, "tsconfig.json"),
      JSON.stringify({
        compilerOptions: { noEmit: true, strict: true, skipLibCheck: true, target: "esnext", module: "nodenext" },
        include: ["consumer/types.ts"],
      }),
    );
    run("pnpm", ["--dir", "npm/wasm", "exec", "tsc", "--project", join(temporary, "tsconfig.json")]);

    await mkdir(join(temporary, "crates/comfit/tests/fixtures"), { recursive: true });
    await cp(
      join(root, "crates/comfit/tests/fixtures/highlighting.json"),
      join(temporary, "crates/comfit/tests/fixtures/highlighting.json"),
    );
    for (const name of ["node", "wasm", "rehype"]) {
      const path = `npm/${name}/test/contracts.test.mjs`;
      await mkdir(join(temporary, `npm/${name}/test`), { recursive: true });
      await cp(join(root, path), join(temporary, path));
    }
    run(
      process.execPath,
      ["--test", ...["node", "wasm", "rehype"].map(name => `npm/${name}/test/contracts.test.mjs`)],
      temporary,
    );
  }

  if (target !== "npm") {
    run("uv", ["run", "--no-project", "python", "tests/package-python.py"]);
  }
} finally {
  await rm(temporary, { recursive: true, force: true });
}
