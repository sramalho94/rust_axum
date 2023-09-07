#![allow(unused)] // For beginning only.

use anyhow::{Result, Ok};

#[tokio::test]
async fn quick_dev() -> Result<()> {
  let hc = httpc_test::new_client("http://localhost:8080")?;

  hc.do_get("/hello?name=Trundle")
    .await?
    .print()
    .await?;
  Ok(())
}