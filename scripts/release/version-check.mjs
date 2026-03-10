import fs from "node:fs";
import path from "node:path";
import process from "node:process";

const repoRoot = path.resolve(
  path.dirname(new URL(import.meta.url).pathname),
  "../..",
);

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, "utf8"));
}

function readCargoPackageVersion(filePath) {
  const contents = fs.readFileSync(filePath, "utf8");
  const packageSection = contents.match(/\[package\][\s\S]*?(?=\n\[|$)/);
  if (!packageSection) {
    throw new Error(`Could not find [package] section in ${filePath}`);
  }

  const versionMatch = packageSection[0].match(/^\s*version\s*=\s*"([^"]+)"/m);
  if (!versionMatch) {
    throw new Error(`Could not find package version in ${filePath}`);
  }

  return versionMatch[1];
}

export function readVersionSnapshot() {
  const packageJsonPath = path.join(repoRoot, "package.json");
  const tauriConfigPath = path.join(repoRoot, "src-tauri", "tauri.conf.json");
  const cargoTomlPath = path.join(repoRoot, "src-tauri", "Cargo.toml");

  return {
    packageJson: readJson(packageJsonPath).version,
    tauriConfig: readJson(tauriConfigPath).version,
    cargoToml: readCargoPackageVersion(cargoTomlPath),
    files: {
      packageJson: packageJsonPath,
      tauriConfig: tauriConfigPath,
      cargoToml: cargoTomlPath,
    },
  };
}

export function assertVersionConsistency(snapshot = readVersionSnapshot()) {
  const versions = new Set([
    snapshot.packageJson,
    snapshot.tauriConfig,
    snapshot.cargoToml,
  ]);

  if (versions.size !== 1) {
    throw new Error(
      [
        "Version mismatch detected:",
        `- package.json: ${snapshot.packageJson}`,
        `- src-tauri/tauri.conf.json: ${snapshot.tauriConfig}`,
        `- src-tauri/Cargo.toml: ${snapshot.cargoToml}`,
      ].join("\n"),
    );
  }

  return snapshot.packageJson;
}

function main() {
  const snapshot = readVersionSnapshot();
  const version = assertVersionConsistency(snapshot);
  console.log(`Version check passed: ${version}`);
  console.log(`- package.json: ${snapshot.packageJson}`);
  console.log(`- src-tauri/tauri.conf.json: ${snapshot.tauriConfig}`);
  console.log(`- src-tauri/Cargo.toml: ${snapshot.cargoToml}`);
}

const invokedDirectly =
  process.argv[1] &&
  fs.realpathSync(process.argv[1]) ===
    fs.realpathSync(new URL(import.meta.url));

if (invokedDirectly) {
  try {
    main();
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    process.exit(1);
  }
}
