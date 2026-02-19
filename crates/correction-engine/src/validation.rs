#[cfg(test)]
mod tests {
    use crate::stage_a::StageA;

    #[test]
    fn test_stage_a_precision() {
        let mut engine = StageA::new();
        engine.load_dictionary(&[
            "casa",
            "carro",
            "computador",
            "português",
            "brasileiro",
            "amanhã",
            "você",
        ]);

        let cases = vec![
            ("cassa", "casa"),
            ("caro", "carro"), // Nota: caro também existe, mas carro é uma correção comum se o typo for por falta de letra
            ("computaodr", "computador"),
            ("amanha", "amanhã"),
            ("voce", "você"),
        ];

        for (input, expected) in cases {
            let result = engine.correct(input, 1);
            assert_eq!(result, expected, "Falha ao corrigir '{}'", input);
        }
    }

    #[test]
    fn benchmark_stage_a_latency() {
        use std::time::Instant;
        let mut engine = StageA::new();
        engine.load_dictionary(&[
            "casa",
            "carro",
            "computador",
            "português",
            "brasileiro",
            "amanhã",
            "você",
        ]);

        let word = "cassa";
        let iterations = 10000;
        let start = Instant::now();

        for _ in 0..iterations {
            let _ = engine.correct(word, 1);
        }

        let duration = start.elapsed();
        let avg = duration.as_micros() as f64 / iterations as f64;

        println!("\n--- Stage A Benchmark ---");
        println!("Média de latência: {:.4} µs por palavra", avg);
        println!("Total para {} iterações: {:?}", iterations, duration);
        println!("-------------------------\n");

        // Garantir que a latência é < 1ms (1000 µs)
        assert!(avg < 1000.0, "Latência muito alta: {} µs", avg);
    }
}
