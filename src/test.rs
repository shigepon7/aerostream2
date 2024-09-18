use crate::*;

#[tokio::test]
async fn subscribe_repos() {
  let atproto = Atproto::default();
  let mut ws = atproto
    .com_atproto_sync_subscribe_repos(None)
    .await
    .unwrap();
  let mut passed = false;
  loop {
    if let Some(commit) = <&reqwest_websocket::Message as TryInto<Object>>::try_into(
      futures_util::StreamExt::next(&mut ws)
        .await
        .unwrap()
        .as_ref()
        .unwrap(),
    )
    .unwrap()
    .as_commit()
    {
      for record in commit.to_records().await {
        if record.as_app_bsky_feed_post().is_some() {
          passed = true;
          break;
        }
      }
    }
    if passed {
      break;
    }
  }
  assert!(passed);
}

#[tokio::test]
async fn resolve() {
  dotenvy::dotenv().ok();
  let id = std::env::var("HANDLE").unwrap();
  let did = std::env::var("DID").unwrap();
  let pw = std::env::var("PASSWORD").unwrap();
  let mut atproto = Atproto::default();
  atproto.login(&id, &pw).await.unwrap();
  let res = atproto
    .com_atproto_identity_resolve_handle(&id)
    .await
    .unwrap();
  assert_eq!(did, res.did);
}

#[tokio::test]
async fn create_record() {
  dotenvy::dotenv().ok();
  let id = std::env::var("HANDLE").unwrap();
  let pw = std::env::var("PASSWORD").unwrap();
  let mut atproto = Atproto::default();
  atproto.login(&id, &pw).await.unwrap();
  let did = atproto
    .com_atproto_identity_resolve_handle(&id)
    .await
    .unwrap()
    .did;
  let text = format!("ちゃんと動いているかな @{id}");
  let facets = TextDecoration::new_mention(&format!("@{id}"), &did).to_atproto(&text);
  assert!(atproto
    .com_atproto_repo_create_record(ComAtprotoRepoCreateRecordInput {
      repo: id.clone(),
      collection: String::from("app.bsky.feed.post"),
      rkey: None,
      validate: Some(true),
      record: serde_json::json!(AppBskyFeedPost {
        text,
        entities: None,
        facets: Some(facets),
        reply: None,
        embed: None,
        langs: Some(vec![String::from("ja")]),
        labels: None,
        tags: None,
        created_at: chrono::Utc::now(),
      }),
      swap_commit: None,
    })
    .await
    .is_ok());
}

#[tokio::test]
async fn plc() {
  dotenvy::dotenv().ok();
  let id = std::env::var("HANDLE").unwrap();
  let did = std::env::var("DID").unwrap();
  let plc = Plc::default();
  let diddoc = plc.resolve_did(&did).await.unwrap();
  assert!(diddoc.also_known_as.iter().any(|a| a.contains(&id)));
}

#[test]
fn aturi() {
  let aturi = "at://did:plc:pwlfo4w6auzwihryxik32t6d/app.bsky.feed.generator/nara2";
  assert_eq!(
    aturi.split("/").skip(2).take(3).collect::<Vec<_>>(),
    vec![
      "did:plc:pwlfo4w6auzwihryxik32t6d",
      "app.bsky.feed.generator",
      "nara2"
    ]
  );
}
