import { devices, defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: "./tests",
  timeout: 90 * 1000,
  expect: { timeout: 15000 },
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: 0,
  workers: 1,
  reporter: "html",
  use: {
    baseURL: "http://127.0.0.1:3000",
    trace: "on-first-retry",
  },

  /* Chromium é o único browser instalado */
  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },
  ],

  /* Inicia servidor novo a cada execução (evita conexão com processo travado) */
  webServer: {
    command: "cd .. && cargo leptos watch",
    port: 3000,
    timeout: 180 * 1000,
    reuseExistingServer: false,
  },
});
