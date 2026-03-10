import { defineConfig } from "@playwright/test";

const E2E_PORT = Number(process.env.E2E_PORT || "1435");

export default defineConfig({
  testDir: ".",
  testMatch: "*.spec.ts",
  timeout: 30_000,
  retries: 1,
  use: {
    baseURL: `http://127.0.0.1:${E2E_PORT}`,
    headless: true,
    screenshot: "only-on-failure",
  },
  webServer: {
    command: `npm run dev -- --host 127.0.0.1 --port ${E2E_PORT} --strictPort`,
    port: E2E_PORT,
    reuseExistingServer: false,
    timeout: 180_000,
    cwd: "..",
  },
});
