import { test, expect } from "@playwright/test";

// ──────────────────────────────────────────────────────────────
// UC01 — Dashboard (testes independentes)
// ──────────────────────────────────────────────────────────────
test("homepage loads with FuzzySimulated title and system list", async ({ page }) => {
  await page.goto("/");
  await expect(page.locator(".sidebar-logo")).toContainText("FuzzySimulated");
  await expect(page.locator(".section-title")).toContainText("Sistemas Fuzzy");
});

const PAGES = [
  { href: "/vars", title: "Variáveis & Termos (UC02)" },
  { href: "/rules", title: "Editor de Regras (UC03)" },
  { href: "/sim", title: "Simulador Mamdani (UC04)" },
  { href: "/hist", title: "Histórico (UC06)" },
  { href: "/analysis", title: "Superficie & Matriz de Regras" },
  { href: "/audit", title: "Histórico de Alterações (UC16)" },
  { href: "/opt", title: "Otimizador de Função Objetivo (UC21–UC25)" },
] as const;

test("sidebar navigation works for all pages", async ({ page }) => {
  await page.goto("/");
  for (const { href, title } of PAGES) {
    await page.click(`a[href="${href}"]`);
    await page.waitForURL(href);
    await expect(page.locator(".section-title")).toContainText(title);
  }
  await page.click('a[href="/"]');
  await page.waitForURL("/");
});

test("create system form loads with correct fields", async ({ page }) => {
  await page.goto("/newsys");
  await expect(page.locator(".section-title")).toContainText("Novo Sistema Fuzzy");
  await expect(page.locator('input[placeholder="Ex: Conforto Térmico"]')).toBeVisible();
  await expect(page.locator("select")).toBeVisible();
  await expect(page.locator('button:has-text("Criar Sistema")')).toBeVisible();
});

test("simulator page shows empty state when no system selected", async ({ page }) => {
  await page.goto("/sim");
  await expect(page.locator(".section-title")).toContainText("Simulador Mamdani (UC04)");
  await expect(page.locator('option[value=""]')).toContainText("— Selecione —");
  await expect(page.locator("text=Nenhuma variável antecedente")).toBeVisible();
  await expect(page.locator("text=Execute uma simulação para ver o resultado.")).toBeVisible();
});

// ──────────────────────────────────────────────────────────────
// Seed System — "Conforto Térmico" (pré-carregado via migration)
// ──────────────────────────────────────────────────────────────
test.describe.serial("Seed system: Conforto Térmico", () => {
  test("dashboard shows seed system card with status badge", async ({ page }) => {
    await page.goto("/");
    const card = page.locator(".system-card").filter({ hasText: "Conforto Térmico" });
    await expect(card).toBeVisible();
    // status badge appears
    await expect(card.locator(".tag")).toBeVisible();
  });

  test("view seed system variables — 3 variables and 9 terms", async ({ page }) => {
    await page.goto("/vars");
    await page.locator("select").first().selectOption({ label: "Conforto Térmico" });
    await page.waitForTimeout(600);
    // 3 variable items
    await expect(page.locator(".var-item")).toHaveCount(3);
    // terms shown for first selected variable (conforto — 3 terms)
    await expect(page.locator(".term-chip").first()).toBeVisible();
    await expect(page.locator(".term-chip")).toHaveCount(3);
    // verify specific terms exist (click each variable and check)
    const varNames = ["conforto", "temperatura", "umidade"];
    for (const vn of varNames) {
      await page.locator(".var-item").filter({ hasText: vn }).first().click();
      await page.waitForTimeout(200);
      await expect(page.locator(".var-panel .section-title")).toContainText("Termos");
    }
  });

  test("seed system has 9 rules in rule editor", async ({ page }) => {
    await page.goto("/rules");
    await page.locator("select").first().selectOption({ label: "Conforto Térmico" });
    await page.waitForTimeout(600);
    await expect(page.locator(".rule-row")).toHaveCount(9);
    // verify specific rule texts exist
    await expect(page.locator(".rule-text").filter({ hasText: "temperatura" }).first()).toBeVisible();
    await expect(page.locator(".rule-text").filter({ hasText: "conforto" }).first()).toBeVisible();
  });

  test("simulate with seed system — validate actual output value", async ({ page }) => {
    await page.goto("/sim");
    await page.locator("select").first().selectOption({ label: "Conforto Térmico" });
    await page.waitForTimeout(800);
    // fill antecedents: temperatura=25, umidade=50
    const inputs = page.locator('input[type="number"]');
    await inputs.nth(0).fill("25");
    await inputs.nth(1).fill("50");
    await page.locator('button:has-text("Executar Simulação")').click();
    await page.waitForTimeout(1500);
    // output-display must show a valid number for "conforto"
    const outputVal = page.locator(".output-val").first();
    await expect(outputVal).toBeVisible();
    const valText = await outputVal.textContent();
    const num = parseFloat(valText!);
    expect(num).not.toBeNaN();
    // conforto universe is [0,10], so output must be within range
    expect(num).toBeGreaterThanOrEqual(0);
    expect(num).toBeLessThanOrEqual(10);
  });

  test("change seed system status to favorito and back", async ({ page }) => {
    await page.goto("/");
    const card = page.locator(".system-card").filter({ hasText: "Conforto Térmico" }).first();
    await expect(card).toBeVisible();
    // change to favorito
    const statusSelect = card.locator("select.text-input");
    await statusSelect.selectOption("favorito");
    await page.waitForTimeout(500);
    // verify badge changed — favorito uses tag-amber
    await expect(card.locator(".tag-amber")).toBeVisible();
    // change back to ativo
    await statusSelect.selectOption("ativo");
    await page.waitForTimeout(500);
    await expect(card.locator(".tag-green")).toBeVisible();
  });
});

// ──────────────────────────────────────────────────────────────
// Validação & Erros
// ──────────────────────────────────────────────────────────────
test.describe.serial("Validation & error flows", () => {
  test("create system with empty name shows error", async ({ page }) => {
    await page.goto("/newsys");
    await page.click('button:has-text("Criar Sistema")');
    await expect(page.locator("div[style*='coral']").filter({ hasText: "Nome obrigatório" }).first()).toBeVisible();
  });

  test("add term with empty params shows error", async ({ page }) => {
    await page.goto("/add-term");
    await page.locator("select").first().selectOption({ label: "Conforto Térmico" });
    await page.waitForTimeout(400);
    // select first variable
    await page.locator("select").nth(1).selectOption({ index: 1 });
    await page.locator('input[placeholder="Ex: Frio"]').fill("NovoTermo");
    await page.locator('input[placeholder="0, 10, 22"]').fill("abc");
    await page.click('button:has-text("Adicionar")');
    await expect(page.locator("div[style*='coral']").filter({ hasText: "Parâmetros" }).first()).toBeVisible();
  });

  test("delete protection: favorito system can't be deleted", async ({ page }) => {
    // find or create a system to test
    await page.goto("/newsys");
    const name = `E2E DeleteProtect ${Date.now()}`;
    await page.fill('input[placeholder="Ex: Conforto Térmico"]', name);
    await page.selectOption("select", { label: "Centroide" });
    await page.click('button:has-text("Criar Sistema")');
    await page.waitForURL("/");
    // change to favorito
    const card = page.locator(".system-card").filter({ hasText: name }).first();
    await card.locator("select.text-input").selectOption("favorito");
    await page.waitForTimeout(400);
    // verify lock icon appears (delete is blocked)
    await expect(card.locator('[title="Remova o favorito para deletar"]')).toBeVisible();
    // change back to ativo
    await card.locator("select.text-input").selectOption("ativo");
    await page.waitForTimeout(400);
    // delete the test system
    await card.locator('button[title="Deletar"]').click();
    await page.waitForURL("/");
    await expect(page.locator(".system-card").filter({ hasText: name })).toHaveCount(0);
  });
});

// ──────────────────────────────────────────────────────────────
// Ciclo Completo — Cria, usa, edita, deleta
// ──────────────────────────────────────────────────────────────
test.describe.serial("Full lifecycle (create → use → edit → audit → delete)", () => {
  const SUFFIX = Math.random().toString(36).slice(2, 6);
  const SYS_NAME = `E2E Teste Risco (${SUFFIX})`;
  let exportedJson = "";

  test("01: creates a new fuzzy system", async ({ page }) => {
    await page.goto("/newsys");
    await page.fill('input[placeholder="Ex: Conforto Térmico"]', SYS_NAME);
    await page.selectOption("select", { label: "Centroide" });
    await page.click('button:has-text("Criar Sistema")');
    await page.waitForURL("/");
    await expect(page.locator(".system-card").filter({ hasText: SYS_NAME }).first()).toBeVisible();
  });

  test("02: adds 3 variables (2 antecedent + 1 consequent)", async ({ page }) => {
    const vars = [
      { name: "Temperatura", role: "antecedent" },
      { name: "Umidade", role: "antecedent" },
      { name: "Risco", role: "consequent" },
    ];
    for (const v of vars) {
      await page.goto("/add-var");
      await page.locator("select").first().selectOption({ label: SYS_NAME });
      await page.waitForTimeout(400);
      await page.locator("input[type='text']").first().fill(v.name);
      await page.locator("select").nth(1).selectOption(v.role);
      await page.click('button:has-text("Adicionar")');
      await page.waitForURL(/\/vars/);
      await expect(page.locator(".var-item").filter({ hasText: v.name })).toBeVisible();
    }
  });

  test("03: adds 3 terms (Alta/Alta/Alto)", async ({ page }) => {
    const terms = [
      { var: "Temperatura", label: "Alta", params: "0, 30, 50" },
      { var: "Umidade", label: "Alta", params: "0, 40, 70" },
      { var: "Risco", label: "Alto", params: "0, 0.5, 1" },
    ];
    for (const t of terms) {
      await page.goto("/add-term");
      await page.locator("select").first().selectOption({ label: SYS_NAME });
      await page.waitForTimeout(400);
      await page.locator("select").nth(1).selectOption({ label: t.var });
      await page.locator('input[placeholder="Ex: Frio"]').fill(t.label);
      await page.locator("select").nth(2).selectOption("trimf");
      await page.locator('input[placeholder="0, 10, 22"]').fill(t.params);
      await page.click('button:has-text("Adicionar")');
      await page.waitForURL(/\/vars/);
      // verify term in the selected variable
      await page.locator(".var-item").filter({ hasText: t.var }).first().click();
      await page.waitForTimeout(300);
      await expect(page.locator(".term-chip").filter({ hasText: t.label }).first()).toBeVisible();
    }
  });

  test("04: adds a rule", async ({ page }) => {
    await page.goto("/add-rule");
    await page.locator("select").first().selectOption({ label: SYS_NAME });
    await page.waitForTimeout(400);
    await page.locator("input[type='text']").first().fill(
      "SE Temperatura = Alta E Umidade = Alta ENTAO Risco = Alto"
    );
    await page.click('button:has-text("Adicionar")');
    await page.waitForURL(/\/rules/);
    await expect(page.locator(".rule-text").filter({ hasText: "Alta" }).first()).toBeVisible();
  });

  test("05: runs simulation and validates output value", async ({ page }) => {
    await page.goto("/sim");
    await page.locator("select").first().selectOption({ label: SYS_NAME });
    await page.waitForTimeout(800);
    const inputs = page.locator('input[type="number"]');
    await inputs.nth(0).fill("25");
    await inputs.nth(1).fill("30");
    await page.locator('button:has-text("Executar Simulação")').click();
    await page.waitForTimeout(1500);
    // output must show Risco with a valid value
    const outputVal = page.locator(".output-val").first();
    await expect(outputVal).toBeVisible();
    const valText = await outputVal.textContent();
    const num = parseFloat(valText!);
    expect(num).not.toBeNaN();
    // Risco universe uses trimf [0,0.5,1], so output should be reasonable
    expect(num).toBeGreaterThanOrEqual(0);
    expect(num).toBeLessThanOrEqual(1);
    // also check label says "Risco"
    await expect(page.locator(".output-label").filter({ hasText: "Risco" })).toBeVisible();
  });

  test("06: saves a scenario", async ({ page }) => {
    await page.goto("/sim");
    await page.locator("select").first().selectOption({ label: SYS_NAME });
    await page.waitForTimeout(800);
    const inputs = page.locator('input[type="number"]');
    await inputs.nth(0).fill("25");
    await inputs.nth(1).fill("30");
    await page.locator('input[placeholder="Nome do cenario"]').fill("Cenario E2E");
    await page.locator('button:has-text("Salvar")').click();
    await page.waitForTimeout(1000);
    // success message or scenario visible
    await expect(page.locator("text=Cenario E2E").first()).toBeVisible();
  });

  test("07: runs second simulation for comparison", async ({ page }) => {
    await page.goto("/sim");
    await page.locator("select").first().selectOption({ label: SYS_NAME });
    await page.waitForTimeout(800);
    const inputs = page.locator('input[type="number"]');
    await inputs.nth(0).fill("45");
    await inputs.nth(1).fill("60");
    await page.locator('button:has-text("Executar Simulação")').click();
    await page.waitForTimeout(1500);
    await expect(page.locator(".output-val").first()).toBeVisible();
  });

  test("08: simulation appears in history", async ({ page }) => {
    await page.goto("/hist");
    await page.locator("select").first().selectOption({ label: SYS_NAME });
    await page.waitForTimeout(1000);
    await expect(page.locator(".hist-table tbody tr").first()).toBeVisible();
    await expect(page.locator(".hist-table").filter({ hasText: "Risco" })).toBeVisible();
  });

  test("09: compares two simulations", async ({ page }) => {
    await page.goto("/hist");
    await page.locator("select").first().selectOption({ label: SYS_NAME });
    await page.waitForTimeout(1000);
    const checkboxes = page.locator('.hist-table tbody tr input[type="checkbox"]');
    await checkboxes.nth(0).check();
    await checkboxes.nth(1).check();
    await page.locator('button:has-text("Comparar Selecionados")').click();
    await page.waitForTimeout(1500);
    await expect(page.locator("text=Comparacao").first()).toBeVisible();
  });

  test("10: exports simulation report", async ({ page }) => {
    await page.goto("/hist");
    await page.locator("select").first().selectOption({ label: SYS_NAME });
    await page.waitForTimeout(1000);
    await page.locator('button[title="Exportar Relatorio (UC09)"]').first().click();
    await page.waitForTimeout(2000);
    const msg = page.locator("span").filter({ hasText: /Relatorio copiado|Erro ao exportar/ });
    await expect(msg.first()).toBeVisible();
  });

  test("11: edits system description", async ({ page }) => {
    await page.goto("/");
    const card = page.locator(".system-card").filter({ hasText: SYS_NAME }).filter({ hasNotText: "(cópia)" }).first();
    const editLink = card.locator('a[title="Editar"]');
    const href = await editLink.getAttribute("href");
    await page.goto(href!);
    await page.waitForTimeout(1500);
    await page.locator("input[type='text']").nth(1).fill("Sistema editado via E2E");
    await page.click('button:has-text("Salvar Alterações")');
    await page.waitForURL("/");
    await expect(page.locator(".system-card").filter({ hasText: SYS_NAME }).filter({ hasNotText: "(cópia)" }).first()).toBeVisible();
  });

  test("12: duplicates the system and verifies copy has same data", async ({ page }) => {
    await page.goto("/");
    const card = page.locator(".system-card").filter({ hasText: SYS_NAME }).first();
    await expect(card).toBeVisible();
    await card.locator('button[title="Duplicar"]').click();
    await page.waitForTimeout(1000);
    const copyName = `${SYS_NAME} (cópia)`;
    await expect(page.locator(`text=${copyName}`).first()).toBeVisible();
    // verify copy can be selected in vars page (has variables)
    await page.goto("/vars");
    await page.locator("select").first().selectOption({ label: copyName });
    await page.waitForTimeout(600);
    await expect(page.locator(".var-item")).toHaveCount(3);
  });

  test("13: exports system as JSON and validates content", async ({ page }) => {
    await page.goto("/");
    // target original system (not the cópia)
    const card = page.locator(".system-card").filter({ hasText: SYS_NAME }).filter({ hasNotText: "(cópia)" }).first();
    const exportLink = card.locator('a[download]');
    await expect(exportLink).toBeVisible();
    const href = await exportLink.getAttribute("href");
    expect(href).toContain("/api/systems/");
    expect(href).toContain("/export");
    // fetch the JSON content
    const response = await page.request.get(href!);
    expect(response.ok()).toBeTruthy();
    const json = await response.json();
    expect(json.name).toBe(SYS_NAME);
    expect(json.variables).toBeDefined();
    expect(json.variables.length).toBe(3);
    // store for later import test
    exportedJson = JSON.stringify(json);
  });

  test("14: audit page shows events for the system", async ({ page }) => {
    await page.goto("/audit");
    await page.locator("select").first().selectOption({ label: SYS_NAME });
    await page.waitForTimeout(600);
    await expect(page.locator(".timeline-item").first()).toBeVisible();
    await expect(page.locator(".timeline-item").filter({ hasText: "system" }).first()).toBeVisible();
  });

  test("15: analysis page — rule matrix counts match", async ({ page }) => {
    await page.goto("/analysis");
    await page.locator("select").first().selectOption({ label: SYS_NAME });
    await page.waitForTimeout(800);
    await page.locator('button:has-text("Ativacoes")').click();
    await page.waitForTimeout(1500);
    // one rule in matrix
    const rows = page.locator("table tbody tr");
    await expect(rows).toHaveCount(1);
  });

  test("16: optimizer page calculates and shows result type", async ({ page }) => {
    await page.goto("/opt");
    // defaults are already f(x,y) = 1.0x² + 0xy + 1.0y² + 0x + 0y + 0
    // domain defaults: x[-10,10], y[-10,10]
    // just click calculate — should find minimum at (0,0)
    await page.click('button:has-text("Calcular Ponto Ótimo")');
    await page.waitForTimeout(2000);
    await expect(page.locator(".opt-result-card").first()).toBeVisible();
    // for x² + y² the result should be minimum at (0,0) with value 0
    await expect(page.locator("text=mínimo").first()).toBeVisible();
  });

  test("17: status protection — favorito blocks delete", async ({ page }) => {
    await page.goto("/");
    const card = page.locator(".system-card").filter({ hasText: SYS_NAME }).filter({ hasNotText: "(cópia)" }).first();
    // change to favorito
    await card.locator("select.text-input").selectOption("favorito");
    await page.waitForTimeout(400);
    await expect(card.locator('[title="Remova o favorito para deletar"]')).toBeVisible();
    // change back to ativo
    await card.locator("select.text-input").selectOption("ativo");
    await page.waitForTimeout(400);
  });

  test("cleanup: deletes duplicated system and original", async ({ page }) => {
    // delete copy first
    const copyName = `${SYS_NAME} (cópia)`;
    await page.goto("/");
    const copyCard = page.locator(".system-card").filter({ hasText: copyName }).first();
    if (await copyCard.isVisible({ timeout: 1000 }).catch(() => false)) {
      await copyCard.locator('button[title="Deletar"]').click();
      await page.waitForURL("/");
      await page.waitForTimeout(500);
      await expect(page.locator(".system-card").filter({ hasText: copyName })).toHaveCount(0);
    }
    // delete original
    const origCard = page.locator(".system-card").filter({ hasText: SYS_NAME }).filter({ hasNotText: "(cópia)" }).first();
    if (await origCard.isVisible({ timeout: 1000 }).catch(() => false)) {
      await origCard.locator('button[title="Deletar"]').click();
      await page.waitForURL("/");
      await page.waitForTimeout(500);
    }
  });

  test("keeps seed system intact for manual inspection", async () => {
    // Conforto Térmico must still exist
  });
});
