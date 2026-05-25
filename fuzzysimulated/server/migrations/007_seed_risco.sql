-- Seed: Sistema Analise de Risco (segundo sistema de demonstracao)
-- Soh insere se o sistema nao existir (evita duplicacao em re-aplicacao)

DO $$
DECLARE
    v_sys_id UUID;
    v_prob_id UUID;
    v_imp_id UUID;
    v_risco_id UUID;
    mf_trapmf CONSTANT TEXT := 'trapmf';
    mf_trimf  CONSTANT TEXT := 'trimf';
    p_0_0_30_50    CONSTANT TEXT := '[0, 0, 30, 50]';
    p_30_50_70     CONSTANT TEXT := '[30, 50, 70]';
    p_50_70_100_100 CONSTANT TEXT := '[50, 70, 100, 100]';
BEGIN
    IF EXISTS (SELECT 1 FROM fuzzy_systems WHERE name = 'Analise de Risco') THEN
        RAISE NOTICE 'Sistema Analise de Risco ja existe — ignorado';
        RETURN;
    END IF;

    INSERT INTO fuzzy_systems (id, name, description, defuzz_method)
    VALUES (uuid_generate_v4(), 'Analise de Risco',
        'Sistema para avaliacao de risco de seguranca baseado em probabilidade e impacto', 'centroid')
    RETURNING id INTO v_sys_id;

    INSERT INTO fuzzy_variables (id, system_id, name, role, universe_min, universe_max, resolution)
    VALUES (uuid_generate_v4(), v_sys_id, 'probabilidade', 'antecedent', 0, 100, 501)
    RETURNING id INTO v_prob_id;

    INSERT INTO fuzzy_variables (id, system_id, name, role, universe_min, universe_max, resolution)
    VALUES (uuid_generate_v4(), v_sys_id, 'impacto', 'antecedent', 0, 100, 501)
    RETURNING id INTO v_imp_id;

    INSERT INTO fuzzy_variables (id, system_id, name, role, universe_min, universe_max, resolution)
    VALUES (uuid_generate_v4(), v_sys_id, 'risco', 'consequent', 0, 100, 501)
    RETURNING id INTO v_risco_id;

    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_prob_id, 'baixa',  mf_trapmf, p_0_0_30_50),
    (v_prob_id, 'media',  mf_trimf,  p_30_50_70),
    (v_prob_id, 'alta',   mf_trapmf, p_50_70_100_100);

    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_imp_id, 'baixo',  mf_trapmf, p_0_0_30_50),
    (v_imp_id, 'medio',  mf_trimf,  p_30_50_70),
    (v_imp_id, 'alto',   mf_trapmf, p_50_70_100_100);

    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_risco_id, 'baixo',    mf_trapmf, '[0, 0, 25, 45]'),
    (v_risco_id, 'moderado', mf_trimf,  p_30_50_70),
    (v_risco_id, 'critico',  mf_trapmf, '[55, 75, 100, 100]');

    INSERT INTO fuzzy_rules (system_id, rule_text, weight, position) VALUES
    (v_sys_id, 'SE probabilidade e baixa E impacto e baixo ENTAO risco e baixo', 1.0, 0),
    (v_sys_id, 'SE probabilidade e baixa E impacto e medio ENTAO risco e baixo', 1.0, 1),
    (v_sys_id, 'SE probabilidade e baixa E impacto e alto ENTAO risco e moderado', 1.0, 2),
    (v_sys_id, 'SE probabilidade e media E impacto e baixo ENTAO risco e baixo', 1.0, 3),
    (v_sys_id, 'SE probabilidade e media E impacto e medio ENTAO risco e moderado', 1.0, 4),
    (v_sys_id, 'SE probabilidade e media E impacto e alto ENTAO risco e critico', 1.0, 5),
    (v_sys_id, 'SE probabilidade e alta E impacto e baixo ENTAO risco e moderado', 1.0, 6),
    (v_sys_id, 'SE probabilidade e alta E impacto e medio ENTAO risco e critico', 1.0, 7),
    (v_sys_id, 'SE probabilidade e alta E impacto e alto ENTAO risco e critico', 1.0, 8);

    RAISE NOTICE 'Seed Analise de Risco inserido — 3 variaveis, 9 termos, 9 regras';
END $$;
