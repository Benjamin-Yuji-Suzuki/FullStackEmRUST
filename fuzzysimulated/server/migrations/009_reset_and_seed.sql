-- Full reset + reseed: 4 sistemas fuzzy
-- Dados em JSONB constante, lógica em loops (zero literais SQL duplicados)

TRUNCATE fuzzy_systems CASCADE;

DO $$
DECLARE
    _res CONSTANT INT := 501;

    _src CONSTANT JSONB := '[
  {"n":"Risco Cibernético Avançado",
   "d":"Avaliação de risco cibernético considerando probabilidade de ataque, impacto financeiro e vulnerabilidade do sistema.",
   "v":[{"n":"probabilidade_ataque","r":"antecedent","mn":0,"mx":100,
         "t":[{"l":"baixa","m":"trapmf","p":[0,0,25,45]},{"l":"media","m":"trimf","p":[30,50,70]},{"l":"alta","m":"trapmf","p":[55,75,100,100]}]},
        {"n":"impacto_financeiro","r":"antecedent","mn":0,"mx":100,
         "t":[{"l":"baixo","m":"trapmf","p":[0,0,25,45]},{"l":"medio","m":"trimf","p":[30,50,70]},{"l":"alto","m":"trapmf","p":[55,75,100,100]}]},
        {"n":"vulnerabilidade_sistema","r":"antecedent","mn":0,"mx":100,
         "t":[{"l":"baixa","m":"trapmf","p":[0,0,20,40]},{"l":"media","m":"trimf","p":[25,50,75]},{"l":"alta","m":"trapmf","p":[60,80,100,100]}]},
        {"n":"nivel_risco","r":"consequent","mn":0,"mx":100,
         "t":[{"l":"muito_baixo","m":"trimf","p":[0,0,20]},{"l":"baixo","m":"trapmf","p":[10,20,35,45]},{"l":"medio","m":"trimf","p":[30,50,70]},{"l":"alto","m":"trapmf","p":[55,70,85,95]},{"l":"critico","m":"trimf","p":[80,100,100]}]}],
   "r":[{"t":"SE probabilidade_ataque e baixa E vulnerabilidade_sistema e baixa ENTAO nivel_risco e muito_baixo","p":0},
        {"t":"SE probabilidade_ataque e baixa E vulnerabilidade_sistema e media ENTAO nivel_risco e baixo","p":1},
        {"t":"SE probabilidade_ataque e media E vulnerabilidade_sistema e baixa ENTAO nivel_risco e baixo","p":2},
        {"t":"SE probabilidade_ataque e media E vulnerabilidade_sistema e media ENTAO nivel_risco e medio","p":3},
        {"t":"SE probabilidade_ataque e alta E vulnerabilidade_sistema e alta ENTAO nivel_risco e critico","p":4},
        {"t":"SE impacto_financeiro e alto E vulnerabilidade_sistema e alta ENTAO nivel_risco e critico","p":5},
        {"t":"SE impacto_financeiro e alto E probabilidade_ataque e alta ENTAO nivel_risco e critico","p":6},
        {"t":"SE impacto_financeiro e medio E vulnerabilidade_sistema e media ENTAO nivel_risco e medio","p":7},
        {"t":"SE probabilidade_ataque e alta E vulnerabilidade_sistema e media ENTAO nivel_risco e alto","p":8},
        {"t":"SE probabilidade_ataque e media E vulnerabilidade_sistema e alta ENTAO nivel_risco e alto","p":9},
        {"t":"SE impacto_financeiro e baixo E vulnerabilidade_sistema e baixa ENTAO nivel_risco e muito_baixo","p":10},
        {"t":"SE impacto_financeiro e alto E probabilidade_ataque e media ENTAO nivel_risco e alto","p":11}],
   "s":[{"n":"Sistema interno de baixo risco","i":{"probabilidade_ataque":10,"impacto_financeiro":15,"vulnerabilidade_sistema":10}},
        {"n":"Equipe com backups e atualizações","i":{"probabilidade_ataque":15,"impacto_financeiro":20,"vulnerabilidade_sistema":15}},
        {"n":"Rede com monitoramento SIEM","i":{"probabilidade_ataque":20,"impacto_financeiro":35,"vulnerabilidade_sistema":15}},
        {"n":"Firewall e antivírus atualizados","i":{"probabilidade_ataque":25,"impacto_financeiro":30,"vulnerabilidade_sistema":25}},
        {"n":"Phishing interno empresa pequena","i":{"probabilidade_ataque":40,"impacto_financeiro":20,"vulnerabilidade_sistema":30}},
        {"n":"Firewall desatualizado rede media","i":{"probabilidade_ataque":50,"impacto_financeiro":40,"vulnerabilidade_sistema":55}},
        {"n":"Phishing sem treinamento funcionarios","i":{"probabilidade_ataque":60,"impacto_financeiro":30,"vulnerabilidade_sistema":50}},
        {"n":"Acesso privilegiado suspeito","i":{"probabilidade_ataque":45,"impacto_financeiro":55,"vulnerabilidade_sistema":80}},
        {"n":"Senhas fracas sistema financeiro","i":{"probabilidade_ataque":55,"impacto_financeiro":70,"vulnerabilidade_sistema":65}},
        {"n":"Sistema legado exposto internet","i":{"probabilidade_ataque":70,"impacto_financeiro":50,"vulnerabilidade_sistema":85}},
        {"n":"Servidor critico sem patch","i":{"probabilidade_ataque":85,"impacto_financeiro":90,"vulnerabilidade_sistema":95}},
        {"n":"Ransomware infraestrutura critica","i":{"probabilidade_ataque":80,"impacto_financeiro":95,"vulnerabilidade_sistema":70}},
        {"n":"DDoS servico bancario","i":{"probabilidade_ataque":95,"impacto_financeiro":85,"vulnerabilidade_sistema":75}},
        {"n":"Vazamento dados via API insegura","i":{"probabilidade_ataque":75,"impacto_financeiro":90,"vulnerabilidade_sistema":85}}]},

  {"n":"Conforto Térmico",
   "d":"Avaliação de conforto térmico baseado em temperatura e umidade com dados da API OpenWeather. Utiliza cidade e dados meteorológicos reais.",
   "v":[{"n":"temperatura","r":"antecedent","mn":0,"mx":50,
         "t":[{"l":"frio","m":"trapmf","p":[0,0,15,22]},{"l":"agradavel","m":"trimf","p":[18,24,30]},{"l":"quente","m":"trapmf","p":[26,32,50,50]}]},
        {"n":"umidade","r":"antecedent","mn":0,"mx":100,
         "t":[{"l":"seco","m":"trapmf","p":[0,0,30,50]},{"l":"normal","m":"trimf","p":[40,55,70]},{"l":"umido","m":"trapmf","p":[60,75,100,100]}]},
        {"n":"conforto","r":"consequent","mn":0,"mx":10,
         "t":[{"l":"desconfortavel","m":"trapmf","p":[0,0,3,5]},{"l":"neutro","m":"trimf","p":[3,5,7]},{"l":"confortavel","m":"trapmf","p":[5,7,10,10]}]}],
   "r":[{"t":"SE temperatura e frio E umidade e seco ENTAO conforto e desconfortavel","p":0},
        {"t":"SE temperatura e frio E umidade e normal ENTAO conforto e neutro","p":1},
        {"t":"SE temperatura e frio E umidade e umido ENTAO conforto e desconfortavel","p":2},
        {"t":"SE temperatura e agradavel E umidade e seco ENTAO conforto e neutro","p":3},
        {"t":"SE temperatura e agradavel E umidade e normal ENTAO conforto e confortavel","p":4},
        {"t":"SE temperatura e agradavel E umidade e umido ENTAO conforto e neutro","p":5},
        {"t":"SE temperatura e quente E umidade e seco ENTAO conforto e desconfortavel","p":6},
        {"t":"SE temperatura e quente E umidade e normal ENTAO conforto e neutro","p":7},
        {"t":"SE temperatura e quente E umidade e umido ENTAO conforto e desconfortavel","p":8}],
   "s":[{"n":"Dia frio e seco em Curitiba","i":{"temperatura":10,"umidade":30}},
        {"n":"Dia frio e úmido em São Paulo","i":{"temperatura":12,"umidade":85}},
        {"n":"Manhã amena em Belo Horizonte","i":{"temperatura":20,"umidade":55}},
        {"n":"Tarde agradável no Rio de Janeiro","i":{"temperatura":25,"umidade":50}},
        {"n":"Dia quente e seco em Brasília","i":{"temperatura":30,"umidade":25}},
        {"n":"Calor úmido em Manaus","i":{"temperatura":35,"umidade":90}},
        {"n":"Verão em Salvador","i":{"temperatura":32,"umidade":75}},
        {"n":"Noite amena em Florianópolis","i":{"temperatura":22,"umidade":65}},
        {"n":"Inverno em Porto Alegre","i":{"temperatura":8,"umidade":70}},
        {"n":"Tarde quente e seca em Cuiabá","i":{"temperatura":40,"umidade":15}}]},

  {"n":"Risco Cibernetico",
   "d":"Classifica incidentes de segurança como ALTO ou BAIXO impacto financeiro usando colunas do dataset_ml.parquet (Prata -> Gold).",
   "v":[{"n":"receita_anual_usd","r":"antecedent","mn":0,"mx":1000000000,
         "t":[{"l":"baixa","m":"trapmf","p":[0,0,50000000,100000000]},{"l":"media","m":"trimf","p":[50000000,200000000,500000000]},{"l":"alta","m":"trapmf","p":[200000000,500000000,1000000000,1000000000]}]},
        {"n":"total_funcionarios","r":"antecedent","mn":0,"mx":500000,
         "t":[{"l":"pequena","m":"trapmf","p":[0,0,5000,20000]},{"l":"media","m":"trimf","p":[5000,50000,150000]},{"l":"grande","m":"trapmf","p":[50000,150000,500000,500000]}]},
        {"n":"gravidade_ataque","r":"antecedent","mn":0,"mx":100,
         "t":[{"l":"baixa","m":"trapmf","p":[0,0,20,40]},{"l":"media","m":"trimf","p":[20,50,70]},{"l":"alta","m":"trapmf","p":[50,70,100,100]}]},
        {"n":"impacto_financeiro","r":"consequent","mn":0,"mx":100,
         "t":[{"l":"baixo","m":"trapmf","p":[0,0,30,50]},{"l":"medio","m":"trimf","p":[30,50,70]},{"l":"alto","m":"trapmf","p":[50,70,100,100]}]}],
   "r":[{"t":"SE receita_anual_usd e baixa E total_funcionarios e pequena E gravidade_ataque e baixa ENTAO impacto_financeiro e baixo","p":0},
        {"t":"SE receita_anual_usd e baixa E total_funcionarios e pequena E gravidade_ataque e alta ENTAO impacto_financeiro e medio","p":1},
        {"t":"SE receita_anual_usd e baixa E total_funcionarios e grande E gravidade_ataque e alta ENTAO impacto_financeiro e alto","p":2},
        {"t":"SE receita_anual_usd e media E total_funcionarios e media E gravidade_ataque e baixa ENTAO impacto_financeiro e baixo","p":3},
        {"t":"SE receita_anual_usd e media E total_funcionarios e media E gravidade_ataque e media ENTAO impacto_financeiro e medio","p":4},
        {"t":"SE receita_anual_usd e media E total_funcionarios e media E gravidade_ataque e alta ENTAO impacto_financeiro e alto","p":5},
        {"t":"SE receita_anual_usd e alta E total_funcionarios e grande E gravidade_ataque e baixa ENTAO impacto_financeiro e medio","p":6},
        {"t":"SE receita_anual_usd e alta E total_funcionarios e grande E gravidade_ataque e media ENTAO impacto_financeiro e alto","p":7},
        {"t":"SE receita_anual_usd e alta E total_funcionarios e grande E gravidade_ataque e alta ENTAO impacto_financeiro e alto","p":8}],
   "s":[{"n":"Startup phishing baixo impacto","i":{"receita_anual_usd":1000000,"total_funcionarios":50,"gravidade_ataque":20}},
        {"n":"Media empresa ataque baixo","i":{"receita_anual_usd":100000000,"total_funcionarios":5000,"gravidade_ataque":15}},
        {"n":"Grande empresa ataque minimo","i":{"receita_anual_usd":800000000,"total_funcionarios":200000,"gravidade_ataque":10}},
        {"n":"Startup ransomware medio impacto","i":{"receita_anual_usd":5000000,"total_funcionarios":100,"gravidade_ataque":85}},
        {"n":"Media empresa malware moderado","i":{"receita_anual_usd":200000000,"total_funcionarios":40000,"gravidade_ataque":45}},
        {"n":"Grande empresa phishing velado","i":{"receita_anual_usd":500000000,"total_funcionarios":100000,"gravidade_ataque":25}},
        {"n":"Media empresa ransomware alto","i":{"receita_anual_usd":150000000,"total_funcionarios":30000,"gravidade_ataque":90}},
        {"n":"Grande empresa data breach","i":{"receita_anual_usd":900000000,"total_funcionarios":250000,"gravidade_ataque":75}},
        {"n":"Corp ransomware maximo impacto","i":{"receita_anual_usd":1000000000,"total_funcionarios":400000,"gravidade_ataque":95}}]},

  {"n":"Detecção de Intrusão",
   "d":"Análise de tráfego de rede para detecção de intrusões baseada em pacotes suspeitos, conexões anômalas e tráfego noturno.",
   "v":[{"n":"pacotes_suspeitos","r":"antecedent","mn":0,"mx":100,
         "t":[{"l":"baixo","m":"trapmf","p":[0,0,25,45]},{"l":"medio","m":"trimf","p":[30,50,70]},{"l":"alto","m":"trapmf","p":[55,75,100,100]}]},
        {"n":"conexoes_anomalas","r":"antecedent","mn":0,"mx":100,
         "t":[{"l":"baixa","m":"trapmf","p":[0,0,25,45]},{"l":"media","m":"trimf","p":[30,50,70]},{"l":"alta","m":"trapmf","p":[55,75,100,100]}]},
        {"n":"trafego_noturno","r":"antecedent","mn":0,"mx":100,
         "t":[{"l":"baixo","m":"trapmf","p":[0,0,20,40]},{"l":"medio","m":"trimf","p":[25,50,75]},{"l":"alto","m":"trapmf","p":[60,80,100,100]}]},
        {"n":"nivel_ameaca","r":"consequent","mn":0,"mx":100,
         "t":[{"l":"muito_baixo","m":"trimf","p":[0,0,20]},{"l":"baixo","m":"trapmf","p":[10,20,35,45]},{"l":"medio","m":"trimf","p":[30,50,70]},{"l":"alto","m":"trapmf","p":[55,70,85,95]},{"l":"critico","m":"trimf","p":[80,100,100]}]}],
   "r":[{"t":"SE pacotes_suspeitos e baixo E conexoes_anomalas e baixa ENTAO nivel_ameaca e muito_baixo","p":0},
        {"t":"SE pacotes_suspeitos e baixo E conexoes_anomalas e media ENTAO nivel_ameaca e baixo","p":1},
        {"t":"SE pacotes_suspeitos e medio E conexoes_anomalas e baixa ENTAO nivel_ameaca e baixo","p":2},
        {"t":"SE pacotes_suspeitos e medio E conexoes_anomalas e media ENTAO nivel_ameaca e medio","p":3},
        {"t":"SE pacotes_suspeitos e alto E conexoes_anomalas e alta ENTAO nivel_ameaca e critico","p":4},
        {"t":"SE trafego_noturno e alto E conexoes_anomalas e alta ENTAO nivel_ameaca e critico","p":5},
        {"t":"SE trafego_noturno e alto E pacotes_suspeitos e alto ENTAO nivel_ameaca e critico","p":6},
        {"t":"SE trafego_noturno e medio E conexoes_anomalas e media ENTAO nivel_ameaca e medio","p":7},
        {"t":"SE pacotes_suspeitos e alto E conexoes_anomalas e media ENTAO nivel_ameaca e alto","p":8},
        {"t":"SE pacotes_suspeitos e medio E conexoes_anomalas e alta ENTAO nivel_ameaca e alto","p":9},
        {"t":"SE trafego_noturno e baixo E conexoes_anomalas e baixa ENTAO nivel_ameaca e muito_baixo","p":10},
        {"t":"SE trafego_noturno e alto E pacotes_suspeitos e medio ENTAO nivel_ameaca e alto","p":11}],
   "s":[{"n":"Tráfego normal horário comercial","i":{"pacotes_suspeitos":5,"conexoes_anomalas":3,"trafego_noturno":10}},
        {"n":"Pico de acesso legítimo","i":{"pacotes_suspeitos":20,"conexoes_anomalas":15,"trafego_noturno":25}},
        {"n":"Varredura de porta suspeita","i":{"pacotes_suspeitos":65,"conexoes_anomalas":40,"trafego_noturno":30}},
        {"n":"Múltiplas conexões fora do horário","i":{"pacotes_suspeitos":40,"conexoes_anomalas":55,"trafego_noturno":80}},
        {"n":"Ataque DDoS noturno","i":{"pacotes_suspeitos":95,"conexoes_anomalas":90,"trafego_noturno":85}},
        {"n":"Tentativa de brute force SSH","i":{"pacotes_suspeitos":80,"conexoes_anomalas":70,"trafego_noturno":60}},
        {"n":"Tráfego suspeito madrugada","i":{"pacotes_suspeitos":55,"conexoes_anomalas":60,"trafego_noturno":90}},
        {"n":"Exfiltração de dados lenta","i":{"pacotes_suspeitos":70,"conexoes_anomalas":80,"trafego_noturno":50}},
        {"n":"Rede interna monitorada sem ameaças","i":{"pacotes_suspeitos":8,"conexoes_anomalas":5,"trafego_noturno":12}},
        {"n":"Horário comercial com anomalias leves","i":{"pacotes_suspeitos":30,"conexoes_anomalas":25,"trafego_noturno":15}}]}
]'::jsonb;

    _sys JSONB; _var JSONB; _term JSONB; _rule JSONB; _sc JSONB;
    _sys_uuid UUID; _var_uuid UUID;
    _nv INT; _nt INT; _nr INT; _nsc INT;
BEGIN
    FOR _sys IN SELECT * FROM jsonb_array_elements(_src) LOOP
        INSERT INTO fuzzy_systems (id, name, description, defuzz_method)
        VALUES (uuid_generate_v4(), _sys->>'n', _sys->>'d', 'centroid')
        RETURNING id INTO _sys_uuid;

        _nv := 0; _nt := 0;
        FOR _var IN SELECT * FROM jsonb_array_elements(_sys->'v') LOOP
            INSERT INTO fuzzy_variables (id, system_id, name, role, universe_min, universe_max, resolution)
            VALUES (uuid_generate_v4(), _sys_uuid, _var->>'n', _var->>'r', (_var->>'mn')::numeric, (_var->>'mx')::numeric, _res)
            RETURNING id INTO _var_uuid;
            _nv := _nv + 1;

            FOR _term IN SELECT * FROM jsonb_array_elements(_var->'t') LOOP
                INSERT INTO fuzzy_terms (variable_id, label, mf_type, params)
                VALUES (_var_uuid, _term->>'l', _term->>'m', _term->'p');
                _nt := _nt + 1;
            END LOOP;
        END LOOP;

        _nr := 0;
        FOR _rule IN SELECT * FROM jsonb_array_elements(COALESCE(_sys->'r', '[]'::jsonb)) LOOP
            INSERT INTO fuzzy_rules (system_id, rule_text, weight, position)
            VALUES (_sys_uuid, _rule->>'t', 1.0, (_rule->>'p')::int);
            _nr := _nr + 1;
        END LOOP;

        _nsc := 0;
        FOR _sc IN SELECT * FROM jsonb_array_elements(COALESCE(_sys->'s', '[]'::jsonb)) LOOP
            INSERT INTO scenarios (system_id, name, inputs)
            VALUES (_sys_uuid, _sc->>'n', _sc->'i');
            _nsc := _nsc + 1;
        END LOOP;

        RAISE NOTICE 'Sistema %: % — % var, % termos, % regras, % cenarios',
            _sys->>'n', CASE _sys->>'n'
                WHEN 'Risco Cibernético Avançado' THEN 'Risco Cibernético Avançado'
                WHEN 'Conforto Térmico' THEN 'Conforto Térmico'
                WHEN 'Risco Cibernetico' THEN 'dataset_ml'
                ELSE 'Detecção de Intrusão'
            END, _nv, _nt, _nr, _nsc;
    END LOOP;
END $$;
