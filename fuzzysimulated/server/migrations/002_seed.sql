-- Seed data: Sistema Conforto Térmico para testes
-- Só insere se não existir nenhum sistema (evita duplicação)

DO $$
DECLARE
    v_sys_id UUID;
    v_temp_id UUID;
    v_umid_id UUID;
    v_conf_id UUID;
    mf_trapmf CONSTANT TEXT := 'trapmf';
    mf_trimf  CONSTANT TEXT := 'trimf';
BEGIN
    IF EXISTS (SELECT 1 FROM fuzzy_systems LIMIT 1) THEN
        RAISE NOTICE 'Sistemas já existem — seed ignorado';
        RETURN;
    END IF;

    -- Sistema
    INSERT INTO fuzzy_systems (id, name, description, defuzz_method)
    VALUES (uuid_generate_v4(), 'Conforto Térmico', 'Sistema para avaliação de conforto térmico baseado em temperatura e umidade', 'centroid')
    RETURNING id INTO v_sys_id;

    -- Variáveis
    INSERT INTO fuzzy_variables (id, system_id, name, role, universe_min, universe_max, resolution)
    VALUES (uuid_generate_v4(), v_sys_id, 'temperatura', 'antecedent', 0, 50, 501)
    RETURNING id INTO v_temp_id;

    INSERT INTO fuzzy_variables (id, system_id, name, role, universe_min, universe_max, resolution)
    VALUES (uuid_generate_v4(), v_sys_id, 'umidade', 'antecedent', 0, 100, 501)
    RETURNING id INTO v_umid_id;

    INSERT INTO fuzzy_variables (id, system_id, name, role, universe_min, universe_max, resolution)
    VALUES (uuid_generate_v4(), v_sys_id, 'conforto', 'consequent', 0, 10, 501)
    RETURNING id INTO v_conf_id;

    -- Termos: temperatura
    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_temp_id, 'frio',       mf_trapmf, '[0, 0, 15, 22]'),
    (v_temp_id, 'agradavel',  mf_trimf,  '[18, 24, 30]'),
    (v_temp_id, 'quente',     mf_trapmf, '[26, 32, 50, 50]');

    -- Termos: umidade
    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_umid_id, 'seco',   mf_trapmf, '[0, 0, 30, 50]'),
    (v_umid_id, 'normal', mf_trimf,  '[40, 55, 70]'),
    (v_umid_id, 'umido',  mf_trapmf, '[60, 75, 100, 100]');

    -- Termos: conforto
    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_conf_id, 'desconfortavel', mf_trapmf, '[0, 0, 3, 5]'),
    (v_conf_id, 'neutro',         mf_trimf,  '[3, 5, 7]'),
    (v_conf_id, 'confortavel',    mf_trapmf, '[5, 7, 10, 10]');

    -- Regras
    INSERT INTO fuzzy_rules (system_id, rule_text, weight, position) VALUES
    (v_sys_id, 'SE temperatura é frio E umidade é seco ENTÃO conforto é desconfortavel', 1.0, 0),
    (v_sys_id, 'SE temperatura é frio E umidade é normal ENTÃO conforto é neutro', 1.0, 1),
    (v_sys_id, 'SE temperatura é frio E umidade é umido ENTÃO conforto é desconfortavel', 1.0, 2),
    (v_sys_id, 'SE temperatura é agradavel E umidade é seco ENTÃO conforto é neutro', 1.0, 3),
    (v_sys_id, 'SE temperatura é agradavel E umidade é normal ENTÃO conforto é confortavel', 1.0, 4),
    (v_sys_id, 'SE temperatura é agradavel E umidade é umido ENTÃO conforto é neutro', 1.0, 5),
    (v_sys_id, 'SE temperatura é quente E umidade é seco ENTÃO conforto é desconfortavel', 1.0, 6),
    (v_sys_id, 'SE temperatura é quente E umidade é normal ENTÃO conforto é neutro', 1.0, 7),
    (v_sys_id, 'SE temperatura é quente E umidade é umido ENTÃO conforto é desconfortavel', 1.0, 8);

    RAISE NOTICE 'Seed data inserido — Sistema Conforto Térmico criado com 3 variáveis, 9 termos, 9 regras';
END $$;
