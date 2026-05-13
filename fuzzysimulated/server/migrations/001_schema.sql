CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE fuzzy_systems (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    description TEXT,
    defuzz_method TEXT NOT NULL DEFAULT 'centroid',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE fuzzy_variables (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    system_id UUID NOT NULL REFERENCES fuzzy_systems(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('antecedent', 'consequent')),
    universe_min FLOAT NOT NULL,
    universe_max FLOAT NOT NULL,
    resolution INT NOT NULL DEFAULT 501
);

CREATE TABLE fuzzy_terms (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    variable_id UUID NOT NULL REFERENCES fuzzy_variables(id) ON DELETE CASCADE,
    label TEXT NOT NULL,
    mf_type TEXT NOT NULL CHECK (mf_type IN ('trimf', 'trapmf', 'gaussmf')),
    params JSONB NOT NULL
);

CREATE TABLE fuzzy_rules (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    system_id UUID NOT NULL REFERENCES fuzzy_systems(id) ON DELETE CASCADE,
    rule_text TEXT NOT NULL,
    weight FLOAT NOT NULL DEFAULT 1.0,
    position INT NOT NULL DEFAULT 0
);

CREATE TABLE simulations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    system_id UUID NOT NULL REFERENCES fuzzy_systems(id) ON DELETE CASCADE,
    inputs JSONB NOT NULL,
    outputs JSONB NOT NULL,
    weather_data JSONB,
    city TEXT,
    executed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE batch_results (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    system_id UUID NOT NULL REFERENCES fuzzy_systems(id) ON DELETE CASCADE,
    source_file TEXT NOT NULL,
    row_index INT NOT NULL,
    inputs JSONB NOT NULL,
    output FLOAT NOT NULL,
    executed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE audit_events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    system_id UUID NOT NULL REFERENCES fuzzy_systems(id) ON DELETE CASCADE,
    action_type TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id UUID,
    description TEXT NOT NULL,
    snapshot_before JSONB,
    snapshot_after JSONB,
    redo_stack BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE scenarios (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    system_id UUID NOT NULL REFERENCES fuzzy_systems(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    inputs JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_variables_system ON fuzzy_variables(system_id);
CREATE INDEX idx_terms_variable ON fuzzy_terms(variable_id);
CREATE INDEX idx_rules_system ON fuzzy_rules(system_id);
CREATE INDEX idx_simulations_system ON simulations(system_id);
CREATE INDEX idx_simulations_executed ON simulations(executed_at DESC);
CREATE INDEX idx_batch_system ON batch_results(system_id);
CREATE INDEX idx_audit_system ON audit_events(system_id);
CREATE INDEX idx_audit_created ON audit_events(created_at DESC);
CREATE INDEX idx_scenarios_system ON scenarios(system_id);
