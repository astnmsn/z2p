use z2p::{get_server, get_state};

#[tokio::test]

async fn health_works() {
    run_for_test().await;
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:3000/health")
        .send()
        .await
        .expect("Failed to send request");
    assert!(response.status().is_success());
    assert_eq!(
        String::from("ok"),
        response
            .text()
            .await
            .expect("Could not get text from response")
    )
}

async fn run_for_test() {
  let state = get_state().await;
  let _ = tokio::spawn(get_server(state));
}
