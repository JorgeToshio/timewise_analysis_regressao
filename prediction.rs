// Função para realizar previsões utilizando os coeficientes da regressão linear
pub fn prever_valores(x: &[f64], slope: f64, intercept: f64) -> Result<Vec<f64>, &'static str> {
    // Verificação para evitar cálculo com vetores vazios
    if x.is_empty() {
        return Err("O vetor x não pode estar vazio.");
    }

    // Realização das previsões: para cada valor em `x`, aplicamos a equação da reta
    let previsoes: Vec<f64> = x.iter().map(|xi| slope * xi + intercept).collect();

    // Retorno do vetor de previsões
    Ok(previsoes)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prever_valores() {
        // Vetores e coeficientes
        let x = vec![1.0, 2.0, 3.0];
        let slope = 2.0;
        let intercept = 0.0;

        // Chamamos a função de previsão
        let result = prever_valores(&x, slope, intercept);

        // Validamos o vetor de previsões
        assert!(result.is_ok());
        let previsoes = result.unwrap();
        assert_eq!(previsoes, vec![2.0, 4.0, 6.0]); // Previsões esperadas
    }

    #[test]
    fn test_prever_valores_vetor_vazio() {
        // Vetor vazio
        let x: Vec<f64> = Vec::new();
        let slope = 2.0;
        let intercept = 0.0;

        // A função deve retornar erro
        let result = prever_valores(&x, slope, intercept);
        assert!(result.is_err()); // Confirmamos que ocorreu um erro
    }
}