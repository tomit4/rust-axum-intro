use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // hc.do_get("/hello?name=Jen").await?.print().await?;
    hc.do_get("/hello2/Mike").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "Welcome"
        })
    );
    req_login.await?.print().await?;

    // serves the actual text of our main.rs file, demonstrating the return of static files
    // hc.do_get("/src/main.rs").await?.print().await?;

    Ok(())
}
