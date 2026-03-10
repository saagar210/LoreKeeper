import { execFileSync, spawnSync } from "node:child_process";

const HELP = `Usage:
  npm run commit -- "type(scope): summary"

Examples:
  npm run commit -- "fix(ui): close mobile drawer on escape"
  npm run commit -- "docs(readme): clarify lean dev workflow"
`;

const COMMIT_RE =
  /^(feat|fix|refactor|perf|test|docs|build|ci|chore|revert)(\([^)]+\))?!?: .+/;

function getStagedFiles() {
  return execFileSync(
    "/usr/bin/git",
    ["diff", "--cached", "--name-only", "--diff-filter=ACMR"],
    { encoding: "utf8" },
  )
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean);
}

function proposeMessage() {
  const result = spawnSync(
    process.execPath,
    ["scripts/git/propose-commit-message.mjs"],
    {
      stdio: "inherit",
    },
  );

  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

const args = process.argv.slice(2);
const helpRequested = args.includes("--help") || args.includes("-h");

if (helpRequested) {
  console.log(HELP);
  process.exit(0);
}

const staged = getStagedFiles();
if (staged.length === 0) {
  console.error("No staged files. Stage your changes first.");
  process.exit(1);
}

if (args.length === 0) {
  proposeMessage();
  console.log("");
  console.log("Pass a Conventional Commit message to create the commit.");
  console.log('Example: npm run commit -- "fix(repo): tighten perf gate"');
  process.exit(0);
}

const message = args.join(" ").trim();
if (!COMMIT_RE.test(message)) {
  console.error("Commit message must follow Conventional Commits.");
  console.error(HELP);
  process.exit(1);
}

const result = spawnSync("/usr/bin/git", ["commit", "-m", message], {
  stdio: "inherit",
});

process.exit(result.status ?? 1);
