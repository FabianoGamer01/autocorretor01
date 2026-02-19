use draco_brain::stage_a::StageA;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_latency(c: &mut Criterion) {
    let mut engine = StageA::new();
    // Inserir algumas palavras comuns para o teste
    engine.load_dictionary(&[
        "casa",
        "carro",
        "computador",
        "português",
        "brasileiro",
        "inteligente",
    ]);

    let mut group = c.benchmark_group("StageA Latency");

    group.bench_function("Correct word (No change)", |b| {
        b.iter(|| engine.correct(black_box("casa"), black_box(1)))
    });

    group.bench_function("Typo Distance 1 (casa -> cassa)", |b| {
        b.iter(|| engine.correct(black_box("cassa"), black_box(1)))
    });

    group.bench_function("Typo Distance 2 (casa -> casssa)", |b| {
        b.iter(|| engine.correct(black_box("casssa"), black_box(1)))
    });

    group.bench_function("Conservador skip Distance 2", |b| {
        b.iter(|| engine.correct(black_box("casssa"), black_box(0)))
    });

    group.finish();
}

criterion_group!(benches, bench_latency);
criterion_main!(benches);
