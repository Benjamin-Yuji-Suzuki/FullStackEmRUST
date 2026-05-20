ALTER TABLE fuzzy_systems ADD COLUMN IF NOT EXISTS status TEXT NOT NULL DEFAULT 'ativo';
CREATE INDEX IF NOT EXISTS idx_systems_status ON fuzzy_systems(status);
