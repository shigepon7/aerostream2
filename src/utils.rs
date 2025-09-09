use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Object {
  Commit(ComAtprotoSyncSubscribeReposCommit),
  Sync(ComAtprotoSyncSubscribeReposSync),
  Identity(ComAtprotoSyncSubscribeReposIdentity),
  Account(ComAtprotoSyncSubscribeReposAccount),
  Info(ComAtprotoSyncSubscribeReposInfo),
  RepoOp(ComAtprotoSyncSubscribeReposRepoOp),
}

impl From<ComAtprotoSyncSubscribeReposCommit> for Object {
  fn from(value: ComAtprotoSyncSubscribeReposCommit) -> Self {
    Self::Commit(value)
  }
}

impl From<ComAtprotoSyncSubscribeReposIdentity> for Object {
  fn from(value: ComAtprotoSyncSubscribeReposIdentity) -> Self {
    Self::Identity(value)
  }
}

impl From<ComAtprotoSyncSubscribeReposAccount> for Object {
  fn from(value: ComAtprotoSyncSubscribeReposAccount) -> Self {
    Self::Account(value)
  }
}

impl From<ComAtprotoSyncSubscribeReposInfo> for Object {
  fn from(value: ComAtprotoSyncSubscribeReposInfo) -> Self {
    Self::Info(value)
  }
}

impl From<ComAtprotoSyncSubscribeReposRepoOp> for Object {
  fn from(value: ComAtprotoSyncSubscribeReposRepoOp) -> Self {
    Self::RepoOp(value)
  }
}

impl TryFrom<&reqwest_websocket::Message> for Object {
  type Error = crate::Error;
  fn try_from(value: &reqwest_websocket::Message) -> std::result::Result<Self, Self::Error> {
    if let reqwest_websocket::Message::Binary(bin) = value {
      let header = ciborium::from_reader::<ciborium::Value, _>(std::io::Cursor::new(bin.to_vec()))?;
      let mut buf = Vec::new();
      ciborium::into_writer(&header, &mut buf)?;
      if let Ok(commit) =
        ciborium::from_reader::<ComAtprotoSyncSubscribeReposCommit, _>(&bin[buf.len()..])
      {
        return Ok(commit.into());
      } else if let Ok(identity) =
        ciborium::from_reader::<ComAtprotoSyncSubscribeReposIdentity, _>(&bin[buf.len()..])
      {
        return Ok(identity.into());
      } else if let Ok(account) =
        ciborium::from_reader::<ComAtprotoSyncSubscribeReposAccount, _>(&bin[buf.len()..])
      {
        return Ok(account.into());
      } else if let Ok(info) =
        ciborium::from_reader::<ComAtprotoSyncSubscribeReposInfo, _>(&bin[buf.len()..])
      {
        return Ok(info.into());
      } else if let Ok(repoop) =
        ciborium::from_reader::<ComAtprotoSyncSubscribeReposRepoOp, _>(&bin[buf.len()..])
      {
        return Ok(repoop.into());
      } else {
        return Err(Error::Other(String::from("unknown data type")));
      }
    }
    return Err(Error::Other(String::from("not binary message")));
  }
}

impl Object {
  pub fn as_commit(&self) -> Option<&ComAtprotoSyncSubscribeReposCommit> {
    match self {
      Self::Commit(c) => Some(c),
      _ => None,
    }
  }

  pub fn as_identity(&self) -> Option<&ComAtprotoSyncSubscribeReposIdentity> {
    match self {
      Self::Identity(i) => Some(i),
      _ => None,
    }
  }

  pub fn as_account(&self) -> Option<&ComAtprotoSyncSubscribeReposAccount> {
    match self {
      Self::Account(a) => Some(a),
      _ => None,
    }
  }

  pub fn as_info(&self) -> Option<&ComAtprotoSyncSubscribeReposInfo> {
    match self {
      Self::Info(i) => Some(i),
      _ => None,
    }
  }

  pub fn as_repo_op(&self) -> Option<&ComAtprotoSyncSubscribeReposRepoOp> {
    match self {
      Self::RepoOp(r) => Some(r),
      _ => None,
    }
  }
}

impl ComAtprotoSyncSubscribeReposCommit {
  pub async fn to_records(&self) -> Vec<Record> {
    let Ok((blocks, _)) = rs_car::car_read_all(&mut self.blocks.as_slice(), true).await else {
      return Vec::new();
    };
    blocks
      .iter()
      .filter_map(|(_, block)| serde_ipld_dagcbor::from_reader::<Record, _>(block.as_slice()).ok())
      .collect::<Vec<_>>()
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum Record {
  #[serde(rename = "app.bsky.actor.profile")]
  AppBskyActorProfile(AppBskyActorProfile),
  #[serde(rename = "app.bsky.feed.generator")]
  AppBskyFeedGenerator(AppBskyFeedGenerator),
  #[serde(rename = "app.bsky.feed.like")]
  AppBskyFeedLike(AppBskyFeedLike),
  #[serde(rename = "app.bsky.feed.post")]
  AppBskyFeedPost(AppBskyFeedPost),
  #[serde(rename = "app.bsky.feed.postgate")]
  AppBskyFeedPostgate(AppBskyFeedPostgate),
  #[serde(rename = "app.bsky.feed.repost")]
  AppBskyFeedRepost(AppBskyFeedRepost),
  #[serde(rename = "app.bsky.feed.threadgate")]
  AppBskyFeedThreadgate(AppBskyFeedThreadgate),
  #[serde(rename = "app.bsky.graph.block")]
  AppBskyGraphBlock(AppBskyGraphBlock),
  #[serde(rename = "app.bsky.graph.follow")]
  AppBskyGraphFollow(AppBskyGraphFollow),
  #[serde(rename = "app.bsky.graph.list")]
  AppBskyGraphList(AppBskyGraphList),
  #[serde(rename = "app.bsky.graph.listblock")]
  AppBskyGraphListblock(AppBskyGraphListblock),
  #[serde(rename = "app.bsky.graph.listitem")]
  AppBskyGraphListitem(AppBskyGraphListitem),
  #[serde(rename = "app.bsky.graph.starterpack")]
  AppBskyGraphStarterpack(AppBskyGraphStarterpack),
  #[serde(rename = "app.bsky.labeler.service")]
  AppBskyLabelerService(AppBskyLabelerService),
  #[serde(rename = "chat.bsky.actor.declaration")]
  ChatBskyActorDeclaration(ChatBskyActorDeclaration),
  #[serde(untagged)]
  Other(serde_json::Value),
}

impl Record {
  pub fn as_app_bsky_actor_profile(&self) -> Option<&AppBskyActorProfile> {
    match self {
      Self::AppBskyActorProfile(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_feed_generator(&self) -> Option<&AppBskyFeedGenerator> {
    match self {
      Self::AppBskyFeedGenerator(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_feed_like(&self) -> Option<&AppBskyFeedLike> {
    match self {
      Self::AppBskyFeedLike(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_feed_post(&self) -> Option<&AppBskyFeedPost> {
    match self {
      Self::AppBskyFeedPost(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_feed_postgate(&self) -> Option<&AppBskyFeedPostgate> {
    match self {
      Self::AppBskyFeedPostgate(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_feed_repost(&self) -> Option<&AppBskyFeedRepost> {
    match self {
      Self::AppBskyFeedRepost(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_feed_threadgate(&self) -> Option<&AppBskyFeedThreadgate> {
    match self {
      Self::AppBskyFeedThreadgate(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_graph_block(&self) -> Option<&AppBskyGraphBlock> {
    match self {
      Self::AppBskyGraphBlock(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_graph_follow(&self) -> Option<&AppBskyGraphFollow> {
    match self {
      Self::AppBskyGraphFollow(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_graph_list(&self) -> Option<&AppBskyGraphList> {
    match self {
      Self::AppBskyGraphList(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_graph_listblock(&self) -> Option<&AppBskyGraphListblock> {
    match self {
      Self::AppBskyGraphListblock(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_graph_listitem(&self) -> Option<&AppBskyGraphListitem> {
    match self {
      Self::AppBskyGraphListitem(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_graph_starterpack(&self) -> Option<&AppBskyGraphStarterpack> {
    match self {
      Self::AppBskyGraphStarterpack(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_labeler_service(&self) -> Option<&AppBskyLabelerService> {
    match self {
      Self::AppBskyLabelerService(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_chat_bsky_actor_declaration(&self) -> Option<&ChatBskyActorDeclaration> {
    match self {
      Self::ChatBskyActorDeclaration(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_other(&self) -> Option<&serde_json::Value> {
    match self {
      Self::Other(v) => Some(v),
      _ => None,
    }
  }
}

pub enum TextFacet {
  Mention(String),
  Link(String),
  Tag(String),
}

impl TextFacet {
  pub fn mention(did: &str) -> Self {
    Self::Mention(did.to_string())
  }

  pub fn link(url: &str) -> Self {
    Self::Link(url.to_string())
  }

  pub fn tag(tag: &str) -> Self {
    Self::Tag(tag.to_string())
  }

  pub fn to_atproto(&self) -> AppBskyRichtextFacetFeaturesUnion {
    match self {
      Self::Mention(v) => AppBskyRichtextFacetFeaturesUnion::AppBskyRichtextFacetMention(Box::new(
        AppBskyRichtextFacetMention {
          did: v.clone(),
          extra: std::collections::HashMap::new(),
        },
      )),
      Self::Link(v) => AppBskyRichtextFacetFeaturesUnion::AppBskyRichtextFacetLink(Box::new(
        AppBskyRichtextFacetLink {
          uri: v.clone(),
          extra: std::collections::HashMap::new(),
        },
      )),
      Self::Tag(v) => AppBskyRichtextFacetFeaturesUnion::AppBskyRichtextFacetTag(Box::new(
        AppBskyRichtextFacetTag {
          tag: v.clone(),
          extra: std::collections::HashMap::new(),
        },
      )),
    }
  }
}

pub struct TextDecoration {
  pub text: String,
  pub facet: TextFacet,
}

impl TextDecoration {
  pub fn new_mention(text: &str, did: &str) -> Self {
    Self {
      text: text.to_string(),
      facet: TextFacet::mention(did),
    }
  }

  pub fn new_link(text: &str, url: &str) -> Self {
    Self {
      text: text.to_string(),
      facet: TextFacet::link(url),
    }
  }

  pub fn new_tag(text: &str, tag: &str) -> Self {
    Self {
      text: text.to_string(),
      facet: TextFacet::tag(tag),
    }
  }

  pub fn to_atproto(&self, description: &str) -> Vec<AppBskyRichtextFacet> {
    let mut ret = Vec::new();
    let mut index = 0;
    while index < description.len() {
      match self.text.get(index..).and_then(|t| description.find(t)) {
        Some(start) => {
          let end = start + self.text.len();
          ret.push(AppBskyRichtextFacet {
            index: AppBskyRichtextFacetByteSlice {
              byte_start: start as i64,
              byte_end: end as i64,
              extra: std::collections::HashMap::new(),
            },
            features: vec![self.facet.to_atproto()],
            extra: std::collections::HashMap::new(),
          });
          index = end;
        }
        None => break,
      }
    }
    ret
  }
}

impl ComAtprotoSyncSubscribeReposCommit {
  pub fn to_aturi(&self) -> Option<String> {
    self
      .ops
      .first()
      .map(|op| format!("at://{}/{}", self.repo, op.path))
  }
}

pub fn get_did_from_request_header(headers: &axum::http::HeaderMap) -> Result<String> {
  let authorization = headers
    .get("Authorization")
    .ok_or_else(|| Error::Other(String::from("no authorization header")))?;
  let authorization = authorization
    .to_str()
    .map_err(|e| Error::Other(e.to_string()))?;
  let body = authorization
    .split(".")
    .nth(1)
    .ok_or_else(|| Error::Other(String::from("invalid jwt format")))?;
  let data = base64::Engine::decode(&base64::prelude::BASE64_STANDARD, &body)
    .map_err(|e| Error::Other(e.to_string()))?;
  let data = String::from_utf8(data).map_err(|e| Error::Other(e.to_string()))?;
  let object =
    serde_json::from_str::<serde_json::Value>(&data).map_err(|e| Error::Other(e.to_string()))?;
  let iss = object
    .get("iss")
    .and_then(|iss| iss.as_str())
    .ok_or_else(|| Error::Other(String::from("no iss entry")))?;
  Ok(iss.to_string())
}
