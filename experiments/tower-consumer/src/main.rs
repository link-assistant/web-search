fn main() {
    assert_eq!(web_search::get_registry().len(), 40);
    let _cors = tower_http::cors::CorsLayer::permissive();
}
