// Importamos a biblioteca padrão para cálculos matemáticos, como ponto flutuante (não essencial aqui, mas útil para projetos maiores)
use std::f64;

// Função para calcular os coeficientes da regressão linear (slope e intercept)
pub fn calcular_coeficientes(x: &[f64], y: &[f64]) -> Result<(f64, f64), &'static str> {
    // Verificação inicial para garantir que os vetores x e y têm o mesmo tamanho.
    // A regressão linear só funciona se houver pares de valores correspondentes entre x e y.
    if x.len() != y.len() {
        return Err("Os vetores x e y devem ter o mesmo tamanho.");
    }

    // Verificação adicional para evitar cálculos com vetores vazios.
    // Vetores vazios não têm dados para análise e causariam erros matemáticos.
    if x.is_empty() || y.is_empty() {
        return Err("Os vetores x e y não podem estar vazios.");
    }

    // Cálculo da média de x (somatório dos valores dividido pelo número de elementos).
    let x_mean = x.iter().sum::<f64>() / x.len() as f64;

    // Cálculo da média de y (idem ao cálculo de x).
    let y_mean = y.iter().sum::<f64>() / y.len() as f64;

    // Cálculo da covariância (medida que indica como x e y variam juntos).
    // Para cada par de (xi, yi), calculamos a diferença deles em relação à média.
    let covariance = x.iter()
        .zip(y.iter()) // Emparelha cada xi com seu correspondente yi.
        .map(|(xi, yi)| (xi - x_mean) * (yi - y_mean)) // Multiplica as diferenças em relação às médias.
        .sum::<f64>(); // Soma tudo para obter a covariância.

    // Cálculo da variância de x (medida que indica a dispersão dos valores de x).
    let variance = x.iter()
        .map(|xi| (xi - x_mean).powi(2)) // Eleva a diferença em relação à média ao quadrado.
        .sum::<f64>(); // Soma tudo para obter a variância.

    // Cálculo do coeficiente angular (slope), que representa a inclinação da reta.
    let slope = covariance / variance;

    // Cálculo do coeficiente linear (intercept), que representa o ponto onde a reta cruza o eixo y.
    let intercept = y_mean - slope * x_mean;

    // Retorno dos coeficientes como uma tupla (slope, intercept) encapsulada no tipo Result.
    Ok((slope, intercept))
}

// Bloco de testes unitários para validar a funcionalidade da função acima.
#[cfg(test)]
mod tests {
    use super::*; // Importa todas as funções e tipos do módulo principal para uso nos testes.

    #[test]
    fn test_calcular_coeficientes_valores_basicos() {
        // Teste com vetores bem definidos, onde o resultado esperado é uma relação perfeita.
        let x = vec![1.0, 2.0, 3.0, 4.0]; // Valores de entrada.
        let y = vec![2.0, 4.0, 6.0, 8.0]; // Resultados associados.

        // Chamamos a função para calcular os coeficientes.
        let result = calcular_coeficientes(&x, &y);

        // Verificamos se o resultado é Ok (não houve erros) e se os valores retornados são os esperados.
        assert!(result.is_ok());
        let (slope, intercept) = result.unwrap();
        assert_eq!(slope, 2.0); // Inclinação esperada.
        assert_eq!(intercept, 0.0); // Intercepto esperado.
    }

    #[test]
    fn test_calcular_coeficientes_vetores_vazios() {
        // Teste com vetores vazios, onde a função deve retornar erro.
        let x: Vec<f64> = Vec::new(); // Vetor vazio.
        let y: Vec<f64> = Vec::new(); // Vetor vazio.

        // A função deve retornar um erro para vetores vazios.
        let result = calcular_coeficientes(&x, &y);
        assert!(result.is_err()); // Verifica se o resultado é um erro.
    }

    #[test]
    fn test_calcular_coeficientes_tamanhos_diferentes() {
        // Teste com vetores de tamanhos diferentes, onde a função deve retornar erro.
        let x = vec![1.0, 2.0]; // Vetor com 2 elementos.
        let y = vec![3.0];      // Vetor com 1 elemento.

        // A função deve retornar um erro para tamanhos incompatíveis.
        let result = calcular_coeficientes(&x, &y);
        assert!(result.is_err()); // Verifica se o resultado é um erro.
    }
}