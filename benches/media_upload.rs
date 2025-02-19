use criterion::{black_box, criterion_group, criterion_main, Criterion};
use actix_web::{test, web, App};
use socialhub_media::handlers;
use bytes::Bytes;

async fn benchmark_upload() {
    let app = test::init_service(
        App::new().service(web::scope("/media").route("/upload", web::post().to(handlers::upload)))
    ).await;

    let payload = concat!(
        "--boundary\r\n",
        "Content-Disposition: form-data; name=\"file\"; filename=\"test.jpg\"\r\n",
        "Content-Type: image/jpeg\r\n\r\n",
        "test file content\r\n",
        "--boundary--\r\n"
    );

    let req = test::TestRequest::post()
        .uri("/media/upload")
        .insert_header(("content-type", "multipart/form-data; boundary=boundary"))
        .set_payload(payload)
        .to_request();

    let _ = test::call_service(&app, req).await;
}

pub fn upload_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    c.bench_function("media_upload", |b| {
        b.to_async(&rt).iter(|| async {
            benchmark_upload().await
        });
    });
}

criterion_group!(benches, upload_benchmark);
criterion_main!(benches);
