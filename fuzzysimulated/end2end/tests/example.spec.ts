import { test, expect } from "@playwright/test";

test("homepage loads with FuzzySimulated title and system list", async ({ page }) => {
  await page.goto("/");
  await expect(page.locator(".sidebar-logo")).toContainText("FuzzySimulated");
  await expect(page.locator(".section-title")).toContainText("Sistemas Fuzzy");
});

test("sidebar navigation works for all pages", async ({ page }) => {
  await page.goto("/");
  await expect(page.locator(".section-title")).toContainText("Sistemas Fuzzy");

  await page.click('a[href="/vars"]');
  await page.waitForURL("/vars");
  await expect(page.locator(".section-title")).toContainText("Variáveis");

  await page.click('a[href="/rules"]');
  await page.waitForURL("/rules");
  await expect(page.locator(".section-title")).toContainText("Regras");

  await page.click('a[href="/sim"]');
  await page.waitForURL("/sim");
  await expect(page.locator(".section-title")).toContainText("Simulador");

  await page.click('a[href="/hist"]');
  await page.waitForURL("/hist");
  await expect(page.locator(".section-title")).toContainText("Histórico");

  await page.click('a[href="/analysis"]');
  await page.waitForURL("/analysis");
  await expect(page.locator(".section-title")).toContainText("Superfície");

  await page.click('a[href="/audit"]');
  await page.waitForURL("/audit");
  await expect(page.locator(".section-title")).toContainText("Histórico de Alterações");

  await page.click('a[href="/opt"]');
  await page.waitForURL("/opt");
  await expect(page.locator(".section-title")).toContainText("Otimizador");

  await page.click('a[href="/"]');
  await page.waitForURL("/");
});

test("create system flow", async ({ page }) => {
  await page.goto("/");
  await page.click('a:has-text("Criar Sistema")');
  await page.waitForURL("/newsys");

  await expect(page.locator(".section-title")).toContainText("Novo Sistema");

  await page.fill('input[placeholder="Ex: Conforto Térmico"]', "Sistema Teste E2E");
  await page.selectOption('select', { label: "Centroide" });
  await page.click('button:has-text("Criar Sistema")');
  await page.waitForURL("/");
  await expect(page.locator("text=Sistema Teste E2E").first()).toBeVisible();
});
