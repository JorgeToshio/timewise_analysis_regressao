// Função para calcular o coeficiente de determinação (R²)
pub fn calcular_r2(x: &[f64], y: &[f64], slope: f64, intercept: f64) -> Result<f64, &'static str> {
    // Verificação inicial para garantir que os vetores x e y têm o mesmo tamanho.
    if x.len() != y.len() {
        return Err("Os vetores x e y devem ter o mesmo tamanho.");
    }

    // Verificação para evitar cálculo com vetores vazios.
    if x.is_empty() || y.is_empty() {
        return Err("Os vetores x e y não podem estar vazios.");
    }

    // Cálculo do somatório total de variações (SST - Total Sum of Squares).
    let y_mean = y.iter().sum::<f64>() / y.len() as f64; // Média dos valores de y
    let sst = y.iter().map(|yi| (yi - y_mean).powi(2)).sum::<f64>();

    // Cálculo do somatório residual de variações (SSR - Residual Sum of Squares).
    let ssr = x.iter()
        .zip(y.iter())
        .map(|(xi, yi)| (yi - (slope * xi + intercept)).powi(2))
        .sum::<f64>();

    // Cálculo do coeficiente de determinação R²
    let r2 = 1.0 - (ssr / sst);

    // Retorno do valor de R²
    Ok(r2)
}

// Função para calcular o erro quadrático médio (MSE)
pub fn calcular_mse(x: &[f64], y: &[f64], slope: f64, intercept: f64) -> Result<f64, &'static str> {
    // Verificação inicial para garantir que os vetores x e y têm o mesmo tamanho.
    if x.len() != y.len() {
        return Err("Os vetores x e y devem ter o mesmo tamanho.");
    }

    // Verificação para evitar cálculo com vetores vazios.
    if x.is_empty() || y.is_empty() {
        return Err("Os vetores x e y não podem estar vazios.");
    }

    // Cálculo do erro quadrático médio
    let mse = x.iter()
        .zip(y.iter())
        .map(|(xi, yi)| (yi - (slope * xi + intercept)).powi(2))
        .sum::<f64>() / x.len() as f64;

    // Retorno do valor de MSE
    Ok(mse)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calcular_r2() {
        // Vetores e coeficientes
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 4.0, 6.0];
        let slope = 2.0;
        let intercept = 0.0;

        // Chamamos a função para calcular R²
        let result = calcular_r2(&x, &y, slope, intercept);

        // Validamos o resultado esperado
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1.0); // R² esperado é 1.0
    }

    #[test]
    fn test_calcular_mse() {
        // Vetores e coeficientes
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 4.0, 6.0];
        let slope = 2.0;
        let intercept = 0.0;

        // Chamamos a função para calcular MSE
        let result = calcular_mse(&x, &y, slope, intercept);

        // Validamos o resultado esperado
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0.0); // MSE esperado é 0.0
    }
}