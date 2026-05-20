/// Calcula o ponto crítico de f(x,y) = ax² + bxy + cy² + dx + ey + f_const
/// Retorna (optimal_x, optimal_y, optimal_value, critical_point_type, explanation,
///          gradient_at_optimum, hessian)
fn compute_optimal_point(
    coef_a: f64, coef_b: f64, coef_c: f64, coef_d: f64, coef_e: f64, coef_f: f64,
    x_min: f64, x_max: f64, y_min: f64, y_max: f64,
) -> Result<(f64, f64, f64, String, String, [f64; 2], [[f64; 2]; 2]), String> {
    if x_min >= x_max || y_min >= y_max {
        return Err("Domínio inválido: min deve ser menor que max".into());
    }

    // Sistema linear 2x2: | 2a  b | |x| = |-d|
    //                      | b  2c| |y|   |-e|
    let det = 4.0 * coef_a * coef_c - coef_b * coef_b;

    if det.abs() < 1e-12 {
        return Err("Sistema singular: determinante Hessiano = 0".into());
    }

    // Regra de Cramer
    let x = (-2.0 * coef_c * coef_d + coef_b * coef_e) / det;
    let y = (coef_b * coef_d - 2.0 * coef_a * coef_e) / det;

    // Verificar domínio
    let x = x.clamp(x_min, x_max);
    let y = y.clamp(y_min, y_max);

    let value = coef_a * x * x + coef_b * x * y + coef_c * y * y + coef_d * x + coef_e * y + coef_f;

    // Hessiana: | 2a  b |
    //           | b  2c |
    let hessian_det = 4.0 * coef_a * coef_c - coef_b * coef_b;
    let _trace_hessian = 2.0 * coef_a + 2.0 * coef_c;

    let (point_type, explanation) = if hessian_det > 0.0 && 2.0 * coef_a > 0.0 {
        ("mínimo".into(), format!(
            "det(H) = {:.4} > 0 e 2a = {:.4} > 0 → ponto de mínimo local.\n\
             A Hessiana é definida positiva, indicando que f(x,y) tem concavidade para cima em todas as direções.",
            hessian_det, 2.0 * coef_a
        ))
    } else if hessian_det > 0.0 && 2.0 * coef_a < 0.0 {
        ("máximo".into(), format!(
            "det(H) = {:.4} > 0 e 2a = {:.4} < 0 → ponto de máximo local.\n\
             A Hessiana é definida negativa, indicando que f(x,y) tem concavidade para baixo em todas as direções.",
            hessian_det, 2.0 * coef_a
        ))
    } else if hessian_det < 0.0 {
        ("sela".into(), format!(
            "det(H) = {:.4} < 0 → ponto de sela.\n\
             A Hessiana tem autovalores de sinais opostos, indicando que f(x,y) cresce em uma direção e decresce em outra.",
            hessian_det
        ))
    } else {
        ("indeterminado".into(), format!(
            "det(H) = {:.4} = 0 → classificação indeterminada.\n\
             Requer análise adicional de termos de ordem superior.",
            hessian_det
        ))
    };

    let gradient = [-2.0 * coef_a * x - coef_b * y - coef_d, -coef_b * x - 2.0 * coef_c * y - coef_e];
    let hessian = [[2.0 * coef_a, coef_b], [coef_b, 2.0 * coef_c]];

    Ok((x, y, value, point_type, explanation, gradient, hessian))
}

#[test]
fn test_optimize_paraboloid_minimo() {
    // f(x,y) = x² + y² → mínimo em (0,0)
    let result = compute_optimal_point(1.0, 0.0, 1.0, 0.0, 0.0, 0.0, -10.0, 10.0, -10.0, 10.0);
    assert!(result.is_ok(), "Esperava Ok mas obteve: {:?}", result);
    let (x, y, val, ptype, ..) = result.unwrap();
    assert!((x - 0.0).abs() < 1e-6, "x* deveria ser 0, obteve {}", x);
    assert!((y - 0.0).abs() < 1e-6, "y* deveria ser 0, obteve {}", y);
    assert!((val - 0.0).abs() < 1e-6, "f(x*,y*) deveria ser 0, obteve {}", val);
    assert_eq!(ptype, "mínimo", "Deveria ser mínimo, obteve {}", ptype);
}

#[test]
fn test_optimize_paraboloid_maximo() {
    // f(x,y) = -x² - y² → máximo em (0,0)
    let result = compute_optimal_point(-1.0, 0.0, -1.0, 0.0, 0.0, 0.0, -10.0, 10.0, -10.0, 10.0);
    assert!(result.is_ok(), "Esperava Ok mas obteve: {:?}", result);
    let (x, y, val, ptype, ..) = result.unwrap();
    assert!((x - 0.0).abs() < 1e-6, "x* deveria ser 0, obteve {}", x);
    assert!((y - 0.0).abs() < 1e-6, "y* deveria ser 0, obteve {}", y);
    assert!((val - 0.0).abs() < 1e-6, "f(x*,y*) deveria ser 0, obteve {}", val);
    assert_eq!(ptype, "máximo", "Deveria ser máximo, obteve {}", ptype);
}

#[test]
fn test_optimize_sela() {
    // f(x,y) = x² - y² → sela em (0,0)
    let result = compute_optimal_point(1.0, 0.0, -1.0, 0.0, 0.0, 0.0, -10.0, 10.0, -10.0, 10.0);
    assert!(result.is_ok(), "Esperava Ok mas obteve: {:?}", result);
    let (x, y, _val, ptype, ..) = result.unwrap();
    assert!((x - 0.0).abs() < 1e-6, "x* deveria ser 0, obteve {}", x);
    assert!((y - 0.0).abs() < 1e-6, "y* deveria ser 0, obteve {}", y);
    assert_eq!(ptype, "sela", "Deveria ser sela, obteve {}", ptype);
}

#[test]
fn test_optimize_domain_invalid() {
    let result = compute_optimal_point(1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 10.0, -10.0, -10.0, 10.0);
    assert!(result.is_err(), "Esperava Err para domínio inválido, obteve: {:?}", result);
}

#[test]
fn test_optimize_singular_system() {
    // f(x,y) = 0*x² + 0*xy + 0*y² + 0*x + 0*y + 0 → Hessiana nula (det = 0)
    let result = compute_optimal_point(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -10.0, 10.0, -10.0, 10.0);
    assert!(result.is_err(), "Esperava Err para sistema singular, obteve: {:?}", result);
}

#[test]
fn test_optimize_gradient_at_optimum() {
    // f(x,y) = 2x² + 3xy + 4y² + 5x + 6y + 7
    // ∂f/∂x = 4x + 3y + 5 = 0
    // ∂f/∂y = 3x + 8y + 6 = 0
    // det = 4*8 - 3*3 = 32 - 9 = 23
    // x = (-8*5 + 3*6) / 23 = (-40 + 18) / 23 = -22/23 ≈ -0.9565
    // y = (3*5 - 4*6) / 23 = (15 - 24) / 23 = -9/23 ≈ -0.3913
    let result = compute_optimal_point(2.0, 3.0, 4.0, 5.0, 6.0, 7.0, -10.0, 10.0, -10.0, 10.0);
    assert!(result.is_ok(), "Esperava Ok mas obteve: {:?}", result);
    let (x, y, _val, _ptype, _expl, gradient, _hessian) = result.unwrap();
    let x_expected = -22.0 / 23.0;
    let y_expected = -9.0 / 23.0;
    assert!((x - x_expected).abs() < 1e-6, "x* = {:.6}, esperado {:.6}", x, x_expected);
    assert!((y - y_expected).abs() < 1e-6, "y* = {:.6}, esperado {:.6}", y, y_expected);
    assert!(gradient[0].abs() < 1e-6, "∂f/∂x no ponto ótimo = {:.10}, esperado ~0", gradient[0]);
    assert!(gradient[1].abs() < 1e-6, "∂f/∂y no ponto ótimo = {:.10}, esperado ~0", gradient[1]);
}
