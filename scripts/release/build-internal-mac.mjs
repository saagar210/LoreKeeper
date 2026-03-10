import crypto from "node:crypto";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import process from "node:process";
import { spawnSync } from "node:child_process";

import {
  assertVersionConsistency,
  readVersionSnapshot,
} from "./version-check.mjs";

const repoRoot = path.resolve(
  path.dirname(new URL(import.meta.url).pathname),
  "../..",
);
const defaultCargoTargetDir = path.join(
  os.homedir(),
  ".cache",
  "lorekeeper",
  "cargo-target",
);

function run(command, args, options = {}) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    stdio: "inherit",
    env: process.env,
    ...options,
  });

  if (result.status !== 0) {
    throw new Error(`Command failed: ${command} ${args.join(" ")}`);
  }
}

function runCapture(command, args, options = {}) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: "utf8",
    env: process.env,
    ...options,
  });

  if (result.status !== 0) {
    throw new Error(
      (result.stderr || result.stdout || `Command failed: ${command}`).trim(),
    );
  }

  return result.stdout.trim();
}

function ensureCleanTrackedState() {
  const trackedChanges = runCapture("git", [
    "status",
    "--porcelain",
    "--untracked-files=no",
  ]);

  if (trackedChanges) {
    throw new Error(
      "Tracked git changes detected. Commit or stash tracked changes before building an internal release.",
    );
  }
}

function ensurePlatform() {
  if (process.platform !== "darwin") {
    throw new Error("Internal macOS release builds must run on macOS.");
  }
}

function getCargoTargetDir() {
  return (
    process.env.CARGO_TARGET_DIR ||
    process.env.LOREKEEPER_CARGO_TARGET_DIR ||
    defaultCargoTargetDir
  );
}

function walkBundleTree(rootDir) {
  if (!fs.existsSync(rootDir)) {
    return [];
  }

  const queue = [rootDir];
  const results = [];

  while (queue.length > 0) {
    const current = queue.shift();
    const entries = fs.readdirSync(current, { withFileTypes: true });
    for (const entry of entries) {
      const fullPath = path.join(current, entry.name);
      results.push({
        path: fullPath,
        name: entry.name,
        isDirectory: entry.isDirectory(),
      });
      if (entry.isDirectory()) {
        queue.push(fullPath);
      }
    }
  }

  return results;
}

function sha256ForFile(filePath) {
  const hash = crypto.createHash("sha256");
  hash.update(fs.readFileSync(filePath));
  return hash.digest("hex");
}

function formatIsoDateForFile(value) {
  return value.replace(/[:]/g, "-");
}

function main() {
  ensurePlatform();
  ensureCleanTrackedState();

  const snapshot = readVersionSnapshot();
  const version = assertVersionConsistency(snapshot);
  const branch = runCapture("git", ["branch", "--show-current"]);
  const commitSha = runCapture("git", ["rev-parse", "HEAD"]);
  const shortSha = runCapture("git", ["rev-parse", "--short", "HEAD"]);
  const builtAtUtc = new Date().toISOString();

  const outputDir = path.join(
    repoRoot,
    "release-artifacts",
    "internal",
    "macos",
    `${version}-${shortSha}`,
  );
  const cargoTargetDir = getCargoTargetDir();
  const bundleDir = path.join(cargoTargetDir, "release", "bundle");

  fs.rmSync(outputDir, { recursive: true, force: true });
  fs.mkdirSync(outputDir, { recursive: true });

  console.log("Preparing internal macOS release build");
  console.log(`- Version: ${version}`);
  console.log(`- Branch: ${branch}`);
  console.log(`- Commit: ${commitSha}`);
  console.log(`- Output: ${outputDir}`);
  console.log("- Signing: disabled (unsigned internal build)");

  run("npm", ["run", "build:frontend"]);
  run("npm", ["run", "tauri", "--", "build", "--bundles", "app,dmg"]);

  const bundleEntries = walkBundleTree(bundleDir);
  const sourceArtifacts = bundleEntries.filter((entry) => {
    if (entry.isDirectory && entry.name.endsWith(".app")) {
      return true;
    }
    if (
      !entry.isDirectory &&
      [".dmg", ".zip"].some((ext) => entry.name.endsWith(ext))
    ) {
      return true;
    }
    return false;
  });

  if (sourceArtifacts.length === 0) {
    throw new Error(`No macOS bundle artifacts found in ${bundleDir}`);
  }

  const producedArtifacts = [];
  for (const artifact of sourceArtifacts) {
    if (artifact.isDirectory && artifact.name.endsWith(".app")) {
      const archiveName = `${artifact.name}.tar.gz`;
      const archivePath = path.join(outputDir, archiveName);
      run("tar", [
        "-czf",
        archivePath,
        "-C",
        path.dirname(artifact.path),
        artifact.name,
      ]);
      producedArtifacts.push({
        name: archiveName,
        kind: "app-archive",
        filePath: archivePath,
      });
      continue;
    }

    const targetPath = path.join(outputDir, artifact.name);
    fs.copyFileSync(artifact.path, targetPath);
    producedArtifacts.push({
      name: artifact.name,
      kind: path.extname(artifact.name).replace(".", "") || "file",
      filePath: targetPath,
    });
  }

  producedArtifacts.sort((left, right) => left.name.localeCompare(right.name));

  const checksums = producedArtifacts.map((artifact) => {
    const stats = fs.statSync(artifact.filePath);
    return {
      ...artifact,
      sha256: sha256ForFile(artifact.filePath),
      bytes: stats.size,
    };
  });

  const checksumLines = checksums.map(
    (artifact) => `${artifact.sha256}  ${artifact.name}`,
  );
  fs.writeFileSync(
    path.join(outputDir, "checksums.txt"),
    `${checksumLines.join("\n")}\n`,
  );

  const manifest = {
    productName: "LoreKeeper",
    version,
    platform: "macos",
    buildType: "internal-unsigned",
    branch,
    commitSha,
    shortSha,
    builtAtUtc,
    sourceVersions: snapshot,
    bundleSourceDir: bundleDir,
    artifactDirectory: outputDir,
    gatekeeperNote:
      "Unsigned internal macOS build. Internal testers may need to Control-click Open or remove the quarantine attribute.",
    artifacts: checksums.map((artifact) => ({
      name: artifact.name,
      kind: artifact.kind,
      sha256: artifact.sha256,
      bytes: artifact.bytes,
    })),
  };

  fs.writeFileSync(
    path.join(outputDir, "release-manifest.json"),
    `${JSON.stringify(manifest, null, 2)}\n`,
  );

  fs.writeFileSync(
    path.join(outputDir, "BUILD_INFO.txt"),
    [
      `LoreKeeper internal macOS build`,
      `Version: ${version}`,
      `Branch: ${branch}`,
      `Commit: ${commitSha}`,
      `Built at (UTC): ${builtAtUtc}`,
      `Unsigned: yes`,
      `Artifacts:`,
      ...checksums.map((artifact) => `- ${artifact.name}`),
    ].join("\n") + "\n",
  );

  console.log("Internal release build complete");
  console.log(`- Artifact directory: ${outputDir}`);
  console.log("- Artifact files:");
  for (const artifact of checksums) {
    console.log(
      `  - ${artifact.name} (${artifact.kind}, ${artifact.bytes} bytes)`,
    );
  }
  console.log(`- Manifest: ${path.join(outputDir, "release-manifest.json")}`);
  console.log(`- Checksums: ${path.join(outputDir, "checksums.txt")}`);
  console.log(`- Built at (UTC): ${formatIsoDateForFile(builtAtUtc)}`);
}

try {
  main();
} catch (error) {
  console.error(error instanceof Error ? error.message : String(error));
  process.exit(1);
}
