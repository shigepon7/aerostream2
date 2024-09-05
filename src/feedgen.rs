use crate::*;

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
  pub cache: Vec<String>,
}

impl FeedGeneratorFeed {
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
      cache: Vec::new(),
    }
  }

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

  pub fn add_avatar(&mut self, avatar: &[u8], mime_type: &str) {
    self.avatar = (!avatar.is_empty()).then(|| (avatar.to_vec(), mime_type.to_string()));
  }

  pub fn add_avatar_from_file(&mut self, path: &str) {
    self.avatar = std::fs::read(path).ok().and_then(|d| {
      mime_guess::from_path(path)
        .first()
        .map(|m| (d, m.to_string()))
    });
  }

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

  pub fn to_atproto(&self, server: &str, avatar: Option<Blob>) -> AppBskyFeedGenerator {
    AppBskyFeedGenerator {
      did: format!("did:web:{server}"),
      display_name: self.display_name.clone(),
      description: self.description.clone(),
      description_facets: self.description_facets.clone(),
      avatar,
      accepts_interactions: self.accepts_interactions,
      labels: self.labels.clone(),
      created_at: self.created_at.to_rfc3339(),
    }
  }

  pub async fn push_post(&mut self, aturi: &str) {
    self.cache.push(aturi.to_string());
  }
}

#[derive(Clone)]
pub struct FeedGenerator {
  pub atproto: Atproto,
  pub did: String,
  pub feeds: Vec<FeedGeneratorFeed>,
}

impl FeedGenerator {
  pub async fn new(id: &str, pw: &str) -> anyhow::Result<Self> {
    let mut atproto = Atproto::default();
    atproto.login(id, pw).await?;
    let repo = atproto.com_atproto_repo_describe_repo(id).await?;
    Ok(Self {
      atproto,
      did: repo.did,
      feeds: Vec::new(),
    })
  }

  pub fn add_feed(&mut self, feed: FeedGeneratorFeed) {
    self.feeds.push(feed);
  }

  pub async fn insert_records(&self, server: &str) -> anyhow::Result<()> {
    let mut input = ComAtprotoRepoPutRecordInput {
      repo: self.did.clone(),
      collection: String::from("app.bsky.feed.generator"),
      rkey: String::new(),
      validate: None,
      record: serde_json::Value::Null,
      swap_record: None,
      swap_commit: None,
    };
    for feed in self.feeds.iter() {
      let avatar = match &feed.avatar {
        Some((d, m)) => self
          .atproto
          .com_atproto_repo_upload_blob(d.clone(), m)
          .await
          .ok()
          .map(|o| o.blob),
        None => None,
      };
      input.rkey = feed.rkey.clone();
      input.record = match serde_json::to_string(&feed.to_atproto(server, avatar))
        .and_then(|v| serde_json::from_str(&v))
      {
        Ok(r) => r,
        Err(e) => {
          tracing::warn!("FeedGenerator : {e}");
          continue;
        }
      };
      self
        .atproto
        .com_atproto_repo_put_record(input.clone())
        .await?;
    }
    Ok(())
  }

  pub fn to_atproto(&self) -> Vec<AppBskyFeedDescribeFeedGeneratorFeed> {
    self
      .feeds
      .iter()
      .map(|f| AppBskyFeedDescribeFeedGeneratorFeed {
        uri: format!("at://{}/app.bsky.feed.generator/{}", self.did, f.rkey),
      })
      .collect()
  }
}

#[derive(Clone)]
pub struct FeedGeneratorServer {
  pub hostname: String,
  pub generators: std::sync::Arc<tokio::sync::RwLock<Vec<FeedGenerator>>>,
  pub privacy_policy: Option<String>,
  pub terms_of_service: Option<String>,
}

impl FeedGeneratorServer {
  pub fn new(hostname: &str) -> Self {
    Self {
      hostname: hostname.to_string(),
      generators: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
      privacy_policy: None,
      terms_of_service: None,
    }
  }

  pub async fn insert_feed_generator(&mut self, gen: FeedGenerator) -> anyhow::Result<()> {
    let did = gen.did.clone();
    let mut generators = { self.generators.read().await.clone() }
      .iter()
      .filter(|g| g.did != did)
      .cloned()
      .collect::<Vec<_>>();
    generators.push(gen);
    let mut lock = self.generators.write().await;
    *lock = generators;
    Ok(())
  }

  pub fn set_privacy_policy(&mut self, privacy_policy: &str) {
    self.privacy_policy = (!privacy_policy.is_empty()).then(|| privacy_policy.to_string());
  }

  pub fn set_terms_of_service(&mut self, terms_of_service: &str) {
    self.terms_of_service = (!terms_of_service.is_empty()).then(|| terms_of_service.to_string());
  }

  pub async fn insert_records(&self) -> anyhow::Result<()> {
    let generators = { self.generators.read().await.clone() };
    for generator in generators.iter() {
      if let Err(e) = generator.insert_records(&self.hostname).await {
        tracing::warn!("cannot put feed generator record {e}");
      }
    }
    Ok(())
  }

  pub async fn start(&self) -> anyhow::Result<()> {
    let app = axum::Router::new()
      .route("/xrpc/:nsid", axum::routing::get(xrpc_server))
      .with_state(self.clone());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
  }
}

async fn xrpc_server(
  axum::extract::Path(nsid): axum::extract::Path<String>,
  axum::extract::Query(query): axum::extract::Query<std::collections::HashMap<String, String>>,
  axum::extract::State(server): axum::extract::State<FeedGeneratorServer>,
) -> Result<axum::response::Response, axum::http::StatusCode> {
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
        feeds: { server.generators.read().await.clone() }
          .iter()
          .map(|gen| gen.to_atproto())
          .flatten()
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
        None => return Err(axum::http::StatusCode::BAD_REQUEST),
      };
      tracing::debug!("FEED : {feed}");
      let mut splitted = feed.split("/").skip(2);
      let did = match splitted.next() {
        Some(d) => d.to_string(),
        None => return Err(axum::http::StatusCode::BAD_REQUEST),
      };
      tracing::debug!("DID : {did}");
      let generators = { server.generators.read().await.clone() };
      let feedgen = match generators.iter().find(|g| g.did == did) {
        Some(g) => g,
        None => return Err(axum::http::StatusCode::NOT_FOUND),
      };
      tracing::debug!("FEEDGEN : {}", feedgen.did);
      let nsid = match splitted.next() {
        Some(p) => p,
        None => return Err(axum::http::StatusCode::BAD_REQUEST),
      };
      tracing::debug!("NSID : {nsid}");
      if nsid != "app.bsky.feed.generator" {
        return Err(axum::http::StatusCode::BAD_REQUEST);
      }
      tracing::debug!("NSID : OK");
      let rkey = match splitted.next() {
        Some(p) => p,
        None => return Err(axum::http::StatusCode::BAD_REQUEST),
      };
      tracing::debug!("RKEY : {rkey}");
      let feed = match feedgen.feeds.iter().find(|f| f.rkey == rkey) {
        Some(f) => f,
        None => return Err(axum::http::StatusCode::NOT_FOUND),
      };
      tracing::debug!("FEED : {}", feed.rkey);
      let limit = query
        .get("limit")
        .and_then(|l| l.parse::<usize>().ok())
        .unwrap_or(30);
      tracing::debug!("LIMIT : {limit}");
      let feeds = match query.get("cursor") {
        Some(cursor) => feed
          .cache
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
        None => feed
          .cache
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
      let cursor = feeds
        .last()
        .map(|f| &f.post)
        .and_then(|p| feed.cache.last().and_then(|l| (p != l).then(|| p.clone())));
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
