use reqwest::Error;

async fn getlink() -> Result<String,Error> {
  let client = reqwest::Client::builder()
    .redirect(reqwest::redirect::Policy::none())
    .build()?;
  let req = client.get("https://en.wikipedia.org/wiki/Special:Random").send().await?;
  if let Some(location) = req.headers().get(reqwest::header::LOCATION) {
    return Ok(location.to_str().unwrap().to_string());
  }
  Ok("".to_string())
}

async fn postlink(link: String) {
  let params = [("todo",link)];
  let client = reqwest::Client::new();
  client.post("http://backend-svc/todos")
    .form(&params)
    .send().await.unwrap();
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
  if let Ok(link) = getlink().await {
    println!("Posting {}",link);
    postlink(link).await;
  }
  Ok(())
}

