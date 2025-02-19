use criterion::{criterion_group, criterion_main, Criterion};
use actix_web::{test, App};

#[actix_rt::test]
async fn test_benchmark_setup() {
    let app = test::init_service(
        App::new()
            .service(actix_web::web::scope("/media"))
    ).await;
    
    let resp = test::TestRequest::get()
        .uri("/media")
        .send_request(&app)
        .await;
        
    assert!(resp.status().is_success());
}

// Teste de performance simples
fn simple_benchmark(c: &mut Criterion) {
    c.bench_function("simple_test", |b| b.iter(|| {
        let x = 1 + 1;
        criterion::black_box(x)
    }));
}

criterion_group!(benches, simple_benchmark);
criterion_main!(benches);
