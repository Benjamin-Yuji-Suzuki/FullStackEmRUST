use server::math::{OptimizationInput, solve_quadratic_optimization};

fn make_input(
    a: f64, b: f64, c: f64, d: f64, e: f64, f: f64,
    xmin: f64, xmax: f64, ymin: f64, ymax: f64,
) -> OptimizationInput {
    OptimizationInput {
        coef_a: a, coef_b: b, coef_c: c,
        coef_d: d, coef_e: e, coef_f: f,
        x_min: xmin, x_max: xmax, y_min: ymin, y_max: ymax,
    }
}

#[test]
fn test_optimize_paraboloid_minimo() {
    let result = solve_quadratic_optimization(&make_input(1.0, 0.0, 1.0, 0.0, 0.0, 0.0, -10.0, 10.0, -10.0, 10.0));
    assert!(result.is_ok());
    let r = result.unwrap();
    assert!((r.optimal_x - 0.0).abs() < 1e-6);
    assert!((r.optimal_y - 0.0).abs() < 1e-6);
    assert!((r.optimal_value - 0.0).abs() < 1e-6);
    assert_eq!(r.critical_point_type, "mínimo");
}

#[test]
fn test_optimize_paraboloid_maximo() {
    let result = solve_quadratic_optimization(&make_input(-1.0, 0.0, -1.0, 0.0, 0.0, 0.0, -10.0, 10.0, -10.0, 10.0));
    assert!(result.is_ok());
    let r = result.unwrap();
    assert!((r.optimal_x - 0.0).abs() < 1e-6);
    assert!((r.optimal_y - 0.0).abs() < 1e-6);
    assert_eq!(r.critical_point_type, "máximo");
}

#[test]
fn test_optimize_sela() {
    let result = solve_quadratic_optimization(&make_input(1.0, 0.0, -1.0, 0.0, 0.0, 0.0, -10.0, 10.0, -10.0, 10.0));
    assert!(result.is_ok());
    let r = result.unwrap();
    assert!((r.optimal_x - 0.0).abs() < 1e-6);
    assert!((r.optimal_y - 0.0).abs() < 1e-6);
    assert_eq!(r.critical_point_type, "sela");
}

#[test]
fn test_optimize_domain_invalid() {
    let result = solve_quadratic_optimization(&make_input(1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 10.0, -10.0, -10.0, 10.0));
    assert!(result.is_err());
}

#[test]
fn test_optimize_singular_system() {
    let result = solve_quadratic_optimization(&make_input(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -10.0, 10.0, -10.0, 10.0));
    assert!(result.is_err());
}

#[test]
fn test_optimize_gradient_at_optimum() {
    let result = solve_quadratic_optimization(&make_input(2.0, 3.0, 4.0, 5.0, 6.0, 7.0, -10.0, 10.0, -10.0, 10.0));
    assert!(result.is_ok());
    let r = result.unwrap();
    let x_expected = -22.0 / 23.0;
    let y_expected = -9.0 / 23.0;
    assert!((r.optimal_x - x_expected).abs() < 1e-6, "x* = {:.6}, esperado {:.6}", r.optimal_x, x_expected);
    assert!((r.optimal_y - y_expected).abs() < 1e-6, "y* = {:.6}, esperado {:.6}", r.optimal_y, y_expected);
    assert!(r.gradient_at_optimum[0].abs() < 1e-6, "∂f/∂x no = {:.10}, esperado ~0", r.gradient_at_optimum[0]);
    assert!(r.gradient_at_optimum[1].abs() < 1e-6, "∂f/∂y no = {:.10}, esperado ~0", r.gradient_at_optimum[1]);
}
