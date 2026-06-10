-- Expansão do Risco Cibernetico: termos gaussmf extremos + regras + cenários
-- Segue o mesmo padrão da migração 012 (Conforto Térmico)
DO $$
DECLARE
    _sys_id UUID;
    _var_id UUID;
    _cnt INT;
    _pos INT;
BEGIN
    SELECT id INTO _sys_id FROM fuzzy_systems WHERE name = 'Risco Cibernetico';
    IF _sys_id IS NULL THEN
        RAISE EXCEPTION 'Sistema Risco Cibernetico não encontrado';
    END IF;

    -- ═══════════════════════════════════════════════════════════
    -- 1. NOVOS TERMOS EXTREMOS (gaussmf)
    -- ═══════════════════════════════════════════════════════════

    -- receita_anual_usd: muito_baixa (mean=0, sigma=50M) e muito_alta (mean=1e9, sigma=50M)
    SELECT id INTO _var_id FROM fuzzy_variables
    WHERE system_id = _sys_id AND name = 'receita_anual_usd';

    SELECT COUNT(*) INTO _cnt FROM fuzzy_terms
    WHERE variable_id = _var_id AND label = 'muito_baixa';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_terms (variable_id, label, mf_type, params)
        VALUES (_var_id, 'muito_baixa', 'gaussmf', '[0,50000000]'::jsonb);
    END IF;

    SELECT COUNT(*) INTO _cnt FROM fuzzy_terms
    WHERE variable_id = _var_id AND label = 'muito_alta';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_terms (variable_id, label, mf_type, params)
        VALUES (_var_id, 'muito_alta', 'gaussmf', '[1000000000,50000000]'::jsonb);
    END IF;

    -- total_funcionarios: micro (mean=0, sigma=1000) e megacorp (mean=500k, sigma=50k)
    SELECT id INTO _var_id FROM fuzzy_variables
    WHERE system_id = _sys_id AND name = 'total_funcionarios';

    SELECT COUNT(*) INTO _cnt FROM fuzzy_terms
    WHERE variable_id = _var_id AND label = 'micro';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_terms (variable_id, label, mf_type, params)
        VALUES (_var_id, 'micro', 'gaussmf', '[0,1000]'::jsonb);
    END IF;

    SELECT COUNT(*) INTO _cnt FROM fuzzy_terms
    WHERE variable_id = _var_id AND label = 'megacorp';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_terms (variable_id, label, mf_type, params)
        VALUES (_var_id, 'megacorp', 'gaussmf', '[500000,50000]'::jsonb);
    END IF;

    -- gravidade_ataque: quase_zero (mean=0, sigma=5) e critico (mean=100, sigma=5)
    SELECT id INTO _var_id FROM fuzzy_variables
    WHERE system_id = _sys_id AND name = 'gravidade_ataque';

    SELECT COUNT(*) INTO _cnt FROM fuzzy_terms
    WHERE variable_id = _var_id AND label = 'quase_zero';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_terms (variable_id, label, mf_type, params)
        VALUES (_var_id, 'quase_zero', 'gaussmf', '[0,5]'::jsonb);
    END IF;

    SELECT COUNT(*) INTO _cnt FROM fuzzy_terms
    WHERE variable_id = _var_id AND label = 'critico';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_terms (variable_id, label, mf_type, params)
        VALUES (_var_id, 'critico', 'gaussmf', '[100,5]'::jsonb);
    END IF;

    -- impacto_financeiro: minimo (mean=0, sigma=5) e catastrofico (mean=100, sigma=5)
    SELECT id INTO _var_id FROM fuzzy_variables
    WHERE system_id = _sys_id AND name = 'impacto_financeiro';

    SELECT COUNT(*) INTO _cnt FROM fuzzy_terms
    WHERE variable_id = _var_id AND label = 'minimo';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_terms (variable_id, label, mf_type, params)
        VALUES (_var_id, 'minimo', 'gaussmf', '[0,5]'::jsonb);
    END IF;

    SELECT COUNT(*) INTO _cnt FROM fuzzy_terms
    WHERE variable_id = _var_id AND label = 'catastrofico';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_terms (variable_id, label, mf_type, params)
        VALUES (_var_id, 'catastrofico', 'gaussmf', '[100,5]'::jsonb);
    END IF;

    -- ═══════════════════════════════════════════════════════════
    -- 2. NOVAS REGRAS EXTREMAS
    -- ═══════════════════════════════════════════════════════════
    SELECT COALESCE(MAX(position), 0) INTO _pos FROM fuzzy_rules WHERE system_id = _sys_id;

    -- Regra 10: micro receita + micro funcionarios + quase_zero ataque → minimo impacto
    SELECT COUNT(*) INTO _cnt FROM fuzzy_rules
    WHERE system_id = _sys_id AND rule_text = 'SE receita_anual_usd e muito_baixa E total_funcionarios e micro E gravidade_ataque e quase_zero ENTAO impacto_financeiro e minimo';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_rules (system_id, rule_text, weight, position)
        VALUES (_sys_id, 'SE receita_anual_usd e muito_baixa E total_funcionarios e micro E gravidade_ataque e quase_zero ENTAO impacto_financeiro e minimo', 1.0, _pos + 1);
        _pos := _pos + 1;
    END IF;

    -- Regra 11: micro receita + micro funcionarios + ataque critico → medio impacto
    SELECT COUNT(*) INTO _cnt FROM fuzzy_rules
    WHERE system_id = _sys_id AND rule_text = 'SE receita_anual_usd e muito_baixa E total_funcionarios e micro E gravidade_ataque e critico ENTAO impacto_financeiro e medio';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_rules (system_id, rule_text, weight, position)
        VALUES (_sys_id, 'SE receita_anual_usd e muito_baixa E total_funcionarios e micro E gravidade_ataque e critico ENTAO impacto_financeiro e medio', 1.0, _pos + 1);
        _pos := _pos + 1;
    END IF;

    -- Regra 12: mega receita + megacorp + quase_zero ataque → baixo impacto
    SELECT COUNT(*) INTO _cnt FROM fuzzy_rules
    WHERE system_id = _sys_id AND rule_text = 'SE receita_anual_usd e muito_alta E total_funcionarios e megacorp E gravidade_ataque e quase_zero ENTAO impacto_financeiro e baixo';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_rules (system_id, rule_text, weight, position)
        VALUES (_sys_id, 'SE receita_anual_usd e muito_alta E total_funcionarios e megacorp E gravidade_ataque e quase_zero ENTAO impacto_financeiro e baixo', 1.0, _pos + 1);
        _pos := _pos + 1;
    END IF;

    -- Regra 13: mega receita + megacorp + ataque critico → catastrofico impacto
    SELECT COUNT(*) INTO _cnt FROM fuzzy_rules
    WHERE system_id = _sys_id AND rule_text = 'SE receita_anual_usd e muito_alta E total_funcionarios e megacorp E gravidade_ataque e critico ENTAO impacto_financeiro e catastrofico';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_rules (system_id, rule_text, weight, position)
        VALUES (_sys_id, 'SE receita_anual_usd e muito_alta E total_funcionarios e megacorp E gravidade_ataque e critico ENTAO impacto_financeiro e catastrofico', 1.0, _pos + 1);
        _pos := _pos + 1;
    END IF;

    -- Regra 14: megacorp + ataque critico → catastrofico (independente da receita)
    SELECT COUNT(*) INTO _cnt FROM fuzzy_rules
    WHERE system_id = _sys_id AND rule_text = 'SE total_funcionarios e megacorp E gravidade_ataque e critico ENTAO impacto_financeiro e catastrofico';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_rules (system_id, rule_text, weight, position)
        VALUES (_sys_id, 'SE total_funcionarios e megacorp E gravidade_ataque e critico ENTAO impacto_financeiro e catastrofico', 1.0, _pos + 1);
        _pos := _pos + 1;
    END IF;

    -- Regra 15: micro receita + ataque critico → medio impacto
    SELECT COUNT(*) INTO _cnt FROM fuzzy_rules
    WHERE system_id = _sys_id AND rule_text = 'SE receita_anual_usd e muito_baixa E gravidade_ataque e critico ENTAO impacto_financeiro e medio';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_rules (system_id, rule_text, weight, position)
        VALUES (_sys_id, 'SE receita_anual_usd e muito_baixa E gravidade_ataque e critico ENTAO impacto_financeiro e medio', 1.0, _pos + 1);
        _pos := _pos + 1;
    END IF;

    -- Regra 16: mega receita + quase_zero ataque → minimo impacto
    SELECT COUNT(*) INTO _cnt FROM fuzzy_rules
    WHERE system_id = _sys_id AND rule_text = 'SE receita_anual_usd e muito_alta E gravidade_ataque e quase_zero ENTAO impacto_financeiro e minimo';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_rules (system_id, rule_text, weight, position)
        VALUES (_sys_id, 'SE receita_anual_usd e muito_alta E gravidade_ataque e quase_zero ENTAO impacto_financeiro e minimo', 1.0, _pos + 1);
        _pos := _pos + 1;
    END IF;

    -- Regra 17: micro funcionarios + ataque critico → alto impacto
    SELECT COUNT(*) INTO _cnt FROM fuzzy_rules
    WHERE system_id = _sys_id AND rule_text = 'SE total_funcionarios e micro E gravidade_ataque e critico ENTAO impacto_financeiro e alto';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_rules (system_id, rule_text, weight, position)
        VALUES (_sys_id, 'SE total_funcionarios e micro E gravidade_ataque e critico ENTAO impacto_financeiro e alto', 1.0, _pos + 1);
        _pos := _pos + 1;
    END IF;

    -- Regra 18: micro receita + micro funcionarios → minimo impacto (sem ataque relevante)
    SELECT COUNT(*) INTO _cnt FROM fuzzy_rules
    WHERE system_id = _sys_id AND rule_text = 'SE receita_anual_usd e muito_baixa E total_funcionarios e micro ENTAO impacto_financeiro e minimo';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_rules (system_id, rule_text, weight, position)
        VALUES (_sys_id, 'SE receita_anual_usd e muito_baixa E total_funcionarios e micro ENTAO impacto_financeiro e minimo', 1.0, _pos + 1);
        _pos := _pos + 1;
    END IF;

    -- Regra 19: mega receita + megacorp → alto impacto (mesmo sem ataque, escala gera risco)
    SELECT COUNT(*) INTO _cnt FROM fuzzy_rules
    WHERE system_id = _sys_id AND rule_text = 'SE receita_anual_usd e muito_alta E total_funcionarios e megacorp ENTAO impacto_financeiro e alto';
    IF _cnt = 0 THEN
        INSERT INTO fuzzy_rules (system_id, rule_text, weight, position)
        VALUES (_sys_id, 'SE receita_anual_usd e muito_alta E total_funcionarios e megacorp ENTAO impacto_financeiro e alto', 1.0, _pos + 1);
        _pos := _pos + 1;
    END IF;

    -- ═══════════════════════════════════════════════════════════
    -- 3. NOVOS CENÁRIOS EXTREMOS
    -- ═══════════════════════════════════════════════════════════

    -- Cenário 10: Micro empresa, ataque quase zero → impacto minimo
    SELECT COUNT(*) INTO _cnt FROM scenarios
    WHERE system_id = _sys_id AND name = 'Micro-empresa sem ataque relevante';
    IF _cnt = 0 THEN
        INSERT INTO scenarios (system_id, name, inputs)
        VALUES (_sys_id, 'Micro-empresa sem ataque relevante',
            '{"receita_anual_usd": 50000, "total_funcionarios": 3, "gravidade_ataque": 2}'::jsonb);
    END IF;

    -- Cenário 11: Micro empresa, ataque critico → impacto medio
    SELECT COUNT(*) INTO _cnt FROM scenarios
    WHERE system_id = _sys_id AND name = 'Micro-empresa com ataque ransomware';
    IF _cnt = 0 THEN
        INSERT INTO scenarios (system_id, name, inputs)
        VALUES (_sys_id, 'Micro-empresa com ataque ransomware',
            '{"receita_anual_usd": 80000, "total_funcionarios": 5, "gravidade_ataque": 85}'::jsonb);
    END IF;

    -- Cenário 12: Mega corp, ataque quase zero → impacto baixo
    SELECT COUNT(*) INTO _cnt FROM scenarios
    WHERE system_id = _sys_id AND name = 'Megacorp com quase nenhum ataque';
    IF _cnt = 0 THEN
        INSERT INTO scenarios (system_id, name, inputs)
        VALUES (_sys_id, 'Megacorp com quase nenhum ataque',
            '{"receita_anual_usd": 950000000, "total_funcionarios": 450000, "gravidade_ataque": 3}'::jsonb);
    END IF;

    -- Cenário 13: Mega corp, ataque critico → catastrofico
    SELECT COUNT(*) INTO _cnt FROM scenarios
    WHERE system_id = _sys_id AND name = 'Megacorp com ataque APT critico';
    IF _cnt = 0 THEN
        INSERT INTO scenarios (system_id, name, inputs)
        VALUES (_sys_id, 'Megacorp com ataque APT critico',
            '{"receita_anual_usd": 990000000, "total_funcionarios": 480000, "gravidade_ataque": 98}'::jsonb);
    END IF;

    -- Cenário 14: Empresa media, ataque APT → impacto alto
    SELECT COUNT(*) INTO _cnt FROM scenarios
    WHERE system_id = _sys_id AND name = 'Media empresa com ataque APT avançado';
    IF _cnt = 0 THEN
        INSERT INTO scenarios (system_id, name, inputs)
        VALUES (_sys_id, 'Media empresa com ataque APT avançado',
            '{"receita_anual_usd": 80000000, "total_funcionarios": 3000, "gravidade_ataque": 80}'::jsonb);
    END IF;

    -- Cenário 15: Grande empresa, ataque DDoS massivo → impacto alto
    SELECT COUNT(*) INTO _cnt FROM scenarios
    WHERE system_id = _sys_id AND name = 'Grande empresa com DDoS massivo';
    IF _cnt = 0 THEN
        INSERT INTO scenarios (system_id, name, inputs)
        VALUES (_sys_id, 'Grande empresa com DDoS massivo',
            '{"receita_anual_usd": 700000000, "total_funcionarios": 180000, "gravidade_ataque": 50}'::jsonb);
    END IF;

    -- Cenário 16: Startup, ataque interno → impacto medio
    SELECT COUNT(*) INTO _cnt FROM scenarios
    WHERE system_id = _sys_id AND name = 'Startup comprometida por insider';
    IF _cnt = 0 THEN
        INSERT INTO scenarios (system_id, name, inputs)
        VALUES (_sys_id, 'Startup comprometida por insider',
            '{"receita_anual_usd": 3000000, "total_funcionarios": 25, "gravidade_ataque": 60}'::jsonb);
    END IF;

    RAISE NOTICE 'Risco Cibernetico expandido: 8 termos gaussmf + 10 regras + 7 cenarios extremos';
END $$;
