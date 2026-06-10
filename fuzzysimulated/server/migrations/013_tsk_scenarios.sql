-- Cenários TSK com coeficientes lineares para o Conforto Térmico
-- TSK: y = a0 + a1*temperatura + a2*umidade (coeffs = [a0, a1, a2])
-- Os coeficientes são projetados para produzir saídas no universo [0,10]:
--   extremo_desconfortavel (~0):   coeffs = [0,0,0]    → y ≈ 0
--   desconfortavel (~1-3):         coeffs = [1,0,0]     → y ≈ 1
--   neutro (~4-6):                coeffs = [5,0,0]     → y ≈ 5
--   confortavel (~7-9):           coeffs = [8,0,0]     → y ≈ 8
--   ideal (~9-10):                coeffs = [10,0,0]    → y ≈ 10
DO $$
DECLARE
    _sys_id UUID;
    _cnt INT;
    _tsk_inputs JSONB;
BEGIN
    SELECT id INTO _sys_id FROM fuzzy_systems WHERE name = 'Conforto Térmico';
    IF _sys_id IS NULL THEN
        RAISE EXCEPTION 'Sistema Conforto Térmico não encontrado';
    END IF;

    -- 1. Cenário: Inverno rigoroso (muito_frio ativa)
    SELECT COUNT(*) INTO _cnt FROM scenarios
    WHERE system_id = _sys_id AND name = 'TSK: Inverno rigoroso (extremo)';
    IF _cnt = 0 THEN
        _tsk_inputs := '{
            "inputs": {"temperatura": 0, "umidade": 30},
            "tsk_coeffs": {
                "conforto_desconfortavel":       [1.0, 0.0, 0.0],
                "conforto_neutro":               [5.0, 0.0, 0.0],
                "conforto_confortavel":          [8.0, 0.0, 0.0],
                "conforto_extremo_desconfortavel":[0.0, 0.0, 0.0],
                "conforto_ideal":                [10.0, 0.0, 0.0]
            },
            "tsk_inputs": {"temperatura": 0, "umidade": 30}
        }'::jsonb;
        INSERT INTO scenarios (system_id, name, inputs)
        VALUES (_sys_id, 'TSK: Inverno rigoroso (extremo)', _tsk_inputs);
    END IF;

    -- 2. Cenário: Deserto ao meio-dia (muito_quente + muito_seco)
    SELECT COUNT(*) INTO _cnt FROM scenarios
    WHERE system_id = _sys_id AND name = 'TSK: Deserto ao meio-dia (extremo)';
    IF _cnt = 0 THEN
        _tsk_inputs := '{
            "inputs": {"temperatura": 50, "umidade": 5},
            "tsk_coeffs": {
                "conforto_desconfortavel":       [1.0, 0.0, 0.0],
                "conforto_neutro":               [5.0, 0.0, 0.0],
                "conforto_confortavel":          [8.0, 0.0, 0.0],
                "conforto_extremo_desconfortavel":[0.0, 0.0, 0.0],
                "conforto_ideal":                [10.0, 0.0, 0.0]
            },
            "tsk_inputs": {"temperatura": 50, "umidade": 5}
        }'::jsonb;
        INSERT INTO scenarios (system_id, name, inputs)
        VALUES (_sys_id, 'TSK: Deserto ao meio-dia (extremo)', _tsk_inputs);
    END IF;

    -- 3. Cenário: Dia ameno e agradável (somente confortavel ativa)
    SELECT COUNT(*) INTO _cnt FROM scenarios
    WHERE system_id = _sys_id AND name = 'TSK: Dia ameno e agradável (ideal)';
    IF _cnt = 0 THEN
        _tsk_inputs := '{
            "inputs": {"temperatura": 24, "umidade": 55},
            "tsk_coeffs": {
                "conforto_desconfortavel":       [1.0, 0.0, 0.0],
                "conforto_neutro":               [5.0, 0.0, 0.0],
                "conforto_confortavel":          [8.0, 0.0, 0.0],
                "conforto_extremo_desconfortavel":[0.0, 0.0, 0.0],
                "conforto_ideal":                [10.0, 0.0, 0.0]
            },
            "tsk_inputs": {"temperatura": 24, "umidade": 55}
        }'::jsonb;
        INSERT INTO scenarios (system_id, name, inputs)
        VALUES (_sys_id, 'TSK: Dia ameno e agradável (ideal)', _tsk_inputs);
    END IF;

    -- 4. Cenário: Calor úmido amazônico (muito_quente + muito_umido)
    SELECT COUNT(*) INTO _cnt FROM scenarios
    WHERE system_id = _sys_id AND name = 'TSK: Calor úmido amazônico (extremo)';
    IF _cnt = 0 THEN
        _tsk_inputs := '{
            "inputs": {"temperatura": 48, "umidade": 98},
            "tsk_coeffs": {
                "conforto_desconfortavel":       [1.0, 0.0, 0.0],
                "conforto_neutro":               [5.0, 0.0, 0.0],
                "conforto_confortavel":          [8.0, 0.0, 0.0],
                "conforto_extremo_desconfortavel":[0.0, 0.0, 0.0],
                "conforto_ideal":                [10.0, 0.0, 0.0]
            },
            "tsk_inputs": {"temperatura": 48, "umidade": 98}
        }'::jsonb;
        INSERT INTO scenarios (system_id, name, inputs)
        VALUES (_sys_id, 'TSK: Calor úmido amazônico (extremo)', _tsk_inputs);
    END IF;

    -- 5. Cenário: Tarde quente e seca em Brasília
    SELECT COUNT(*) INTO _cnt FROM scenarios
    WHERE system_id = _sys_id AND name = 'TSK: Tarde quente e seca (Brasília)';
    IF _cnt = 0 THEN
        _tsk_inputs := '{
            "inputs": {"temperatura": 30, "umidade": 25},
            "tsk_coeffs": {
                "conforto_desconfortavel":       [1.0, 0.0, 0.0],
                "conforto_neutro":               [5.0, 0.0, 0.0],
                "conforto_confortavel":          [8.0, 0.0, 0.0],
                "conforto_extremo_desconfortavel":[0.0, 0.0, 0.0],
                "conforto_ideal":                [10.0, 0.0, 0.0]
            },
            "tsk_inputs": {"temperatura": 30, "umidade": 25}
        }'::jsonb;
        INSERT INTO scenarios (system_id, name, inputs)
        VALUES (_sys_id, 'TSK: Tarde quente e seca (Brasília)', _tsk_inputs);
    END IF;

    -- 6. Cenário: Manhã fria e seca (Curitiba)
    SELECT COUNT(*) INTO _cnt FROM scenarios
    WHERE system_id = _sys_id AND name = 'TSK: Manhã fria e seca (Curitiba)';
    IF _cnt = 0 THEN
        _tsk_inputs := '{
            "inputs": {"temperatura": 10, "umidade": 30},
            "tsk_coeffs": {
                "conforto_desconfortavel":       [1.0, 0.0, 0.0],
                "conforto_neutro":               [5.0, 0.0, 0.0],
                "conforto_confortavel":          [8.0, 0.0, 0.0],
                "conforto_extremo_desconfortavel":[0.0, 0.0, 0.0],
                "conforto_ideal":                [10.0, 0.0, 0.0]
            },
            "tsk_inputs": {"temperatura": 10, "umidade": 30}
        }'::jsonb;
        INSERT INTO scenarios (system_id, name, inputs)
        VALUES (_sys_id, 'TSK: Manhã fria e seca (Curitiba)', _tsk_inputs);
    END IF;

    RAISE NOTICE 'Cenários TSK adicionados com sucesso (6 novos)';
END $$;
