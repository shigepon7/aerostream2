//! Feed Generator

use crate::*;

/// Dynamic Feed Generator, which returns different feeds each time it is accessed
#[async_trait::async_trait]
pub trait FeedGeneratorDynamic: Sync + Send {
  fn feed(&self) -> FeedGeneratorFeed;
  async fn algorithm(
    &self,
    headers: &axum::http::HeaderMap,
    cursor: Option<String>,
    limit: Option<usize>,
  ) -> std::result::Result<AppBskyFeedGetFeedSkeletonOutput, axum::http::StatusCode>;
}

/// Feed Generator record
#[derive(Clone)]
pub struct FeedGeneratorFeed {
  pub owner: String,
  pub rkey: String,
  pub display_name: String,
  pub description: Option<String>,
  pub description_facets: Option<Vec<AppBskyRichtextFacet>>,
  pub avatar: Option<(Vec<u8>, String)>,
  pub accepts_interactions: Option<bool>,
  pub labels: Option<AppBskyFeedGeneratorLabelsUnion>,
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub cache: std::sync::Arc<tokio::sync::RwLock<std::collections::VecDeque<String>>>,
  pub alias: Option<String>,
}

/// Feed Generator snapshot for serialization
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FeedGeneratorFeedSnapshot {
  pub owner: String,
  pub rkey: String,
  pub display_name: String,
  pub description: Option<String>,
  pub description_facets: Option<Vec<AppBskyRichtextFacet>>,
  pub avatar: Option<(String, String)>,
  pub accepts_interactions: Option<bool>,
  pub labels: Option<AppBskyFeedGeneratorLabelsUnion>,
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub cache: std::collections::VecDeque<String>,
  pub alias: Option<String>,
}

impl FeedGeneratorFeed {
  /// convert Feed Generator snapshot from Feed Generator record
  pub async fn snapshot(&self, avatar_filename: Option<&str>) -> FeedGeneratorFeedSnapshot {
    let cache = { self.cache.read().await.clone() };
    let mut snapshot = FeedGeneratorFeedSnapshot {
      owner: self.owner.clone(),
      rkey: self.rkey.clone(),
      display_name: self.display_name.clone(),
      description: self.description.clone(),
      description_facets: self.description_facets.clone(),
      avatar: None,
      accepts_interactions: self.accepts_interactions,
      labels: self.labels.clone(),
      created_at: self.created_at,
      cache,
      alias: self.alias.clone(),
    };
    if let Some(avatar) = &self.avatar {
      if let Some(filename) = avatar_filename {
        match std::fs::File::create(filename) {
          Ok(mut file) => match std::io::Write::write_all(&mut file, &avatar.0) {
            Ok(_) => snapshot.avatar = Some((filename.to_string(), avatar.1.clone())),
            Err(e) => tracing::warn!("avatar save failed : {} : {e}", self.to_aturi()),
          },
          Err(e) => {
            tracing::warn!("avatar file create failed : {} : {e}", self.to_aturi());
          }
        }
      }
    }
    snapshot
  }

  /// convert Feed Generator record from Feed Generator snapshot
  pub fn from_snapshot(snapshot: &FeedGeneratorFeedSnapshot) -> Self {
    let mut feed = Self {
      owner: snapshot.owner.clone(),
      rkey: snapshot.rkey.clone(),
      display_name: snapshot.display_name.clone(),
      description: snapshot.description.clone(),
      description_facets: snapshot.description_facets.clone(),
      avatar: None,
      accepts_interactions: snapshot.accepts_interactions,
      labels: snapshot.labels.clone(),
      created_at: snapshot.created_at,
      cache: std::sync::Arc::new(tokio::sync::RwLock::new(snapshot.cache.clone())),
      alias: snapshot.alias.clone(),
    };
    if let Some((filename, mimetype)) = &snapshot.avatar {
      match std::fs::read(filename) {
        Ok(avatar) => {
          feed.avatar = Some((avatar, mimetype.clone()));
        }
        Err(e) => {
          tracing::warn!("avatar file read failed : {} : {e}", feed.to_aturi());
        }
      }
    }
    feed
  }

  /// create new Feed Generator record
  pub fn new(owner_did: &str, rkey: &str, display_name: &str) -> Self {
    Self {
      owner: owner_did.to_string(),
      rkey: rkey.to_string(),
      display_name: display_name.to_string(),
      description: None,
      description_facets: None,
      avatar: None,
      accepts_interactions: None,
      labels: None,
      created_at: chrono::Utc::now(),
      cache: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::VecDeque::new())),
      alias: None,
    }
  }

  /// create new Feed Generator record, which is an alias of the other Feed Generator record
  pub fn alias(owner_did: &str, rkey: &str, display_name: &str, alias: &str) -> Self {
    Self {
      owner: owner_did.to_string(),
      rkey: rkey.to_string(),
      display_name: display_name.to_string(),
      description: None,
      description_facets: None,
      avatar: None,
      accepts_interactions: None,
      labels: None,
      created_at: chrono::Utc::now(),
      cache: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::VecDeque::new())),
      alias: Some(alias.to_string()),
    }
  }

  /// add description to Feed Generator record
  pub fn add_description(&mut self, description: &str, decoration: Vec<TextDecoration>) {
    self.description = (!description.is_empty()).then(|| description.to_string());
    self.description_facets = (!decoration.is_empty()).then(|| {
      decoration
        .iter()
        .map(|d| d.to_atproto(description))
        .flatten()
        .collect::<Vec<_>>()
    });
  }

  /// add avatar to Feed Generator record from binary
  pub fn add_avatar(&mut self, avatar: &[u8], mime_type: &str) {
    self.avatar = (!avatar.is_empty()).then(|| (avatar.to_vec(), mime_type.to_string()));
  }

  /// add avatar to Feed Generator record from file
  pub fn add_avatar_from_file(&mut self, path: &str) {
    self.avatar = std::fs::read(path).ok().and_then(|d| {
      mime_guess::from_path(path)
        .first()
        .map(|m| (d, m.to_string()))
    });
  }

  /// add labels to Feed Generator record
  pub fn add_labels(&mut self, labels: &[String]) {
    self.labels = (!labels.is_empty()).then(|| {
      AppBskyFeedGeneratorLabelsUnion::ComAtprotoLabelDefsSelfLabels(Box::new(
        ComAtprotoLabelDefsSelfLabels {
          values: labels
            .iter()
            .map(|l| ComAtprotoLabelDefsSelfLabel { val: l.clone() })
            .collect::<Vec<_>>(),
        },
      ))
    });
  }

  /// convert atproto Record from Feed Generator record
  pub fn to_atproto(&self, server: &str, avatar: Option<Blob>) -> AppBskyFeedGenerator {
    AppBskyFeedGenerator {
      did: format!("did:web:{server}"),
      display_name: self.display_name.clone(),
      description: self.description.clone(),
      description_facets: self.description_facets.clone(),
      avatar,
      accepts_interactions: self.accepts_interactions,
      labels: self.labels.clone(),
      created_at: self.created_at,
    }
  }

  /// get at-uri from Feed Generator record
  pub fn to_aturi(&self) -> String {
    format!("at://{}/app.bsky.feed.generator/{}", self.owner, self.rkey)
  }

  /// push post into Feed Generator to return feeds when it is accessed
  pub async fn push_post(&mut self, aturi: &str) {
    self.cache.write().await.push_front(aturi.to_string());
  }
}

/// access logs of the Feed Generators
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FeedGeneratorAccessLog {
  pub did: String,
  pub feed: String,
  pub cursor: Option<String>,
  pub limit: Option<usize>,
  pub status_code: u16,
  pub len: Option<usize>,
  pub next: Option<String>,
  pub accessed_at: chrono::DateTime<chrono::Utc>,
  pub returned_at: chrono::DateTime<chrono::Utc>,
}

impl FeedGeneratorAccessLog {
  /// create a log
  pub fn new(
    feed: &str,
    cursor: &Option<String>,
    limit: &Option<usize>,
    headers: &axum::http::HeaderMap<axum::http::HeaderValue>,
  ) -> Result<Self> {
    Ok(Self {
      did: get_did_from_request_header(headers)?,
      feed: feed.to_string(),
      cursor: cursor.clone(),
      limit: limit.clone(),
      status_code: 0,
      len: None,
      next: None,
      accessed_at: chrono::Utc::now(),
      returned_at: chrono::DateTime::default(),
    })
  }

  /// store result SUCCESS
  pub fn success(&mut self, len: usize, next: &Option<String>) {
    self.status_code = axum::http::StatusCode::OK.as_u16();
    self.len = Some(len);
    self.next = next.clone();
    self.returned_at = chrono::Utc::now();
  }

  /// store result ERROR
  pub fn error(&mut self, code: &axum::http::StatusCode) {
    self.status_code = code.as_u16();
    self.len = None;
    self.next = None;
    self.returned_at = chrono::Utc::now();
  }
}

/// Feed Generator server
#[derive(Clone)]
pub struct FeedGenerator {
  pub hostname: String,
  pub feeds: std::sync::Arc<tokio::sync::RwLock<indexmap::IndexMap<String, FeedGeneratorFeed>>>,
  pub dynamic_feeds:
    std::sync::Arc<tokio::sync::RwLock<indexmap::IndexMap<String, Box<dyn FeedGeneratorDynamic>>>>,
  pub privacy_policy: Option<String>,
  pub terms_of_service: Option<String>,
  pub access_log: std::sync::Arc<tokio::sync::RwLock<Vec<FeedGeneratorAccessLog>>>,
  pub sessions: std::collections::HashMap<String, Atproto>,
}

impl FeedGenerator {
  /// create a new Feed Generator server
  pub fn new(hostname: &str) -> Self {
    Self {
      hostname: hostname.to_string(),
      feeds: std::sync::Arc::new(tokio::sync::RwLock::new(indexmap::IndexMap::new())),
      dynamic_feeds: std::sync::Arc::new(tokio::sync::RwLock::new(indexmap::IndexMap::new())),
      privacy_policy: None,
      terms_of_service: None,
      access_log: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
      sessions: std::collections::HashMap::new(),
    }
  }

  /// insert a Feed Generator record
  pub async fn insert_feed(
    &mut self,
    feed: FeedGeneratorFeed,
    server: &str,
    handle: &str,
    password: &str,
  ) {
    let atproto = match self.sessions.get_mut(handle) {
      Some(session) => match session.refresh().await {
        Ok(_) => {
          tracing::debug!("{} refresh session succeeded", feed.display_name);
          session
        }
        Err(e) => {
          tracing::warn!("{} refresh session error {e:?}", feed.display_name);
          return;
        }
      },
      None => {
        let mut session = Atproto::default();
        if let Err(e) = session.login(handle, password).await {
          tracing::warn!("{} login error {e:?}", feed.display_name);
          return;
        }
        tracing::debug!("{} login succeeded", feed.display_name);
        self.sessions.insert(handle.to_string(), session);
        match self.sessions.get_mut(handle) {
          Some(session) => session,
          None => {
            tracing::warn!("{} reload error", feed.display_name);
            return;
          }
        }
      }
    };
    let avatar = match &feed.avatar {
      Some((d, m)) => atproto
        .com_atproto_repo_upload_blob(d.clone(), m)
        .await
        .ok()
        .map(|o| o.blob),
      None => None,
    };
    match serde_json::to_string(&feed.to_atproto(server, avatar))
      .and_then(|v| serde_json::from_str(&v))
      .map_err(|e| crate::Error::Parse((e, String::new())))
    {
      Ok(record) => {
        let input = ComAtprotoRepoPutRecordInput {
          repo: feed.owner.clone(),
          collection: String::from("app.bsky.feed.generator"),
          rkey: feed.rkey.clone(),
          validate: None,
          record,
          swap_record: None,
          swap_commit: None,
        };
        if let Err(e) = atproto.com_atproto_repo_put_record(input.clone()).await {
          tracing::warn!("{} putRecord error {e:?}", feed.display_name);
        }
        self.feeds.write().await.insert(feed.to_aturi(), feed);
      }
      Err(e) => {
        tracing::warn!("{} feed convert error {e:?}", feed.display_name);
      }
    }
  }

  /// insert a Dynamic Feed Generator
  pub async fn insert_dynamic(
    &mut self,
    dynamic: Box<dyn FeedGeneratorDynamic>,
    server: &str,
    handle: &str,
    password: &str,
  ) {
    let feed = dynamic.feed();
    let uri = feed.to_aturi();
    self.insert_feed(feed, server, handle, password).await;
    self.dynamic_feeds.write().await.insert(uri, dynamic);
  }

  /// set Feed Generator server privacy policy
  pub fn set_privacy_policy(&mut self, privacy_policy: &str) {
    self.privacy_policy = (!privacy_policy.is_empty()).then(|| privacy_policy.to_string());
  }

  /// set Feed Generator terms of service
  pub fn set_terms_of_service(&mut self, terms_of_service: &str) {
    self.terms_of_service = (!terms_of_service.is_empty()).then(|| terms_of_service.to_string());
  }

  /// start Feed Generator server
  pub async fn start(&self) -> crate::Result<()> {
    let app = axum::Router::new()
      .route("/xrpc/:nsid", axum::routing::get(xrpc_server))
      .route("/.well-known/did.json", axum::routing::get(did_document))
      .with_state(self.clone());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
  }

  /// record access log
  pub async fn insert_log(&self, log: FeedGeneratorAccessLog) {
    self.access_log.write().await.push(log);
  }

  /// read access logs
  pub async fn read_log(&self) -> Vec<FeedGeneratorAccessLog> {
    self.access_log.read().await.clone()
  }

  /// read access logs and erase all access logs
  pub async fn read_log_and_clean(&self) -> Vec<FeedGeneratorAccessLog> {
    let mut lock = self.access_log.write().await;
    let ret = lock.clone();
    *lock = Vec::new();
    ret
  }
}

/// axum handler of .well-known did-doc
async fn did_document(
  axum::extract::State(server): axum::extract::State<FeedGenerator>,
) -> axum::response::Response {
  let document = serde_json::json!({
  "@context":["https://www.w3.org/ns/did/v1"],
  "id":format!("did:web:{}", &server.hostname),
  "service":[{
    "id":"#bsky_fg",
    "type":"BskyFeedGenerator",
    "serviceEndpoint":format!("https://{}", &server.hostname)}]
  });
  axum::response::IntoResponse::into_response(axum::Json(document))
}

/// axum handler of xrpc server
async fn xrpc_server(
  headers: axum::http::HeaderMap,
  axum::extract::Path(nsid): axum::extract::Path<String>,
  axum::extract::Query(query): axum::extract::Query<std::collections::HashMap<String, String>>,
  axum::extract::State(server): axum::extract::State<FeedGenerator>,
) -> std::result::Result<axum::response::Response, axum::http::StatusCode> {
  match nsid.as_str() {
    "app.bsky.feed.describeFeedGenerator" => {
      let links =
        (server.privacy_policy.is_some() || server.terms_of_service.is_some()).then(|| {
          AppBskyFeedDescribeFeedGeneratorLinks {
            privacy_policy: server.privacy_policy.clone(),
            terms_of_service: server.terms_of_service.clone(),
          }
        });
      let output = AppBskyFeedDescribeFeedGeneratorOutput {
        did: format!("did:web:{}", server.hostname),
        feeds: { server.feeds.read().await.clone() }
          .values()
          .map(|feed| AppBskyFeedDescribeFeedGeneratorFeed {
            uri: feed.to_aturi(),
          })
          .collect(),
        links,
      };
      Ok(axum::response::IntoResponse::into_response(axum::Json(
        output,
      )))
    }
    "app.bsky.feed.getFeedSkeleton" => {
      let feed = match query.get("feed") {
        Some(f) => f,
        None => {
          tracing::warn!("no feed query");
          return Err(axum::http::StatusCode::BAD_REQUEST);
        }
      };
      let cursor = query.get("cursor").cloned();
      let limit = query.get("limit").and_then(|l| l.parse().ok());
      let mut log = FeedGeneratorAccessLog::new(&feed, &cursor, &limit, &headers).ok();
      tracing::debug!("app.bsky.feed.getFeedSkeleton : {feed}");

      {
        if let Some(d) = server.dynamic_feeds.read().await.get(feed) {
          tracing::debug!("dynamic : {feed}");
          match d.algorithm(&headers, cursor.clone(), limit.clone()).await {
            Ok(r) => {
              if let Some(log) = log.as_mut() {
                log.success(r.feed.len(), &r.cursor);
                server.insert_log(log.clone()).await;
              }
              return Ok(axum::response::IntoResponse::into_response(axum::Json(r)));
            }
            Err(e) => {
              if let Some(log) = log.as_mut() {
                log.error(&e);
                server.insert_log(log.clone()).await;
              }
              return Err(e);
            }
          }
        }
      }

      tracing::debug!("static : {feed}");
      let feeds = { server.feeds.read().await.clone() };
      let mut feed = match feeds.get(feed).clone() {
        Some(f) => f,
        None => {
          tracing::warn!("no such feed {feed}");
          if let Some(log) = log.as_mut() {
            log.error(&axum::http::StatusCode::NOT_FOUND);
            server.insert_log(log.clone()).await;
          }
          return Err(axum::http::StatusCode::NOT_FOUND);
        }
      };
      if let Some(alias) = &feed.alias {
        {
          if let Some(d) = server.dynamic_feeds.read().await.get(alias) {
            tracing::debug!("dynamic alias : {alias}");
            match d.algorithm(&headers, cursor.clone(), limit.clone()).await {
              Ok(r) => {
                if let Some(log) = log.as_mut() {
                  log.success(r.feed.len(), &r.cursor);
                  server.insert_log(log.clone()).await;
                }
                return Ok(axum::response::IntoResponse::into_response(axum::Json(r)));
              }
              Err(e) => {
                if let Some(log) = log.as_mut() {
                  log.error(&e);
                  server.insert_log(log.clone()).await;
                }
                return Err(e);
              }
            }
          }
        }
        if let Some(destination) = feeds.get(alias) {
          tracing::debug!("alias {} to {}", feed.to_aturi(), alias);
          feed = destination;
        }
      }
      let limit = limit.unwrap_or(30);
      tracing::debug!("LIMIT : {limit}");
      let cache = { feed.cache.read().await.clone() };
      let feeds = match &cursor {
        Some(cursor) => cache
          .iter()
          .skip_while(|c| cursor != *c)
          .skip(1)
          .take(limit)
          .map(|f| AppBskyFeedDefsSkeletonFeedPost {
            post: f.clone(),
            reason: None,
            feed_context: None,
          })
          .collect::<Vec<_>>(),
        None => cache
          .iter()
          .take(limit)
          .map(|f| AppBskyFeedDefsSkeletonFeedPost {
            post: f.clone(),
            reason: None,
            feed_context: None,
          })
          .collect::<Vec<_>>(),
      };
      tracing::debug!("FEEDS : {feeds:?}");
      let cursor = feeds.last().map(|f| &f.post).and_then(|p| {
        cache
          .iter()
          .last()
          .and_then(|l| (p != l).then(|| p.clone()))
      });
      if let Some(log) = log.as_mut() {
        log.success(feeds.len(), &cursor);
        server.insert_log(log.clone()).await;
      }
      tracing::debug!("CURSOR : {cursor:?}");
      Ok(axum::response::IntoResponse::into_response(axum::Json(
        AppBskyFeedGetFeedSkeletonOutput {
          cursor,
          feed: feeds,
        },
      )))
    }
    _ => Err(axum::http::StatusCode::NOT_FOUND),
  }
}
