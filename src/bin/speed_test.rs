/**
 * Rust vs. C++ Performance Benchmark
 * Este programa avalia a velocidade de execução do Rust realizando 100 mil milhões de operações bitwise.
 * A saída está formatada para comparar diretamente com a precisão do C++.
 */

fn main() {
    // Utilizamos um inteiro de 64 bits sem sinal (u64) para o resultado.
    // O 'mut' permite que a variável seja atualizada dentro do loop.
    let mut val: u64 = 0;

    println!("Rust está a trabalhar... a processar 100 mil milhões de operações...");

    // Captura o tempo exato antes do início do processamento.
    let now = std::time::Instant::now();

    // Loop externo: 1.000.000 de iterações.
    for i in 0..1_000_000 {
        // Loop interno: 100.000 iterações.
        // Total: 1.000.000 * 100.000 = 100.000.000.000 (100 Biliões).
        for j in 0..100_000 {
            // Operação XOR (^) e adição com wrapping para evitar crash por overflow.
            // Isto simula o comportamento padrão dos inteiros no C++.
            val = val.wrapping_add(i ^ j);
        }
    }

    // Calcula o tempo decorrido.
    let elapsed = now.elapsed();

    // Formatação da saída idêntica ao C++.
    println!("Finished in: {}ms", elapsed.as_millis());
    println!("Finished in: {}s", elapsed.as_secs());
    println!("Result: {}", val);
}