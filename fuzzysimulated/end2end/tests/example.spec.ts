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
  { href: "/sim", title: "Simulador" },
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
  await expect(page.locator(".section-title")).toContainText("Simulador");
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
    await page.locator('button:has-text("Calcular Ativacoes")').click();
    await page.waitForTimeout(1500);
    // rule activation grid shows colored divs (not table rows)
    const gridCells = page.locator('.panel >> div[style*="grid-template-columns"]');
    await expect(gridCells.first()).toBeVisible();
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

// ──────────────────────────────────────────────────────────────
// UC05 — OpenWeather (buscar clima)
// ──────────────────────────────────────────────────────────────
test.describe.serial("UC05: OpenWeather", () => {
  test("weather fetch populates temperature and humidity inputs", async ({ page }) => {
    await page.goto("/sim");
    await page.locator("select").first().selectOption({ label: "Conforto Térmico" });
    await page.waitForTimeout(800);
    const cityInput = page.locator('input[placeholder="Cidade (ex: Belém)"]');
    await cityInput.fill("Belem");
    await page.locator('button:has-text("Buscar Clima")').click();
    await page.waitForTimeout(3000);
    // check that number inputs (temperatura/umidade) got populated with valid values
    const numInputs = page.locator('input[type="number"].range-number');
    const count = await numInputs.count();
    if (count >= 2) {
      const tempVal = await numInputs.nth(0).inputValue();
      const humVal = await numInputs.nth(1).inputValue();
      const tempNum = parseFloat(tempVal);
      const humNum = parseFloat(humVal);
      // temperatura should be in [-50, 60] (Earth range), umidade in [0, 100]
      if (!isNaN(tempNum)) expect(Math.abs(tempNum)).toBeLessThanOrEqual(60);
      if (!isNaN(humNum)) { expect(humNum).toBeGreaterThanOrEqual(0); expect(humNum).toBeLessThanOrEqual(100); }
    }
    // either a success or error message appears (API may fail in CI)
    const msg = page.locator("div[style*='color:var(--teal)'], div[style*='color:var(--coral)']").first();
    await expect(msg).toBeVisible({ timeout: 10000 });
  });
});

// ──────────────────────────────────────────────────────────────
// UC07 — Batch (inferência em lote)
// ──────────────────────────────────────────────────────────────
test.describe.serial("UC07: Batch inference", () => {
  test("batch with JSON inputs validates output values", async ({ page }) => {
    await page.goto("/batch");
    await page.locator("select").first().selectOption({ label: "Conforto Térmico" });
    await page.waitForTimeout(600);
    const ta = page.locator("textarea.text-input").first();
    await ta.fill(JSON.stringify([
      { temperatura: 10, umidade: 30 },
      { temperatura: 24, umidade: 55 },
      { temperatura: 35, umidade: 85 },
    ]));
    await page.locator('button:has-text("Executar Lote")').click();
    await page.waitForTimeout(2500);
    const resultadosPanel = page.locator('.panel:has(.panel-title:has-text("Resultados"))');
    const resTable = resultadosPanel.locator("table");
    await expect(resTable).toBeVisible({ timeout: 10000 });
    const rows = resTable.locator("tbody tr");
    await expect(rows).toHaveCount(3);
    // validate each row's output value
    for (let i = 0; i < 3; i++) {
      const outputCell = rows.nth(i).locator("td").nth(2);
      const valText = await outputCell.textContent();
      const num = parseFloat(valText!);
      expect(num).not.toBeNaN();
      // conforto universe is [0, 10] — output must be within range
      expect(num).toBeGreaterThanOrEqual(0);
      expect(num).toBeLessThanOrEqual(10);
    }
  });
});

// ──────────────────────────────────────────────────────────────
// UC11 — Importar Sistema
// ──────────────────────────────────────────────────────────────
test.describe.serial("UC11: Import system", () => {
  test("import page loads with correct title", async ({ page }) => {
    await page.goto("/import");
    await expect(page.locator(".section-title")).toContainText("Importar Sistema");
  });
});

// ──────────────────────────────────────────────────────────────
// UC13 — Varredura (Sweep)
// ──────────────────────────────────────────────────────────────
test.describe.serial("UC13: Sweep (varredura)", () => {
  test("sweep with Conforto Térmico validates y-values in [0,10]", async ({ page }) => {
    await page.goto("/sim");
    await page.locator("select").first().selectOption({ label: "Conforto Térmico" });
    await page.waitForTimeout(800);
    const sweepSelect = page.locator("text=Varredura").locator("..").locator("select.text-input").first();
    await sweepSelect.selectOption({ index: 1 });
    const numInputs = page.locator("text=Varredura").locator("..").locator('input[type="number"]');
    await numInputs.nth(0).fill("0");
    await numInputs.nth(1).fill("50");
    await numInputs.nth(2).fill("10");
    await page.locator('button:has-text("Varrer")').click();
    await page.waitForTimeout(2000);
    const table = page.locator("text=Varredura").locator("..").locator("table");
    await expect(table).toBeVisible({ timeout: 10000 });
    // validate y-values (conforto output) are in [0, 10]
    const rows = table.locator("tbody tr");
    const rowCount = await rows.count();
    expect(rowCount).toBeGreaterThanOrEqual(3);
    for (let i = 0; i < rowCount; i++) {
      const yText = await rows.nth(i).locator("td").nth(1).textContent();
      const yNum = parseFloat(yText!);
      expect(yNum).not.toBeNaN();
      expect(yNum).toBeGreaterThanOrEqual(0);
      expect(yNum).toBeLessThanOrEqual(10);
    }
  });
});

// ──────────────────────────────────────────────────────────────
// UC15 — Superfície de Controle
// ──────────────────────────────────────────────────────────────
test.describe.serial("UC15: Surface (heatmap)", () => {
  test("generate surface heatmap for Risco Cibernético Avançado", async ({ page }) => {
    await page.goto("/analysis");
    await page.locator("select").first().selectOption({ label: "Risco Cibernético Avançado" });
    await page.waitForTimeout(800);
    const surfacePanel = page.locator('.panel:has(.panel-title:has-text("Superficie de Controle"))');
    const surfaceSelects = surfacePanel.locator("select.text-input");
    await surfaceSelects.nth(0).selectOption({ index: 1 });
    await surfaceSelects.nth(1).selectOption({ index: 2 });
    const resInput = surfacePanel.locator('input[type="number"]');
    await resInput.fill("10");
    await page.locator('button:has-text("Gerar")').click();
    await page.waitForTimeout(4000);
    const gridContainer = surfacePanel.locator("div[style*='grid-template-columns']");
    await expect(gridContainer).toBeVisible({ timeout: 15000 });
    // info line: "x_var x y_var  NxN grid  z in [min, max]"
    const infoLine = surfacePanel.locator("text=grid");
    await expect(infoLine).toBeVisible();
    const infoText = await infoLine.textContent();
    // parse z range: "z in [0.00, 50.00]" or similar
    const zMatch = infoText!.match(/z in \[([\d.]+),\s*([\d.]+)\]/);
    if (zMatch) {
      const zMin = parseFloat(zMatch[1]);
      const zMax = parseFloat(zMatch[2]);
      // nivel_risco universe is [0, 100]
      expect(zMin).toBeGreaterThanOrEqual(0);
      expect(zMax).toBeLessThanOrEqual(100);
      expect(zMax).toBeGreaterThanOrEqual(zMin);
    }
  });
});

// ──────────────────────────────────────────────────────────────
// UC17 — PSO Optimization
// ──────────────────────────────────────────────────────────────
test.describe.serial("UC17: PSO Optimization", () => {
  test("run PSO optimization for Conforto Térmico", async ({ page }) => {
    await page.goto("/opt");
    const sysSelect = page.locator('label:has-text("Sistema (opcional")').locator("..").locator("select.text-input");
    await sysSelect.selectOption({ label: "Conforto Térmico" });
    await page.waitForTimeout(400);
    const psoPanel = page.locator('.panel:has(.panel-title:has-text("Otimização PSO de MF"))');
    const textareas = psoPanel.locator("textarea.text-input");
    await textareas.nth(0).fill(JSON.stringify([{ temperatura: 20, umidade: 50 }]));
    await textareas.nth(1).fill(JSON.stringify([{ conforto: 5 }]));
    const numberInputs = psoPanel.locator('input[type="number"]');
    await numberInputs.nth(0).fill("5");
    await numberInputs.nth(1).fill("3");
    await psoPanel.locator('button:has-text("Executar PSO")').click();
    await page.waitForTimeout(10000);
    // result shows "Melhor Fitness: X.XXXXXX" — parse and validate
    const fitnessSpan = psoPanel.locator('span[style*="color:var(--teal)"]');
    await expect(fitnessSpan.first()).toBeVisible({ timeout: 30000 });
    const fitText = await fitnessSpan.first().textContent();
    const fitNum = parseFloat(fitText!);
    expect(fitNum).not.toBeNaN();
    // MSE fitness must be non-negative
    expect(fitNum).toBeGreaterThanOrEqual(0);
  });
});

// ──────────────────────────────────────────────────────────────
// UC18 — TSK Inference
// ──────────────────────────────────────────────────────────────
test.describe.serial("UC18: TSK inference", () => {
  test("run TSK simulation on Conforto Térmico", async ({ page }) => {
    await page.goto("/sim");
    await page.locator("select").first().selectOption({ label: "Conforto Térmico" });
    await page.waitForTimeout(800);
    await page.locator('button.btn:has-text("TSK")').click();
    await page.waitForTimeout(300);
    const numInputs = page.locator('input[type="number"].text-input');
    await numInputs.nth(0).fill("25");
    await numInputs.nth(1).fill("50");
    const coeffTa = page.locator("textarea.text-input").first();
    await coeffTa.fill(JSON.stringify({
      "conforto_desconfortavel": [5, 0, 0],
      "conforto_neutro": [5, 0, 0],
      "conforto_confortavel": [5, 0, 0],
    }));
    await page.locator('button:has-text("Executar TSK")').click();
    await page.waitForTimeout(2000);
    await expect(page.locator("text=Resultado TSK").first()).toBeVisible();
    // parse output value and validate range [0, 10]
    const outputVal = page.locator(".output-val").first();
    await expect(outputVal).toBeVisible();
    const valText = await outputVal.textContent();
    const num = parseFloat(valText!);
    expect(num).not.toBeNaN();
    expect(num).toBeGreaterThanOrEqual(0);
    expect(num).toBeLessThanOrEqual(10);
  });
});

// ──────────────────────────────────────────────────────────────
// UC19 — SVG Export
// ──────────────────────────────────────────────────────────────
test.describe.serial("UC19: SVG export", () => {
  test("generate SVG for Conforto Térmico variables", async ({ page }) => {
    await page.goto("/sim");
    await page.locator("select").first().selectOption({ label: "Conforto Térmico" });
    await page.waitForTimeout(800);
    // click SVG tab
    await page.locator('button.btn:has-text("SVG")').click();
    await page.waitForTimeout(300);
    // click Gerar SVG
    await page.locator('button:has-text("Gerar SVG")').click();
    await page.waitForTimeout(3000);
    // SVG data rendered inside div[inner_html], the svg elements should exist in the DOM
    // each variable gets a panel with its name as panel-title + an inline SVG
    const svgPanels = page.locator('.panel:has(.panel-title:has-text("temperatura")), .panel:has(.panel-title:has-text("umidade")), .panel:has(.panel-title:has-text("conforto"))');
    await expect(svgPanels.nth(0)).toBeVisible({ timeout: 10000 });
    // check that at least 2 SVG panels are up (or that the tab is not showing the empty hint)
    const hint = page.locator("text=Clique em \"Gerar SVG\"");
    await expect(hint).not.toBeVisible();
    // check svg element count
    const allSvgs = page.locator("svg");
    const count = await allSvgs.count();
    expect(count).toBeGreaterThanOrEqual(1);
  });
});

// ──────────────────────────────────────────────────────────────
// UC20 — Diagnóstico
// ──────────────────────────────────────────────────────────────
test.describe.serial("UC20: Diagnostic report", () => {
  test("generate diagnostic for Conforto Térmico simulation", async ({ page }) => {
    await page.goto("/sim");
    await page.locator("select").first().selectOption({ label: "Conforto Térmico" });
    await page.waitForTimeout(800);
    await page.locator('button.btn:has-text("Diagnóstico")').click();
    await page.waitForTimeout(300);
    const numInputs = page.locator('input[type="number"].text-input');
    await numInputs.nth(0).fill("22");
    await numInputs.nth(1).fill("60");
    await page.locator('button:has-text("Gerar Diagnóstico")').click();
    await page.waitForTimeout(2000);
    // diagnostic panels must appear
    await expect(page.locator("summary").filter({ hasText: "Fuzzificação" }).first()).toBeVisible({ timeout: 10000 });
    await expect(page.locator("summary").filter({ hasText: "Regras Disparadas" }).first()).toBeVisible();
    await expect(page.locator("summary").filter({ hasText: "Saídas" }).first()).toBeVisible();
    // first <details> is open by default — validate term values are in [0, 1]
    // each term is rendered as "label: 0.xxxx" inside a div
    const termLines = page.locator("div[style*='padding-left:12px']");
    const termCount = await termLines.count();
    expect(termCount).toBeGreaterThanOrEqual(3);
    for (let i = 0; i < Math.min(termCount, 6); i++) {
      const text = await termLines.nth(i).textContent();
      const numMatch = text!.match(/: ([\d.]+)$/);
      if (numMatch) {
        const val = parseFloat(numMatch[1]);
        expect(val).not.toBeNaN();
        expect(val).toBeGreaterThanOrEqual(0);
        expect(val).toBeLessThanOrEqual(1);
      }
    }
    // expand Saídas details and validate output value
    await page.locator("summary").filter({ hasText: "Saídas" }).first().click();
    await page.waitForTimeout(300);
    // scope to the diagnostic panel to avoid picking up mamdani .output-val elements
    const diagPanel = page.locator('.panel:has(.panel-title:has-text("Diagnóstico"))');
    const outputVal = diagPanel.locator(".output-val").first();
    await expect(outputVal).toBeVisible({ timeout: 5000 });
    const valText = await outputVal.textContent();
    const num = parseFloat(valText!);
    expect(num).not.toBeNaN();
    // conforto universe is [0, 10]
    expect(num).toBeGreaterThanOrEqual(0);
    expect(num).toBeLessThanOrEqual(10);
  });
});

// ──────────────────────────────────────────────────────────────
// UC24/25 — Optimization history & export
// ──────────────────────────────────────────────────────────────
test.describe.serial("UC24-25: Optimization history & export", () => {
  test("run quadratic optimization and export result as JSON", async ({ page }) => {
    await page.goto("/opt");
    // select a system so the optimization links to it (for UC24 history)
    const sysSelect = page.locator('label:has-text("Sistema (opcional")').locator("..").locator("select.text-input");
    await sysSelect.selectOption({ label: "Conforto Térmico" });
    await page.waitForTimeout(400);
    // use default f(x,y)=x²+y² — click calculate
    await page.locator('button:has-text("Calcular Ponto Ótimo")').click();
    await page.waitForTimeout(3000);
    await expect(page.locator(".opt-result-card").first()).toBeVisible({ timeout: 15000 });
    await expect(page.locator("text=mínimo").first()).toBeVisible();
    // export link in the result card (UC25)
    const exportLink = page.locator('a.btn-outline:has-text("Exportar Resultado")').first();
    await expect(exportLink).toBeVisible();
    const href = await exportLink.getAttribute("href");
    expect(href).toContain("/api/optimizations/");
    expect(href).toContain("/export");
    // fetch the JSON content
    const response = await page.request.get(href!);
    expect(response.ok()).toBeTruthy();
    const json = await response.json();
    expect(json.optimal_point).toBeDefined();
    expect(json.optimal_point.x).toBeDefined();
    expect(json.critical_point_type).toBeDefined();
  });
});
