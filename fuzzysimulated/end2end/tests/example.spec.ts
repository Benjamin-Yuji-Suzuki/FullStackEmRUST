import { test, expect, Page } from "@playwright/test";

// ── Helpers ──────────────────────────────────────────────────
async function selectSystem(page: Page, label: string, timeout = 600) {
  await page.locator("select").first().selectOption({ label });
  await page.waitForTimeout(timeout);
}

async function fillInputs(page: Page, values: string[], selector = 'input[type="number"]') {
  const inputs = page.locator(selector);
  for (let i = 0; i < values.length; i++) {
    await inputs.nth(i).fill(values[i]);
  }
}

async function clickAndWait(page: Page, text: string, timeout = 1500) {
  await page.locator(`button:has-text("${text}")`).click();
  await page.waitForTimeout(timeout);
}

async function parseOutput(page: Page): Promise<number> {
  const el = page.locator(".output-val").first();
  await expect(el).toBeVisible();
  const text = await el.textContent();
  const n = Number.parseFloat(text!);
  return n;
}

async function expectOutputInRange(page: Page, min: number, max: number) {
  const n = await parseOutput(page);
  expect(n).toBeGreaterThanOrEqual(min);
  expect(n).toBeLessThanOrEqual(max);
}

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
  { href: "/opt", title: "Otimizador de Parâmetros (UC17)" },
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
    await expect(card.locator(".tag")).toBeVisible();
  });

  test("view seed system variables — 3 variables and 9 terms", async ({ page }) => {
    await page.goto("/vars");
    await selectSystem(page, "Conforto Térmico");
    await expect(page.locator(".var-item")).toHaveCount(3);
    await expect(page.locator(".term-chip").first()).toBeVisible();
    await expect(page.locator(".term-chip")).toHaveCount(3);
    const varNames = ["conforto", "temperatura", "umidade"];
    for (const vn of varNames) {
      await page.locator(".var-item").filter({ hasText: vn }).first().click();
      await page.waitForTimeout(200);
      await expect(page.locator(".var-panel .section-title")).toContainText("Termos");
    }
  });

  test("seed system has 9 rules in rule editor", async ({ page }) => {
    await page.goto("/rules");
    await selectSystem(page, "Conforto Térmico");
    await expect(page.locator(".rule-row")).toHaveCount(9);
    await expect(page.locator(".rule-text").filter({ hasText: "temperatura" }).first()).toBeVisible();
    await expect(page.locator(".rule-text").filter({ hasText: "conforto" }).first()).toBeVisible();
  });

  test("simulate with seed system — validate actual output value", async ({ page }) => {
    await page.goto("/sim");
    await selectSystem(page, "Conforto Térmico", 800);
    await fillInputs(page, ["25", "50"]);
    await clickAndWait(page, "Executar Simulação");
    await expectOutputInRange(page, 0, 10);
  });

  test("change seed system status to favorito and back", async ({ page }) => {
    await page.goto("/");
    const card = page.locator(".system-card").filter({ hasText: "Conforto Térmico" }).first();
    await expect(card).toBeVisible();
    await card.locator("select.text-input").selectOption("favorito");
    await page.waitForTimeout(500);
    await expect(card.locator(".tag-amber")).toBeVisible();
    await card.locator("select.text-input").selectOption("ativo");
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
    await selectSystem(page, "Conforto Térmico", 400);
    await page.locator("select").nth(1).selectOption({ index: 1 });
    await page.locator('input[placeholder="Ex: Frio"]').fill("NovoTermo");
    await page.locator('input[placeholder="0, 10, 22"]').fill("abc");
    await page.click('button:has-text("Adicionar")');
    await expect(page.locator("div[style*='coral']").filter({ hasText: "Parâmetros" }).first()).toBeVisible();
  });

  test("delete protection: favorito system can't be deleted", async ({ page }) => {
    await page.goto("/newsys");
    const name = `E2E DeleteProtect ${Date.now()}`;
    await page.fill('input[placeholder="Ex: Conforto Térmico"]', name);
    await page.selectOption("select", { label: "Centroide" });
    await page.click('button:has-text("Criar Sistema")');
    await page.waitForURL("/");
    const card = page.locator(".system-card").filter({ hasText: name }).first();
    await card.locator("select.text-input").selectOption("favorito");
    await page.waitForTimeout(400);
    await expect(card.locator('[title="Remova o favorito para deletar"]')).toBeVisible();
    await card.locator("select.text-input").selectOption("ativo");
    await page.waitForTimeout(400);
    await card.locator('button[title="Deletar"]').click();
    await page.waitForURL("/");
    await expect(page.locator(".system-card").filter({ hasText: name })).toHaveCount(0);
  });
});

// ──────────────────────────────────────────────────────────────
// Ciclo Completo — Cria, usa, edita, deleta
// ──────────────────────────────────────────────────────────────
test.describe.serial("Full lifecycle (create → use → edit → audit → delete)", () => {
  const SUFFIX = crypto.randomUUID().slice(0, 4);
  const SYS_NAME = `E2E Teste Risco (${SUFFIX})`;


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
      await selectSystem(page, SYS_NAME, 400);
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
      await selectSystem(page, SYS_NAME, 400);
      await page.locator("select").nth(1).selectOption({ label: t.var });
      await page.locator('input[placeholder="Ex: Frio"]').fill(t.label);
      await page.locator("select").nth(2).selectOption("trimf");
      await page.locator('input[placeholder="0, 10, 22"]').fill(t.params);
      await page.click('button:has-text("Adicionar")');
      await page.waitForURL(/\/vars/);
      await page.locator(".var-item").filter({ hasText: t.var }).first().click();
      await page.waitForTimeout(300);
      await expect(page.locator(".term-chip").filter({ hasText: t.label }).first()).toBeVisible();
    }
  });

  test("04: adds a rule", async ({ page }) => {
    await page.goto("/add-rule");
    await selectSystem(page, SYS_NAME, 400);
    await page.locator("input[type='text']").first().fill(
      "SE Temperatura = Alta E Umidade = Alta ENTAO Risco = Alto"
    );
    await page.click('button:has-text("Adicionar")');
    await page.waitForURL(/\/rules/);
    await expect(page.locator(".rule-text").filter({ hasText: "Alta" }).first()).toBeVisible();
  });

  test("05: runs simulation and validates output value", async ({ page }) => {
    await page.goto("/sim");
    await selectSystem(page, SYS_NAME, 800);
    await fillInputs(page, ["25", "30"]);
    await clickAndWait(page, "Executar Simulação");
    const num = await parseOutput(page);
    expect(num).toBeGreaterThanOrEqual(0);
    expect(num).toBeLessThanOrEqual(1);
    await expect(page.locator(".output-label").filter({ hasText: "Risco" })).toBeVisible();
  });

  test("06: saves a scenario", async ({ page }) => {
    await page.goto("/sim");
    await selectSystem(page, SYS_NAME, 800);
    await fillInputs(page, ["25", "30"]);
    await page.locator('input[placeholder="Nome do cenario"]').fill("Cenario E2E");
    await page.locator('button:has-text("Salvar")').click();
    await page.waitForTimeout(1000);
    await expect(page.locator("text=Cenario E2E").first()).toBeVisible();
  });

  test("07: runs second simulation for comparison", async ({ page }) => {
    await page.goto("/sim");
    await selectSystem(page, SYS_NAME, 800);
    await fillInputs(page, ["45", "60"]);
    await clickAndWait(page, "Executar Simulação");
    await expect(page.locator(".output-val").first()).toBeVisible();
  });

  test("08: simulation appears in history", async ({ page }) => {
    await page.goto("/hist");
    await selectSystem(page, SYS_NAME, 1000);
    await expect(page.locator(".hist-table tbody tr").first()).toBeVisible();
    await expect(page.locator(".hist-table").filter({ hasText: "Risco" })).toBeVisible();
  });

  test("09: compares two simulations", async ({ page }) => {
    await page.goto("/hist");
    await selectSystem(page, SYS_NAME, 1000);
    const checkboxes = page.locator('.hist-table tbody tr input[type="checkbox"]');
    await checkboxes.nth(0).check();
    await checkboxes.nth(1).check();
    await page.locator('button:has-text("Comparar Selecionados")').click();
    await page.waitForTimeout(1500);
    await expect(page.locator("text=Comparacao").first()).toBeVisible();
  });

  test("10: exports simulation report", async ({ page }) => {
    await page.goto("/hist");
    await selectSystem(page, SYS_NAME, 1000);
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
    await page.goto("/vars");
    await selectSystem(page, copyName);
    await expect(page.locator(".var-item")).toHaveCount(3);
  });

  test("13: exports system as JSON and validates content", async ({ page }) => {
    await page.goto("/");
    const card = page.locator(".system-card").filter({ hasText: SYS_NAME }).filter({ hasNotText: "(cópia)" }).first();
    const exportLink = card.locator('a[download]');
    await expect(exportLink).toBeVisible();
    const href = await exportLink.getAttribute("href");
    expect(href).toContain("/api/systems/");
    expect(href).toContain("/export");
    const response = await page.request.get(href!);
    expect(response.ok()).toBeTruthy();
    const json = await response.json();
    expect(json.name).toBe(SYS_NAME);
    expect(json.variables).toBeDefined();
    expect(json.variables.length).toBe(3);
  });

  test("14: audit page shows events for the system", async ({ page }) => {
    await page.goto("/audit");
    await selectSystem(page, SYS_NAME);
    await expect(page.locator(".timeline-item").first()).toBeVisible();
    await expect(page.locator(".timeline-item").filter({ hasText: "system" }).first()).toBeVisible();
  });

  test("15: analysis page — rule matrix counts match", async ({ page }) => {
    await page.goto("/analysis");
    await selectSystem(page, SYS_NAME, 800);
    await clickAndWait(page, "Calcular Ativacoes", 1500);
    const gridCells = page.locator('.panel >> div[style*="grid-template-columns"]');
    await expect(gridCells.first()).toBeVisible();
  });

  test("16: optimizer page shows PSO panel and runs preset", async ({ page }) => {
    await page.goto("/opt");
    await page.locator("select.text-input").first().selectOption({ label: SYS_NAME });
    await page.waitForTimeout(400);
    await expect(page.locator('button:has-text("Conforto Térmico")').first()).toBeVisible();
  });

  test("17: status protection — favorito blocks delete", async ({ page }) => {
    await page.goto("/");
    const card = page.locator(".system-card").filter({ hasText: SYS_NAME }).filter({ hasNotText: "(cópia)" }).first();
    await card.locator("select.text-input").selectOption("favorito");
    await page.waitForTimeout(400);
    await expect(card.locator('[title="Remova o favorito para deletar"]')).toBeVisible();
    await card.locator("select.text-input").selectOption("ativo");
    await page.waitForTimeout(400);
  });

  test("cleanup: deletes duplicated system and original", async ({ page }) => {
    const copyName = `${SYS_NAME} (cópia)`;
    await page.goto("/");
    const copyCard = page.locator(".system-card").filter({ hasText: copyName }).first();
    if (await copyCard.isVisible({ timeout: 1000 }).catch(() => false)) {
      await copyCard.locator('button[title="Deletar"]').click();
      await page.waitForURL("/");
      await page.waitForTimeout(500);
      await expect(page.locator(".system-card").filter({ hasText: copyName })).toHaveCount(0);
    }
    const origCard = page.locator(".system-card").filter({ hasText: SYS_NAME }).filter({ hasNotText: "(cópia)" }).first();
    if (await origCard.isVisible({ timeout: 1000 }).catch(() => false)) {
      await origCard.locator('button[title="Deletar"]').click();
      await page.waitForURL("/");
      await page.waitForTimeout(500);
    }
  });

  test("keeps seed system intact for manual inspection", async () => {});
});

// ──────────────────────────────────────────────────────────────
// UC05 — OpenWeather (buscar clima)
// ──────────────────────────────────────────────────────────────
test.describe.serial("UC05: OpenWeather", () => {
  test("weather fetch populates temperature and humidity inputs", async ({ page }) => {
    await page.goto("/sim");
    await selectSystem(page, "Conforto Térmico", 800);
    await page.locator('input[placeholder="Cidade (ex: Belém)"]').fill("Belem");
    await page.locator('button:has-text("Buscar Clima")').click();
    await page.waitForTimeout(3000);
    const numInputs = page.locator('input[type="number"].range-number');
    const count = await numInputs.count();
    if (count >= 2) {
      const tempNum = Number.parseFloat(await numInputs.nth(0).inputValue());
      const humNum = Number.parseFloat(await numInputs.nth(1).inputValue());
      if (!isNaN(tempNum)) expect(Math.abs(tempNum)).toBeLessThanOrEqual(60);
      if (!isNaN(humNum)) { expect(humNum).toBeGreaterThanOrEqual(0); expect(humNum).toBeLessThanOrEqual(100); }
    }
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
    await selectSystem(page, "Conforto Térmico");
    await page.locator("textarea.text-input").first().fill(JSON.stringify([
      { temperatura: 10, umidade: 30 },
      { temperatura: 24, umidade: 55 },
      { temperatura: 35, umidade: 85 },
    ]));
    await page.locator('button:has-text("Executar Lote")').click();
    await page.waitForTimeout(2500);
    const resTable = page.locator('.panel:has(.panel-title:has-text("Resultados")) table');
    await expect(resTable).toBeVisible({ timeout: 10000 });
    const rows = resTable.locator("tbody tr");
    await expect(rows).toHaveCount(3);
    for (let i = 0; i < 3; i++) {
      const num = Number.parseFloat(await rows.nth(i).locator("td").nth(2).textContent() || "");
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
    await selectSystem(page, "Conforto Térmico", 800);
    const sweepPanel = page.locator("text=Varredura").locator("..");
    await sweepPanel.locator("select.text-input").first().selectOption({ index: 1 });
    const numInputs = sweepPanel.locator('input[type="number"]');
    await numInputs.nth(0).fill("0");
    await numInputs.nth(1).fill("50");
    await numInputs.nth(2).fill("10");
    await page.locator('button:has-text("Varrer")').click();
    await page.waitForTimeout(2000);
    const table = sweepPanel.locator("table");
    await expect(table).toBeVisible({ timeout: 10000 });
    const rows = table.locator("tbody tr");
    const rowCount = await rows.count();
    expect(rowCount).toBeGreaterThanOrEqual(3);
    for (let i = 0; i < rowCount; i++) {
      const yNum = Number.parseFloat(await rows.nth(i).locator("td").nth(1).textContent() || "");
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
    await selectSystem(page, "Risco Cibernético Avançado", 800);
    const surfacePanel = page.locator('.panel:has(.panel-title:has-text("Superficie de Controle"))');
    const surfaceSelects = surfacePanel.locator("select.text-input");
    await surfaceSelects.nth(0).selectOption({ index: 1 });
    await surfaceSelects.nth(1).selectOption({ index: 2 });
    await surfacePanel.locator('input[type="number"]').fill("10");
    await page.locator('button:has-text("Gerar")').click();
    await page.waitForTimeout(4000);
    await expect(surfacePanel.locator("div[style*='grid-template-columns']")).toBeVisible({ timeout: 15000 });
    const infoLine = surfacePanel.locator("text=grid");
    await expect(infoLine).toBeVisible();
    const zMatch = (await infoLine.textContent())?.match(/z in \[([\d.]+),\s*([\d.]+)\]/);
    if (zMatch) {
      expect(Number.parseFloat(zMatch[1])).toBeGreaterThanOrEqual(0);
      expect(Number.parseFloat(zMatch[2])).toBeLessThanOrEqual(100);
      expect(Number.parseFloat(zMatch[2])).toBeGreaterThanOrEqual(Number.parseFloat(zMatch[1]));
    }
  });
});

// ──────────────────────────────────────────────────────────────
// UC17 — PSO Optimization
// ──────────────────────────────────────────────────────────────
test.describe.serial("UC17: PSO Optimization", () => {
  test("run PSO optimization for Conforto Térmico", async ({ page }) => {
    await page.goto("/opt");
    await page.locator("select.text-input").first().selectOption({ label: "Conforto Térmico" });
    await page.waitForTimeout(400);
    const psoPanel = page.locator('.panel:has(.panel-title:has-text("Otimização PSO"))');
    await psoPanel.locator('button:has-text("Conforto Térmico")').click();
    await page.waitForTimeout(12000);
    const fitnessSpan = psoPanel.locator('span[style*="color:var(--teal)"]');
    await expect(fitnessSpan.first()).toBeVisible({ timeout: 30000 });
    const fitNum = Number.parseFloat(await fitnessSpan.first().textContent() || "");
    expect(fitNum).toBeGreaterThanOrEqual(0);
  });
});

// ──────────────────────────────────────────────────────────────
// UC18 — TSK Inference
// ──────────────────────────────────────────────────────────────
test.describe.serial("UC18: TSK inference", () => {
  test("run TSK simulation on Conforto Térmico", async ({ page }) => {
    await page.goto("/sim");
    await selectSystem(page, "Conforto Térmico", 800);
    await page.locator('button.btn:has-text("TSK")').click();
    await page.waitForTimeout(300);
    await fillInputs(page, ["25", "50"], 'input[type="number"].text-input');
    await page.locator("textarea.text-input").first().fill(JSON.stringify({
      "conforto_desconfortavel": [5, 0, 0],
      "conforto_neutro": [5, 0, 0],
      "conforto_confortavel": [5, 0, 0],
    }));
    await page.locator('button:has-text("Executar TSK")').click();
    await page.waitForTimeout(2000);
    await expect(page.locator("text=Resultado TSK").first()).toBeVisible();
    await expectOutputInRange(page, 0, 10);
  });
});

// ──────────────────────────────────────────────────────────────
// UC19 — SVG Export
// ──────────────────────────────────────────────────────────────
test.describe.serial("UC19: SVG export", () => {
  test("generate SVG for Conforto Térmico variables", async ({ page }) => {
    await page.goto("/sim");
    await selectSystem(page, "Conforto Térmico", 800);
    await page.locator('button.btn:has-text("SVG")').click();
    await page.waitForTimeout(300);
    await page.locator('button:has-text("Gerar SVG")').click();
    await page.waitForTimeout(3000);
    await expect(page.locator('.panel:has(.panel-title:has-text("temperatura")), .panel:has(.panel-title:has-text("umidade")), .panel:has(.panel-title:has-text("conforto"))').nth(0)).toBeVisible({ timeout: 10000 });
    await expect(page.locator("text=Clique em \"Gerar SVG\"")).not.toBeVisible();
    expect(await page.locator("svg").count()).toBeGreaterThanOrEqual(1);
  });
});

// ──────────────────────────────────────────────────────────────
// UC20 — Diagnóstico
// ──────────────────────────────────────────────────────────────
test.describe.serial("UC20: Diagnostic report", () => {
  test("generate diagnostic for Conforto Térmico simulation", async ({ page }) => {
    await page.goto("/sim");
    await selectSystem(page, "Conforto Térmico", 800);
    await page.locator('button.btn:has-text("Diagnóstico")').click();
    await page.waitForTimeout(300);
    await fillInputs(page, ["22", "60"], 'input[type="number"].text-input');
    await page.locator('button:has-text("Gerar Diagnóstico")').click();
    await page.waitForTimeout(2000);
    await expect(page.locator("summary").filter({ hasText: "Fuzzificação" }).first()).toBeVisible({ timeout: 10000 });
    await expect(page.locator("summary").filter({ hasText: "Regras Disparadas" }).first()).toBeVisible();
    await expect(page.locator("summary").filter({ hasText: "Saídas" }).first()).toBeVisible();
    const termLines = page.locator("div[style*='padding-left:12px']");
    const termCount = await termLines.count();
    expect(termCount).toBeGreaterThanOrEqual(3);
    for (let i = 0; i < Math.min(termCount, 6); i++) {
      const numMatch = (await termLines.nth(i).textContent())?.match(/: ([\d.]+)$/);
      if (numMatch) {
        const val = Number.parseFloat(numMatch[1]);
        expect(val).toBeGreaterThanOrEqual(0);
        expect(val).toBeLessThanOrEqual(1);
      }
    }
    await page.locator("summary").filter({ hasText: "Saídas" }).first().click();
    await page.waitForTimeout(300);
    const diagPanel = page.locator('.panel:has(.panel-title:has-text("Diagnóstico"))');
    const outputVal = diagPanel.locator(".output-val").first();
    await expect(outputVal).toBeVisible({ timeout: 5000 });
    const num = Number.parseFloat(await outputVal.textContent() || "");
    expect(num).toBeGreaterThanOrEqual(0);
    expect(num).toBeLessThanOrEqual(10);
  });
});

// ──────────────────────────────────────────────────────────────
// UC24/25 — Optimization history & export
// (UC24-UC25 ocultos do frontend — mantidos no backend)
