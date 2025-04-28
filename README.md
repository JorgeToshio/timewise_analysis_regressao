<h1> Portfólio em Rust: Implementação de Regressão Linear Pura para Séries Temporais</h1>

### Sobre o Projeto
Este projeto foi desenvolvido para atender às necessidades da startup **TimeWise Analytics**, que busca uma solução eficiente para análise e previsão de séries temporais. Com foco na implementação de regressão linear em **Rust**, o módulo oferece alta performance e precisão para explorar padrões e tendências em dados temporais.

### Funcionalidades Implementadas
1. **Regressão Linear**:
   - Calcula os coeficientes da reta (slope e intercept) que melhor se ajusta aos dados fornecidos.
   - A implementação é "pura", sem dependências externas, utilizando exclusivamente os recursos nativos do Rust.

2. **Cálculo de Métricas de Avaliação**:
   - **R² (Coeficiente de Determinação)**: Avalia a qualidade do ajuste entre os dados e o modelo.
   - **MSE (Erro Quadrático Médio)**: Mede a precisão da regressão linear.

3. **Realização de Previsões**:
   - Utiliza os coeficientes calculados para prever valores futuros com base em novas entradas.

### Estrutura do Código
```
timewise_analysis/
├── src/
│   ├── main.rs              # Arquivo principal do programa
│   ├── regression.rs        # Módulo de regressão linear
│   ├── metrics.rs           # Módulo de cálculo de métricas
│   ├── prediction.rs        # Módulo de previsões
├── Cargo.toml               # Configurações e dependências do projeto
```

### Como Executar
1. **Clonar o Repositório**:
   ```bash
   git clone <https://github.com/JorgeToshio/timewise_analysis_regressao.git>
   cd timewise_analysis
   ```

2. **Compilar o Código**:
   Execute o seguinte comando na raiz do projeto:
   ```bash
   cargo build
   ```

3. **Executar o Programa**:
   Para rodar o programa, use:
   ```bash
   cargo run
   ```

4. **Executar os Testes**:
   Para verificar os testes unitários implementados:
   ```bash
   cargo test
   ```

### Exemplos de Uso
#### Exemplo 1: Dados para Regressão Linear
Entrada:
```rust
let x = vec![1.0, 2.0, 3.0];
let y = vec![2.0, 4.0, 6.0];
```

Saída:
```
Coeficientes calculados:
Slope: 2.0
Intercept: 0.0
R²: 1.0
MSE: 0.0
```

#### Exemplo 2: Previsões para Novos Dados
Entrada:
```rust
let novos_x = vec![4.0, 5.0];
```

Saída:
```
Previsões: [8.0, 10.0]
```

### Testes Unitários
Foram implementados e validados os seguintes testes:
1. Função de regressão linear:
   - Teste com valores básicos, vetores vazios e tamanhos diferentes.
2. Cálculo de métricas:
   - Teste para calcular **R²** e **MSE** em cenários perfeitos.
3. Função de previsão:
   - Teste com entrada válida e cenário de vetor vazio.

### Estratégias Adotadas
1. **Estrutura Modular**:
   - Cada funcionalidade está separada em módulos (`regression`, `metrics`, `prediction`) para facilitar a manutenção e escalabilidade.

2. **Tratamento de Erros**:
   - Utilização de `Result` para gerenciar cenários inválidos (vetores vazios, tamanhos diferentes).

3. **Cobertura Completa de Testes**:
   - Implementação de testes unitários para garantir robustez do código.

4. **Documentação Clara e Concisa**:
   - Detalhamento das funções e exemplos de uso.

### Desafios e Soluções
- **Desafio:** Implementar cálculos matemáticos sem crates externas.  
  **Solução:** Uso de iteradores e operações matemáticas nativas do Rust.
- **Desafio:** Garantir a qualidade do código com diferentes cenários.  
  **Solução:** Desenvolvimento de testes unitários abrangentes.


