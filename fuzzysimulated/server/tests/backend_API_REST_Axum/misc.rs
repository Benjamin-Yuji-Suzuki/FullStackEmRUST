use super::*;
use serial_test::serial;

#[serial]
#[tokio::test]
async fn test_weather_missing_city() {
    let mut app = TestApp::new().await;
    let req = json_get("/api/weather");
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[serial]
#[tokio::test]
async fn test_weather_missing_api_key() {
    let mut app = TestApp::new().await;
    std::env::remove_var("OPENWEATHER_API_KEY");
    let req = json_get("/api/weather?city=Belem");
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::BAD_GATEWAY);
}

#[serial]
#[tokio::test]
async fn test_all_404_endpoints() {
    let mut app = TestApp::new().await;
    let uid = "00000000-0000-0000-0000-000000000000";
    for path in &[
        format!("/api/systems/{uid}"),
        format!("/api/variables/{uid}"),
        format!("/api/terms/{uid}"),
        format!("/api/rules/{uid}"),
        format!("/api/optimizations/{uid}"),
        format!("/api/optimizations/{uid}/export"),
    ] {
        let req = json_get(path);
        let resp = app.call(req).await;
        assert_eq!(
            resp.status(),
            StatusCode::NOT_FOUND,
            "Expected 404 for {}",
            path
        );
    }
}
