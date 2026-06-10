-- Expansão do Conforto Térmico: termos extremos + regras + cenários + TSK
-- Usa gaussmf (e^(-(x-μ)²/(2σ²))) para comportamento quadrático natural nos extremos
DO $$
DECLARE
    _sys_id UUID;
    _var_id UUID;
    _term_id UUID;
    _rule_id BIGINT;
    _cnt INT;
BEGIN
    SELECT id INTO _sys_id FROM fuzzy_systems WHERE name = 'Conforto Térmico';
    IF _sys_id IS NULL THEN
        RAISE EXCEPTION 'Sistema Conforto Térmico não encontrado';
    END IF;

    -- 1. NOVOS TERMOS EXTREMOS (gaussmf = squared exponential)
    -- temperatura: muito_frio (mean=0, sigma=7) e muito_quente (mean=50, sigma=7)
    SELECT id INTO _var_id FROM fuzzy_variables
    WHERE system_id = _sys_id AND name = 'temperatura';

    SELECT COUNT(*) INTO _cnt FROM fuzzy_terms
    WHERE variable_id = _var_id AND label = 'muito_frio';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_terms (variable_id, label, mf_type, params)
        VALUES (_var_id, 'muito_frio', 'gaussmf', '[0,7]'::jsonb);
    END IF;

    SELECT COUNT(*) INTO _cnt FROM fuzzy_terms
    WHERE variable_id = _var_id AND label = 'muito_quente';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_terms (variable_id, label, mf_type, params)
        VALUES (_var_id, 'muito_quente', 'gaussmf', '[50,7]'::jsonb);
    END IF;

    -- umidade: muito_seco (mean=0, sigma=10) e muito_umido (mean=100, sigma=10)
    SELECT id INTO _var_id FROM fuzzy_variables
    WHERE system_id = _sys_id AND name = 'umidade';

    SELECT COUNT(*) INTO _cnt FROM fuzzy_terms
    WHERE variable_id = _var_id AND label = 'muito_seco';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_terms (variable_id, label, mf_type, params)
        VALUES (_var_id, 'muito_seco', 'gaussmf', '[0,10]'::jsonb);
    END IF;

    SELECT COUNT(*) INTO _cnt FROM fuzzy_terms
    WHERE variable_id = _var_id AND label = 'muito_umido';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_terms (variable_id, label, mf_type, params)
        VALUES (_var_id, 'muito_umido', 'gaussmf', '[100,10]'::jsonb);
    END IF;

    -- conforto: extremo_desconfortavel (mean=0, sigma=1.5) e ideal (mean=10, sigma=1.5)
    SELECT id INTO _var_id FROM fuzzy_variables
    WHERE system_id = _sys_id AND name = 'conforto';

    SELECT COUNT(*) INTO _cnt FROM fuzzy_terms
    WHERE variable_id = _var_id AND label = 'extremo_desconfortavel';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_terms (variable_id, label, mf_type, params)
        VALUES (_var_id, 'extremo_desconfortavel', 'gaussmf', '[0,1.5]'::jsonb);
    END IF;

    SELECT COUNT(*) INTO _cnt FROM fuzzy_terms
    WHERE variable_id = _var_id AND label = 'ideal';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_terms (variable_id, label, mf_type, params)
        VALUES (_var_id, 'ideal', 'gaussmf', '[10,1.5]'::jsonb);
    END IF;

    -- 2. NOVAS REGRAS (9 → 16, usando os novos termos extremos)
    SELECT COALESCE(MAX(position), 0) INTO _rule_id FROM fuzzy_rules WHERE system_id = _sys_id;

    -- Regra 10: muito_frio → extremo_desconfortavel
    SELECT COUNT(*) INTO _cnt FROM fuzzy_rules
    WHERE system_id = _sys_id AND rule_text = 'SE temperatura e muito_frio ENTAO conforto e extremo_desconfortavel';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_rules (system_id, rule_text, weight, position)
        VALUES (_sys_id, 'SE temperatura e muito_frio ENTAO conforto e extremo_desconfortavel', 1.0, _rule_id + 1);
        _rule_id := _rule_id + 1;
    END IF;

    -- Regra 11: muito_frio + muito_umido → extremo_desconfortavel
    SELECT COUNT(*) INTO _cnt FROM fuzzy_rules
    WHERE system_id = _sys_id AND rule_text = 'SE temperatura e muito_frio E umidade e muito_umido ENTAO conforto e extremo_desconfortavel';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_rules (system_id, rule_text, weight, position)
        VALUES (_sys_id, 'SE temperatura e muito_frio E umidade e muito_umido ENTAO conforto e extremo_desconfortavel', 1.0, _rule_id + 1);
        _rule_id := _rule_id + 1;
    END IF;

    -- Regra 12: muito_quente → extremo_desconfortavel
    SELECT COUNT(*) INTO _cnt FROM fuzzy_rules
    WHERE system_id = _sys_id AND rule_text = 'SE temperatura e muito_quente ENTAO conforto e extremo_desconfortavel';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_rules (system_id, rule_text, weight, position)
        VALUES (_sys_id, 'SE temperatura e muito_quente ENTAO conforto e extremo_desconfortavel', 1.0, _rule_id + 1);
        _rule_id := _rule_id + 1;
    END IF;

    -- Regra 13: muito_quente + muito_seco → extremo_desconfortavel
    SELECT COUNT(*) INTO _cnt FROM fuzzy_rules
    WHERE system_id = _sys_id AND rule_text = 'SE temperatura e muito_quente E umidade e muito_seco ENTAO conforto e extremo_desconfortavel';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_rules (system_id, rule_text, weight, position)
        VALUES (_sys_id, 'SE temperatura e muito_quente E umidade e muito_seco ENTAO conforto e extremo_desconfortavel', 1.0, _rule_id + 1);
        _rule_id := _rule_id + 1;
    END IF;

    -- Regra 14: agradavel + muito_seco → neutro
    SELECT COUNT(*) INTO _cnt FROM fuzzy_rules
    WHERE system_id = _sys_id AND rule_text = 'SE temperatura e agradavel E umidade e muito_seco ENTAO conforto e neutro';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_rules (system_id, rule_text, weight, position)
        VALUES (_sys_id, 'SE temperatura e agradavel E umidade e muito_seco ENTAO conforto e neutro', 1.0, _rule_id + 1);
        _rule_id := _rule_id + 1;
    END IF;

    -- Regra 15: agradavel + muito_umido → neutro
    SELECT COUNT(*) INTO _cnt FROM fuzzy_rules
    WHERE system_id = _sys_id AND rule_text = 'SE temperatura e agradavel E umidade e muito_umido ENTAO conforto e neutro';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_rules (system_id, rule_text, weight, position)
        VALUES (_sys_id, 'SE temperatura e agradavel E umidade e muito_umido ENTAO conforto e neutro', 1.0, _rule_id + 1);
        _rule_id := _rule_id + 1;
    END IF;

    -- Regra 16: muito_frio + seco → extremo_desconfortavel (frio intenso + ar seco)
    SELECT COUNT(*) INTO _cnt FROM fuzzy_rules
    WHERE system_id = _sys_id AND rule_text = 'SE temperatura e muito_frio E umidade e seco ENTAO conforto e extremo_desconfortavel';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_rules (system_id, rule_text, weight, position)
        VALUES (_sys_id, 'SE temperatura e muito_frio E umidade e seco ENTAO conforto e extremo_desconfortavel', 1.0, _rule_id + 1);
        _rule_id := _rule_id + 1;
    END IF;

    -- Regra 17: muito_quente + muito_umido → extremo_desconfortavel (abafado extremo)
    SELECT COUNT(*) INTO _cnt FROM fuzzy_rules
    WHERE system_id = _sys_id AND rule_text = 'SE temperatura e muito_quente E umidade e muito_umido ENTAO conforto e extremo_desconfortavel';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_rules (system_id, rule_text, weight, position)
        VALUES (_sys_id, 'SE temperatura e muito_quente E umidade e muito_umido ENTAO conforto e extremo_desconfortavel', 1.0, _rule_id + 1);
        _rule_id := _rule_id + 1;
    END IF;

    -- 3. NOVOS CENÁRIOS PRÉ-DEFINIDOS (incluindo extremos)
    -- Cenário 11: Frio extremo seco (Sibéria)
    SELECT COUNT(*) INTO _cnt FROM scenarios
    WHERE system_id = _sys_id AND name = 'Frio extremo seco (Sibéria)';
    IF _cnt = 0 THEN
        INSERT INTO scenarios (system_id, name, inputs)
        VALUES (_sys_id, 'Frio extremo seco (Sibéria)',
            '{"inputs":{"temperatura":-5,"umidade":15}}'::jsonb);
    END IF;

    -- Cenário 12: Frio intenso úmido (Nevoa congelante)
    SELECT COUNT(*) INTO _cnt FROM scenarios
    WHERE system_id = _sys_id AND name = 'Frio intenso úmido (Nevoa congelante)';
    IF _cnt = 0 THEN
        INSERT INTO scenarios (system_id, name, inputs)
        VALUES (_sys_id, 'Frio intenso úmido (Nevoa congelante)',
            '{"inputs":{"temperatura":2,"umidade":95}}'::jsonb);
    END IF;

    -- Cenário 13: Calor extremo seco (Deserto)
    SELECT COUNT(*) INTO _cnt FROM scenarios
    WHERE system_id = _sys_id AND name = 'Calor extremo seco (Deserto)';
    IF _cnt = 0 THEN
        INSERT INTO scenarios (system_id, name, inputs)
        VALUES (_sys_id, 'Calor extremo seco (Deserto)',
            '{"inputs":{"temperatura":50,"umidade":5}}'::jsonb);
    END IF;

    -- Cenário 14: Calor extremo úmido (Amazônia)
    SELECT COUNT(*) INTO _cnt FROM scenarios
    WHERE system_id = _sys_id AND name = 'Calor extremo úmido (Amazônia)';
    IF _cnt = 0 THEN
        INSERT INTO scenarios (system_id, name, inputs)
        VALUES (_sys_id, 'Calor extremo úmido (Amazônia)',
            '{"inputs":{"temperatura":48,"umidade":98}}'::jsonb);
    END IF;

    -- Cenário 15: Temperatura amena com ar extremamente seco
    SELECT COUNT(*) INTO _cnt FROM scenarios
    WHERE system_id = _sys_id AND name = 'Temperatura amena com ar extremamente seco';
    IF _cnt = 0 THEN
        INSERT INTO scenarios (system_id, name, inputs)
        VALUES (_sys_id, 'Temperatura amena com ar extremamente seco',
            '{"inputs":{"temperatura":23,"umidade":3}}'::jsonb);
    END IF;

    -- Cenário 16: Temperatura amena com ar saturado
    SELECT COUNT(*) INTO _cnt FROM scenarios
    WHERE system_id = _sys_id AND name = 'Temperatura amena com ar saturado';
    IF _cnt = 0 THEN
        INSERT INTO scenarios (system_id, name, inputs)
        VALUES (_sys_id, 'Temperatura amena com ar saturado',
            '{"inputs":{"temperatura":23,"umidade":100}}'::jsonb);
    END IF;

    -- Cenário 17: Calor moderado com umidade crítica
    SELECT COUNT(*) INTO _cnt FROM scenarios
    WHERE system_id = _sys_id AND name = 'Calor moderado com umidade crítica';
    IF _cnt = 0 THEN
        INSERT INTO scenarios (system_id, name, inputs)
        VALUES (_sys_id, 'Calor moderado com umidade crítica',
            '{"inputs":{"temperatura":35,"umidade":85}}'::jsonb);
    END IF;

    RAISE NOTICE 'Conforto Térmico expandido com sucesso: termos extremos + 8 regras + 7 cenários';
END $$;
