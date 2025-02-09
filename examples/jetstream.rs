#[tokio::main]
async fn main() {
  let jetstream = aerostream2::Jetstream::default();
  let mut rx = jetstream.add_token_receiver().await.unwrap();
  jetstream.connect().await.unwrap();
  loop {
    match rx.recv().await {
      Some((_, token)) => {
        println!("{token:?}");
      }
      None => {
        break;
      }
    }
  }
}
