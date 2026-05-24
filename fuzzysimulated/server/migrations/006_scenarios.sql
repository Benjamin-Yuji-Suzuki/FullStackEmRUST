-- Migration 006: Cenários de simulação (UC12)
CREATE TABLE IF NOT EXISTS scenarios (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    system_id UUID NOT NULL REFERENCES fuzzy_systems(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    inputs JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_scenarios_system_id ON scenarios(system_id);
