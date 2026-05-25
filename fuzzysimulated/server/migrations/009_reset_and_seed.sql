-- Full reset + reseed: 4 sistemas fuzzy
-- Remove dados de seeds anteriores e insere 4 sistemas completos

TRUNCATE fuzzy_systems CASCADE;

-- ═══════════════════════════════════════════════════════════════════════════
-- SISTEMA 1: Risco Cibernético Avançado (risco cibernético geral)
-- ═══════════════════════════════════════════════════════════════════════════
DO $$
DECLARE
    v_sys UUID; v_pob UUID; v_imp UUID; v_vuln UUID; v_risk UUID;
    t CONSTANT TEXT := 'trapmf'; r CONSTANT TEXT := 'trimf';
    ant CONSTANT TEXT := 'antecedent';
    con CONSTANT TEXT := 'consequent';
    res CONSTANT INT  := 501;
BEGIN
    INSERT INTO fuzzy_systems (id, name, description, defuzz_method)
    VALUES (uuid_generate_v4(), 'Risco Cibernético Avançado',
        'Avaliação de risco cibernético considerando probabilidade de ataque, impacto financeiro e vulnerabilidade do sistema.',
        'centroid') RETURNING id INTO v_sys;

    INSERT INTO fuzzy_variables VALUES (uuid_generate_v4(), v_sys, 'probabilidade_ataque',  ant, 0, 100, res) RETURNING id INTO v_pob;
    INSERT INTO fuzzy_variables VALUES (uuid_generate_v4(), v_sys, 'impacto_financeiro',    ant, 0, 100, res) RETURNING id INTO v_imp;
    INSERT INTO fuzzy_variables VALUES (uuid_generate_v4(), v_sys, 'vulnerabilidade_sistema', ant, 0, 100, res) RETURNING id INTO v_vuln;
    INSERT INTO fuzzy_variables VALUES (uuid_generate_v4(), v_sys, 'nivel_risco',           con, 0, 100, res) RETURNING id INTO v_risk;

    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_pob, 'baixa', t, '[0,0,25,45]'), (v_pob, 'media', r, '[30,50,70]'), (v_pob, 'alta', t, '[55,75,100,100]');
    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_imp, 'baixo', t, '[0,0,25,45]'), (v_imp, 'medio', r, '[30,50,70]'), (v_imp, 'alto', t, '[55,75,100,100]');
    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_vuln, 'baixa', t, '[0,0,20,40]'), (v_vuln, 'media', r, '[25,50,75]'), (v_vuln, 'alta', t, '[60,80,100,100]');
    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_risk, 'muito_baixo', r, '[0,0,20]'), (v_risk, 'baixo', t, '[10,20,35,45]'),
    (v_risk, 'medio', r, '[30,50,70]'), (v_risk, 'alto', t, '[55,70,85,95]'), (v_risk, 'critico', r, '[80,100,100]');

    INSERT INTO fuzzy_rules (system_id, rule_text, weight, position) VALUES
    (v_sys, 'SE probabilidade_ataque e baixa E vulnerabilidade_sistema e baixa ENTAO nivel_risco e muito_baixo', 1.0, 0),
    (v_sys, 'SE probabilidade_ataque e baixa E vulnerabilidade_sistema e media ENTAO nivel_risco e baixo', 1.0, 1),
    (v_sys, 'SE probabilidade_ataque e media E vulnerabilidade_sistema e baixa ENTAO nivel_risco e baixo', 1.0, 2),
    (v_sys, 'SE probabilidade_ataque e media E vulnerabilidade_sistema e media ENTAO nivel_risco e medio', 1.0, 3),
    (v_sys, 'SE probabilidade_ataque e alta E vulnerabilidade_sistema e alta ENTAO nivel_risco e critico', 1.0, 4),
    (v_sys, 'SE impacto_financeiro e alto E vulnerabilidade_sistema e alta ENTAO nivel_risco e critico', 1.0, 5),
    (v_sys, 'SE impacto_financeiro e alto E probabilidade_ataque e alta ENTAO nivel_risco e critico', 1.0, 6),
    (v_sys, 'SE impacto_financeiro e medio E vulnerabilidade_sistema e media ENTAO nivel_risco e medio', 1.0, 7),
    (v_sys, 'SE probabilidade_ataque e alta E vulnerabilidade_sistema e media ENTAO nivel_risco e alto', 1.0, 8),
    (v_sys, 'SE probabilidade_ataque e media E vulnerabilidade_sistema e alta ENTAO nivel_risco e alto', 1.0, 9),
    (v_sys, 'SE impacto_financeiro e baixo E vulnerabilidade_sistema e baixa ENTAO nivel_risco e muito_baixo', 1.0, 10),
    (v_sys, 'SE impacto_financeiro e alto E probabilidade_ataque e media ENTAO nivel_risco e alto', 1.0, 11);
    RAISE NOTICE 'Sistema 1: Risco Cibernético Avançado (12 regras)';
END $$;

-- ═══════════════════════════════════════════════════════════════════════════
-- SISTEMA 2: Conforto Térmico (OpenWeather — temperatura + umidade)
-- ═══════════════════════════════════════════════════════════════════════════
DO $$
DECLARE
    v_sys UUID; v_temp UUID; v_umid UUID; v_conf UUID;
    t CONSTANT TEXT := 'trapmf'; r CONSTANT TEXT := 'trimf';
    ant CONSTANT TEXT := 'antecedent';
    con CONSTANT TEXT := 'consequent';
    res CONSTANT INT  := 501;
BEGIN
    INSERT INTO fuzzy_systems (id, name, description, defuzz_method)
    VALUES (uuid_generate_v4(), 'Conforto Térmico',
        'Avaliação de conforto térmico baseado em temperatura e umidade com dados da API OpenWeather. Utiliza cidade e dados meteorológicos reais.',
        'centroid') RETURNING id INTO v_sys;

    INSERT INTO fuzzy_variables VALUES (uuid_generate_v4(), v_sys, 'temperatura', ant, 0, 50, res) RETURNING id INTO v_temp;
    INSERT INTO fuzzy_variables VALUES (uuid_generate_v4(), v_sys, 'umidade',     ant, 0, 100, res) RETURNING id INTO v_umid;
    INSERT INTO fuzzy_variables VALUES (uuid_generate_v4(), v_sys, 'conforto',    con, 0, 10, res) RETURNING id INTO v_conf;

    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_temp, 'frio',       t, '[0,0,15,22]'),
    (v_temp, 'agradavel',  r, '[18,24,30]'),
    (v_temp, 'quente',     t, '[26,32,50,50]');

    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_umid, 'seco',   t, '[0,0,30,50]'),
    (v_umid, 'normal', r, '[40,55,70]'),
    (v_umid, 'umido',  t, '[60,75,100,100]');

    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_conf, 'desconfortavel', t, '[0,0,3,5]'),
    (v_conf, 'neutro',         r, '[3,5,7]'),
    (v_conf, 'confortavel',    t, '[5,7,10,10]');

    INSERT INTO fuzzy_rules (system_id, rule_text, weight, position) VALUES
    (v_sys, 'SE temperatura e frio E umidade e seco ENTAO conforto e desconfortavel', 1.0, 0),
    (v_sys, 'SE temperatura e frio E umidade e normal ENTAO conforto e neutro', 1.0, 1),
    (v_sys, 'SE temperatura e frio E umidade e umido ENTAO conforto e desconfortavel', 1.0, 2),
    (v_sys, 'SE temperatura e agradavel E umidade e seco ENTAO conforto e neutro', 1.0, 3),
    (v_sys, 'SE temperatura e agradavel E umidade e normal ENTAO conforto e confortavel', 1.0, 4),
    (v_sys, 'SE temperatura e agradavel E umidade e umido ENTAO conforto e neutro', 1.0, 5),
    (v_sys, 'SE temperatura e quente E umidade e seco ENTAO conforto e desconfortavel', 1.0, 6),
    (v_sys, 'SE temperatura e quente E umidade e normal ENTAO conforto e neutro', 1.0, 7),
    (v_sys, 'SE temperatura e quente E umidade e umido ENTAO conforto e desconfortavel', 1.0, 8);
    RAISE NOTICE 'Sistema 2: Conforto Térmico — OpenWeather (9 regras)';
END $$;

-- ═══════════════════════════════════════════════════════════════════════════
-- SISTEMA 3: Risco Cibernetico (dataset_ml.parquet — colunas Prata/Gold)
-- ═══════════════════════════════════════════════════════════════════════════
DO $$
DECLARE
    v_sys UUID; v_rec UUID; v_func UUID; v_grav UUID; v_imp UUID;
    t CONSTANT TEXT := 'trapmf'; r CONSTANT TEXT := 'trimf';
    ant CONSTANT TEXT := 'antecedent';
    con CONSTANT TEXT := 'consequent';
    res CONSTANT INT  := 501;
BEGIN
    INSERT INTO fuzzy_systems (id, name, description, defuzz_method)
    VALUES (uuid_generate_v4(), 'Risco Cibernetico',
        'Classifica incidentes de segurança como ALTO ou BAIXO impacto financeiro usando colunas do dataset_ml.parquet (Prata -> Gold).',
        'centroid') RETURNING id INTO v_sys;

    INSERT INTO fuzzy_variables VALUES (uuid_generate_v4(), v_sys, 'receita_anual_usd',  ant, 0, 1000000000, res) RETURNING id INTO v_rec;
    INSERT INTO fuzzy_variables VALUES (uuid_generate_v4(), v_sys, 'total_funcionarios', ant, 0, 500000, res) RETURNING id INTO v_func;
    INSERT INTO fuzzy_variables VALUES (uuid_generate_v4(), v_sys, 'gravidade_ataque',   ant, 0, 100, res) RETURNING id INTO v_grav;
    INSERT INTO fuzzy_variables VALUES (uuid_generate_v4(), v_sys, 'impacto_financeiro', con, 0, 100, res) RETURNING id INTO v_imp;

    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_rec, 'baixa', t, '[0,0,50000000,100000000]'),
    (v_rec, 'media', r, '[50000000,200000000,500000000]'),
    (v_rec, 'alta',  t, '[200000000,500000000,1000000000,1000000000]');

    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_func, 'pequena', t, '[0,0,5000,20000]'),
    (v_func, 'media',   r, '[5000,50000,150000]'),
    (v_func, 'grande',  t, '[50000,150000,500000,500000]');

    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_grav, 'baixa', t, '[0,0,20,40]'),
    (v_grav, 'media', r, '[20,50,70]'),
    (v_grav, 'alta',  t, '[50,70,100,100]');

    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_imp, 'baixo', t, '[0,0,30,50]'),
    (v_imp, 'medio', r, '[30,50,70]'),
    (v_imp, 'alto',  t, '[50,70,100,100]');

    INSERT INTO fuzzy_rules (system_id, rule_text, weight, position) VALUES
    (v_sys, 'SE receita_anual_usd e baixa E total_funcionarios e pequena E gravidade_ataque e baixa ENTAO impacto_financeiro e baixo', 1.0, 0),
    (v_sys, 'SE receita_anual_usd e baixa E total_funcionarios e pequena E gravidade_ataque e alta ENTAO impacto_financeiro e medio', 1.0, 1),
    (v_sys, 'SE receita_anual_usd e baixa E total_funcionarios e grande E gravidade_ataque e alta ENTAO impacto_financeiro e alto', 1.0, 2),
    (v_sys, 'SE receita_anual_usd e media E total_funcionarios e media E gravidade_ataque e baixa ENTAO impacto_financeiro e baixo', 1.0, 3),
    (v_sys, 'SE receita_anual_usd e media E total_funcionarios e media E gravidade_ataque e media ENTAO impacto_financeiro e medio', 1.0, 4),
    (v_sys, 'SE receita_anual_usd e media E total_funcionarios e media E gravidade_ataque e alta ENTAO impacto_financeiro e alto', 1.0, 5),
    (v_sys, 'SE receita_anual_usd e alta E total_funcionarios e grande E gravidade_ataque e baixa ENTAO impacto_financeiro e medio', 1.0, 6),
    (v_sys, 'SE receita_anual_usd e alta E total_funcionarios e grande E gravidade_ataque e media ENTAO impacto_financeiro e alto', 1.0, 7),
    (v_sys, 'SE receita_anual_usd e alta E total_funcionarios e grande E gravidade_ataque e alta ENTAO impacto_financeiro e alto', 1.0, 8);
    RAISE NOTICE 'Sistema 3: Risco Cibernetico — dataset_ml.parquet (9 regras)';
END $$;

-- ═══════════════════════════════════════════════════════════════════════════
-- SISTEMA 4: Detecção de Intrusão (análise de tráfego de rede)
-- ═══════════════════════════════════════════════════════════════════════════
DO $$
DECLARE
    v_sys UUID; v_pac UUID; v_con UUID; v_traf UUID; v_ame UUID;
    t CONSTANT TEXT := 'trapmf'; r CONSTANT TEXT := 'trimf';
    ant CONSTANT TEXT := 'antecedent';
    con CONSTANT TEXT := 'consequent';
    res CONSTANT INT  := 501;
BEGIN
    INSERT INTO fuzzy_systems (id, name, description, defuzz_method)
    VALUES (uuid_generate_v4(), 'Detecção de Intrusão',
        'Análise de tráfego de rede para detecção de intrusões baseada em pacotes suspeitos, conexões anômalas e tráfego noturno.',
        'centroid') RETURNING id INTO v_sys;

    INSERT INTO fuzzy_variables VALUES (uuid_generate_v4(), v_sys, 'pacotes_suspeitos',  ant, 0, 100, res) RETURNING id INTO v_pac;
    INSERT INTO fuzzy_variables VALUES (uuid_generate_v4(), v_sys, 'conexoes_anomalas',  ant, 0, 100, res) RETURNING id INTO v_con;
    INSERT INTO fuzzy_variables VALUES (uuid_generate_v4(), v_sys, 'trafego_noturno',    ant, 0, 100, res) RETURNING id INTO v_traf;
    INSERT INTO fuzzy_variables VALUES (uuid_generate_v4(), v_sys, 'nivel_ameaca',       con, 0, 100, res) RETURNING id INTO v_ame;

    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_pac, 'baixo',  t, '[0,0,25,45]'),
    (v_pac, 'medio',  r, '[30,50,70]'),
    (v_pac, 'alto',   t, '[55,75,100,100]');

    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_con, 'baixa',  t, '[0,0,25,45]'),
    (v_con, 'media',  r, '[30,50,70]'),
    (v_con, 'alta',   t, '[55,75,100,100]');

    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_traf, 'baixo',  t, '[0,0,20,40]'),
    (v_traf, 'medio',  r, '[25,50,75]'),
    (v_traf, 'alto',   t, '[60,80,100,100]');

    INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES
    (v_ame, 'muito_baixo', r, '[0,0,20]'),
    (v_ame, 'baixo',       t, '[10,20,35,45]'),
    (v_ame, 'medio',       r, '[30,50,70]'),
    (v_ame, 'alto',        t, '[55,70,85,95]'),
    (v_ame, 'critico',     r, '[80,100,100]');

    INSERT INTO fuzzy_rules (system_id, rule_text, weight, position) VALUES
    (v_sys, 'SE pacotes_suspeitos e baixo E conexoes_anomalas e baixa ENTAO nivel_ameaca e muito_baixo', 1.0, 0),
    (v_sys, 'SE pacotes_suspeitos e baixo E conexoes_anomalas e media ENTAO nivel_ameaca e baixo', 1.0, 1),
    (v_sys, 'SE pacotes_suspeitos e medio E conexoes_anomalas e baixa ENTAO nivel_ameaca e baixo', 1.0, 2),
    (v_sys, 'SE pacotes_suspeitos e medio E conexoes_anomalas e media ENTAO nivel_ameaca e medio', 1.0, 3),
    (v_sys, 'SE pacotes_suspeitos e alto E conexoes_anomalas e alta ENTAO nivel_ameaca e critico', 1.0, 4),
    (v_sys, 'SE trafego_noturno e alto E conexoes_anomalas e alta ENTAO nivel_ameaca e critico', 1.0, 5),
    (v_sys, 'SE trafego_noturno e alto E pacotes_suspeitos e alto ENTAO nivel_ameaca e critico', 1.0, 6),
    (v_sys, 'SE trafego_noturno e medio E conexoes_anomalas e media ENTAO nivel_ameaca e medio', 1.0, 7),
    (v_sys, 'SE pacotes_suspeitos e alto E conexoes_anomalas e media ENTAO nivel_ameaca e alto', 1.0, 8),
    (v_sys, 'SE pacotes_suspeitos e medio E conexoes_anomalas e alta ENTAO nivel_ameaca e alto', 1.0, 9),
    (v_sys, 'SE trafego_noturno e baixo E conexoes_anomalas e baixa ENTAO nivel_ameaca e muito_baixo', 1.0, 10),
    (v_sys, 'SE trafego_noturno e alto E pacotes_suspeitos e medio ENTAO nivel_ameaca e alto', 1.0, 11);
    RAISE NOTICE 'Sistema 4: Detecção de Intrusão (12 regras)';
END $$;

-- ═══════════════════════════════════════════════════════════════════════════
-- CENÁRIOS: Risco Cibernético Avançado
-- ═══════════════════════════════════════════════════════════════════════════
DO $$
DECLARE v_sys UUID;
BEGIN
    SELECT id INTO v_sys FROM fuzzy_systems WHERE name = 'Risco Cibernético Avançado';
    IF NOT FOUND THEN RETURN; END IF;
    INSERT INTO scenarios (system_id, name, inputs) VALUES
    (v_sys, 'Sistema interno de baixo risco',       '{"probabilidade_ataque":10,"impacto_financeiro":15,"vulnerabilidade_sistema":10}'),
    (v_sys, 'Equipe com backups e atualizações',     '{"probabilidade_ataque":15,"impacto_financeiro":20,"vulnerabilidade_sistema":15}'),
    (v_sys, 'Rede com monitoramento SIEM',           '{"probabilidade_ataque":20,"impacto_financeiro":35,"vulnerabilidade_sistema":15}'),
    (v_sys, 'Firewall e antivírus atualizados',      '{"probabilidade_ataque":25,"impacto_financeiro":30,"vulnerabilidade_sistema":25}'),
    (v_sys, 'Phishing interno empresa pequena',      '{"probabilidade_ataque":40,"impacto_financeiro":20,"vulnerabilidade_sistema":30}'),
    (v_sys, 'Firewall desatualizado rede media',     '{"probabilidade_ataque":50,"impacto_financeiro":40,"vulnerabilidade_sistema":55}'),
    (v_sys, 'Phishing sem treinamento funcionarios', '{"probabilidade_ataque":60,"impacto_financeiro":30,"vulnerabilidade_sistema":50}'),
    (v_sys, 'Acesso privilegiado suspeito',          '{"probabilidade_ataque":45,"impacto_financeiro":55,"vulnerabilidade_sistema":80}'),
    (v_sys, 'Senhas fracas sistema financeiro',      '{"probabilidade_ataque":55,"impacto_financeiro":70,"vulnerabilidade_sistema":65}'),
    (v_sys, 'Sistema legado exposto internet',       '{"probabilidade_ataque":70,"impacto_financeiro":50,"vulnerabilidade_sistema":85}'),
    (v_sys, 'Servidor critico sem patch',            '{"probabilidade_ataque":85,"impacto_financeiro":90,"vulnerabilidade_sistema":95}'),
    (v_sys, 'Ransomware infraestrutura critica',     '{"probabilidade_ataque":80,"impacto_financeiro":95,"vulnerabilidade_sistema":70}'),
    (v_sys, 'DDoS servico bancario',                 '{"probabilidade_ataque":95,"impacto_financeiro":85,"vulnerabilidade_sistema":75}'),
    (v_sys, 'Vazamento dados via API insegura',      '{"probabilidade_ataque":75,"impacto_financeiro":90,"vulnerabilidade_sistema":85}');
    RAISE NOTICE 'Cenarios: Risco Cibernético Avançado (14)';
END $$;

-- ═══════════════════════════════════════════════════════════════════════════
-- CENÁRIOS: Conforto Térmico (climas brasileiros via OpenWeather)
-- ═══════════════════════════════════════════════════════════════════════════
DO $$
DECLARE v_sys UUID;
BEGIN
    SELECT id INTO v_sys FROM fuzzy_systems WHERE name = 'Conforto Térmico';
    IF NOT FOUND THEN RETURN; END IF;
    INSERT INTO scenarios (system_id, name, inputs) VALUES
    (v_sys, 'Dia frio e seco em Curitiba',          '{"temperatura":10,"umidade":30}'),
    (v_sys, 'Dia frio e úmido em São Paulo',        '{"temperatura":12,"umidade":85}'),
    (v_sys, 'Manhã amena em Belo Horizonte',        '{"temperatura":20,"umidade":55}'),
    (v_sys, 'Tarde agradável no Rio de Janeiro',    '{"temperatura":25,"umidade":50}'),
    (v_sys, 'Dia quente e seco em Brasília',        '{"temperatura":30,"umidade":25}'),
    (v_sys, 'Calor úmido em Manaus',                '{"temperatura":35,"umidade":90}'),
    (v_sys, 'Verão em Salvador',                    '{"temperatura":32,"umidade":75}'),
    (v_sys, 'Noite amena em Florianópolis',         '{"temperatura":22,"umidade":65}'),
    (v_sys, 'Inverno em Porto Alegre',              '{"temperatura":8,"umidade":70}'),
    (v_sys, 'Tarde quente e seca em Cuiabá',        '{"temperatura":40,"umidade":15}');
    RAISE NOTICE 'Cenarios: Conforto Térmico (10)';
END $$;

-- ═══════════════════════════════════════════════════════════════════════════
-- CENÁRIOS: Risco Cibernetico (dataset_ml.parquet)
-- ═══════════════════════════════════════════════════════════════════════════
DO $$
DECLARE v_sys UUID;
BEGIN
    SELECT id INTO v_sys FROM fuzzy_systems WHERE name = 'Risco Cibernetico';
    IF NOT FOUND THEN RETURN; END IF;
    INSERT INTO scenarios (system_id, name, inputs) VALUES
    (v_sys, 'Startup phishing baixo impacto',    '{"receita_anual_usd":1000000,"total_funcionarios":50,"gravidade_ataque":20}'),
    (v_sys, 'Media empresa ataque baixo',        '{"receita_anual_usd":100000000,"total_funcionarios":5000,"gravidade_ataque":15}'),
    (v_sys, 'Grande empresa ataque minimo',      '{"receita_anual_usd":800000000,"total_funcionarios":200000,"gravidade_ataque":10}'),
    (v_sys, 'Startup ransomware medio impacto',  '{"receita_anual_usd":5000000,"total_funcionarios":100,"gravidade_ataque":85}'),
    (v_sys, 'Media empresa malware moderado',    '{"receita_anual_usd":200000000,"total_funcionarios":40000,"gravidade_ataque":45}'),
    (v_sys, 'Grande empresa phishing velado',    '{"receita_anual_usd":500000000,"total_funcionarios":100000,"gravidade_ataque":25}'),
    (v_sys, 'Media empresa ransomware alto',     '{"receita_anual_usd":150000000,"total_funcionarios":30000,"gravidade_ataque":90}'),
    (v_sys, 'Grande empresa data breach',        '{"receita_anual_usd":900000000,"total_funcionarios":250000,"gravidade_ataque":75}'),
    (v_sys, 'Corp ransomware maximo impacto',    '{"receita_anual_usd":1000000000,"total_funcionarios":400000,"gravidade_ataque":95}');
    RAISE NOTICE 'Cenarios: Risco Cibernetico dataset (9)';
END $$;

-- ═══════════════════════════════════════════════════════════════════════════
-- CENÁRIOS: Detecção de Intrusão
-- ═══════════════════════════════════════════════════════════════════════════
DO $$
DECLARE v_sys UUID;
BEGIN
    SELECT id INTO v_sys FROM fuzzy_systems WHERE name = 'Detecção de Intrusão';
    IF NOT FOUND THEN RETURN; END IF;
    INSERT INTO scenarios (system_id, name, inputs) VALUES
    (v_sys, 'Tráfego normal horário comercial',          '{"pacotes_suspeitos":5,"conexoes_anomalas":3,"trafego_noturno":10}'),
    (v_sys, 'Pico de acesso legítimo',                    '{"pacotes_suspeitos":20,"conexoes_anomalas":15,"trafego_noturno":25}'),
    (v_sys, 'Varredura de porta suspeita',                '{"pacotes_suspeitos":65,"conexoes_anomalas":40,"trafego_noturno":30}'),
    (v_sys, 'Múltiplas conexões fora do horário',         '{"pacotes_suspeitos":40,"conexoes_anomalas":55,"trafego_noturno":80}'),
    (v_sys, 'Ataque DDoS noturno',                        '{"pacotes_suspeitos":95,"conexoes_anomalas":90,"trafego_noturno":85}'),
    (v_sys, 'Tentativa de brute force SSH',               '{"pacotes_suspeitos":80,"conexoes_anomalas":70,"trafego_noturno":60}'),
    (v_sys, 'Tráfego suspeito madrugada',                 '{"pacotes_suspeitos":55,"conexoes_anomalas":60,"trafego_noturno":90}'),
    (v_sys, 'Exfiltração de dados lenta',                 '{"pacotes_suspeitos":70,"conexoes_anomalas":80,"trafego_noturno":50}'),
    (v_sys, 'Rede interna monitorada sem ameaças',        '{"pacotes_suspeitos":8,"conexoes_anomalas":5,"trafego_noturno":12}'),
    (v_sys, 'Horário comercial com anomalias leves',      '{"pacotes_suspeitos":30,"conexoes_anomalas":25,"trafego_noturno":15}');
    RAISE NOTICE 'Cenarios: Detecção de Intrusão (10)';
END $$;
