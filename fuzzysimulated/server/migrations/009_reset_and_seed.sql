-- Full reset + reseed: 4 sistemas fuzzy
-- Dados em temp tables, lógica em um único loop (evita duplicação estrutural)

TRUNCATE fuzzy_systems CASCADE;

-- ═══════════════════════════════════════════════════════════════════════════
-- DADOS: sistemas, variáveis, termos, regras, cenários
-- ═══════════════════════════════════════════════════════════════════════════
CREATE TEMP TABLE _s (id SERIAL PRIMARY KEY, name TEXT, descr TEXT);
INSERT INTO _s (name, descr) VALUES
('Risco Cibernético Avançado', 'Avaliação de risco cibernético considerando probabilidade de ataque, impacto financeiro e vulnerabilidade do sistema.'),
('Conforto Térmico', 'Avaliação de conforto térmico baseado em temperatura e umidade com dados da API OpenWeather. Utiliza cidade e dados meteorológicos reais.'),
('Risco Cibernetico', 'Classifica incidentes de segurança como ALTO ou BAIXO impacto financeiro usando colunas do dataset_ml.parquet (Prata -> Gold).'),
('Detecção de Intrusão', 'Análise de tráfego de rede para detecção de intrusões baseada em pacotes suspeitos, conexões anômalas e tráfego noturno.');

CREATE TEMP TABLE _v (id SERIAL, sys_id INT, name TEXT, role TEXT, min_v NUMERIC, max_v NUMERIC, ord INT);
INSERT INTO _v (sys_id, name, role, min_v, max_v, ord) VALUES
(1, 'probabilidade_ataque',  'antecedent', 0, 100, 0),
(1, 'impacto_financeiro',    'antecedent', 0, 100, 1),
(1, 'vulnerabilidade_sistema', 'antecedent', 0, 100, 2),
(1, 'nivel_risco',           'consequent', 0, 100, 3),
(2, 'temperatura',           'antecedent', 0,  50, 0),
(2, 'umidade',               'antecedent', 0, 100, 1),
(2, 'conforto',              'consequent', 0,  10, 2),
(3, 'receita_anual_usd',     'antecedent', 0, 1000000000, 0),
(3, 'total_funcionarios',    'antecedent', 0, 500000, 1),
(3, 'gravidade_ataque',      'antecedent', 0, 100, 2),
(3, 'impacto_financeiro',    'consequent', 0, 100, 3),
(4, 'pacotes_suspeitos',     'antecedent', 0, 100, 0),
(4, 'conexoes_anomalas',     'antecedent', 0, 100, 1),
(4, 'trafego_noturno',       'antecedent', 0, 100, 2),
(4, 'nivel_ameaca',          'consequent', 0, 100, 3);

CREATE TEMP TABLE _t (sys_id INT, var_ord INT, label TEXT, mf TEXT, params TEXT);
INSERT INTO _t VALUES
-- Sistema 1: prob_ataque
(1,0,'baixa','trapmf','[0,0,25,45]'),(1,0,'media','trimf','[30,50,70]'),(1,0,'alta','trapmf','[55,75,100,100]'),
-- Sistema 1: impacto_financeiro
(1,1,'baixo','trapmf','[0,0,25,45]'),(1,1,'medio','trimf','[30,50,70]'),(1,1,'alto','trapmf','[55,75,100,100]'),
-- Sistema 1: vulnerabilidade
(1,2,'baixa','trapmf','[0,0,20,40]'),(1,2,'media','trimf','[25,50,75]'),(1,2,'alta','trapmf','[60,80,100,100]'),
-- Sistema 1: nivel_risco (consequente — 5 termos)
(1,3,'muito_baixo','trimf','[0,0,20]'),(1,3,'baixo','trapmf','[10,20,35,45]'),
(1,3,'medio','trimf','[30,50,70]'),(1,3,'alto','trapmf','[55,70,85,95]'),(1,3,'critico','trimf','[80,100,100]'),
-- Sistema 2: temperatura
(2,0,'frio','trapmf','[0,0,15,22]'),(2,0,'agradavel','trimf','[18,24,30]'),(2,0,'quente','trapmf','[26,32,50,50]'),
-- Sistema 2: umidade
(2,1,'seco','trapmf','[0,0,30,50]'),(2,1,'normal','trimf','[40,55,70]'),(2,1,'umido','trapmf','[60,75,100,100]'),
-- Sistema 2: conforto
(2,2,'desconfortavel','trapmf','[0,0,3,5]'),(2,2,'neutro','trimf','[3,5,7]'),(2,2,'confortavel','trapmf','[5,7,10,10]'),
-- Sistema 3: receita
(3,0,'baixa','trapmf','[0,0,50000000,100000000]'),(3,0,'media','trimf','[50000000,200000000,500000000]'),(3,0,'alta','trapmf','[200000000,500000000,1000000000,1000000000]'),
-- Sistema 3: funcionarios
(3,1,'pequena','trapmf','[0,0,5000,20000]'),(3,1,'media','trimf','[5000,50000,150000]'),(3,1,'grande','trapmf','[50000,150000,500000,500000]'),
-- Sistema 3: gravidade
(3,2,'baixa','trapmf','[0,0,20,40]'),(3,2,'media','trimf','[20,50,70]'),(3,2,'alta','trapmf','[50,70,100,100]'),
-- Sistema 3: impacto
(3,3,'baixo','trapmf','[0,0,30,50]'),(3,3,'medio','trimf','[30,50,70]'),(3,3,'alto','trapmf','[50,70,100,100]'),
-- Sistema 4: pacotes
(4,0,'baixo','trapmf','[0,0,25,45]'),(4,0,'medio','trimf','[30,50,70]'),(4,0,'alto','trapmf','[55,75,100,100]'),
-- Sistema 4: conexoes
(4,1,'baixa','trapmf','[0,0,25,45]'),(4,1,'media','trimf','[30,50,70]'),(4,1,'alta','trapmf','[55,75,100,100]'),
-- Sistema 4: trafego
(4,2,'baixo','trapmf','[0,0,20,40]'),(4,2,'medio','trimf','[25,50,75]'),(4,2,'alto','trapmf','[60,80,100,100]'),
-- Sistema 4: nivel_ameaca
(4,3,'muito_baixo','trimf','[0,0,20]'),(4,3,'baixo','trapmf','[10,20,35,45]'),
(4,3,'medio','trimf','[30,50,70]'),(4,3,'alto','trapmf','[55,70,85,95]'),(4,3,'critico','trimf','[80,100,100]');

CREATE TEMP TABLE _r (sys_id INT, rule_text TEXT, pos INT);
INSERT INTO _r VALUES
(1, 'SE probabilidade_ataque e baixa E vulnerabilidade_sistema e baixa ENTAO nivel_risco e muito_baixo', 0),
(1, 'SE probabilidade_ataque e baixa E vulnerabilidade_sistema e media ENTAO nivel_risco e baixo', 1),
(1, 'SE probabilidade_ataque e media E vulnerabilidade_sistema e baixa ENTAO nivel_risco e baixo', 2),
(1, 'SE probabilidade_ataque e media E vulnerabilidade_sistema e media ENTAO nivel_risco e medio', 3),
(1, 'SE probabilidade_ataque e alta E vulnerabilidade_sistema e alta ENTAO nivel_risco e critico', 4),
(1, 'SE impacto_financeiro e alto E vulnerabilidade_sistema e alta ENTAO nivel_risco e critico', 5),
(1, 'SE impacto_financeiro e alto E probabilidade_ataque e alta ENTAO nivel_risco e critico', 6),
(1, 'SE impacto_financeiro e medio E vulnerabilidade_sistema e media ENTAO nivel_risco e medio', 7),
(1, 'SE probabilidade_ataque e alta E vulnerabilidade_sistema e media ENTAO nivel_risco e alto', 8),
(1, 'SE probabilidade_ataque e media E vulnerabilidade_sistema e alta ENTAO nivel_risco e alto', 9),
(1, 'SE impacto_financeiro e baixo E vulnerabilidade_sistema e baixa ENTAO nivel_risco e muito_baixo', 10),
(1, 'SE impacto_financeiro e alto E probabilidade_ataque e media ENTAO nivel_risco e alto', 11),
(2, 'SE temperatura e frio E umidade e seco ENTAO conforto e desconfortavel', 0),
(2, 'SE temperatura e frio E umidade e normal ENTAO conforto e neutro', 1),
(2, 'SE temperatura e frio E umidade e umido ENTAO conforto e desconfortavel', 2),
(2, 'SE temperatura e agradavel E umidade e seco ENTAO conforto e neutro', 3),
(2, 'SE temperatura e agradavel E umidade e normal ENTAO conforto e confortavel', 4),
(2, 'SE temperatura e agradavel E umidade e umido ENTAO conforto e neutro', 5),
(2, 'SE temperatura e quente E umidade e seco ENTAO conforto e desconfortavel', 6),
(2, 'SE temperatura e quente E umidade e normal ENTAO conforto e neutro', 7),
(2, 'SE temperatura e quente E umidade e umido ENTAO conforto e desconfortavel', 8),
(3, 'SE receita_anual_usd e baixa E total_funcionarios e pequena E gravidade_ataque e baixa ENTAO impacto_financeiro e baixo', 0),
(3, 'SE receita_anual_usd e baixa E total_funcionarios e pequena E gravidade_ataque e alta ENTAO impacto_financeiro e medio', 1),
(3, 'SE receita_anual_usd e baixa E total_funcionarios e grande E gravidade_ataque e alta ENTAO impacto_financeiro e alto', 2),
(3, 'SE receita_anual_usd e media E total_funcionarios e media E gravidade_ataque e baixa ENTAO impacto_financeiro e baixo', 3),
(3, 'SE receita_anual_usd e media E total_funcionarios e media E gravidade_ataque e media ENTAO impacto_financeiro e medio', 4),
(3, 'SE receita_anual_usd e media E total_funcionarios e media E gravidade_ataque e alta ENTAO impacto_financeiro e alto', 5),
(3, 'SE receita_anual_usd e alta E total_funcionarios e grande E gravidade_ataque e baixa ENTAO impacto_financeiro e medio', 6),
(3, 'SE receita_anual_usd e alta E total_funcionarios e grande E gravidade_ataque e media ENTAO impacto_financeiro e alto', 7),
(3, 'SE receita_anual_usd e alta E total_funcionarios e grande E gravidade_ataque e alta ENTAO impacto_financeiro e alto', 8),
(4, 'SE pacotes_suspeitos e baixo E conexoes_anomalas e baixa ENTAO nivel_ameaca e muito_baixo', 0),
(4, 'SE pacotes_suspeitos e baixo E conexoes_anomalas e media ENTAO nivel_ameaca e baixo', 1),
(4, 'SE pacotes_suspeitos e medio E conexoes_anomalas e baixa ENTAO nivel_ameaca e baixo', 2),
(4, 'SE pacotes_suspeitos e medio E conexoes_anomalas e media ENTAO nivel_ameaca e medio', 3),
(4, 'SE pacotes_suspeitos e alto E conexoes_anomalas e alta ENTAO nivel_ameaca e critico', 4),
(4, 'SE trafego_noturno e alto E conexoes_anomalas e alta ENTAO nivel_ameaca e critico', 5),
(4, 'SE trafego_noturno e alto E pacotes_suspeitos e alto ENTAO nivel_ameaca e critico', 6),
(4, 'SE trafego_noturno e medio E conexoes_anomalas e media ENTAO nivel_ameaca e medio', 7),
(4, 'SE pacotes_suspeitos e alto E conexoes_anomalas e media ENTAO nivel_ameaca e alto', 8),
(4, 'SE pacotes_suspeitos e medio E conexoes_anomalas e alta ENTAO nivel_ameaca e alto', 9),
(4, 'SE trafego_noturno e baixo E conexoes_anomalas e baixa ENTAO nivel_ameaca e muito_baixo', 10),
(4, 'SE trafego_noturno e alto E pacotes_suspeitos e medio ENTAO nivel_ameaca e alto', 11);

CREATE TEMP TABLE _sc (sys_id INT, name TEXT, inputs TEXT);
INSERT INTO _sc VALUES
(1, 'Sistema interno de baixo risco',       '{"probabilidade_ataque":10,"impacto_financeiro":15,"vulnerabilidade_sistema":10}'),
(1, 'Equipe com backups e atualizações',     '{"probabilidade_ataque":15,"impacto_financeiro":20,"vulnerabilidade_sistema":15}'),
(1, 'Rede com monitoramento SIEM',           '{"probabilidade_ataque":20,"impacto_financeiro":35,"vulnerabilidade_sistema":15}'),
(1, 'Firewall e antivírus atualizados',      '{"probabilidade_ataque":25,"impacto_financeiro":30,"vulnerabilidade_sistema":25}'),
(1, 'Phishing interno empresa pequena',      '{"probabilidade_ataque":40,"impacto_financeiro":20,"vulnerabilidade_sistema":30}'),
(1, 'Firewall desatualizado rede media',     '{"probabilidade_ataque":50,"impacto_financeiro":40,"vulnerabilidade_sistema":55}'),
(1, 'Phishing sem treinamento funcionarios', '{"probabilidade_ataque":60,"impacto_financeiro":30,"vulnerabilidade_sistema":50}'),
(1, 'Acesso privilegiado suspeito',          '{"probabilidade_ataque":45,"impacto_financeiro":55,"vulnerabilidade_sistema":80}'),
(1, 'Senhas fracas sistema financeiro',      '{"probabilidade_ataque":55,"impacto_financeiro":70,"vulnerabilidade_sistema":65}'),
(1, 'Sistema legado exposto internet',       '{"probabilidade_ataque":70,"impacto_financeiro":50,"vulnerabilidade_sistema":85}'),
(1, 'Servidor critico sem patch',            '{"probabilidade_ataque":85,"impacto_financeiro":90,"vulnerabilidade_sistema":95}'),
(1, 'Ransomware infraestrutura critica',     '{"probabilidade_ataque":80,"impacto_financeiro":95,"vulnerabilidade_sistema":70}'),
(1, 'DDoS servico bancario',                 '{"probabilidade_ataque":95,"impacto_financeiro":85,"vulnerabilidade_sistema":75}'),
(1, 'Vazamento dados via API insegura',      '{"probabilidade_ataque":75,"impacto_financeiro":90,"vulnerabilidade_sistema":85}'),
(2, 'Dia frio e seco em Curitiba',          '{"temperatura":10,"umidade":30}'),
(2, 'Dia frio e úmido em São Paulo',        '{"temperatura":12,"umidade":85}'),
(2, 'Manhã amena em Belo Horizonte',        '{"temperatura":20,"umidade":55}'),
(2, 'Tarde agradável no Rio de Janeiro',    '{"temperatura":25,"umidade":50}'),
(2, 'Dia quente e seco em Brasília',        '{"temperatura":30,"umidade":25}'),
(2, 'Calor úmido em Manaus',                '{"temperatura":35,"umidade":90}'),
(2, 'Verão em Salvador',                    '{"temperatura":32,"umidade":75}'),
(2, 'Noite amena em Florianópolis',         '{"temperatura":22,"umidade":65}'),
(2, 'Inverno em Porto Alegre',              '{"temperatura":8,"umidade":70}'),
(2, 'Tarde quente e seca em Cuiabá',        '{"temperatura":40,"umidade":15}'),
(3, 'Startup phishing baixo impacto',    '{"receita_anual_usd":1000000,"total_funcionarios":50,"gravidade_ataque":20}'),
(3, 'Media empresa ataque baixo',        '{"receita_anual_usd":100000000,"total_funcionarios":5000,"gravidade_ataque":15}'),
(3, 'Grande empresa ataque minimo',      '{"receita_anual_usd":800000000,"total_funcionarios":200000,"gravidade_ataque":10}'),
(3, 'Startup ransomware medio impacto',  '{"receita_anual_usd":5000000,"total_funcionarios":100,"gravidade_ataque":85}'),
(3, 'Media empresa malware moderado',    '{"receita_anual_usd":200000000,"total_funcionarios":40000,"gravidade_ataque":45}'),
(3, 'Grande empresa phishing velado',    '{"receita_anual_usd":500000000,"total_funcionarios":100000,"gravidade_ataque":25}'),
(3, 'Media empresa ransomware alto',     '{"receita_anual_usd":150000000,"total_funcionarios":30000,"gravidade_ataque":90}'),
(3, 'Grande empresa data breach',        '{"receita_anual_usd":900000000,"total_funcionarios":250000,"gravidade_ataque":75}'),
(3, 'Corp ransomware maximo impacto',    '{"receita_anual_usd":1000000000,"total_funcionarios":400000,"gravidade_ataque":95}'),
(4, 'Tráfego normal horário comercial',          '{"pacotes_suspeitos":5,"conexoes_anomalas":3,"trafego_noturno":10}'),
(4, 'Pico de acesso legítimo',                    '{"pacotes_suspeitos":20,"conexoes_anomalas":15,"trafego_noturno":25}'),
(4, 'Varredura de porta suspeita',                '{"pacotes_suspeitos":65,"conexoes_anomalas":40,"trafego_noturno":30}'),
(4, 'Múltiplas conexões fora do horário',         '{"pacotes_suspeitos":40,"conexoes_anomalas":55,"trafego_noturno":80}'),
(4, 'Ataque DDoS noturno',                        '{"pacotes_suspeitos":95,"conexoes_anomalas":90,"trafego_noturno":85}'),
(4, 'Tentativa de brute force SSH',               '{"pacotes_suspeitos":80,"conexoes_anomalas":70,"trafego_noturno":60}'),
(4, 'Tráfego suspeito madrugada',                 '{"pacotes_suspeitos":55,"conexoes_anomalas":60,"trafego_noturno":90}'),
(4, 'Exfiltração de dados lenta',                 '{"pacotes_suspeitos":70,"conexoes_anomalas":80,"trafego_noturno":50}'),
(4, 'Rede interna monitorada sem ameaças',        '{"pacotes_suspeitos":8,"conexoes_anomalas":5,"trafego_noturno":12}'),
(4, 'Horário comercial com anomalias leves',      '{"pacotes_suspeitos":30,"conexoes_anomalas":25,"trafego_noturno":15}');

-- ═══════════════════════════════════════════════════════════════════════════
-- LOOP ÚNICO: cria todos os 4 sistemas programaticamente
-- ═══════════════════════════════════════════════════════════════════════════
DO $$
DECLARE
    s RECORD; v RECORD; t RECORD; r RECORD; sc RECORD;
    sys_uuid UUID; var_uuid UUID;
    nv INT; nt INT; nr INT; nsc INT;
BEGIN
    FOR s IN SELECT * FROM _s ORDER BY id LOOP
        INSERT INTO fuzzy_systems (id, name, description, defuzz_method)
        VALUES (uuid_generate_v4(), s.name, s.descr, 'centroid')
        RETURNING id INTO sys_uuid;

        nv := 0; nt := 0;
        FOR v IN SELECT * FROM _v WHERE sys_id = s.id ORDER BY ord LOOP
            INSERT INTO fuzzy_variables (id, system_id, name, role, universe_min, universe_max, resolution)
            VALUES (uuid_generate_v4(), sys_uuid, v.name, v.role, v.min_v, v.max_v, 501)
            RETURNING id INTO var_uuid;
            nv := nv + 1;

            FOR t IN SELECT * FROM _t WHERE sys_id = s.id AND var_ord = v.ord LOOP
                INSERT INTO fuzzy_terms (variable_id, label, mf_type, params)
                VALUES (var_uuid, t.label, t.mf, t.params::jsonb);
                nt := nt + 1;
            END LOOP;
        END LOOP;

        nr := 0;
        FOR r IN SELECT * FROM _r WHERE sys_id = s.id ORDER BY pos LOOP
            INSERT INTO fuzzy_rules (system_id, rule_text, weight, position)
            VALUES (sys_uuid, r.rule_text, 1.0, r.pos);
            nr := nr + 1;
        END LOOP;

        nsc := 0;
        FOR sc IN SELECT * FROM _sc WHERE sys_id = s.id LOOP
            INSERT INTO scenarios (system_id, name, inputs)
            VALUES (sys_uuid, sc.name, sc.inputs::jsonb);
            nsc := nsc + 1;
        END LOOP;

        RAISE NOTICE 'Sistema %: % (%) — % var, % termos, % regras, % cenarios',
            s.id, s.name, CASE s.id WHEN 1 THEN 'Risco Cibernético Avançado' WHEN 2 THEN 'Conforto Térmico' WHEN 3 THEN 'dataset_ml' ELSE 'Detecção de Intrusão' END,
            nv, nt, nr, nsc;
    END LOOP;
END $$;
