import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { spawn } from "node:child_process";
import { setTimeout as delay } from "node:timers/promises";
import process from "node:process";
import lighthouse from "lighthouse";
import { launch } from "chrome-launcher";

const configPath = process.argv[2] ?? "lighthouserc.json";
const config = JSON.parse(readFileSync(configPath, "utf8"));

const collect = config.ci?.collect ?? {};
const assertConfig = config.ci?.assert?.assertions ?? {};
const urls = (collect.url ?? []).map((url) => url.replace("localhost", "127.0.0.1"));
const runs = Number(collect.numberOfRuns ?? 1);
const previewPort = Number(new URL(urls[0] ?? "http://127.0.0.1:4173/").port || 4173);

if (urls.length === 0) {
  console.error(`No URLs configured in ${configPath}`);
  process.exit(1);
}

function parseAssertion(key, value) {
  if (!key.startsWith("categories:")) return null;
  const category = key.split(":")[1];
  const [level, options] = value;
  return {
    category,
    level,
    minScore: Number(options?.minScore ?? 0),
  };
}

function median(values) {
  const sorted = [...values].sort((a, b) => a - b);
  const mid = Math.floor(sorted.length / 2);
  if (sorted.length % 2 === 0) {
    return (sorted[mid - 1] + sorted[mid]) / 2;
  }
  return sorted[mid];
}

async function waitForServer(url) {
  for (let attempt = 0; attempt < 60; attempt += 1) {
    try {
      const response = await fetch(url);
      if (response.ok) return;
    } catch {
      // Keep polling until the preview server is ready.
    }
    await delay(500);
  }
  throw new Error(`Timed out waiting for preview server at ${url}`);
}

const preview = spawn(
  process.execPath,
  ["./node_modules/vite/bin/vite.js", "preview", "--host", "127.0.0.1", "--port", String(previewPort), "--strictPort"],
  {
    stdio: "inherit",
  },
);

let chrome;

try {
  await waitForServer(urls[0]);

  chrome = await launch({
    chromeFlags: ["--headless=new", "--no-sandbox", "--disable-dev-shm-usage"],
  });

  const allRuns = [];
  for (const url of urls) {
    const categoryRuns = {};
    for (let runIndex = 0; runIndex < runs; runIndex += 1) {
      const result = await lighthouse(
        url,
        {
          port: chrome.port,
          output: "json",
          logLevel: "error",
        },
      );

      const categories = {};
      for (const [name, value] of Object.entries(result.lhr.categories)) {
        const score = Number(value?.score ?? 0);
        categories[name] = score;
        categoryRuns[name] ??= [];
        categoryRuns[name].push(score);
      }

      allRuns.push({
        url,
        run: runIndex + 1,
        finalDisplayedUrl: result.lhr.finalDisplayedUrl,
        categories,
      });
    }

    for (const [name, values] of Object.entries(categoryRuns)) {
      categoryRuns[name] = {
        scores: values,
        median: median(values),
      };
    }

    allRuns.push({
      url,
      aggregate: true,
      categories: categoryRuns,
    });
  }

  const aggregates = allRuns.filter((entry) => entry.aggregate);
  const assertions = Object.entries(assertConfig)
    .map(([key, value]) => parseAssertion(key, value))
    .filter(Boolean);

  const results = assertions.map((assertion) => {
    const medians = aggregates
      .map((entry) => entry.categories[assertion.category]?.median)
      .filter((value) => typeof value === "number");
    const score = medians.length > 0 ? Math.min(...medians) : 0;
    const passed = score >= assertion.minScore;
    return { ...assertion, score, passed };
  });

  mkdirSync(".perf-results", { recursive: true });
  writeFileSync(
    ".perf-results/lighthouse.json",
    JSON.stringify(
      {
        configPath,
        urls,
        runs,
        results,
        entries: allRuns,
      },
      null,
      2,
    ),
  );

  for (const result of results) {
    const scorePct = Math.round(result.score * 100);
    const minPct = Math.round(result.minScore * 100);
    const line = `${result.category}: ${scorePct} (min ${minPct}, ${result.level})`;
    if (result.passed) {
      console.log(`PASS ${line}`);
    } else if (result.level === "warn") {
      console.warn(`WARN ${line}`);
    } else {
      console.error(`FAIL ${line}`);
    }
  }

  if (results.some((result) => !result.passed && result.level === "error")) {
    process.exitCode = 1;
  }
} finally {
  if (chrome) {
    await chrome.kill();
  }
  preview.kill("SIGTERM");
}
