# UI/UX Pro Max — Skill para Leptos + Axum

Skill de design intelligence para construir UI/UX profissionais com **Leptos** (frontend Rust/WASM) e **Axum** (backend Rust).

Baseada no [ui-ux-pro-max-skill](https://github.com/nextlevelbuilder/ui-ux-pro-max-skill) — 67 estilos, 161 paletas, 57 fontes, 99 guidelines de UX, 25 tipos de gráfico.

---

## Como Usar Esta Skill

Ative quando o usuário pedir qualquer trabalho de UI/UX.

| Cenário | O que fazer |
|---------|-------------|
| **Novo projeto / página** | Gerar design system completo (Passo 1 → 2) |
| **Novo componente** | Busca de domínio: style, ux |
| **Escolher estilo / cor / fonte** | Gerar design system |
| **Revisar UI existente** | Checklist de pré-entrega |
| **Otimizar / melhorar** | Busca de domínio: ux, leptos |
| **Adicionar gráficos** | Busca de domínio: chart |

---

## Aplicação no FuzzySimulated (04/06/2026)

Tema **Rust** aplicado ao FuzzySimulated — paleta laranja ferrugem (#DE4838), preto industrial, engrenagem como logo.

### Mudanças realizadas:

| Arquivo | O que foi feito |
|---------|----------------|
| `style/main.scss` | Reescrevido com design system Rust: cores, gradientes, glassmorphism, animações, light mode, responsivo, utilidades |
| `app/src/lib.rs` | ~60 inline styles → classes utilitárias; `amber` → `rust` em classes e variáveis; logo `⬡` → `⚙` |
| `Cargo.toml` | wasm-bindgen atualizado 0.2.121 → 0.2.122 |

### Paleta Rust:

```
--rust:       #DE4838   (primário)
--rust-dark:  #B83220   (hover/active)
--rust-dim:   #E87A5A   (glow/variante)
--orange:     #FF8C42   (gradiente accent)
--amber-warm: #FFB347   (gradiente secundário)
--surface:    #0D0D0D   (fundo principal)
--surface2:   #161616   (cards/sidebar)
--surface3:   #1E1E1E   (inputs)
```

### Características implementadas:
- Gradientes lineares nos botões e sidebar
- Barra gradiente no topo dos cards ao hover
- Glow effect (drop-shadow + box-shadow) nos elementos primários
- `backdrop-filter: blur(6px)` no modal
- Keyframes: `fadeIn`, `pulse-glow`, `gear-spin`
- Stagger animation em listas de cards
- Scrollbar customizada
- `prefers-reduced-motion` respeitado
- Light mode automático via `prefers-color-scheme`
- Responsivo com 4 breakpoints (1100px, 900px, 768px, 480px)
- Sidebar colapsa para 56px em mobile
- 40+ classes utilitárias CSS para reduzir inline styles

---

## Passo 1: Analisar Requisitos

Extrair do pedido do usuário:
- **Tipo de produto**: SaaS, E-commerce, Fintech, Portfólio, Saúde, etc.
- **Público-alvo**: B2B, B2C, idade, contexto de uso
- **Keywords de estilo**: minimalista, vibrante, dark mode, luxuoso, divertido
- **Stack**: Leptos (frontend) + Axum (backend)

---

## Passo 2: Gerar Design System (OBRIGATÓRIO)

Sempre começar gerando um design system completo. Use a tabela abaixo para selecionar com base no tipo de produto.

### 161 Categorias de Produto → Estilo Principal

| Categoria | Estilo Principal | Cores Base | Fontes |
|-----------|-----------------|------------|--------|
| SaaS (General) | Glassmorphism + Flat Design | #2563EB, #F8FAFC, #1E293B | Inter / Inter |
| Micro SaaS | Flat Design + Vibrant & Block | #6366F1, #F5F3FF, #1E1B4B | Inter / Inter |
| E-commerce | Vibrant & Block-based | #059669, #ECFDF5, #064E3B | Poppins / Inter |
| E-commerce Luxury | Liquid Glass + Glassmorphism | #1C1917, #FAFAF9, #0C0A09 | Cormorant Garamond / Montserrat |
| B2B Service | Trust & Authority + Minimal | #0F172A, #F8FAFC, #020617 | Inter / Inter |
| Fintech/Crypto | Glassmorphism + Dark Mode (OLED) | #F59E0B, #0F172A, #F8FAFC | DM Sans / Inter |
| Healthcare App | Neumorphism + Accessible | #0891B2, #ECFEFF, #164E63 | Nunito / Inter |
| Educational App | Claymorphism + Micro-interactions | #4F46E5, #EEF2FF, #1E1B4B | Fredoka / Inter |
| Creative Agency | Brutalism + Motion-Driven | #EC4899, #FDF2F8, #831843 | Space Grotesk / Inter |
| Portfolio/Personal | Motion-Driven + Minimalism | #18181B, #FAFAFA, #09090B | Inter / Inter |
| Gaming | 3D + Retro-Futurism | #7C3AED, #0F0F23, #E2E8F0 | Orbitron / Inter |
| AI/Chatbot Platform | AI-Native UI + Minimalism | #7C3AED, #FAF5FF, #1E1B4B | Plus Jakarta Sans / Inter |
| Banking | Minimalism + Accessible | #0A1628, #FFFFFF, #1E293B | Inter / Inter |
| Dark Mode (OLED) | Dark Mode + Minimalism | #000000, #121212, #FFFFFF | Inter / Inter |
| Mental Health App | Neumorphism + Accessible | #8B5CF6, #FAF5FF, #4C1D95 | Quicksand / Inter |
| Restaurant/Food | Vibrant + Motion-Driven | #FF6B35, #FFF5F0, #1A1A1A | Playfair Display / Inter |

> Para produto não listado, usar **Minimalism + Flat Design** como fallback com **Inter / Inter** e paleta azul profissional.

### Paletas de Cor Completas (161 produtos)

Use a cor primária + esquema abaixo. Sempre incluir variantes:

```css
/* Tokens de cor CSS */
--color-primary:    #2563EB;
--color-on-primary: #FFFFFF;
--color-secondary:  #3B82F6;
--color-accent:     #EA580C;
--color-background: #F8FAFC;
--color-foreground: #1E293B;
--color-muted:      #E9EFF8;
--color-muted-fg:   #64748B;
--color-border:     #E2E8F0;
--color-destructive:#DC2626;
--color-ring:       #2563EB;
```

### Font Pairings (57 combinações)

| Mood | Heading | Body | Google Fonts URL |
|------|---------|------|------------------|
| Clean, Professional | Inter | Inter | `Inter:wght@400;500;600;700` |
| Elegant, Luxury | Cormorant Garamond | Montserrat | `Cormorant+Garamond:wght@400;600;700&family=Montserrat:wght@400;500;600` |
| Playful, Modern | Poppins | Inter | `Poppins:wght@400;600;700&family=Inter:wght@400;500` |
| Tech, Futuristic | Space Grotesk | DM Sans | `Space+Grotesk:wght@400;500;700&family=DM+Sans:wght@400;500` |
| Friendly, Casual | Nunito | Inter | `Nunito:wght@400;600;700&family=Inter:wght@400;500` |
| Editorial, Serif | Playfair Display | Source Sans | `Playfair+Display:wght@400;600;700&family=Source+Sans+3:wght@400;600` |
| Creative, Bold | Plus Jakarta Sans | Inter | `Plus+Jakarta+Sans:wght@400;500;700;800&family=Inter:wght@400;500` |
| Gaming, Display | Orbitron | Rajdhani | `Orbitron:wght@400;500;700;900&family=Rajdhani:wght@400;500;600;700` |

---

## Passo 3: Implementação Leptos + Axum

### Estrutura de Projeto Recomendada

```
src/
├── main.rs              # Entrypoint Axum + Leptos CSR/SSR
├── app.rs               # Componente raiz Leptos
├── components/          # Componentes reutilizáveis
│   ├── button.rs
│   ├── card.rs
│   ├── modal.rs
│   └── input.rs
├── pages/               # Páginas
│   ├── home.rs
│   ├── dashboard.rs
│   └── pricing.rs
├── styles/              # Tokens de design
│   ├── colors.rs
│   ├── typography.rs
│   └── spacing.rs
├── server/              # Lógica Axum
│   ├── routes.rs
│   ├── handlers.rs
│   └── models.rs
└── utils.rs
```

### Tokens de Design em Rust

```rust
// src/styles/colors.rs
pub struct Colors {
    pub primary: &'static str,
    pub secondary: &'static str,
    pub accent: &'static str,
    pub background: &'static str,
    pub foreground: &'static str,
    pub muted: &'static str,
    pub border: &'static str,
    pub destructive: &'static str,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            primary: "#2563EB",
            secondary: "#3B82F6",
            accent: "#EA580C",
            background: "#F8FAFC",
            foreground: "#1E293B",
            muted: "#E9EFF8",
            border: "#E2E8F0",
            destructive: "#DC2626",
        }
    }
}
```

### Componentes Leptos

```rust
// Botão primário
#[component]
pub fn Button(
    #[prop(default = "button")] r#type: &'static str,
    #[prop(default = "md")] size: &'static str,
    #[prop(default = false)] disabled: bool,
    children: Children,
) -> impl IntoView {
    let base = "inline-flex items-center justify-center font-medium rounded-lg transition-all duration-200 cursor-pointer";
    let size_class = match size {
        "sm" => "px-3 py-1.5 text-sm",
        "md" => "px-4 py-2 text-base",
        "lg" => "px-6 py-3 text-lg",
        _ => "px-4 py-2 text-base",
    };
    let disabled_class = if disabled { "opacity-50 cursor-not-allowed" } else { "hover:opacity-90 hover:-translate-y-0.5" };

    view! {
        <button
            type=r#type
            disabled=disabled
            class=format!("{} {} {} bg-[#2563EB] text-white", base, size_class, disabled_class)
        >
            {children()}
        </button>
    }
}

// Card
#[component]
pub fn Card(
    #[prop(default = false)] hoverable: bool,
    children: Children,
) -> impl IntoView {
    let hover = if hoverable { "hover:shadow-lg hover:-translate-y-1 transition-all duration-200" } else { "" };
    view! {
        <div class=format!("bg-white rounded-xl p-6 shadow-md {}", hover)>
            {children()}
        </div>
    }
}

// Input
#[component]
pub fn Input(
    #[prop(default = "text")] r#type: &'static str,
    #[prop(default = "")] placeholder: &'static str,
    value: RwSignal<String>,
) -> impl IntoView {
    view! {
        <input
            type=r#type
            placeholder=placeholder
            prop:value=value
            on:input=move |e| value.set(event_target_value(&e))
            class="w-full px-4 py-2.5 border border-[#E2E8F0] rounded-lg text-base transition-colors duration-200 focus:border-[#2563EB] focus:outline-none focus:ring-3 focus:ring-[#2563EB]/20"
        />
    }
}
```

### Exemplo de Página

```rust
// src/pages/home.rs
#[component]
pub fn HomePage() -> impl IntoView {
    let count = create_rw_signal(0);

    view! {
        <div class="min-h-screen bg-[#F8FAFC]">
            <header class="max-w-6xl mx-auto px-6 py-16 text-center">
                <h1 class="text-5xl font-bold text-[#1E293B] mb-4">
                    "Construa Algo Incrível"
                </h1>
                <p class="text-lg text-[#64748B] max-w-2xl mx-auto mb-8">
                    "UI/UX profissional com Leptos + Axum. Design system gerado por IA."
                </p>
                <div class="flex gap-4 justify-center">
                    <Button size="lg">
                        "Começar Agora"
                    </Button>
                    <Button size="lg">
                        "Saiba Mais"
                    </Button>
                </div>
            </header>

            <section class="max-w-6xl mx-auto px-6 py-12 grid grid-cols-1 md:grid-cols-3 gap-6">
                <Card hoverable=true>
                    <h3 class="text-xl font-semibold text-[#1E293B] mb-2">"Rápido"</h3>
                    <p class="text-[#64748B]">"Compilado para WASM, performance nativa."</p>
                </Card>
                <Card hoverable=true>
                    <h3 class="text-xl font-semibold text-[#1E293B] mb-2">"Seguro"</h3>
                    <p class="text-[#64748B]">"Tipagem forte do Rust, sem erros em runtime."</p>
                </Card>
                <Card hoverable=true>
                    <h3 class="text-xl font-semibold text-[#1E293B] mb-2">"Bonito"</h3>
                    <p class="text-[#64748B]">"Design system profissional com 67 estilos."</p>
                </Card>
            </section>
        </div>
    }
}
```

### Boas Práticas Leptos

| Categoria | Regra | Faça | Não Faça |
|-----------|-------|------|----------|
| **Signals** | Use `create_signal` para estado local | `let (count, set_count) = create_signal(0)` | Mutar variável diretamente |
| **Signals** | Use `create_rw_signal` para estado compartilhado | `let count = create_rw_signal(0)` | Prop drilling excessivo |
| **Signals** | Use `create_resource` para dados async | `create_resource(move || id, fetch_data)` | Fetch dentro do render |
| **Signals** | Use `create_effect` para side effects | `create_effect(move |_| log(count()))` | Efeitos aninhados |
| **Reactivity** | Prefira `.get()` / `.set()` explícitos | `count.set(42)` | Atribuição direta |
| **Reactivity** | Use `move |_|` em closures | `on:click=move |_| count.set(count() + 1)` | Closures sem `move` |
| **Components** | Use `#[component]` + `fn` | `#[component] fn Card() -> impl IntoView` | Struct-based components |
| **Components** | Use `Children` para composição | `children: Children` | Props de slot manuais |
| **Components** | Destructure props no argumento | `fn Button(#[prop(default)] size: &str)` | Props object genérico |
| **Derived signals** | Use `Signal::derive` para valores computados | `let doubled = Signal::derive(move || count() * 2)` | Memo desnecessário |
| **Forms** | Use `on:input` + `event_target_value` | `on:input=move |e| name.set(event_target_value(&e))` | Refs de DOM |
| **Forms** | Submit via `on:submit` com `prevent_default` | `on:submit=|ev| ev.prevent_default()` | `on:click` no botão submit |
| **Routing** | Use `leptos_router::A` para navegação | `<A href="/about">About</A>` | `<a>` cru sem roteamento |
| **Styling** | Use Tailwind CSS classes | `class="flex gap-4"` | Style objects inline |
| **Styling** | Defina cores como constantes | `const PRIMARY: &str = "#2563EB"` | Hex hardcoded espalhado |

### Boas Práticas Axum

| Categoria | Regra | Faça | Não Faça |
|-----------|-------|------|----------|
| **Routes** | Use Router com typed paths | `Router::new().route("/api/users", get(list_users))` | String matching manual |
| **State** | Compartilhe estado com `Extension` ou `State` | `.with_state(app_state)` | State global mutável |
| **JSON** | Use `Json<Model>` para request/response | `async fn create(Json(payload): Json<CreateUser>)` | Serialização manual |
| **Errors** | Use `Result<Json<T>, AppError>` | `impl IntoResponse for AppError` | `unwrap()` ou `expect()` |
| **Validation** | Use `serde` + `validator` crate | `#[derive(Deserialize, Validate)]` | Validar manualmente no handler |
| **Middleware** | Use tower layers para cors/auth/log | `.layer(TraceLayer::new_for_http())` | Middleware caseiro |
| **DB** | Use `sqlx` com connection pool | `PgPool::connect(&dotenv::var("DATABASE_URL"))` | Conexão por request |

---

## Regras Comuns para UI Profissional

### Ícones

- Use **Lucide** (SVG inline ou `lucide-leptos` crate). NUNCA use emoji como ícone estrutural.
- Ícones decorativos sem label → `aria-hidden="true"`
- Tamanho consistente: 16px (sm), 20px (md), 24px (lg)

```rust
// Exemplo: ícone SVG inline em Leptos
view! {
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
        <path d="M20 5H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2Z"/>
        <path d="M12 12v.01"/>
        <path d="M16 12v.01"/>
        <path d="M8 12v.01"/>
    </svg>
}
```

### Interação

| Regra | Faça | Não Faça |
|-------|------|----------|
| **Feedback de clique** | Transição de opacidade/elevação em 150-200ms | Sem resposta visual |
| **Timing de animação** | Micro-interações 150-300ms com easing suave | Transições instantâneas ou >500ms |
| **Touch targets** | Mínimo 44x44px (clicável + padding) | Ícones pequenos sem área expandida |
| **Disabled state** | Opacidade reduzida + `cursor-not-allowed` + sem ação | Elemento parece clicável mas não funciona |
| **Cursor** | `cursor-pointer` em todo elemento clicável | Cursor padrão em botões/links |
| **Focus** | `focus-visible:ring-3` em inputs e botões | Focus invisível para navegação por teclado |
| **Hover** | `hover:opacity-90` ou `hover:-translate-y-0.5` | Hover que desloca layout |
| **Reduced motion** | Respeitar `prefers-reduced-motion` | Animações sem media query |

### Light/Dark Mode

| Regra | Faça |
|-------|------|
| **Contraste** | Texto body ≥4.5:1, texto secundário ≥3:1 em ambos os modos |
| **Superfícies** | Cards claramente separados do fundo em light e dark |
| **Bordas** | Visíveis em ambos os temas |
| **Scrim modal** | 40-60% preto, suficiente para isolar conteúdo |
| **Tokens** | Use variáveis CSS semânticas, NUNCA hex hardcoded |

```css
:root {
  --bg-primary: #F8FAFC;
  --bg-card: #FFFFFF;
  --text-primary: #1E293B;
  --text-secondary: #64748B;
  --border: #E2E8F0;
}

[data-theme="dark"] {
  --bg-primary: #0F172A;
  --bg-card: #1E293B;
  --text-primary: #F8FAFC;
  --text-secondary: #94A3B8;
  --border: #334155;
}
```

### Layout

| Regra | Faça |
|-------|------|
| **Max-width** | 1200px para conteúdo, 1400px para dashboards |
| **Grid** | 12 colunas para flexibilidade |
| **Espaçamento** | 4/8dp rhythm: 4px, 8px, 16px, 24px, 32px, 48px, 64px |
| **Responsivo** | 375px, 768px, 1024px, 1440px |
| **Hero** | Padding vertical 64-96px, CTA acima da dobra |

---

## Checklist de Pré-Entrega

- [ ] Nenhum emoji usado como ícone (use SVG/Lucide)
- [ ] `cursor-pointer` em todo elemento clicável
- [ ] Hover states com transição suave (150-300ms)
- [ ] Light mode: contraste de texto ≥4.5:1
- [ ] Focus states visíveis para navegação por teclado
- [ ] `prefers-reduced-motion` respeitado
- [ ] Responsivo: 375px, 768px, 1024px, 1440px
- [ ] Nenhum conteúdo escondido atrás de navbar fixa
- [ ] Nenhum scroll horizontal em mobile
- [ ] Touch targets ≥44x44px
- [ ] Ícones consistentes (Lucide, mesmo stroke)
- [ ] Tokens de cor semânticos (sem hex hardcoded)
- [ ] Leptos: signals gerenciados corretamente (sem mutação direta)
- [ ] Leptos: `move` closures nos event handlers
- [ ] Axum: errors tratados com `Result<Json<T>, AppError>`
- [ ] Axum: sem `unwrap()` ou `expect()` em produção

---

## Anti-Patterns (NÃO Use)

- ❌ Emoji como ícone estrutural (🎨🚀⚙️)
- ❌ `cursor:pointer` faltando em elementos clicáveis
- ❌ Hover que desloca layout
- ❌ Texto com baixo contraste (<4.5:1)
- ❌ Transições instantâneas (sempre 150-300ms)
- ❌ Focus states invisíveis
- ❌ Cores hex hardcoded sem tokens
- ❌ Leptos: mutar signal sem `.set()`
- ❌ Leptos: `unwrap()` em signals opcionais
- ❌ Axum: `unwrap()` em handlers
- ❌ Axum: estado global mutável sem `Arc<RwLock<>>`
- ❌ Gradientes roxo/rosa "AI" em apps financeiros/bancários
- ❌ Neon brilhante em apps de saúde/bem-estar
