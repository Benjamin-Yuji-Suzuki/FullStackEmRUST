use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationInput {
    pub coef_a: f64,
    pub coef_b: f64,
    pub coef_c: f64,
    pub coef_d: f64,
    pub coef_e: f64,
    pub coef_f: f64,
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationOutput {
    pub optimal_x: f64,
    pub optimal_y: f64,
    pub optimal_value: f64,
    pub critical_point_type: String,
    pub explanation: String,
    pub gradient_at_optimum: [f64; 2],
    pub hessian_matrix: [[f64; 2]; 2],
    pub hessian_det: f64,
}

pub fn hessian_det(a: f64, b: f64, c: f64) -> f64 {
    4.0 * a * c - b * b
}

pub fn cramer_x(_a: f64, b: f64, c: f64, d: f64, e: f64, det: f64) -> f64 {
    (-2.0 * c * d + b * e) / det
}

pub fn cramer_y(a: f64, b: f64, _c: f64, d: f64, e: f64, det: f64) -> f64 {
    (b * d - 2.0 * a * e) / det
}

pub fn quadratic_value(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, x: f64, y: f64) -> f64 {
    a * x * x + b * x * y + c * y * y + d * x + e * y + f
}

pub fn classify_critical_point(hdet: f64, coef_a: f64) -> (String, String) {
    if hdet > 0.0 && 2.0 * coef_a > 0.0 {
        (
            "mínimo".into(),
            format!(
                "det(H) = {:.4} > 0 e 2a = {:.4} > 0 ⇒ ponto de **mínimo local**.\n\n\
                 A matriz Hessiana é **definida positiva** (todos os autovalores > 0), \
                 indicando que a função f(x,y) tem concavidade voltada para cima \
                 em todas as direções ao redor do ponto crítico.",
                hdet, 2.0 * coef_a
            ),
        )
    } else if hdet > 0.0 && 2.0 * coef_a < 0.0 {
        (
            "máximo".into(),
            format!(
                "det(H) = {:.4} > 0 e 2a = {:.4} < 0 ⇒ ponto de **máximo local**.\n\n\
                 A matriz Hessiana é **definida negativa** (todos os autovalores < 0), \
                 indicando que a função f(x,y) tem concavidade voltada para baixo \
                 em todas as direções ao redor do ponto crítico.",
                hdet, 2.0 * coef_a
            ),
        )
    } else if hdet < 0.0 {
        (
            "sela".into(),
            format!(
                "det(H) = {:.4} < 0 ⇒ **ponto de sela**.\n\n\
                 A matriz Hessiana tem autovalores de sinais opostos, \
                 indicando que f(x,y) cresce em uma direção e decresce em outra. \
                 O ponto crítico não é nem mínimo nem máximo local.",
                hdet
            ),
        )
    } else {
        (
            "indeterminado".into(),
            format!(
                "det(H) = {:.4} = 0 ⇒ classificação **indeterminada**.\n\n\
                 O teste da Hessiana é inconclusivo. Análise adicional de termos \
                 de ordem superior seria necessária para classificar o ponto crítico.",
                hdet
            ),
        )
    }
}

pub fn solve_quadratic_optimization(input: &OptimizationInput) -> Result<OptimizationOutput, String> {
    if input.x_min >= input.x_max || input.y_min >= input.y_max {
        return Err("Domínio inválido: x_min < x_max e y_min < y_max são obrigatórios".into());
    }

    let det = hessian_det(input.coef_a, input.coef_b, input.coef_c);
    if det.abs() < 1e-12 {
        return Err("Sistema linear singular: determinante Hessiano é zero".into());
    }

    let raw_x = cramer_x(input.coef_a, input.coef_b, input.coef_c, input.coef_d, input.coef_e, det);
    let raw_y = cramer_y(input.coef_a, input.coef_b, input.coef_c, input.coef_d, input.coef_e, det);

    let optimal_x = raw_x.clamp(input.x_min, input.x_max);
    let optimal_y = raw_y.clamp(input.y_min, input.y_max);

    let optimal_value = quadratic_value(
        input.coef_a, input.coef_b, input.coef_c,
        input.coef_d, input.coef_e, input.coef_f,
        optimal_x, optimal_y,
    );

    let hdet = hessian_det(input.coef_a, input.coef_b, input.coef_c);
    let (point_type, explanation) = classify_critical_point(hdet, input.coef_a);

    let grad_x = 2.0 * input.coef_a * optimal_x + input.coef_b * optimal_y + input.coef_d;
    let grad_y = input.coef_b * optimal_x + 2.0 * input.coef_c * optimal_y + input.coef_e;

    Ok(OptimizationOutput {
        optimal_x,
        optimal_y,
        optimal_value,
        critical_point_type: point_type,
        explanation,
        gradient_at_optimum: [grad_x, grad_y],
        hessian_matrix: [[2.0 * input.coef_a, input.coef_b], [input.coef_b, 2.0 * input.coef_c]],
        hessian_det: hdet,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hessian_det_positive() {
        let d = hessian_det(2.0, 0.0, 3.0);
        assert!((d - 24.0).abs() < 1e-10);
    }

    #[test]
    fn test_hessian_det_negative() {
        let d = hessian_det(1.0, 5.0, 1.0);
        assert!(d < 0.0); // 4 - 25 = -21
    }

    #[test]
    fn test_cramer_x_basic() {
        let x = cramer_x(1.0, 0.0, 1.0, -2.0, -4.0, 4.0);
        assert!((x - 1.0).abs() < 1e-10); // 2x = 2, 2y = 4 => x=1, y=2
    }

    #[test]
    fn test_quadratic_value_at_origin() {
        let v = quadratic_value(1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert!((v - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_paraboloid_minimo() {
        let input = OptimizationInput {
            coef_a: 2.0, coef_b: 0.0, coef_c: 3.0,
            coef_d: -4.0, coef_e: -12.0, coef_f: 10.0,
            x_min: -10.0, x_max: 10.0, y_min: -10.0, y_max: 10.0,
        };
        let result = solve_quadratic_optimization(&input).unwrap();
        assert!((result.optimal_x - 1.0).abs() < 1e-6);
        assert!((result.optimal_y - 2.0).abs() < 1e-6);
        assert_eq!(result.critical_point_type, "mínimo");
    }

    #[test]
    fn test_paraboloid_maximo() {
        let input = OptimizationInput {
            coef_a: -2.0, coef_b: 0.0, coef_c: -1.0,
            coef_d: 4.0, coef_e: 2.0, coef_f: -5.0,
            x_min: -10.0, x_max: 10.0, y_min: -10.0, y_max: 10.0,
        };
        let result = solve_quadratic_optimization(&input).unwrap();
        assert_eq!(result.critical_point_type, "máximo");
    }

    #[test]
    fn test_ponto_de_sela() {
        let input = OptimizationInput {
            coef_a: 1.0, coef_b: 5.0, coef_c: 1.0,
            coef_d: 0.0, coef_e: 0.0, coef_f: 0.0,
            x_min: -10.0, x_max: 10.0, y_min: -10.0, y_max: 10.0,
        };
        let result = solve_quadratic_optimization(&input).unwrap();
        assert_eq!(result.critical_point_type, "sela");
    }

    #[test]
    fn test_sistema_singular() {
        let input = OptimizationInput {
            coef_a: 1.0, coef_b: 2.0, coef_c: 1.0,
            coef_d: 0.0, coef_e: 0.0, coef_f: 0.0,
            x_min: -10.0, x_max: 10.0, y_min: -10.0, y_max: 10.0,
        };
        let result = solve_quadratic_optimization(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_dominio_invalido() {
        let input = OptimizationInput {
            coef_a: 1.0, coef_b: 0.0, coef_c: 1.0,
            coef_d: 0.0, coef_e: 0.0, coef_f: 0.0,
            x_min: 10.0, x_max: 5.0, y_min: -10.0, y_max: 10.0,
        };
        let result = solve_quadratic_optimization(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_gradiente_zero_no_ponto_otimo() {
        let input = OptimizationInput {
            coef_a: 1.0, coef_b: 0.0, coef_c: 1.0,
            coef_d: 0.0, coef_e: 0.0, coef_f: 0.0,
            x_min: -10.0, x_max: 10.0, y_min: -10.0, y_max: 10.0,
        };
        let result = solve_quadratic_optimization(&input).unwrap();
        assert!(result.gradient_at_optimum[0].abs() < 1e-10);
        assert!(result.gradient_at_optimum[1].abs() < 1e-10);
    }
}
