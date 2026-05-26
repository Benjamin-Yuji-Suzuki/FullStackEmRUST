-- Adiciona cenário TSK com coeficientes diferenciados (Belém)
DO $$
DECLARE
    _sys_id UUID;
    _exists INTEGER;
BEGIN
    SELECT id INTO _sys_id FROM fuzzy_systems WHERE name = 'Conforto Térmico';
    SELECT COUNT(*) INTO _exists FROM scenarios WHERE system_id = _sys_id AND name = 'Clima de Belém (temp alta + umid altissima)';

    IF _exists = 0 THEN
        INSERT INTO scenarios (system_id, name, inputs) VALUES (
            _sys_id,
            'Clima de Belém (temp alta + umid altissima)',
            '{
                "inputs": {"temperatura": 32.0, "umidade": 88.0},
                "tsk_coeffs": {
                    "conforto_desconfortavel": [3.0, 0.0, 0.0],
                    "conforto_neutro":         [5.0, 0.0, 0.0],
                    "conforto_confortavel":    [7.0, 0.0, 0.0]
                },
                "tsk_inputs": {"temperatura": 32.0, "umidade": 88.0}
            }'::jsonb
        );
    END IF;
END $$;
