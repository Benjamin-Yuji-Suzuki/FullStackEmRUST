-- Tornar system_id nullable e mudar FK para SET NULL
-- para que eventos de auditoria sobrevivam à exclusão do sistema

DO $$
DECLARE
    constraint_name TEXT;
BEGIN
    SELECT conname INTO constraint_name
    FROM pg_constraint
    WHERE conrelid = 'audit_events'::regclass
      AND contype = 'f'
      AND connamespace = 'public'::regnamespace;
    
    IF constraint_name IS NOT NULL THEN
        EXECUTE 'ALTER TABLE audit_events DROP CONSTRAINT ' || constraint_name;
    END IF;
END $$;

ALTER TABLE audit_events ALTER COLUMN system_id DROP NOT NULL;

ALTER TABLE audit_events ADD CONSTRAINT audit_events_system_id_fkey
    FOREIGN KEY (system_id) REFERENCES fuzzy_systems(id) ON DELETE SET NULL;

CREATE INDEX IF NOT EXISTS idx_audit_system_null ON audit_events(system_id) WHERE system_id IS NULL;
