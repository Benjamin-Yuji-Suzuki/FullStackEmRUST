import { test, expect } from "@playwright/test";

test("homepage loads with FuzzySimulated title and system list", async ({ page }) => {
  await page.goto("/");
  await expect(page.locator(".sidebar-logo")).toContainText("FuzzySimulated");
  await expect(page.locator(".section-title")).toContainText("Sistemas Fuzzy");
});

const PAGES = [
  { href: "/vars", title: "Variáveis" },
  { href: "/rules", title: "Regras" },
  { href: "/sim", title: "Simulador" },
  { href: "/hist", title: "Histórico" },
  { href: "/analysis", title: "Superfície" },
  { href: "/audit", title: "Histórico de Alterações" },
  { href: "/opt", title: "Otimizador" },
] as const;

test("sidebar navigation works for all pages", async ({ page }) => {
  await page.goto("/");
  await expect(page.locator(".section-title")).toContainText("Sistemas Fuzzy");

  for (const { href, title } of PAGES) {
    await page.click(`a[href="${href}"]`);
    await page.waitForURL(href);
    await expect(page.locator(".section-title")).toContainText(title);
  }

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

test("simulator page shows system selector", async ({ page }) => {
  await page.goto("/sim");
  await expect(page.locator(".section-title")).toContainText("Simulador");
  await expect(page.locator("text=Selecione um sistema")).toBeVisible();
});
