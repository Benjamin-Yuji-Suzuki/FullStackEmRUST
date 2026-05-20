CREATE TABLE optimizations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    system_id UUID REFERENCES fuzzy_systems(id) ON DELETE SET NULL,
    coef_a FLOAT NOT NULL,
    coef_b FLOAT NOT NULL,
    coef_c FLOAT NOT NULL,
    coef_d FLOAT NOT NULL,
    coef_e FLOAT NOT NULL,
    coef_f FLOAT NOT NULL,
    x_min FLOAT NOT NULL,
    x_max FLOAT NOT NULL,
    y_min FLOAT NOT NULL,
    y_max FLOAT NOT NULL,
    optimal_x FLOAT,
    optimal_y FLOAT,
    optimal_value FLOAT,
    critical_point_type TEXT,
    explanation TEXT,
    gradient_at_optimum JSONB,
    hessian_matrix JSONB,
    executed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_optimizations_system ON optimizations(system_id);
CREATE INDEX idx_optimizations_executed ON optimizations(executed_at DESC);
