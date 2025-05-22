// api/tests/utilities/hello_world.rs
// Integration test for route /hello

use actix_web::{test, App};
use api::routes::configure_routes;

#[actix_web::test]
async fn test_hello_route() {
    let app = test::init_service(App::new().configure(configure_routes)).await;
    let req = test::TestRequest::get().uri("/hello").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}