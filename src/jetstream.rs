#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JetstreamOperation {
  Create,
  Update,
  Delete,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum JetstreamKind {
  Commit,
  Identity,
  Account,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JetstreamCommit {
  pub rev: String,
  pub operation: JetstreamOperation,
  pub collection: String,
  pub rkey: String,
  pub record: Option<crate::Record>,
  pub cid: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JetstreamIdentity {
  pub did: String,
  pub handle: String,
  pub seq: u64,
  pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JetstreamAccount {
  pub active: bool,
  pub did: String,
  pub seq: u64,
  pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JetstreamEvent {
  pub did: String,
  pub time_us: u64,
  pub kind: JetstreamKind,
  pub commit: Option<JetstreamCommit>,
  pub identity: Option<JetstreamIdentity>,
  pub account: Option<JetstreamAccount>,
}

impl JetstreamEvent {
  pub fn to_aturi(&self) -> Option<String> {
    self
      .commit
      .as_ref()
      .map(|c| format!("at://{}/{}/{}", self.did, c.collection, c.rkey))
  }
}

#[derive(Debug, Clone)]
pub struct Jetstream {
  pub host: String,
  pub wanted_collections: Vec<String>,
  pub wanted_dids: Vec<String>,
  pub max_message_size_bytes: Option<u64>,
  pub cursor: Option<i64>,
  pub compress: Option<bool>,
  pub require_hello: Option<bool>,
  pub size: usize,
  pub commit_receivers:
    std::sync::Arc<tokio::sync::RwLock<Vec<tokio::sync::mpsc::Sender<JetstreamEvent>>>>,
  pub post_receivers:
    std::sync::Arc<tokio::sync::RwLock<Vec<tokio::sync::mpsc::Sender<JetstreamEvent>>>>,
  pub ja_receivers:
    std::sync::Arc<tokio::sync::RwLock<Vec<tokio::sync::mpsc::Sender<JetstreamEvent>>>>,
  pub token_receivers: std::sync::Arc<
    tokio::sync::RwLock<Vec<tokio::sync::mpsc::Sender<(JetstreamEvent, Vec<Vec<String>>)>>>,
  >,
  pub user_dict: Option<String>,
}

impl Default for Jetstream {
  fn default() -> Self {
    Self::new("jetstream1.us-west.bsky.network")
  }
}

impl Jetstream {
  pub fn new(host: &str) -> Self {
    Self {
      host: host.to_string(),
      wanted_collections: Vec::new(),
      wanted_dids: Vec::new(),
      max_message_size_bytes: None,
      cursor: None,
      compress: None,
      require_hello: None,
      size: 1_000_000,
      commit_receivers: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
      post_receivers: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
      ja_receivers: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
      token_receivers: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
      user_dict: None,
    }
  }

  pub fn with_collection(mut self, collection: &str) -> Self {
    self.wanted_collections.push(collection.to_string());
    self
  }

  pub fn with_did(mut self, did: &str) -> Self {
    self.wanted_dids.push(did.to_string());
    self
  }

  pub fn with_max_message_size(mut self, max_message_size_bytes: u64) -> Self {
    self.max_message_size_bytes = Some(max_message_size_bytes);
    self
  }

  pub fn with_cursor(mut self, cursor: i64) -> Self {
    self.cursor = Some(cursor);
    self
  }

  pub fn with_compress(mut self, compress: bool) -> Self {
    self.compress = Some(compress);
    self
  }

  pub fn with_require_hello(mut self, require_hello: bool) -> Self {
    self.require_hello = Some(require_hello);
    self
  }

  pub fn with_size(mut self, size: usize) -> Self {
    self.size = size;
    self
  }

  pub fn with_user_dict<T: ToString>(mut self, user_dict: T) -> Self {
    self.user_dict = Some(user_dict.to_string());
    self
  }

  pub async fn add_commit_receiver(&self) -> tokio::sync::mpsc::Receiver<JetstreamEvent> {
    let (tx, rx) = tokio::sync::mpsc::channel::<JetstreamEvent>(self.size);
    self.commit_receivers.write().await.push(tx);
    rx
  }

  pub async fn add_post_receiver(&self) -> tokio::sync::mpsc::Receiver<JetstreamEvent> {
    let (tx, rx) = tokio::sync::mpsc::channel::<JetstreamEvent>(self.size);
    self.post_receivers.write().await.push(tx);
    rx
  }

  pub async fn add_ja_receiver(&self) -> tokio::sync::mpsc::Receiver<JetstreamEvent> {
    let (tx, rx) = tokio::sync::mpsc::channel::<JetstreamEvent>(self.size);
    self.ja_receivers.write().await.push(tx);
    rx
  }

  pub async fn add_token_receiver(
    &self,
  ) -> tokio::sync::mpsc::Receiver<(JetstreamEvent, Vec<Vec<String>>)> {
    let (tx, rx) = tokio::sync::mpsc::channel::<(JetstreamEvent, Vec<Vec<String>>)>(self.size);
    self.token_receivers.write().await.push(tx);
    rx
  }

  pub async fn connect(&self) -> crate::Result<()> {
    let rx = self.add_ja_receiver().await;
    let token_receivers = self.token_receivers.clone();
    let user_dict = self.user_dict.clone();
    tokio::spawn(async move { token_receiver_thread(rx, token_receivers, user_dict).await });
    let rx = self.add_post_receiver().await;
    let ja_receivers = self.ja_receivers.clone();
    tokio::spawn(async move { ja_receiver_thread(rx, ja_receivers).await });
    let rx = self.add_commit_receiver().await;
    let post_receivers = self.post_receivers.clone();
    tokio::spawn(async move { post_receiver_thread(rx, post_receivers).await });
    let (commit_thread_tx, rx) = tokio::sync::mpsc::channel::<JetstreamEvent>(self.size);
    let commit_receivers = self.commit_receivers.clone();
    tokio::spawn(async move { commit_receiver_thread(rx, commit_receivers).await });
    let config = self.clone();
    tokio::spawn(async move { event_receiver_thread(config, commit_thread_tx).await });
    Ok(())
  }
}

async fn event_receiver_thread(config: Jetstream, tx: tokio::sync::mpsc::Sender<JetstreamEvent>) {
  loop {
    let mut request = reqwest::Client::new().get(&format!("wss://{}/subscribe", config.host));
    request = request.query(
      config
        .wanted_collections
        .iter()
        .map(|c| ("wantedCollections", c))
        .collect::<Vec<_>>()
        .as_slice(),
    );
    request = request.query(
      config
        .wanted_dids
        .iter()
        .map(|d| ("wantedDids", d))
        .collect::<Vec<_>>()
        .as_slice(),
    );
    if let Some(max_message_size_bytes) = &config.max_message_size_bytes {
      request = request.query(&[("maxMessageSizeBytes", max_message_size_bytes)]);
    }
    if let Some(cursor) = &config.cursor {
      request = request.query(&[("cursor", cursor)]);
    }
    if let Some(compress) = &config.compress {
      request = request.query(&[("compress", compress)]);
    }
    if let Some(require_hello) = &config.require_hello {
      request = request.query(&[("requireHello", require_hello)]);
    }
    let response = match reqwest_websocket::RequestBuilderExt::upgrade(request)
      .send()
      .await
    {
      Ok(r) => r,
      Err(e) => {
        tracing::error!("{e}");
        std::process::exit(0);
      }
    };
    let mut socket = match response.into_websocket().await {
      Ok(s) => s,
      Err(e) => {
        tracing::error!("{e}");
        std::process::exit(0);
      }
    };
    loop {
      match futures_util::StreamExt::next(&mut socket).await {
        Some(e) => match e {
          Ok(e) => match e {
            reqwest_websocket::Message::Text(t) => {
              let Ok(event) = serde_json::from_str(&t) else {
                continue;
              };
              tracing::debug!("{event:?}");
              if let Err(e) = tx.send(event).await {
                tracing::error!("{e}");
                std::process::exit(0);
              }
            }
            _ => {}
          },
          Err(e) => {
            tracing::warn!("{e}");
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            break;
          }
        },
        None => {}
      }
    }
  }
}

async fn commit_receiver_thread(
  mut receiver: tokio::sync::mpsc::Receiver<JetstreamEvent>,
  commit_receivers: std::sync::Arc<
    tokio::sync::RwLock<Vec<tokio::sync::mpsc::Sender<JetstreamEvent>>>,
  >,
) {
  let mut counter: u64 = 0;
  loop {
    let event = match receiver.recv().await {
      Some(e) => e,
      None => continue,
    };
    if event.kind == JetstreamKind::Commit {
      counter += 1;
      if counter % 1000 == 0 {
        tracing::debug!("COMMIT_RECEIVER : received {counter}");
      }
      for tx in commit_receivers.read().await.iter() {
        if let Err(e) = tx.send(event.clone()).await {
          tracing::warn!("COMMIT_RECEIVER : send record error {e}");
        }
      }
    }
  }
}

async fn post_receiver_thread(
  mut receiver: tokio::sync::mpsc::Receiver<JetstreamEvent>,
  post_receivers: std::sync::Arc<
    tokio::sync::RwLock<Vec<tokio::sync::mpsc::Sender<JetstreamEvent>>>,
  >,
) {
  let mut counter: u64 = 0;
  loop {
    let event = match receiver.recv().await {
      Some(e) => e,
      None => continue,
    };
    if let Some(commit) = &event.commit {
      if let Some(record) = &commit.record {
        if record.as_app_bsky_feed_post().is_some() {
          counter += 1;
          if counter % 1000 == 0 {
            tracing::debug!("POST_RECEIVER : received {counter}");
          }
          for tx in post_receivers.read().await.iter() {
            if let Err(e) = tx.send(event.clone()).await {
              tracing::warn!("POST_RECEIVER : send record error {e}");
            }
          }
        }
      }
    }
  }
}

async fn ja_receiver_thread(
  mut receiver: tokio::sync::mpsc::Receiver<JetstreamEvent>,
  ja_receivers: std::sync::Arc<tokio::sync::RwLock<Vec<tokio::sync::mpsc::Sender<JetstreamEvent>>>>,
) {
  let mut counter: u64 = 0;
  loop {
    let event = match receiver.recv().await {
      Some(e) => e,
      None => continue,
    };
    if let Some(commit) = &event.commit {
      if let Some(record) = &commit.record {
        if let Some(post) = record.as_app_bsky_feed_post() {
          if post
            .langs
            .as_ref()
            .map(|l| l.contains(&String::from("ja")))
            .unwrap_or(false)
          {
            counter += 1;
            if counter % 1000 == 0 {
              tracing::debug!("JA_RECEIVER : received {counter}");
            }
            for tx in ja_receivers.read().await.iter() {
              if let Err(e) = tx.send(event.clone()).await {
                tracing::warn!("JA_RECEIVER : send record error {e}");
              }
            }
          }
        }
      }
    }
  }
}

async fn token_receiver_thread(
  mut receiver: tokio::sync::mpsc::Receiver<JetstreamEvent>,
  token_receivers: std::sync::Arc<
    tokio::sync::RwLock<Vec<tokio::sync::mpsc::Sender<(JetstreamEvent, Vec<Vec<String>>)>>>,
  >,
  user_dict: Option<String>,
) {
  let mut counter: u64 = 0;
  let mut builder = lindera::tokenizer::TokenizerBuilder::new().unwrap();
  builder.set_segmenter_dictionary_kind(&lindera::dictionary::DictionaryKind::IPADIC);
  builder.set_segmenter_mode(&lindera::mode::Mode::Normal);
  if let Some(u) = &user_dict {
    let path = std::path::Path::new(u);
    if path.exists() {
      builder.set_segmenter_user_dictionary_path(path);
      builder.set_segmenter_user_dictionary_kind(&lindera::dictionary::DictionaryKind::IPADIC);
      tracing::info!("TOKEN_RECEIVER : LOAD USER DICTIONARY : {u}");
    }
  }
  let mut tokenizer = builder.build().unwrap();
  let mut last_loaded = tokio::time::Instant::now();
  loop {
    if last_loaded.elapsed() > std::time::Duration::from_secs(600) {
      let mut builder = lindera::tokenizer::TokenizerBuilder::new().unwrap();
      builder.set_segmenter_dictionary_kind(&lindera::dictionary::DictionaryKind::IPADIC);
      builder.set_segmenter_mode(&lindera::mode::Mode::Normal);
      if let Some(u) = &user_dict {
        let path = std::path::Path::new(u);
        if path.exists() {
          builder.set_segmenter_user_dictionary_path(path);
          builder.set_segmenter_user_dictionary_kind(&lindera::dictionary::DictionaryKind::IPADIC);
        }
      }
      tokenizer = builder.build().unwrap();
      last_loaded = tokio::time::Instant::now();
      if let Some(u) = &user_dict {
        tracing::info!("TOKEN_RECEIVER : RELOAD USER DICTIONARY : {u}");
      } else {
        tracing::info!("TOKEN_RECEIVER : NO USER DICTIONARY");
      }
    }
    let event = match receiver.recv().await {
      Some(e) => e,
      None => continue,
    };
    if let Some(commit) = &event.commit {
      if let Some(record) = &commit.record {
        if let Some(post) = record.as_app_bsky_feed_post() {
          match tokenizer
            .tokenize(&post.text)
            .map(|mut tokens| {
              tokens
                .iter_mut()
                .map(|t| {
                  vec![
                    vec![t.text.to_string()],
                    t.details()
                      .iter()
                      .map(|t| t.to_string())
                      .collect::<Vec<_>>(),
                  ]
                  .concat()
                })
                .collect::<Vec<_>>()
            })
            .as_ref()
          {
            Ok(t) => {
              counter += 1;
              if counter % 1000 == 0 {
                tracing::debug!("TOKEN_RECEIVER : received {counter}");
              }
              for tx in token_receivers.read().await.iter() {
                if let Err(e) = tx.send((event.clone(), t.clone())).await {
                  tracing::warn!("TOKEN_RECEIVER : send record error {e}");
                }
              }
            }
            Err(e) => {
              tracing::warn!("TOKEN_RECEIVER : tokenize error {e}");
            }
          }
        }
      }
    }
  }
}
