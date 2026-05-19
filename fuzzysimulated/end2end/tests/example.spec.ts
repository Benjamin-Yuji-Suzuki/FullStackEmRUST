import { test, expect } from "@playwright/test";

test("homepage loads with FuzzySimulated title and system list", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await expect(page.locator(".topbar")).toContainText("FuzzySimulated");
  await expect(page.locator(".section-title")).toContainText("Dashboard");
});

test("can navigate to Variaveis page", async ({ page }) => {
  await page.goto("http://localhost:3000/");
  await page.locator('a[href="/variaveis"]').first().click();
  await expect(page.locator(".section-title")).toContainText("Variáveis");
});

test("can navigate to Regras page", async ({ page }) => {
  await page.goto("http://localhost:3000/");
  await page.locator('a[href="/regras"]').first().click();
  await expect(page.locator(".section-title")).toContainText("Regras");
});

test("can navigate to Simulador page", async ({ page }) => {
  await page.goto("http://localhost:3000/");
  await page.locator('a[href="/simulador"]').first().click();
  await expect(page.locator(".section-title")).toContainText("Simulador");
});
