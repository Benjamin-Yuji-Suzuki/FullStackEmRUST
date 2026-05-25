-- Seed: Sistema Risco Cibernetico (camada Prata -> Gold)
-- Variaveis interpretaveis para classificar impacto financeiro ALTO/BAIXO
-- Domain: ciberseguranca, baseado no schema do dataset_ml.parquet

DO $$
DECLARE
    v_sys_id UUID;
    v_receita_id UUID;
    v_func_id UUID;
    v_gravidade_id UUID;
    v_impacto_id UUID;
    sys_name   CONSTANT TEXT := 'Risco Cibernetico';
    mf_trapmf CONSTANT TEXT := 'trapmf';
    mf_trimf  CONSTANT TEXT := 'trimf';
    role_ant   CONSTANT TEXT := 'antecedent';
    role_con   CONSTANT TEXT := 'consequent';
    label_baixa CONSTANT TEXT := 'baixa';
    label_media CONSTANT TEXT := 'media';
    label_alta  CONSTANT TEXT := 'alta';
    label_baixo CONSTANT TEXT := 'baixo';
    label_medio CONSTANT TEXT := 'medio';
    label_alto  CONSTANT TEXT := 'alto';
    label_pequena CONSTANT TEXT := 'pequena';
    label_grande  CONSTANT TEXT := 'grande';
    univ_res   CONSTANT INT  := 501;
BEGIN
    IF EXISTS (SELECT 1 FROM fuzzy_systems WHERE name = sys_name) THEN
        RAISE NOTICE 'Sistema % ja existe', sys_name;
        RETURN;
    END IF;

    INSERT INTO fuzzy_systems (id, name, description, defuzz_method)
    VALUES (uuid_generate_v4(), sys_name,
        'Classifica incidentes de seguranca como ALTO ou BAIXO impacto financeiro (Prata -> Gold)',
        'centroid')
    RETURNING id INTO v_sys_id;

    -- Variaveis antecedentes (camada Prata)
    INSERT INTO fuzzy_variables (id, system_id, name, role, universe_min, universe_max, resolution)
    VALUES (uuid_generate_v4(), v_sys_id, 'receita_anual_usd', role_ant, 0, 1000000000, univ_res)
    RETURNING id INTO v_receita_id;

    INSERT INTO fuzzy_variables (id, system_id, name, role, universe_min, universe_max, resolution)
    VALUES (uuid_generate_v4(), v_sys_id, 'total_funcionarios', role_ant, 0, 500000, univ_res)
    RETURNING id INTO v_func_id;

    INSERT INTO fuzzy_variables (id, system_id, name, role, universe_min, universe_max, resolution)
    VALUES (uuid_generate_v4(), v_sys_id, 'gravidade_ataque', role_ant, 0, 100, univ_res)
    RETURNING id INTO v_gravidade_id;

    -- Consequente
    INSERT INTO fuzzy_variables (id, system_id, name, role, universe_min, universe_max, resolution)
    VALUES (uuid_generate_v4(), v_sys_id, 'impacto_financeiro', role_con, 0, 100, univ_res)
    RETURNING id INTO v_impacto_id;

    -- Termos: receita_anual_usd
    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_receita_id, label_baixa, mf_trapmf, '[0, 0, 50000000, 100000000]'::jsonb),
    (v_receita_id, label_media, mf_trimf,  '[50000000, 200000000, 500000000]'::jsonb),
    (v_receita_id, label_alta,  mf_trapmf, '[200000000, 500000000, 1000000000, 1000000000]'::jsonb);

    -- Termos: total_funcionarios
    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_func_id, label_pequena, mf_trapmf, '[0, 0, 5000, 20000]'::jsonb),
    (v_func_id, label_media,   mf_trimf,  '[5000, 50000, 150000]'::jsonb),
    (v_func_id, label_grande,  mf_trapmf, '[50000, 150000, 500000, 500000]'::jsonb);

    -- Termos: gravidade_ataque (mapeamento de attack_vector_primary)
    -- phishing=20, malware=40, dos=50, insider=60, data_breach=70, ransomware=85
    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_gravidade_id, label_baixa, mf_trapmf, '[0, 0, 20, 40]'::jsonb),
    (v_gravidade_id, label_media, mf_trimf,  '[20, 50, 70]'::jsonb),
    (v_gravidade_id, label_alta,  mf_trapmf, '[50, 70, 100, 100]'::jsonb);

    -- Termos: impacto_financeiro (consequente)
    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_impacto_id, label_baixo, mf_trapmf, '[0, 0, 30, 50]'::jsonb),
    (v_impacto_id, label_medio, mf_trimf,  '[30, 50, 70]'::jsonb),
    (v_impacto_id, label_alto,  mf_trapmf, '[50, 70, 100, 100]'::jsonb);

    -- Regras (3x3x3 = 27 combinacoes, usando as mais relevantes)
    -- Regra de negocio: receita alta + ataque severo = impacto alto
    --                empresa pequena + ataque severo = impacto alto (proporcionalmente)
    INSERT INTO fuzzy_rules (system_id, rule_text, weight, position) VALUES
    -- Receita BAIXA
    (v_sys_id, 'SE receita_anual_usd e baixa E total_funcionarios e pequena E gravidade_ataque e baixa ENTAO impacto_financeiro e baixo', 1.0, 0),
    (v_sys_id, 'SE receita_anual_usd e baixa E total_funcionarios e pequena E gravidade_ataque e alta ENTAO impacto_financeiro e medio', 1.0, 1),
    (v_sys_id, 'SE receita_anual_usd e baixa E total_funcionarios e grande E gravidade_ataque e alta ENTAO impacto_financeiro e alto', 1.0, 2),
    -- Receita MEDIA
    (v_sys_id, 'SE receita_anual_usd e media E total_funcionarios e media E gravidade_ataque e baixa ENTAO impacto_financeiro e baixo', 1.0, 3),
    (v_sys_id, 'SE receita_anual_usd e media E total_funcionarios e media E gravidade_ataque e media ENTAO impacto_financeiro e medio', 1.0, 4),
    (v_sys_id, 'SE receita_anual_usd e media E total_funcionarios e media E gravidade_ataque e alta ENTAO impacto_financeiro e alto', 1.0, 5),
    -- Receita ALTA
    (v_sys_id, 'SE receita_anual_usd e alta E total_funcionarios e grande E gravidade_ataque e baixa ENTAO impacto_financeiro e medio', 1.0, 6),
    (v_sys_id, 'SE receita_anual_usd e alta E total_funcionarios e grande E gravidade_ataque e media ENTAO impacto_financeiro e alto', 1.0, 7),
    (v_sys_id, 'SE receita_anual_usd e alta E total_funcionarios e grande E gravidade_ataque e alta ENTAO impacto_financeiro e alto', 1.0, 8);

    RAISE NOTICE 'Sistema Risco Cibernetico criado: 4 variaveis, 12 termos, 9 regras';
END $$;

-- Cenarios representativos (simulando dados tipicos da Prata)
DO $$
DECLARE
    v_sys_id UUID;
    sys_name CONSTANT TEXT := 'Risco Cibernetico';
BEGIN
    SELECT id INTO v_sys_id FROM fuzzy_systems WHERE name = sys_name;
    IF NOT FOUND THEN RETURN; END IF;

    DELETE FROM scenarios WHERE system_id = v_sys_id;

    INSERT INTO scenarios (system_id, name, inputs) VALUES
    -- impacto BAIXO esperado
    (v_sys_id, 'Startup phishing baixo impacto',    '{"receita_anual_usd": 1000000, "total_funcionarios": 50, "gravidade_ataque": 20}'),
    (v_sys_id, 'Media empresa ataque baixo',        '{"receita_anual_usd": 100000000, "total_funcionarios": 5000, "gravidade_ataque": 15}'),
    (v_sys_id, 'Grande empresa ataque minimo',      '{"receita_anual_usd": 800000000, "total_funcionarios": 200000, "gravidade_ataque": 10}'),
    -- impacto MEDIO esperado
    (v_sys_id, 'Startup ransomware impacto medio',  '{"receita_anual_usd": 5000000, "total_funcionarios": 100, "gravidade_ataque": 85}'),
    (v_sys_id, 'Media empresa malware moderado',    '{"receita_anual_usd": 200000000, "total_funcionarios": 40000, "gravidade_ataque": 45}'),
    (v_sys_id, 'Grande empresa phishing velado',    '{"receita_anual_usd": 500000000, "total_funcionarios": 100000, "gravidade_ataque": 25}'),
    -- impacto ALTO esperado
    (v_sys_id, 'Media empresa ransomware alto',     '{"receita_anual_usd": 150000000, "total_funcionarios": 30000, "gravidade_ataque": 90}'),
    (v_sys_id, 'Grande empresa data breach',        '{"receita_anual_usd": 900000000, "total_funcionarios": 250000, "gravidade_ataque": 75}'),
    (v_sys_id, 'Corp ransomware maximo impacto',    '{"receita_anual_usd": 1000000000, "total_funcionarios": 400000, "gravidade_ataque": 95}');
END $$;
