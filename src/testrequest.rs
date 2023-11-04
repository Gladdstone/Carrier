pub mod testrequest {
  use http::{Request, Response};

  fn send(uri: String) {
    let mut request = Request::builder()
      .uri(uri)
      .header("User-Agent", "test-agent");
  }

}
