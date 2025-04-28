mod regression; // Módulo responsável por cálculos de regressão linear
mod metrics;    // Módulo para avaliação do modelo (R² e MSE)
mod prediction; // Módulo para realizar previsões com base nos coeficientes da regressão

fn main() {
    // Mensagem inicial exibida no console para contextualizar o usuário
    println!("Bem-vindo ao módulo de análise de séries temporais!");

    // Definição dos vetores de entrada para análise (dados de exemplo)
    // `x` representa as variáveis independentes (ex.: tempo ou outra métrica)
    // `y` representa as variáveis dependentes (ex.: resultados associados ao tempo)
    let x = vec![1.0, 2.0, 3.0, 4.0];
    let y = vec![2.0, 4.0, 6.0, 8.0];

    // Chamamos a função para calcular os coeficientes da regressão linear
    // `match` é usado para tratar erros que podem ser retornados pela função
    match regression::calcular_coeficientes(&x, &y) {
        Ok((slope, intercept)) => { // Caso de sucesso
            println!("Coeficientes calculados:");
            println!("Slope(coeficiente angular): {}", slope); // Exibe o coeficiente angular
            println!("Intercept(coeficiente linear): {}", intercept); // Exibe o coeficiente linear

            // Cálculo do coeficiente de determinação (R²), que avalia a qualidade do modelo
            match metrics::calcular_r2(&x, &y, slope, intercept) {
                Ok(r2) => println!("R²: {}", r2), // Exibe o valor de R² no console
                Err(err) => println!("Erro ao calcular R²: {}", err), // Caso de erro
            }

            // Cálculo do Erro Quadrático Médio (MSE), que mede a precisão do modelo
            match metrics::calcular_mse(&x, &y, slope, intercept) {
                Ok(mse) => println!("MSE: {}", mse), // Exibe o valor de MSE
                Err(err) => println!("Erro ao calcular MSE: {}", err), // Caso de erro
            }

            // Realização de previsões com novos valores de entrada
            let novos_x = vec![11.0, 21.0, 13.0, 9.0]; // Vetor contendo novos valores para prever
            match prediction::prever_valores(&novos_x, slope, intercept) {
                Ok(previsoes) => println!("Previsões: {:?}", previsoes), // Exibe as previsões
                Err(err) => println!("Erro ao prever valores: {}", err), // Caso de erro
            }
        }
        Err(err) => println!("Erro ao calcular os coeficientes: {}", err), // Caso de erro no cálculo da regressão
    }
}