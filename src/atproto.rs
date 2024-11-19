//! auto generated from https://github.com/bluesky-social/atproto/tree/main/lexicons

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  /// http request error
  Reqwest(reqwest::Error),
  /// websocket error
  WebSocket(reqwest_websocket::Error),
  /// json parse error
  Parse((serde_json::Error, String)),
  /// I/O error
  Io(std::io::Error),
  /// block decode error
  CarDecode(String),
  /// block encode error
  CarEncode(String),
  /// rate limit
  Rate((i64, i64, i64, String)),
  /// other error
  Other(String),
}

impl From<reqwest::Error> for Error {
  fn from(value: reqwest::Error) -> Self {
    Self::Reqwest(value)
  }
}

impl From<reqwest_websocket::Error> for Error {
  fn from(value: reqwest_websocket::Error) -> Self {
    Self::WebSocket(value)
  }
}

impl From<std::io::Error> for Error {
  fn from(value: std::io::Error) -> Self {
    Self::Io(value)
  }
}

impl<T: std::fmt::Debug> From<ciborium::de::Error<T>> for Error {
  fn from(value: ciborium::de::Error<T>) -> Self {
    Self::CarDecode(format!("{:?}", value))
  }
}

impl<T: std::fmt::Debug> From<ciborium::ser::Error<T>> for Error {
  fn from(value: ciborium::ser::Error<T>) -> Self {
    Self::CarEncode(format!("{:?}", value))
  }
}

impl From<(serde_json::Error, String)> for Error {
  fn from(value: (serde_json::Error, String)) -> Self {
    Self::Parse(value)
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tid(pub String);

impl Tid {
  pub fn encoding() -> Result<data_encoding::Encoding> {
    let mut spec = data_encoding::Specification::new();
    spec.symbols.push_str("234567abcdefghijklmnopqrstuvwxyz");
    spec.bit_order = data_encoding::BitOrder::MostSignificantFirst;
    spec.check_trailing_bits = true;
    spec.padding = None;
    Ok(spec.encoding().map_err(|e| Error::Other(format!("{e}")))?)
  }
}

impl From<String> for Tid {
  fn from(value: String) -> Self {
    Self(value)
  }
}

impl From<&str> for Tid {
  fn from(value: &str) -> Self {
    Self(value.to_string())
  }
}

impl Tid {
  pub fn new(datetime: chrono::DateTime<chrono::Utc>, clock_id: u16) -> Self {
    let ts = ((datetime.timestamp_micros() as i128) << 73) & 0x7ffffffffffffe000000000000000000;
    let id = ((clock_id as i128) << 63) & 0x00000000000001ff8000000000000000;
    let data = ts | id;
    Self(
      Tid::encoding()
        .unwrap()
        .encode(&data.to_be_bytes())
        .chars()
        .take(13)
        .collect(),
    )
  }

  pub fn get_datetime(&self) -> Result<chrono::DateTime<chrono::Utc>> {
    let bytes = Tid::encoding()?
      .decode(format!("{}2222222222222", &self.0).as_bytes())
      .map_err(|e| Error::Other(format!("{e}")))?;
    let mut array = [0u8; 16];
    array.copy_from_slice(&bytes);
    let ts = (i128::from_be_bytes(array) & 0x7ffffffffffffe000000000000000000) >> 73;
    Ok(
      chrono::TimeZone::timestamp_micros(&chrono::Utc, ts as i64)
        .earliest()
        .ok_or_else(|| Error::Other(format!("cannot convert to datetime {}", &self.0)))?,
    )
  }

  pub fn get_clock_id(&self) -> Result<u16> {
    let bytes = Tid::encoding()?
      .decode(format!("{}2222222222222", self.0).as_bytes())
      .map_err(|e| Error::Other(format!("{e}")))?;
    let mut array = [0u8; 16];
    array.copy_from_slice(&bytes);
    let ts = (i128::from_be_bytes(array) & 0x70000000000001ff8000000000000000) >> 63;
    Ok(ts as u16)
  }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Link {
  #[serde(rename = "$link")]
  pub link: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Blob {
  #[serde(rename = "$type")]
  pub type_: String,
  #[serde(rename = "ref")]
  pub ref_: Option<Link>,
  pub mime_type: String,
  pub size: Option<i64>,
  pub cid: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsProfileViewBasic {
  /// [format: did]
  pub did: String,
  /// [format: handle]
  pub handle: String,
  /// [max_graphemes: 64] [max_length: 640]
  pub display_name: Option<String>,
  /// [format: uri]
  pub avatar: Option<String>,
  pub associated: Option<AppBskyActorDefsProfileAssociated>,
  pub viewer: Option<AppBskyActorDefsViewerState>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  /// [format: datetime]
  pub created_at: Option<chrono::DateTime<chrono::Utc>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsProfileView {
  /// [format: did]
  pub did: String,
  /// [format: handle]
  pub handle: String,
  /// [max_graphemes: 64] [max_length: 640]
  pub display_name: Option<String>,
  /// [max_graphemes: 256] [max_length: 2560]
  pub description: Option<String>,
  /// [format: uri]
  pub avatar: Option<String>,
  pub associated: Option<AppBskyActorDefsProfileAssociated>,
  /// [format: datetime]
  pub indexed_at: Option<chrono::DateTime<chrono::Utc>>,
  /// [format: datetime]
  pub created_at: Option<chrono::DateTime<chrono::Utc>>,
  pub viewer: Option<AppBskyActorDefsViewerState>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsProfileViewDetailed {
  /// [format: did]
  pub did: String,
  /// [format: handle]
  pub handle: String,
  /// [max_graphemes: 64] [max_length: 640]
  pub display_name: Option<String>,
  /// [max_graphemes: 256] [max_length: 2560]
  pub description: Option<String>,
  /// [format: uri]
  pub avatar: Option<String>,
  /// [format: uri]
  pub banner: Option<String>,
  pub followers_count: Option<i64>,
  pub follows_count: Option<i64>,
  pub posts_count: Option<i64>,
  pub associated: Option<AppBskyActorDefsProfileAssociated>,
  pub joined_via_starter_pack: Option<AppBskyGraphDefsStarterPackViewBasic>,
  /// [format: datetime]
  pub indexed_at: Option<chrono::DateTime<chrono::Utc>>,
  /// [format: datetime]
  pub created_at: Option<chrono::DateTime<chrono::Utc>>,
  pub viewer: Option<AppBskyActorDefsViewerState>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  pub pinned_post: Option<ComAtprotoRepoStrongRef>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsProfileAssociated {
  pub lists: Option<i64>,
  pub feedgens: Option<i64>,
  pub starter_packs: Option<i64>,
  pub labeler: Option<bool>,
  pub chat: Option<AppBskyActorDefsProfileAssociatedChat>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsProfileAssociatedChat {
  /// [known_values: ["all", "none", "following"]]
  pub allow_incoming: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Metadata about the requesting account's relationship with the subject account. Only has meaningful content for authed requests.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsViewerState {
  pub muted: Option<bool>,
  pub muted_by_list: Option<AppBskyGraphDefsListViewBasic>,
  pub blocked_by: Option<bool>,
  /// [format: at-uri]
  pub blocking: Option<String>,
  pub blocking_by_list: Option<AppBskyGraphDefsListViewBasic>,
  /// [format: at-uri]
  pub following: Option<String>,
  /// [format: at-uri]
  pub followed_by: Option<String>,
  pub known_followers: Option<AppBskyActorDefsKnownFollowers>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// The subject's followers whom you also follow
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsKnownFollowers {
  pub count: i64,
  /// [min_length: 0] [max_length: 5]
  pub followers: Vec<AppBskyActorDefsProfileViewBasic>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyActorDefsPreferencesUnion {
  #[serde(rename = "app.bsky.actor.defs#adultContentPref")]
  AppBskyActorDefsAdultContentPref(Box<AppBskyActorDefsAdultContentPref>),
  #[serde(rename = "app.bsky.actor.defs#contentLabelPref")]
  AppBskyActorDefsContentLabelPref(Box<AppBskyActorDefsContentLabelPref>),
  #[serde(rename = "app.bsky.actor.defs#savedFeedsPref")]
  AppBskyActorDefsSavedFeedsPref(Box<AppBskyActorDefsSavedFeedsPref>),
  #[serde(rename = "app.bsky.actor.defs#savedFeedsPrefV2")]
  AppBskyActorDefsSavedFeedsPrefV2(Box<AppBskyActorDefsSavedFeedsPrefV2>),
  #[serde(rename = "app.bsky.actor.defs#personalDetailsPref")]
  AppBskyActorDefsPersonalDetailsPref(Box<AppBskyActorDefsPersonalDetailsPref>),
  #[serde(rename = "app.bsky.actor.defs#feedViewPref")]
  AppBskyActorDefsFeedViewPref(Box<AppBskyActorDefsFeedViewPref>),
  #[serde(rename = "app.bsky.actor.defs#threadViewPref")]
  AppBskyActorDefsThreadViewPref(Box<AppBskyActorDefsThreadViewPref>),
  #[serde(rename = "app.bsky.actor.defs#interestsPref")]
  AppBskyActorDefsInterestsPref(Box<AppBskyActorDefsInterestsPref>),
  #[serde(rename = "app.bsky.actor.defs#mutedWordsPref")]
  AppBskyActorDefsMutedWordsPref(Box<AppBskyActorDefsMutedWordsPref>),
  #[serde(rename = "app.bsky.actor.defs#hiddenPostsPref")]
  AppBskyActorDefsHiddenPostsPref(Box<AppBskyActorDefsHiddenPostsPref>),
  #[serde(rename = "app.bsky.actor.defs#bskyAppStatePref")]
  AppBskyActorDefsBskyAppStatePref(Box<AppBskyActorDefsBskyAppStatePref>),
  #[serde(rename = "app.bsky.actor.defs#labelersPref")]
  AppBskyActorDefsLabelersPref(Box<AppBskyActorDefsLabelersPref>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppBskyActorDefsPreferences(pub Vec<AppBskyActorDefsPreferencesUnion>);

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsAdultContentPref {
  /// [default: false]
  pub enabled: bool,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsContentLabelPref {
  /// [format: did] Which labeler does this preference apply to? If undefined, applies globally.
  pub labeler_did: Option<String>,
  pub label: String,
  /// [known_values: ["ignore", "show", "warn", "hide"]]
  pub visibility: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsSavedFeed {
  pub id: String,
  /// [known_values: ["feed", "list", "timeline"]]
  pub type_: String,
  pub value: String,
  pub pinned: bool,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsSavedFeedsPrefV2 {
  pub items: Vec<AppBskyActorDefsSavedFeed>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsSavedFeedsPref {
  pub pinned: Vec<String>,
  pub saved: Vec<String>,
  pub timeline_index: Option<i64>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsPersonalDetailsPref {
  /// [format: datetime] The birth date of account owner.
  pub birth_date: Option<chrono::DateTime<chrono::Utc>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsFeedViewPref {
  /// The URI of the feed, or an identifier which describes the feed.
  pub feed: String,
  /// Hide replies in the feed.
  pub hide_replies: Option<bool>,
  /// [default: true] Hide replies in the feed if they are not by followed users.
  pub hide_replies_by_unfollowed: Option<bool>,
  /// Hide replies in the feed if they do not have this number of likes.
  pub hide_replies_by_like_count: Option<i64>,
  /// Hide reposts in the feed.
  pub hide_reposts: Option<bool>,
  /// Hide quote posts in the feed.
  pub hide_quote_posts: Option<bool>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsThreadViewPref {
  /// [known_values: ["oldest", "newest", "most-likes", "random"]] Sorting mode for threads.
  pub sort: Option<String>,
  /// Show followed users at the top of all replies.
  pub prioritize_followed_users: Option<bool>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsInterestsPref {
  /// [max_length: 100] A list of tags which describe the account owner's interests gathered during onboarding.
  pub tags: Vec<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppBskyActorDefsMutedWordTarget(pub String);

/// A word that the account owner has muted.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsMutedWord {
  pub id: Option<String>,
  /// [max_graphemes: 1000] [max_length: 10000] The muted word itself.
  pub value: String,
  /// The intended targets of the muted word.
  pub targets: Vec<AppBskyActorDefsMutedWordTarget>,
  /// [known_values: ["all", "exclude-following"]] [default: all] Groups of users to apply the muted word to. If undefined, applies to all users.
  pub actor_target: Option<String>,
  /// [format: datetime] The date and time at which the muted word will expire and no longer be applied.
  pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsMutedWordsPref {
  /// A list of words the account owner has muted.
  pub items: Vec<AppBskyActorDefsMutedWord>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsHiddenPostsPref {
  /// A list of URIs of posts the account owner has hidden.
  pub items: Vec<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsLabelersPref {
  pub labelers: Vec<AppBskyActorDefsLabelerPrefItem>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsLabelerPrefItem {
  /// [format: did]
  pub did: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// A grab bag of state that's specific to the bsky.app program. Third-party apps shouldn't use this.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsBskyAppStatePref {
  pub active_progress_guide: Option<AppBskyActorDefsBskyAppProgressGuide>,
  /// [max_length: 1000] An array of tokens which identify nudges (modals, popups, tours, highlight dots) that should be shown to the user.
  pub queued_nudges: Option<Vec<String>>,
  /// [max_length: 100] Storage for NUXs the user has encountered.
  pub nuxs: Option<Vec<AppBskyActorDefsNux>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// If set, an active progress guide. Once completed, can be set to undefined. Should have unspecced fields tracking progress.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsBskyAppProgressGuide {
  /// [max_length: 100]
  pub guide: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// A new user experiences (NUX) storage object
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsNux {
  /// [max_length: 100]
  pub id: String,
  /// [default: false]
  pub completed: bool,
  /// [max_graphemes: 300] [max_length: 3000] Arbitrary data for the NUX. The structure is defined by the NUX itself. Limited to 300 characters.
  pub data: Option<String>,
  /// [format: datetime] The date and time at which the NUX will expire and should be considered completed.
  pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorGetPreferencesOutput {
  pub preferences: AppBskyActorDefsPreferences,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorGetProfilesOutput {
  pub profiles: Vec<AppBskyActorDefsProfileViewDetailed>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorGetSuggestionsOutput {
  pub cursor: Option<String>,
  pub actors: Vec<AppBskyActorDefsProfileView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Self-label values, specific to the Bluesky application, on the overall account.

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyActorProfileLabelsUnion {
  #[serde(rename = "com.atproto.label.defs#selfLabels")]
  ComAtprotoLabelDefsSelfLabels(Box<ComAtprotoLabelDefsSelfLabels>),
}

/// A declaration of a Bluesky account profile.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorProfile {
  /// [max_graphemes: 64] [max_length: 640],
  pub display_name: Option<String>,
  /// [max_graphemes: 256] [max_length: 2560] Free-form profile description text.,
  pub description: Option<String>,
  /// [accept: ["image/png", "image/jpeg"]] [max_size: 1000000] Small image to be displayed next to posts from account. AKA, 'profile picture',
  pub avatar: Option<Blob>,
  /// [accept: ["image/png", "image/jpeg"]] [max_size: 1000000] Larger horizontal image to display behind profile view.,
  pub banner: Option<Blob>,
  /// Self-label values, specific to the Bluesky application, on the overall account.,
  pub labels: Option<AppBskyActorProfileLabelsUnion>,
  pub joined_via_starter_pack: Option<ComAtprotoRepoStrongRef>,
  pub pinned_post: Option<ComAtprotoRepoStrongRef>,
  /// [format: datetime],
  pub created_at: Option<chrono::DateTime<chrono::Utc>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorPutPreferencesInput {
  pub preferences: AppBskyActorDefsPreferences,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorSearchActorsOutput {
  pub cursor: Option<String>,
  pub actors: Vec<AppBskyActorDefsProfileView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorSearchActorsTypeaheadOutput {
  pub actors: Vec<AppBskyActorDefsProfileViewBasic>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// width:height represents an aspect ratio. It may be approximate, and may not correspond to absolute dimensions in any given unit.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedDefsAspectRatio {
  /// [minimum: 1]
  pub width: i64,
  /// [minimum: 1]
  pub height: i64,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// A representation of some externally linked content (eg, a URL and 'card'), embedded in a Bluesky record (eg, a post).
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedExternal {
  pub external: AppBskyEmbedExternalExternal,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedExternalExternal {
  /// [format: uri]
  pub uri: String,
  pub title: String,
  pub description: String,
  /// [accept: ["image/*"]] [max_size: 1000000]
  pub thumb: Option<Blob>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedExternalView {
  pub external: AppBskyEmbedExternalViewExternal,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedExternalViewExternal {
  /// [format: uri]
  pub uri: String,
  pub title: String,
  pub description: String,
  /// [format: uri]
  pub thumb: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedImages {
  /// [max_length: 4]
  pub images: Vec<AppBskyEmbedImagesImage>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedImagesImage {
  /// [accept: ["image/*"]] [max_size: 1000000]
  pub image: Blob,
  /// Alt text description of the image, for accessibility.
  pub alt: String,
  pub aspect_ratio: Option<AppBskyEmbedDefsAspectRatio>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedImagesView {
  /// [max_length: 4]
  pub images: Vec<AppBskyEmbedImagesViewImage>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedImagesViewImage {
  /// [format: uri] Fully-qualified URL where a thumbnail of the image can be fetched. For example, CDN location provided by the App View.
  pub thumb: String,
  /// [format: uri] Fully-qualified URL where a large version of the image can be fetched. May or may not be the exact original blob. For example, CDN location provided by the App View.
  pub fullsize: String,
  /// Alt text description of the image, for accessibility.
  pub alt: String,
  pub aspect_ratio: Option<AppBskyEmbedDefsAspectRatio>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedRecord {
  pub record: ComAtprotoRepoStrongRef,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyEmbedRecordViewRecordUnion {
  #[serde(rename = "app.bsky.embed.record#viewRecord")]
  AppBskyEmbedRecordViewRecord(Box<AppBskyEmbedRecordViewRecord>),
  #[serde(rename = "app.bsky.embed.record#viewNotFound")]
  AppBskyEmbedRecordViewNotFound(Box<AppBskyEmbedRecordViewNotFound>),
  #[serde(rename = "app.bsky.embed.record#viewBlocked")]
  AppBskyEmbedRecordViewBlocked(Box<AppBskyEmbedRecordViewBlocked>),
  #[serde(rename = "app.bsky.embed.record#viewDetached")]
  AppBskyEmbedRecordViewDetached(Box<AppBskyEmbedRecordViewDetached>),
  #[serde(rename = "app.bsky.feed.defs#generatorView")]
  AppBskyFeedDefsGeneratorView(Box<AppBskyFeedDefsGeneratorView>),
  #[serde(rename = "app.bsky.graph.defs#listView")]
  AppBskyGraphDefsListView(Box<AppBskyGraphDefsListView>),
  #[serde(rename = "app.bsky.labeler.defs#labelerView")]
  AppBskyLabelerDefsLabelerView(Box<AppBskyLabelerDefsLabelerView>),
  #[serde(rename = "app.bsky.graph.defs#starterPackViewBasic")]
  AppBskyGraphDefsStarterPackViewBasic(Box<AppBskyGraphDefsStarterPackViewBasic>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedRecordView {
  pub record: AppBskyEmbedRecordViewRecordUnion,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyEmbedRecordViewRecordEmbedsUnion {
  #[serde(rename = "app.bsky.embed.images#view")]
  AppBskyEmbedImagesView(Box<AppBskyEmbedImagesView>),
  #[serde(rename = "app.bsky.embed.video#view")]
  AppBskyEmbedVideoView(Box<AppBskyEmbedVideoView>),
  #[serde(rename = "app.bsky.embed.external#view")]
  AppBskyEmbedExternalView(Box<AppBskyEmbedExternalView>),
  #[serde(rename = "app.bsky.embed.record#view")]
  AppBskyEmbedRecordView(Box<AppBskyEmbedRecordView>),
  #[serde(rename = "app.bsky.embed.recordWithMedia#view")]
  AppBskyEmbedRecordWithMediaView(Box<AppBskyEmbedRecordWithMediaView>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedRecordViewRecord {
  /// [format: at-uri]
  pub uri: String,
  /// [format: cid]
  pub cid: String,
  pub author: AppBskyActorDefsProfileViewBasic,
  /// The record data itself.
  pub value: serde_json::Value,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  pub reply_count: Option<i64>,
  pub repost_count: Option<i64>,
  pub like_count: Option<i64>,
  pub quote_count: Option<i64>,
  pub embeds: Option<Vec<AppBskyEmbedRecordViewRecordEmbedsUnion>>,
  /// [format: datetime]
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedRecordViewNotFound {
  /// [format: at-uri]
  pub uri: String,
  /// [const: true]
  pub not_found: bool,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedRecordViewBlocked {
  /// [format: at-uri]
  pub uri: String,
  /// [const: true]
  pub blocked: bool,
  pub author: AppBskyFeedDefsBlockedAuthor,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedRecordViewDetached {
  /// [format: at-uri]
  pub uri: String,
  /// [const: true]
  pub detached: bool,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyEmbedRecordWithMediaMediaUnion {
  #[serde(rename = "app.bsky.embed.images")]
  AppBskyEmbedImages(Box<AppBskyEmbedImages>),
  #[serde(rename = "app.bsky.embed.video")]
  AppBskyEmbedVideo(Box<AppBskyEmbedVideo>),
  #[serde(rename = "app.bsky.embed.external")]
  AppBskyEmbedExternal(Box<AppBskyEmbedExternal>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedRecordWithMedia {
  pub record: AppBskyEmbedRecord,
  pub media: AppBskyEmbedRecordWithMediaMediaUnion,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyEmbedRecordWithMediaViewMediaUnion {
  #[serde(rename = "app.bsky.embed.images#view")]
  AppBskyEmbedImagesView(Box<AppBskyEmbedImagesView>),
  #[serde(rename = "app.bsky.embed.video#view")]
  AppBskyEmbedVideoView(Box<AppBskyEmbedVideoView>),
  #[serde(rename = "app.bsky.embed.external#view")]
  AppBskyEmbedExternalView(Box<AppBskyEmbedExternalView>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedRecordWithMediaView {
  pub record: AppBskyEmbedRecordView,
  pub media: AppBskyEmbedRecordWithMediaViewMediaUnion,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedVideo {
  /// [accept: ["video/mp4"]] [max_size: 50000000]
  pub video: Blob,
  /// [max_length: 20]
  pub captions: Option<Vec<AppBskyEmbedVideoCaption>>,
  /// [max_graphemes: 1000] [max_length: 10000] Alt text description of the video, for accessibility.
  pub alt: Option<String>,
  pub aspect_ratio: Option<AppBskyEmbedDefsAspectRatio>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedVideoCaption {
  /// [format: language]
  pub lang: String,
  /// [accept: ["text/vtt"]] [max_size: 20000]
  pub file: Blob,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedVideoView {
  /// [format: cid]
  pub cid: String,
  /// [format: uri]
  pub playlist: String,
  /// [format: uri]
  pub thumbnail: Option<String>,
  /// [max_graphemes: 1000] [max_length: 10000]
  pub alt: Option<String>,
  pub aspect_ratio: Option<AppBskyEmbedDefsAspectRatio>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedDefsPostViewEmbedUnion {
  #[serde(rename = "app.bsky.embed.images#view")]
  AppBskyEmbedImagesView(Box<AppBskyEmbedImagesView>),
  #[serde(rename = "app.bsky.embed.video#view")]
  AppBskyEmbedVideoView(Box<AppBskyEmbedVideoView>),
  #[serde(rename = "app.bsky.embed.external#view")]
  AppBskyEmbedExternalView(Box<AppBskyEmbedExternalView>),
  #[serde(rename = "app.bsky.embed.record#view")]
  AppBskyEmbedRecordView(Box<AppBskyEmbedRecordView>),
  #[serde(rename = "app.bsky.embed.recordWithMedia#view")]
  AppBskyEmbedRecordWithMediaView(Box<AppBskyEmbedRecordWithMediaView>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsPostView {
  /// [format: at-uri]
  pub uri: String,
  /// [format: cid]
  pub cid: String,
  pub author: AppBskyActorDefsProfileViewBasic,
  pub record: serde_json::Value,
  pub embed: Option<AppBskyFeedDefsPostViewEmbedUnion>,
  pub reply_count: Option<i64>,
  pub repost_count: Option<i64>,
  pub like_count: Option<i64>,
  pub quote_count: Option<i64>,
  /// [format: datetime]
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  pub viewer: Option<AppBskyFeedDefsViewerState>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  pub threadgate: Option<AppBskyFeedDefsThreadgateView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Metadata about the requesting account's relationship with the subject content. Only has meaningful content for authed requests.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsViewerState {
  /// [format: at-uri]
  pub repost: Option<String>,
  /// [format: at-uri]
  pub like: Option<String>,
  pub thread_muted: Option<bool>,
  pub reply_disabled: Option<bool>,
  pub embedding_disabled: Option<bool>,
  pub pinned: Option<bool>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedDefsFeedViewPostReasonUnion {
  #[serde(rename = "app.bsky.feed.defs#reasonRepost")]
  AppBskyFeedDefsReasonRepost(Box<AppBskyFeedDefsReasonRepost>),
  #[serde(rename = "app.bsky.feed.defs#reasonPin")]
  AppBskyFeedDefsReasonPin(Box<AppBskyFeedDefsReasonPin>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsFeedViewPost {
  pub post: AppBskyFeedDefsPostView,
  pub reply: Option<AppBskyFeedDefsReplyRef>,
  pub reason: Option<AppBskyFeedDefsFeedViewPostReasonUnion>,
  /// [max_length: 2000] Context provided by feed generator that may be passed back alongside interactions.
  pub feed_context: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedDefsReplyRefRootUnion {
  #[serde(rename = "app.bsky.feed.defs#postView")]
  AppBskyFeedDefsPostView(Box<AppBskyFeedDefsPostView>),
  #[serde(rename = "app.bsky.feed.defs#notFoundPost")]
  AppBskyFeedDefsNotFoundPost(Box<AppBskyFeedDefsNotFoundPost>),
  #[serde(rename = "app.bsky.feed.defs#blockedPost")]
  AppBskyFeedDefsBlockedPost(Box<AppBskyFeedDefsBlockedPost>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedDefsReplyRefParentUnion {
  #[serde(rename = "app.bsky.feed.defs#postView")]
  AppBskyFeedDefsPostView(Box<AppBskyFeedDefsPostView>),
  #[serde(rename = "app.bsky.feed.defs#notFoundPost")]
  AppBskyFeedDefsNotFoundPost(Box<AppBskyFeedDefsNotFoundPost>),
  #[serde(rename = "app.bsky.feed.defs#blockedPost")]
  AppBskyFeedDefsBlockedPost(Box<AppBskyFeedDefsBlockedPost>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsReplyRef {
  pub root: AppBskyFeedDefsReplyRefRootUnion,
  pub parent: AppBskyFeedDefsReplyRefParentUnion,
  /// When parent is a reply to another post, this is the author of that post.
  pub grandparent_author: Option<AppBskyActorDefsProfileViewBasic>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsReasonRepost {
  pub by: AppBskyActorDefsProfileViewBasic,
  /// [format: datetime]
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppBskyFeedDefsReasonPin(pub serde_json::Value);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedDefsThreadViewPostParentUnion {
  #[serde(rename = "app.bsky.feed.defs#threadViewPost")]
  AppBskyFeedDefsThreadViewPost(Box<AppBskyFeedDefsThreadViewPost>),
  #[serde(rename = "app.bsky.feed.defs#notFoundPost")]
  AppBskyFeedDefsNotFoundPost(Box<AppBskyFeedDefsNotFoundPost>),
  #[serde(rename = "app.bsky.feed.defs#blockedPost")]
  AppBskyFeedDefsBlockedPost(Box<AppBskyFeedDefsBlockedPost>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedDefsThreadViewPostRepliesUnion {
  #[serde(rename = "app.bsky.feed.defs#threadViewPost")]
  AppBskyFeedDefsThreadViewPost(Box<AppBskyFeedDefsThreadViewPost>),
  #[serde(rename = "app.bsky.feed.defs#notFoundPost")]
  AppBskyFeedDefsNotFoundPost(Box<AppBskyFeedDefsNotFoundPost>),
  #[serde(rename = "app.bsky.feed.defs#blockedPost")]
  AppBskyFeedDefsBlockedPost(Box<AppBskyFeedDefsBlockedPost>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsThreadViewPost {
  pub post: AppBskyFeedDefsPostView,
  pub parent: Option<AppBskyFeedDefsThreadViewPostParentUnion>,
  pub replies: Option<Vec<AppBskyFeedDefsThreadViewPostRepliesUnion>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsNotFoundPost {
  /// [format: at-uri]
  pub uri: String,
  /// [const: true]
  pub not_found: bool,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsBlockedPost {
  /// [format: at-uri]
  pub uri: String,
  /// [const: true]
  pub blocked: bool,
  pub author: AppBskyFeedDefsBlockedAuthor,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsBlockedAuthor {
  /// [format: did]
  pub did: String,
  pub viewer: Option<AppBskyActorDefsViewerState>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsGeneratorView {
  /// [format: at-uri]
  pub uri: String,
  /// [format: cid]
  pub cid: String,
  /// [format: did]
  pub did: String,
  pub creator: AppBskyActorDefsProfileView,
  pub display_name: String,
  /// [max_graphemes: 300] [max_length: 3000]
  pub description: Option<String>,
  pub description_facets: Option<Vec<AppBskyRichtextFacet>>,
  /// [format: uri]
  pub avatar: Option<String>,
  /// [minimum: 0]
  pub like_count: Option<i64>,
  pub accepts_interactions: Option<bool>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  pub viewer: Option<AppBskyFeedDefsGeneratorViewerState>,
  /// [format: datetime]
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsGeneratorViewerState {
  /// [format: at-uri]
  pub like: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedDefsSkeletonFeedPostReasonUnion {
  #[serde(rename = "app.bsky.feed.defs#skeletonReasonRepost")]
  AppBskyFeedDefsSkeletonReasonRepost(Box<AppBskyFeedDefsSkeletonReasonRepost>),
  #[serde(rename = "app.bsky.feed.defs#skeletonReasonPin")]
  AppBskyFeedDefsSkeletonReasonPin(Box<AppBskyFeedDefsSkeletonReasonPin>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsSkeletonFeedPost {
  /// [format: at-uri]
  pub post: String,
  pub reason: Option<AppBskyFeedDefsSkeletonFeedPostReasonUnion>,
  /// [max_length: 2000] Context that will be passed through to client and may be passed to feed generator back alongside interactions.
  pub feed_context: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsSkeletonReasonRepost {
  /// [format: at-uri]
  pub repost: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppBskyFeedDefsSkeletonReasonPin(pub serde_json::Value);

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsThreadgateView {
  /// [format: at-uri]
  pub uri: Option<String>,
  /// [format: cid]
  pub cid: Option<String>,
  pub record: Option<serde_json::Value>,
  pub lists: Option<Vec<AppBskyGraphDefsListViewBasic>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsInteraction {
  /// [format: at-uri]
  pub item: Option<String>,
  /// [known_values: ["app.bsky.feed.defs#requestLess", "app.bsky.feed.defs#requestMore", "app.bsky.feed.defs#clickthroughItem", "app.bsky.feed.defs#clickthroughAuthor", "app.bsky.feed.defs#clickthroughReposter", "app.bsky.feed.defs#clickthroughEmbed", "app.bsky.feed.defs#interactionSeen", "app.bsky.feed.defs#interactionLike", "app.bsky.feed.defs#interactionRepost", "app.bsky.feed.defs#interactionReply", "app.bsky.feed.defs#interactionQuote", "app.bsky.feed.defs#interactionShare"]]
  pub event: Option<String>,
  /// [max_length: 2000] Context on a feed item that was originally supplied by the feed generator on getFeedSkeleton.
  pub feed_context: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDescribeFeedGeneratorOutput {
  /// [format: did]
  pub did: String,
  pub feeds: Vec<AppBskyFeedDescribeFeedGeneratorFeed>,
  pub links: Option<AppBskyFeedDescribeFeedGeneratorLinks>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDescribeFeedGeneratorFeed {
  /// [format: at-uri]
  pub uri: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDescribeFeedGeneratorLinks {
  pub privacy_policy: Option<String>,
  pub terms_of_service: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Self-label values

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedGeneratorLabelsUnion {
  #[serde(rename = "com.atproto.label.defs#selfLabels")]
  ComAtprotoLabelDefsSelfLabels(Box<ComAtprotoLabelDefsSelfLabels>),
}

/// Record declaring of the existence of a feed generator, and containing metadata about it. The record can exist in any repository.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGenerator {
  /// [format: did],
  pub did: String,
  /// [max_graphemes: 24] [max_length: 240],
  pub display_name: String,
  /// [max_graphemes: 300] [max_length: 3000],
  pub description: Option<String>,
  pub description_facets: Option<Vec<AppBskyRichtextFacet>>,
  /// [accept: ["image/png", "image/jpeg"]] [max_size: 1000000],
  pub avatar: Option<Blob>,
  /// Declaration that a feed accepts feedback interactions from a client through app.bsky.feed.sendInteractions,
  pub accepts_interactions: Option<bool>,
  /// Self-label values,
  pub labels: Option<AppBskyFeedGeneratorLabelsUnion>,
  /// [format: datetime],
  pub created_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetActorFeedsOutput {
  pub cursor: Option<String>,
  pub feeds: Vec<AppBskyFeedDefsGeneratorView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetActorLikesOutput {
  pub cursor: Option<String>,
  pub feed: Vec<AppBskyFeedDefsFeedViewPost>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetAuthorFeedOutput {
  pub cursor: Option<String>,
  pub feed: Vec<AppBskyFeedDefsFeedViewPost>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetFeedOutput {
  pub cursor: Option<String>,
  pub feed: Vec<AppBskyFeedDefsFeedViewPost>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetFeedGeneratorOutput {
  pub view: AppBskyFeedDefsGeneratorView,
  /// Indicates whether the feed generator service has been online recently, or else seems to be inactive.
  pub is_online: bool,
  /// Indicates whether the feed generator service is compatible with the record declaration.
  pub is_valid: bool,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetFeedGeneratorsOutput {
  pub feeds: Vec<AppBskyFeedDefsGeneratorView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetFeedSkeletonOutput {
  pub cursor: Option<String>,
  pub feed: Vec<AppBskyFeedDefsSkeletonFeedPost>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetLikesOutput {
  /// [format: at-uri]
  pub uri: String,
  /// [format: cid]
  pub cid: Option<String>,
  pub cursor: Option<String>,
  pub likes: Vec<AppBskyFeedGetLikesLike>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetLikesLike {
  /// [format: datetime]
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  /// [format: datetime]
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub actor: AppBskyActorDefsProfileView,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetListFeedOutput {
  pub cursor: Option<String>,
  pub feed: Vec<AppBskyFeedDefsFeedViewPost>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedGetPostThreadOutputThreadUnion {
  #[serde(rename = "app.bsky.feed.defs#threadViewPost")]
  AppBskyFeedDefsThreadViewPost(Box<AppBskyFeedDefsThreadViewPost>),
  #[serde(rename = "app.bsky.feed.defs#notFoundPost")]
  AppBskyFeedDefsNotFoundPost(Box<AppBskyFeedDefsNotFoundPost>),
  #[serde(rename = "app.bsky.feed.defs#blockedPost")]
  AppBskyFeedDefsBlockedPost(Box<AppBskyFeedDefsBlockedPost>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetPostThreadOutput {
  pub thread: AppBskyFeedGetPostThreadOutputThreadUnion,
  pub threadgate: Option<AppBskyFeedDefsThreadgateView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetPostsOutput {
  pub posts: Vec<AppBskyFeedDefsPostView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetQuotesOutput {
  /// [format: at-uri]
  pub uri: String,
  /// [format: cid]
  pub cid: Option<String>,
  pub cursor: Option<String>,
  pub posts: Vec<AppBskyFeedDefsPostView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetRepostedByOutput {
  /// [format: at-uri]
  pub uri: String,
  /// [format: cid]
  pub cid: Option<String>,
  pub cursor: Option<String>,
  pub reposted_by: Vec<AppBskyActorDefsProfileView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetSuggestedFeedsOutput {
  pub cursor: Option<String>,
  pub feeds: Vec<AppBskyFeedDefsGeneratorView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetTimelineOutput {
  pub cursor: Option<String>,
  pub feed: Vec<AppBskyFeedDefsFeedViewPost>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Record declaring a 'like' of a piece of subject content.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedLike {
  pub subject: ComAtprotoRepoStrongRef,
  /// [format: datetime],
  pub created_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedPostEmbedUnion {
  #[serde(rename = "app.bsky.embed.images")]
  AppBskyEmbedImages(Box<AppBskyEmbedImages>),
  #[serde(rename = "app.bsky.embed.video")]
  AppBskyEmbedVideo(Box<AppBskyEmbedVideo>),
  #[serde(rename = "app.bsky.embed.external")]
  AppBskyEmbedExternal(Box<AppBskyEmbedExternal>),
  #[serde(rename = "app.bsky.embed.record")]
  AppBskyEmbedRecord(Box<AppBskyEmbedRecord>),
  #[serde(rename = "app.bsky.embed.recordWithMedia")]
  AppBskyEmbedRecordWithMedia(Box<AppBskyEmbedRecordWithMedia>),
}

/// Self-label values for this post. Effectively content warnings.

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedPostLabelsUnion {
  #[serde(rename = "com.atproto.label.defs#selfLabels")]
  ComAtprotoLabelDefsSelfLabels(Box<ComAtprotoLabelDefsSelfLabels>),
}

/// Record containing a Bluesky post.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedPost {
  /// [max_graphemes: 300] [max_length: 3000] The primary post content. May be an empty string, if there are embeds.,
  pub text: String,
  /// DEPRECATED: replaced by app.bsky.richtext.facet.,
  pub entities: Option<Vec<AppBskyFeedPostEntity>>,
  /// Annotations of text (mentions, URLs, hashtags, etc),
  pub facets: Option<Vec<AppBskyRichtextFacet>>,
  pub reply: Option<AppBskyFeedPostReplyRef>,
  pub embed: Option<AppBskyFeedPostEmbedUnion>,
  /// [max_length: 3] Indicates human language of post primary text content.,
  pub langs: Option<Vec<String>>,
  /// Self-label values for this post. Effectively content warnings.,
  pub labels: Option<AppBskyFeedPostLabelsUnion>,
  /// [max_length: 8] Additional hashtags, in addition to any included in post text and facets.,
  pub tags: Option<Vec<String>>,
  /// [format: datetime] Client-declared timestamp when this post was originally created.,
  pub created_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedPostReplyRef {
  pub root: ComAtprotoRepoStrongRef,
  pub parent: ComAtprotoRepoStrongRef,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Deprecated: use facets instead.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedPostEntity {
  pub index: AppBskyFeedPostTextSlice,
  /// Expected values are 'mention' and 'link'.
  pub type_: String,
  pub value: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Deprecated. Use app.bsky.richtext instead -- A text segment. Start is inclusive, end is exclusive. Indices are for utf16-encoded strings.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedPostTextSlice {
  /// [minimum: 0]
  pub start: i64,
  /// [minimum: 0]
  pub end: i64,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedPostgateEmbeddingRulesUnion {
  #[serde(rename = "app.bsky.feed.postgate#disableRule")]
  AppBskyFeedPostgateDisableRule(Box<AppBskyFeedPostgateDisableRule>),
}

/// Record defining interaction rules for a post. The record key (rkey) of the postgate record must match the record key of the post, and that record must be in the same repository.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedPostgate {
  /// [format: datetime],
  pub created_at: chrono::DateTime<chrono::Utc>,
  /// [format: at-uri] Reference (AT-URI) to the post record.,
  pub post: String,
  /// [max_length: 50] List of AT-URIs embedding this post that the author has detached from.,
  pub detached_embedding_uris: Option<Vec<String>>,
  /// [max_length: 5],
  pub embedding_rules: Option<Vec<AppBskyFeedPostgateEmbeddingRulesUnion>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Disables embedding of this post.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppBskyFeedPostgateDisableRule(pub serde_json::Value);

/// Record representing a 'repost' of an existing Bluesky post.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedRepost {
  pub subject: ComAtprotoRepoStrongRef,
  /// [format: datetime],
  pub created_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedSearchPostsOutput {
  pub cursor: Option<String>,
  /// Count of search hits. Optional, may be rounded/truncated, and may not be possible to paginate through all hits.
  pub hits_total: Option<i64>,
  pub posts: Vec<AppBskyFeedDefsPostView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedSendInteractionsInput {
  pub interactions: Vec<AppBskyFeedDefsInteraction>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppBskyFeedSendInteractionsOutput(pub serde_json::Value);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedThreadgateAllowUnion {
  #[serde(rename = "app.bsky.feed.threadgate#mentionRule")]
  AppBskyFeedThreadgateMentionRule(Box<AppBskyFeedThreadgateMentionRule>),
  #[serde(rename = "app.bsky.feed.threadgate#followingRule")]
  AppBskyFeedThreadgateFollowingRule(Box<AppBskyFeedThreadgateFollowingRule>),
  #[serde(rename = "app.bsky.feed.threadgate#listRule")]
  AppBskyFeedThreadgateListRule(Box<AppBskyFeedThreadgateListRule>),
}

/// Record defining interaction gating rules for a thread (aka, reply controls). The record key (rkey) of the threadgate record must match the record key of the thread's root post, and that record must be in the same repository.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedThreadgate {
  /// [format: at-uri] Reference (AT-URI) to the post record.,
  pub post: String,
  /// [max_length: 5],
  pub allow: Option<Vec<AppBskyFeedThreadgateAllowUnion>>,
  /// [format: datetime],
  pub created_at: chrono::DateTime<chrono::Utc>,
  /// [max_length: 50] List of hidden reply URIs.,
  pub hidden_replies: Option<Vec<String>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Allow replies from actors mentioned in your post.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppBskyFeedThreadgateMentionRule(pub serde_json::Value);

/// Allow replies from actors you follow.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppBskyFeedThreadgateFollowingRule(pub serde_json::Value);

/// Allow replies from actors on a list.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedThreadgateListRule {
  /// [format: at-uri]
  pub list: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Record declaring a 'block' relationship against another account. NOTE: blocks are public in Bluesky; see blog posts for details.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphBlock {
  /// [format: did] DID of the account to be blocked.,
  pub subject: String,
  /// [format: datetime],
  pub created_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphDefsListViewBasic {
  /// [format: at-uri]
  pub uri: String,
  /// [format: cid]
  pub cid: String,
  /// [max_length: 64] [min_length: 1]
  pub name: String,
  pub purpose: AppBskyGraphDefsListPurpose,
  /// [format: uri]
  pub avatar: Option<String>,
  /// [minimum: 0]
  pub list_item_count: Option<i64>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  pub viewer: Option<AppBskyGraphDefsListViewerState>,
  /// [format: datetime]
  pub indexed_at: Option<chrono::DateTime<chrono::Utc>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphDefsListView {
  /// [format: at-uri]
  pub uri: String,
  /// [format: cid]
  pub cid: String,
  pub creator: AppBskyActorDefsProfileView,
  /// [max_length: 64] [min_length: 1]
  pub name: String,
  pub purpose: AppBskyGraphDefsListPurpose,
  /// [max_graphemes: 300] [max_length: 3000]
  pub description: Option<String>,
  pub description_facets: Option<Vec<AppBskyRichtextFacet>>,
  /// [format: uri]
  pub avatar: Option<String>,
  /// [minimum: 0]
  pub list_item_count: Option<i64>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  pub viewer: Option<AppBskyGraphDefsListViewerState>,
  /// [format: datetime]
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphDefsListItemView {
  /// [format: at-uri]
  pub uri: String,
  pub subject: AppBskyActorDefsProfileView,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphDefsStarterPackView {
  /// [format: at-uri]
  pub uri: String,
  /// [format: cid]
  pub cid: String,
  pub record: serde_json::Value,
  pub creator: AppBskyActorDefsProfileViewBasic,
  pub list: Option<AppBskyGraphDefsListViewBasic>,
  /// [max_length: 12]
  pub list_items_sample: Option<Vec<AppBskyGraphDefsListItemView>>,
  /// [max_length: 3]
  pub feeds: Option<Vec<AppBskyFeedDefsGeneratorView>>,
  /// [minimum: 0]
  pub joined_week_count: Option<i64>,
  /// [minimum: 0]
  pub joined_all_time_count: Option<i64>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  /// [format: datetime]
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphDefsStarterPackViewBasic {
  /// [format: at-uri]
  pub uri: String,
  /// [format: cid]
  pub cid: String,
  pub record: serde_json::Value,
  pub creator: AppBskyActorDefsProfileViewBasic,
  /// [minimum: 0]
  pub list_item_count: Option<i64>,
  /// [minimum: 0]
  pub joined_week_count: Option<i64>,
  /// [minimum: 0]
  pub joined_all_time_count: Option<i64>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  /// [format: datetime]
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppBskyGraphDefsListPurpose(pub String);

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphDefsListViewerState {
  pub muted: Option<bool>,
  /// [format: at-uri]
  pub blocked: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// indicates that a handle or DID could not be resolved
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphDefsNotFoundActor {
  /// [format: at-identifier]
  pub actor: String,
  /// [const: true]
  pub not_found: bool,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// lists the bi-directional graph relationships between one actor (not indicated in the object), and the target actors (the DID included in the object)
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphDefsRelationship {
  /// [format: did]
  pub did: String,
  /// [format: at-uri] if the actor follows this DID, this is the AT-URI of the follow record
  pub following: Option<String>,
  /// [format: at-uri] if the actor is followed by this DID, contains the AT-URI of the follow record
  pub followed_by: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Record declaring a social 'follow' relationship of another account. Duplicate follows will be ignored by the AppView.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphFollow {
  /// [format: did],
  pub subject: String,
  /// [format: datetime],
  pub created_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetActorStarterPacksOutput {
  pub cursor: Option<String>,
  pub starter_packs: Vec<AppBskyGraphDefsStarterPackViewBasic>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetBlocksOutput {
  pub cursor: Option<String>,
  pub blocks: Vec<AppBskyActorDefsProfileView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetFollowersOutput {
  pub subject: AppBskyActorDefsProfileView,
  pub cursor: Option<String>,
  pub followers: Vec<AppBskyActorDefsProfileView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetFollowsOutput {
  pub subject: AppBskyActorDefsProfileView,
  pub cursor: Option<String>,
  pub follows: Vec<AppBskyActorDefsProfileView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetKnownFollowersOutput {
  pub subject: AppBskyActorDefsProfileView,
  pub cursor: Option<String>,
  pub followers: Vec<AppBskyActorDefsProfileView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetListOutput {
  pub cursor: Option<String>,
  pub list: AppBskyGraphDefsListView,
  pub items: Vec<AppBskyGraphDefsListItemView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetListBlocksOutput {
  pub cursor: Option<String>,
  pub lists: Vec<AppBskyGraphDefsListView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetListMutesOutput {
  pub cursor: Option<String>,
  pub lists: Vec<AppBskyGraphDefsListView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetListsOutput {
  pub cursor: Option<String>,
  pub lists: Vec<AppBskyGraphDefsListView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetMutesOutput {
  pub cursor: Option<String>,
  pub mutes: Vec<AppBskyActorDefsProfileView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyGraphGetRelationshipsOutputRelationshipsUnion {
  #[serde(rename = "app.bsky.graph.defs#relationship")]
  AppBskyGraphDefsRelationship(Box<AppBskyGraphDefsRelationship>),
  #[serde(rename = "app.bsky.graph.defs#notFoundActor")]
  AppBskyGraphDefsNotFoundActor(Box<AppBskyGraphDefsNotFoundActor>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetRelationshipsOutput {
  /// [format: did]
  pub actor: Option<String>,
  pub relationships: Vec<AppBskyGraphGetRelationshipsOutputRelationshipsUnion>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetStarterPackOutput {
  pub starter_pack: AppBskyGraphDefsStarterPackView,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetStarterPacksOutput {
  pub starter_packs: Vec<AppBskyGraphDefsStarterPackViewBasic>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetSuggestedFollowsByActorOutput {
  pub suggestions: Vec<AppBskyActorDefsProfileView>,
  /// [default: false] If true, response has fallen-back to generic results, and is not scoped using relativeToDid
  pub is_fallback: Option<bool>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyGraphListLabelsUnion {
  #[serde(rename = "com.atproto.label.defs#selfLabels")]
  ComAtprotoLabelDefsSelfLabels(Box<ComAtprotoLabelDefsSelfLabels>),
}

/// Record representing a list of accounts (actors). Scope includes both moderation-oriented lists and curration-oriented lists.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphList {
  /// Defines the purpose of the list (aka, moderation-oriented or curration-oriented),
  pub purpose: AppBskyGraphDefsListPurpose,
  /// [max_length: 64] [min_length: 1] Display name for list; can not be empty.,
  pub name: String,
  /// [max_graphemes: 300] [max_length: 3000],
  pub description: Option<String>,
  pub description_facets: Option<Vec<AppBskyRichtextFacet>>,
  /// [accept: ["image/png", "image/jpeg"]] [max_size: 1000000],
  pub avatar: Option<Blob>,
  pub labels: Option<AppBskyGraphListLabelsUnion>,
  /// [format: datetime],
  pub created_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Record representing a block relationship against an entire an entire list of accounts (actors).
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphListblock {
  /// [format: at-uri] Reference (AT-URI) to the mod list record.,
  pub subject: String,
  /// [format: datetime],
  pub created_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Record representing an account's inclusion on a specific list. The AppView will ignore duplicate listitem records.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphListitem {
  /// [format: did] The account which is included on the list.,
  pub subject: String,
  /// [format: at-uri] Reference (AT-URI) to the list record (app.bsky.graph.list).,
  pub list: String,
  /// [format: datetime],
  pub created_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphMuteActorInput {
  /// [format: at-identifier]
  pub actor: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphMuteActorListInput {
  /// [format: at-uri]
  pub list: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphMuteThreadInput {
  /// [format: at-uri]
  pub root: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphSearchStarterPacksOutput {
  pub cursor: Option<String>,
  pub starter_packs: Vec<AppBskyGraphDefsStarterPackViewBasic>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Record defining a starter pack of actors and feeds for new users.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphStarterpack {
  /// [max_graphemes: 50] [max_length: 500] [min_length: 1] Display name for starter pack; can not be empty.,
  pub name: String,
  /// [max_graphemes: 300] [max_length: 3000],
  pub description: Option<String>,
  pub description_facets: Option<Vec<AppBskyRichtextFacet>>,
  /// [format: at-uri] Reference (AT-URI) to the list record.,
  pub list: String,
  /// [max_length: 3],
  pub feeds: Option<Vec<AppBskyGraphStarterpackFeedItem>>,
  /// [format: datetime],
  pub created_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphStarterpackFeedItem {
  /// [format: at-uri]
  pub uri: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphUnmuteActorInput {
  /// [format: at-identifier]
  pub actor: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphUnmuteActorListInput {
  /// [format: at-uri]
  pub list: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphUnmuteThreadInput {
  /// [format: at-uri]
  pub root: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyLabelerDefsLabelerView {
  /// [format: at-uri]
  pub uri: String,
  /// [format: cid]
  pub cid: String,
  pub creator: AppBskyActorDefsProfileView,
  /// [minimum: 0]
  pub like_count: Option<i64>,
  pub viewer: Option<AppBskyLabelerDefsLabelerViewerState>,
  /// [format: datetime]
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyLabelerDefsLabelerViewDetailed {
  /// [format: at-uri]
  pub uri: String,
  /// [format: cid]
  pub cid: String,
  pub creator: AppBskyActorDefsProfileView,
  pub policies: AppBskyLabelerDefsLabelerPolicies,
  /// [minimum: 0]
  pub like_count: Option<i64>,
  pub viewer: Option<AppBskyLabelerDefsLabelerViewerState>,
  /// [format: datetime]
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyLabelerDefsLabelerViewerState {
  /// [format: at-uri]
  pub like: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyLabelerDefsLabelerPolicies {
  /// The label values which this labeler publishes. May include global or custom labels.
  pub label_values: Vec<ComAtprotoLabelDefsLabelValue>,
  /// Label values created by this labeler and scoped exclusively to it. Labels defined here will override global label definitions for this labeler.
  pub label_value_definitions: Option<Vec<ComAtprotoLabelDefsLabelValueDefinition>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyLabelerGetServicesOutputViewsUnion {
  #[serde(rename = "app.bsky.labeler.defs#labelerView")]
  AppBskyLabelerDefsLabelerView(Box<AppBskyLabelerDefsLabelerView>),
  #[serde(rename = "app.bsky.labeler.defs#labelerViewDetailed")]
  AppBskyLabelerDefsLabelerViewDetailed(Box<AppBskyLabelerDefsLabelerViewDetailed>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyLabelerGetServicesOutput {
  pub views: Vec<AppBskyLabelerGetServicesOutputViewsUnion>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyLabelerServiceLabelsUnion {
  #[serde(rename = "com.atproto.label.defs#selfLabels")]
  ComAtprotoLabelDefsSelfLabels(Box<ComAtprotoLabelDefsSelfLabels>),
}

/// A declaration of the existence of labeler service.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyLabelerService {
  pub policies: AppBskyLabelerDefsLabelerPolicies,
  pub labels: Option<AppBskyLabelerServiceLabelsUnion>,
  /// [format: datetime],
  pub created_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyNotificationGetUnreadCountOutput {
  pub count: i64,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyNotificationListNotificationsOutput {
  pub cursor: Option<String>,
  pub notifications: Vec<AppBskyNotificationListNotificationsNotification>,
  pub priority: Option<bool>,
  /// [format: datetime]
  pub seen_at: Option<chrono::DateTime<chrono::Utc>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyNotificationListNotificationsNotification {
  /// [format: at-uri]
  pub uri: String,
  /// [format: cid]
  pub cid: String,
  pub author: AppBskyActorDefsProfileView,
  /// [known_values: ["like", "repost", "follow", "mention", "reply", "quote", "starterpack-joined"]] Expected values are 'like', 'repost', 'follow', 'mention', 'reply', 'quote', and 'starterpack-joined'.
  pub reason: String,
  /// [format: at-uri]
  pub reason_subject: Option<String>,
  pub record: serde_json::Value,
  pub is_read: bool,
  /// [format: datetime]
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyNotificationPutPreferencesInput {
  pub priority: bool,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyNotificationRegisterPushInput {
  /// [format: did]
  pub service_did: String,
  pub token: String,
  /// [known_values: ["ios", "android", "web"]]
  pub platform: String,
  pub app_id: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyNotificationUpdateSeenInput {
  /// [format: datetime]
  pub seen_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyRichtextFacetFeaturesUnion {
  #[serde(rename = "app.bsky.richtext.facet#mention")]
  AppBskyRichtextFacetMention(Box<AppBskyRichtextFacetMention>),
  #[serde(rename = "app.bsky.richtext.facet#link")]
  AppBskyRichtextFacetLink(Box<AppBskyRichtextFacetLink>),
  #[serde(rename = "app.bsky.richtext.facet#tag")]
  AppBskyRichtextFacetTag(Box<AppBskyRichtextFacetTag>),
}

/// Annotation of a sub-string within rich text.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyRichtextFacet {
  pub index: AppBskyRichtextFacetByteSlice,
  pub features: Vec<AppBskyRichtextFacetFeaturesUnion>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Facet feature for mention of another account. The text is usually a handle, including a '@' prefix, but the facet reference is a DID.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyRichtextFacetMention {
  /// [format: did]
  pub did: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Facet feature for a URL. The text URL may have been simplified or truncated, but the facet reference should be a complete URL.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyRichtextFacetLink {
  /// [format: uri]
  pub uri: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Facet feature for a hashtag. The text usually includes a '#' prefix, but the facet reference should not (except in the case of 'double hash tags').
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyRichtextFacetTag {
  /// [max_graphemes: 64] [max_length: 640]
  pub tag: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Specifies the sub-string range a facet feature applies to. Start index is inclusive, end index is exclusive. Indices are zero-indexed, counting bytes of the UTF-8 encoded text. NOTE: some languages, like Javascript, use UTF-16 or Unicode codepoints for string slice indexing; in these languages, convert to byte arrays before working with facets.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyRichtextFacetByteSlice {
  /// [minimum: 0]
  pub byte_start: i64,
  /// [minimum: 0]
  pub byte_end: i64,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyUnspeccedDefsSkeletonSearchPost {
  /// [format: at-uri]
  pub uri: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyUnspeccedDefsSkeletonSearchActor {
  /// [format: did]
  pub did: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyUnspeccedDefsSkeletonSearchStarterPack {
  /// [format: at-uri]
  pub uri: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyUnspeccedGetConfigOutput {
  pub check_email_confirmed: Option<bool>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyUnspeccedGetPopularFeedGeneratorsOutput {
  pub cursor: Option<String>,
  pub feeds: Vec<AppBskyFeedDefsGeneratorView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyUnspeccedGetSuggestionsSkeletonOutput {
  pub cursor: Option<String>,
  pub actors: Vec<AppBskyUnspeccedDefsSkeletonSearchActor>,
  /// [format: did] DID of the account these suggestions are relative to. If this is returned undefined, suggestions are based on the viewer.
  pub relative_to_did: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyUnspeccedGetTaggedSuggestionsOutput {
  pub suggestions: Vec<AppBskyUnspeccedGetTaggedSuggestionsSuggestion>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyUnspeccedGetTaggedSuggestionsSuggestion {
  pub tag: String,
  /// [known_values: ["actor", "feed"]]
  pub subject_type: String,
  /// [format: uri]
  pub subject: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyUnspeccedSearchActorsSkeletonOutput {
  pub cursor: Option<String>,
  /// Count of search hits. Optional, may be rounded/truncated, and may not be possible to paginate through all hits.
  pub hits_total: Option<i64>,
  pub actors: Vec<AppBskyUnspeccedDefsSkeletonSearchActor>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyUnspeccedSearchPostsSkeletonOutput {
  pub cursor: Option<String>,
  /// Count of search hits. Optional, may be rounded/truncated, and may not be possible to paginate through all hits.
  pub hits_total: Option<i64>,
  pub posts: Vec<AppBskyUnspeccedDefsSkeletonSearchPost>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyUnspeccedSearchStarterPacksSkeletonOutput {
  pub cursor: Option<String>,
  /// Count of search hits. Optional, may be rounded/truncated, and may not be possible to paginate through all hits.
  pub hits_total: Option<i64>,
  pub starter_packs: Vec<AppBskyUnspeccedDefsSkeletonSearchStarterPack>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyVideoDefsJobStatus {
  pub job_id: String,
  /// [format: did]
  pub did: String,
  /// [known_values: ["JOB_STATE_COMPLETED", "JOB_STATE_FAILED"]] The state of the video processing job. All values not listed as a known value indicate that the job is in process.
  pub state: String,
  /// [minimum: 0] [maximum: 100] Progress within the current processing state.
  pub progress: Option<i64>,
  pub blob: Option<Blob>,
  pub error: Option<String>,
  pub message: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyVideoGetJobStatusOutput {
  pub job_status: AppBskyVideoDefsJobStatus,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyVideoGetUploadLimitsOutput {
  pub can_upload: bool,
  pub remaining_daily_videos: Option<i64>,
  pub remaining_daily_bytes: Option<i64>,
  pub message: Option<String>,
  pub error: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyVideoUploadVideoOutput {
  pub job_status: AppBskyVideoDefsJobStatus,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// A declaration of a Bluesky chat account.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyActorDeclaration {
  /// [known_values: ["all", "none", "following"]],
  pub allow_incoming: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyActorDefsProfileViewBasic {
  /// [format: did]
  pub did: String,
  /// [format: handle]
  pub handle: String,
  /// [max_graphemes: 64] [max_length: 640]
  pub display_name: Option<String>,
  /// [format: uri]
  pub avatar: Option<String>,
  pub associated: Option<AppBskyActorDefsProfileAssociated>,
  pub viewer: Option<AppBskyActorDefsViewerState>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  /// Set to true when the actor cannot actively participate in converations
  pub chat_disabled: Option<bool>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatBskyActorDeleteAccountOutput(pub serde_json::Value);

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoDefsMessageRef {
  /// [format: did]
  pub did: String,
  pub convo_id: String,
  pub message_id: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ChatBskyConvoDefsMessageInputEmbedUnion {
  #[serde(rename = "app.bsky.embed.record")]
  AppBskyEmbedRecord(Box<AppBskyEmbedRecord>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoDefsMessageInput {
  /// [max_graphemes: 1000] [max_length: 10000]
  pub text: String,
  /// Annotations of text (mentions, URLs, hashtags, etc)
  pub facets: Option<Vec<AppBskyRichtextFacet>>,
  pub embed: Option<ChatBskyConvoDefsMessageInputEmbedUnion>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ChatBskyConvoDefsMessageViewEmbedUnion {
  #[serde(rename = "app.bsky.embed.record#view")]
  AppBskyEmbedRecordView(Box<AppBskyEmbedRecordView>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoDefsMessageView {
  pub id: String,
  pub rev: String,
  /// [max_graphemes: 1000] [max_length: 10000]
  pub text: String,
  /// Annotations of text (mentions, URLs, hashtags, etc)
  pub facets: Option<Vec<AppBskyRichtextFacet>>,
  pub embed: Option<ChatBskyConvoDefsMessageViewEmbedUnion>,
  pub sender: ChatBskyConvoDefsMessageViewSender,
  /// [format: datetime]
  pub sent_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoDefsDeletedMessageView {
  pub id: String,
  pub rev: String,
  pub sender: ChatBskyConvoDefsMessageViewSender,
  /// [format: datetime]
  pub sent_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoDefsMessageViewSender {
  /// [format: did]
  pub did: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ChatBskyConvoDefsConvoViewLastMessageUnion {
  #[serde(rename = "chat.bsky.convo.defs#messageView")]
  ChatBskyConvoDefsMessageView(Box<ChatBskyConvoDefsMessageView>),
  #[serde(rename = "chat.bsky.convo.defs#deletedMessageView")]
  ChatBskyConvoDefsDeletedMessageView(Box<ChatBskyConvoDefsDeletedMessageView>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoDefsConvoView {
  pub id: String,
  pub rev: String,
  pub members: Vec<ChatBskyActorDefsProfileViewBasic>,
  pub last_message: Option<ChatBskyConvoDefsConvoViewLastMessageUnion>,
  pub muted: bool,
  pub opened: Option<bool>,
  pub unread_count: i64,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoDefsLogBeginConvo {
  pub rev: String,
  pub convo_id: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoDefsLogLeaveConvo {
  pub rev: String,
  pub convo_id: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ChatBskyConvoDefsLogCreateMessageMessageUnion {
  #[serde(rename = "chat.bsky.convo.defs#messageView")]
  ChatBskyConvoDefsMessageView(Box<ChatBskyConvoDefsMessageView>),
  #[serde(rename = "chat.bsky.convo.defs#deletedMessageView")]
  ChatBskyConvoDefsDeletedMessageView(Box<ChatBskyConvoDefsDeletedMessageView>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoDefsLogCreateMessage {
  pub rev: String,
  pub convo_id: String,
  pub message: ChatBskyConvoDefsLogCreateMessageMessageUnion,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ChatBskyConvoDefsLogDeleteMessageMessageUnion {
  #[serde(rename = "chat.bsky.convo.defs#messageView")]
  ChatBskyConvoDefsMessageView(Box<ChatBskyConvoDefsMessageView>),
  #[serde(rename = "chat.bsky.convo.defs#deletedMessageView")]
  ChatBskyConvoDefsDeletedMessageView(Box<ChatBskyConvoDefsDeletedMessageView>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoDefsLogDeleteMessage {
  pub rev: String,
  pub convo_id: String,
  pub message: ChatBskyConvoDefsLogDeleteMessageMessageUnion,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoDeleteMessageForSelfInput {
  pub convo_id: String,
  pub message_id: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoGetConvoOutput {
  pub convo: ChatBskyConvoDefsConvoView,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoGetConvoForMembersOutput {
  pub convo: ChatBskyConvoDefsConvoView,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ChatBskyConvoGetLogOutputLogsUnion {
  #[serde(rename = "chat.bsky.convo.defs#logBeginConvo")]
  ChatBskyConvoDefsLogBeginConvo(Box<ChatBskyConvoDefsLogBeginConvo>),
  #[serde(rename = "chat.bsky.convo.defs#logLeaveConvo")]
  ChatBskyConvoDefsLogLeaveConvo(Box<ChatBskyConvoDefsLogLeaveConvo>),
  #[serde(rename = "chat.bsky.convo.defs#logCreateMessage")]
  ChatBskyConvoDefsLogCreateMessage(Box<ChatBskyConvoDefsLogCreateMessage>),
  #[serde(rename = "chat.bsky.convo.defs#logDeleteMessage")]
  ChatBskyConvoDefsLogDeleteMessage(Box<ChatBskyConvoDefsLogDeleteMessage>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoGetLogOutput {
  pub cursor: Option<String>,
  pub logs: Vec<ChatBskyConvoGetLogOutputLogsUnion>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ChatBskyConvoGetMessagesOutputMessagesUnion {
  #[serde(rename = "chat.bsky.convo.defs#messageView")]
  ChatBskyConvoDefsMessageView(Box<ChatBskyConvoDefsMessageView>),
  #[serde(rename = "chat.bsky.convo.defs#deletedMessageView")]
  ChatBskyConvoDefsDeletedMessageView(Box<ChatBskyConvoDefsDeletedMessageView>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoGetMessagesOutput {
  pub cursor: Option<String>,
  pub messages: Vec<ChatBskyConvoGetMessagesOutputMessagesUnion>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoLeaveConvoInput {
  pub convo_id: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoLeaveConvoOutput {
  pub convo_id: String,
  pub rev: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoListConvosOutput {
  pub cursor: Option<String>,
  pub convos: Vec<ChatBskyConvoDefsConvoView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoMuteConvoInput {
  pub convo_id: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoMuteConvoOutput {
  pub convo: ChatBskyConvoDefsConvoView,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoSendMessageInput {
  pub convo_id: String,
  pub message: ChatBskyConvoDefsMessageInput,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoSendMessageBatchInput {
  /// [max_length: 100]
  pub items: Vec<ChatBskyConvoSendMessageBatchBatchItem>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoSendMessageBatchOutput {
  pub items: Vec<ChatBskyConvoDefsMessageView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoSendMessageBatchBatchItem {
  pub convo_id: String,
  pub message: ChatBskyConvoDefsMessageInput,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoUnmuteConvoInput {
  pub convo_id: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoUnmuteConvoOutput {
  pub convo: ChatBskyConvoDefsConvoView,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoUpdateReadInput {
  pub convo_id: String,
  pub message_id: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoUpdateReadOutput {
  pub convo: ChatBskyConvoDefsConvoView,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyModerationGetActorMetadataOutput {
  pub day: ChatBskyModerationGetActorMetadataMetadata,
  pub month: ChatBskyModerationGetActorMetadataMetadata,
  pub all: ChatBskyModerationGetActorMetadataMetadata,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyModerationGetActorMetadataMetadata {
  pub messages_sent: i64,
  pub messages_received: i64,
  pub convos: i64,
  pub convos_started: i64,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ChatBskyModerationGetMessageContextOutputMessagesUnion {
  #[serde(rename = "chat.bsky.convo.defs#messageView")]
  ChatBskyConvoDefsMessageView(Box<ChatBskyConvoDefsMessageView>),
  #[serde(rename = "chat.bsky.convo.defs#deletedMessageView")]
  ChatBskyConvoDefsDeletedMessageView(Box<ChatBskyConvoDefsDeletedMessageView>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyModerationGetMessageContextOutput {
  pub messages: Vec<ChatBskyModerationGetMessageContextOutputMessagesUnion>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyModerationUpdateActorAccessInput {
  /// [format: did]
  pub actor: String,
  pub allow_access: bool,
  pub ref_: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminDefsStatusAttr {
  pub applied: bool,
  pub ref_: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminDefsAccountView {
  /// [format: did]
  pub did: String,
  /// [format: handle]
  pub handle: String,
  pub email: Option<String>,
  pub related_records: Option<Vec<serde_json::Value>>,
  /// [format: datetime]
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  pub invited_by: Option<ComAtprotoServerDefsInviteCode>,
  pub invites: Option<Vec<ComAtprotoServerDefsInviteCode>>,
  pub invites_disabled: Option<bool>,
  /// [format: datetime]
  pub email_confirmed_at: Option<chrono::DateTime<chrono::Utc>>,
  pub invite_note: Option<String>,
  /// [format: datetime]
  pub deactivated_at: Option<chrono::DateTime<chrono::Utc>>,
  pub threat_signatures: Option<Vec<ComAtprotoAdminDefsThreatSignature>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminDefsRepoRef {
  /// [format: did]
  pub did: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminDefsRepoBlobRef {
  /// [format: did]
  pub did: String,
  /// [format: cid]
  pub cid: String,
  /// [format: at-uri]
  pub record_uri: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminDefsThreatSignature {
  pub property: String,
  pub value: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminDeleteAccountInput {
  /// [format: did]
  pub did: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminDisableAccountInvitesInput {
  /// [format: did]
  pub account: String,
  /// Optional reason for disabled invites.
  pub note: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminDisableInviteCodesInput {
  pub codes: Option<Vec<String>>,
  pub accounts: Option<Vec<String>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminEnableAccountInvitesInput {
  /// [format: did]
  pub account: String,
  /// Optional reason for enabled invites.
  pub note: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminGetAccountInfosOutput {
  pub infos: Vec<ComAtprotoAdminDefsAccountView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminGetInviteCodesOutput {
  pub cursor: Option<String>,
  pub codes: Vec<ComAtprotoServerDefsInviteCode>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoAdminGetSubjectStatusOutputSubjectUnion {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsRepoRef(Box<ComAtprotoAdminDefsRepoRef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongRef(Box<ComAtprotoRepoStrongRef>),
  #[serde(rename = "com.atproto.admin.defs#repoBlobRef")]
  ComAtprotoAdminDefsRepoBlobRef(Box<ComAtprotoAdminDefsRepoBlobRef>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminGetSubjectStatusOutput {
  pub subject: ComAtprotoAdminGetSubjectStatusOutputSubjectUnion,
  pub takedown: Option<ComAtprotoAdminDefsStatusAttr>,
  pub deactivated: Option<ComAtprotoAdminDefsStatusAttr>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminSearchAccountsOutput {
  pub cursor: Option<String>,
  pub accounts: Vec<ComAtprotoAdminDefsAccountView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminSendEmailInput {
  /// [format: did]
  pub recipient_did: String,
  pub content: String,
  pub subject: Option<String>,
  /// [format: did]
  pub sender_did: String,
  /// Additional comment by the sender that won't be used in the email itself but helpful to provide more context for moderators/reviewers
  pub comment: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminSendEmailOutput {
  pub sent: bool,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminUpdateAccountEmailInput {
  /// [format: at-identifier] The handle or DID of the repo.
  pub account: String,
  pub email: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminUpdateAccountHandleInput {
  /// [format: did]
  pub did: String,
  /// [format: handle]
  pub handle: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminUpdateAccountPasswordInput {
  /// [format: did]
  pub did: String,
  pub password: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoAdminUpdateSubjectStatusInputSubjectUnion {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsRepoRef(Box<ComAtprotoAdminDefsRepoRef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongRef(Box<ComAtprotoRepoStrongRef>),
  #[serde(rename = "com.atproto.admin.defs#repoBlobRef")]
  ComAtprotoAdminDefsRepoBlobRef(Box<ComAtprotoAdminDefsRepoBlobRef>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminUpdateSubjectStatusInput {
  pub subject: ComAtprotoAdminUpdateSubjectStatusInputSubjectUnion,
  pub takedown: Option<ComAtprotoAdminDefsStatusAttr>,
  pub deactivated: Option<ComAtprotoAdminDefsStatusAttr>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoAdminUpdateSubjectStatusOutputSubjectUnion {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsRepoRef(Box<ComAtprotoAdminDefsRepoRef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongRef(Box<ComAtprotoRepoStrongRef>),
  #[serde(rename = "com.atproto.admin.defs#repoBlobRef")]
  ComAtprotoAdminDefsRepoBlobRef(Box<ComAtprotoAdminDefsRepoBlobRef>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminUpdateSubjectStatusOutput {
  pub subject: ComAtprotoAdminUpdateSubjectStatusOutputSubjectUnion,
  pub takedown: Option<ComAtprotoAdminDefsStatusAttr>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoIdentityGetRecommendedDidCredentialsOutput {
  /// Recommended rotation keys for PLC dids. Should be undefined (or ignored) for did:webs.
  pub rotation_keys: Option<Vec<String>>,
  pub also_known_as: Option<Vec<String>>,
  pub verification_methods: Option<serde_json::Value>,
  pub services: Option<serde_json::Value>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoIdentityResolveHandleOutput {
  /// [format: did]
  pub did: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoIdentitySignPlcOperationInput {
  /// A token received through com.atproto.identity.requestPlcOperationSignature
  pub token: Option<String>,
  pub rotation_keys: Option<Vec<String>>,
  pub also_known_as: Option<Vec<String>>,
  pub verification_methods: Option<serde_json::Value>,
  pub services: Option<serde_json::Value>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoIdentitySignPlcOperationOutput {
  /// A signed DID PLC operation.
  pub operation: serde_json::Value,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoIdentitySubmitPlcOperationInput {
  pub operation: serde_json::Value,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoIdentityUpdateHandleInput {
  /// [format: handle] The new handle.
  pub handle: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Metadata tag on an atproto resource (eg, repo or record).
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoLabelDefsLabel {
  /// The AT Protocol version of the label object.
  pub ver: Option<i64>,
  /// [format: did] DID of the actor who created this label.
  pub src: String,
  /// [format: uri] AT URI of the record, repository (account), or other resource that this label applies to.
  pub uri: String,
  /// [format: cid] Optionally, CID specifying the specific version of 'uri' resource this label applies to.
  pub cid: Option<String>,
  /// [max_length: 128] The short string name of the value or type of this label.
  pub val: String,
  /// If true, this is a negation label, overwriting a previous label.
  pub neg: Option<bool>,
  /// [format: datetime] Timestamp when this label was created.
  pub cts: chrono::DateTime<chrono::Utc>,
  /// [format: datetime] Timestamp at which this label expires (no longer applies).
  pub exp: Option<chrono::DateTime<chrono::Utc>>,
  pub sig: Option<Vec<u8>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Metadata tags on an atproto record, published by the author within the record.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoLabelDefsSelfLabels {
  /// [max_length: 10]
  pub values: Vec<ComAtprotoLabelDefsSelfLabel>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Metadata tag on an atproto record, published by the author within the record. Note that schemas should use #selfLabels, not #selfLabel.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoLabelDefsSelfLabel {
  /// [max_length: 128] The short string name of the value or type of this label.
  pub val: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Declares a label value and its expected interpretations and behaviors.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoLabelDefsLabelValueDefinition {
  /// [max_graphemes: 100] [max_length: 100] The value of the label being defined. Must only include lowercase ascii and the '-' character ([a-z-]+).
  pub identifier: String,
  /// [known_values: ["inform", "alert", "none"]] How should a client visually convey this label? 'inform' means neutral and informational; 'alert' means negative and warning; 'none' means show nothing.
  pub severity: String,
  /// [known_values: ["content", "media", "none"]] What should this label hide in the UI, if applied? 'content' hides all of the target; 'media' hides the images/video/audio; 'none' hides nothing.
  pub blurs: String,
  /// [known_values: ["ignore", "warn", "hide"]] [default: warn] The default setting for this label.
  pub default_setting: Option<String>,
  /// Does the user need to have adult content enabled in order to configure this label?
  pub adult_only: Option<bool>,
  pub locales: Vec<ComAtprotoLabelDefsLabelValueDefinitionStrings>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Strings which describe the label in the UI, localized into a specific language.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoLabelDefsLabelValueDefinitionStrings {
  /// [format: language] The code of the language these strings are written in.
  pub lang: String,
  /// [max_graphemes: 64] [max_length: 640] A short human-readable name for the label.
  pub name: String,
  /// [max_graphemes: 10000] [max_length: 100000] A longer description of what the label means and why it might be applied.
  pub description: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComAtprotoLabelDefsLabelValue(pub String);

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoLabelQueryLabelsOutput {
  pub cursor: Option<String>,
  pub labels: Vec<ComAtprotoLabelDefsLabel>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoLabelSubscribeLabelsLabels {
  pub seq: i64,
  pub labels: Vec<ComAtprotoLabelDefsLabel>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoLabelSubscribeLabelsInfo {
  /// [known_values: ["OutdatedCursor"]]
  pub name: String,
  pub message: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoModerationCreateReportInputSubjectUnion {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsRepoRef(Box<ComAtprotoAdminDefsRepoRef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongRef(Box<ComAtprotoRepoStrongRef>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoModerationCreateReportInput {
  /// Indicates the broad category of violation the report is for.
  pub reason_type: ComAtprotoModerationDefsReasonType,
  /// [max_graphemes: 2000] [max_length: 20000] Additional context about the content and violation.
  pub reason: Option<String>,
  pub subject: ComAtprotoModerationCreateReportInputSubjectUnion,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoModerationCreateReportOutputSubjectUnion {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsRepoRef(Box<ComAtprotoAdminDefsRepoRef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongRef(Box<ComAtprotoRepoStrongRef>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoModerationCreateReportOutput {
  pub id: i64,
  pub reason_type: ComAtprotoModerationDefsReasonType,
  /// [max_graphemes: 2000] [max_length: 20000]
  pub reason: Option<String>,
  pub subject: ComAtprotoModerationCreateReportOutputSubjectUnion,
  /// [format: did]
  pub reported_by: String,
  /// [format: datetime]
  pub created_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComAtprotoModerationDefsReasonType(pub String);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoRepoApplyWritesInputWritesUnion {
  #[serde(rename = "com.atproto.repo.applyWrites#create")]
  ComAtprotoRepoApplyWritesCreate(Box<ComAtprotoRepoApplyWritesCreate>),
  #[serde(rename = "com.atproto.repo.applyWrites#update")]
  ComAtprotoRepoApplyWritesUpdate(Box<ComAtprotoRepoApplyWritesUpdate>),
  #[serde(rename = "com.atproto.repo.applyWrites#delete")]
  ComAtprotoRepoApplyWritesDelete(Box<ComAtprotoRepoApplyWritesDelete>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoApplyWritesInput {
  /// [format: at-identifier] The handle or DID of the repo (aka, current account).
  pub repo: String,
  /// Can be set to 'false' to skip Lexicon schema validation of record data across all operations, 'true' to require it, or leave unset to validate only for known Lexicons.
  pub validate: Option<bool>,
  pub writes: Vec<ComAtprotoRepoApplyWritesInputWritesUnion>,
  /// [format: cid] If provided, the entire operation will fail if the current repo commit CID does not match this value. Used to prevent conflicting repo mutations.
  pub swap_commit: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoRepoApplyWritesOutputResultsUnion {
  #[serde(rename = "com.atproto.repo.applyWrites#createResult")]
  ComAtprotoRepoApplyWritesCreateResult(Box<ComAtprotoRepoApplyWritesCreateResult>),
  #[serde(rename = "com.atproto.repo.applyWrites#updateResult")]
  ComAtprotoRepoApplyWritesUpdateResult(Box<ComAtprotoRepoApplyWritesUpdateResult>),
  #[serde(rename = "com.atproto.repo.applyWrites#deleteResult")]
  ComAtprotoRepoApplyWritesDeleteResult(Box<ComAtprotoRepoApplyWritesDeleteResult>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoApplyWritesOutput {
  pub commit: Option<ComAtprotoRepoDefsCommitMeta>,
  pub results: Option<Vec<ComAtprotoRepoApplyWritesOutputResultsUnion>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Operation which creates a new record.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoApplyWritesCreate {
  /// [format: nsid]
  pub collection: String,
  /// [max_length: 512]
  pub rkey: Option<String>,
  pub value: serde_json::Value,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Operation which updates an existing record.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoApplyWritesUpdate {
  /// [format: nsid]
  pub collection: String,
  pub rkey: String,
  pub value: serde_json::Value,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Operation which deletes an existing record.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoApplyWritesDelete {
  /// [format: nsid]
  pub collection: String,
  pub rkey: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoApplyWritesCreateResult {
  /// [format: at-uri]
  pub uri: String,
  /// [format: cid]
  pub cid: String,
  /// [known_values: ["valid", "unknown"]]
  pub validation_status: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoApplyWritesUpdateResult {
  /// [format: at-uri]
  pub uri: String,
  /// [format: cid]
  pub cid: String,
  /// [known_values: ["valid", "unknown"]]
  pub validation_status: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComAtprotoRepoApplyWritesDeleteResult(pub serde_json::Value);

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoCreateRecordInput {
  /// [format: at-identifier] The handle or DID of the repo (aka, current account).
  pub repo: String,
  /// [format: nsid] The NSID of the record collection.
  pub collection: String,
  /// [max_length: 512] The Record Key.
  pub rkey: Option<String>,
  /// Can be set to 'false' to skip Lexicon schema validation of record data, 'true' to require it, or leave unset to validate only for known Lexicons.
  pub validate: Option<bool>,
  /// The record itself. Must contain a $type field.
  pub record: serde_json::Value,
  /// [format: cid] Compare and swap with the previous commit by CID.
  pub swap_commit: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoCreateRecordOutput {
  /// [format: at-uri]
  pub uri: String,
  /// [format: cid]
  pub cid: String,
  pub commit: Option<ComAtprotoRepoDefsCommitMeta>,
  /// [known_values: ["valid", "unknown"]]
  pub validation_status: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoDefsCommitMeta {
  /// [format: cid]
  pub cid: String,
  pub rev: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoDeleteRecordInput {
  /// [format: at-identifier] The handle or DID of the repo (aka, current account).
  pub repo: String,
  /// [format: nsid] The NSID of the record collection.
  pub collection: String,
  /// The Record Key.
  pub rkey: String,
  /// [format: cid] Compare and swap with the previous record by CID.
  pub swap_record: Option<String>,
  /// [format: cid] Compare and swap with the previous commit by CID.
  pub swap_commit: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoDeleteRecordOutput {
  pub commit: Option<ComAtprotoRepoDefsCommitMeta>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoDescribeRepoOutput {
  /// [format: handle]
  pub handle: String,
  /// [format: did]
  pub did: String,
  /// The complete DID document for this account.
  pub did_doc: serde_json::Value,
  /// List of all the collections (NSIDs) for which this repo contains at least one record.
  pub collections: Vec<String>,
  /// Indicates if handle is currently valid (resolves bi-directionally)
  pub handle_is_correct: bool,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoGetRecordOutput {
  /// [format: at-uri]
  pub uri: String,
  /// [format: cid]
  pub cid: Option<String>,
  pub value: serde_json::Value,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoListMissingBlobsOutput {
  pub cursor: Option<String>,
  pub blobs: Vec<ComAtprotoRepoListMissingBlobsRecordBlob>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoListMissingBlobsRecordBlob {
  /// [format: cid]
  pub cid: String,
  /// [format: at-uri]
  pub record_uri: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoListRecordsOutput {
  pub cursor: Option<String>,
  pub records: Vec<ComAtprotoRepoListRecordsRecord>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoListRecordsRecord {
  /// [format: at-uri]
  pub uri: String,
  /// [format: cid]
  pub cid: String,
  pub value: serde_json::Value,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoPutRecordInput {
  /// [format: at-identifier] The handle or DID of the repo (aka, current account).
  pub repo: String,
  /// [format: nsid] The NSID of the record collection.
  pub collection: String,
  /// [max_length: 512] The Record Key.
  pub rkey: String,
  /// Can be set to 'false' to skip Lexicon schema validation of record data, 'true' to require it, or leave unset to validate only for known Lexicons.
  pub validate: Option<bool>,
  /// The record to write.
  pub record: serde_json::Value,
  /// [format: cid] Compare and swap with the previous record by CID. WARNING: nullable and optional field; may cause problems with golang implementation
  pub swap_record: Option<String>,
  /// [format: cid] Compare and swap with the previous commit by CID.
  pub swap_commit: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoPutRecordOutput {
  /// [format: at-uri]
  pub uri: String,
  /// [format: cid]
  pub cid: String,
  pub commit: Option<ComAtprotoRepoDefsCommitMeta>,
  /// [known_values: ["valid", "unknown"]]
  pub validation_status: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoStrongRef {
  /// [format: at-uri]
  pub uri: String,
  /// [format: cid]
  pub cid: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoUploadBlobOutput {
  pub blob: Blob,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerCheckAccountStatusOutput {
  pub activated: bool,
  pub valid_did: bool,
  /// [format: cid]
  pub repo_commit: String,
  pub repo_rev: String,
  pub repo_blocks: i64,
  pub indexed_records: i64,
  pub private_state_values: i64,
  pub expected_blobs: i64,
  pub imported_blobs: i64,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerConfirmEmailInput {
  pub email: String,
  pub token: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerCreateAccountInput {
  pub email: Option<String>,
  /// [format: handle] Requested handle for the account.
  pub handle: String,
  /// [format: did] Pre-existing atproto DID, being imported to a new account.
  pub did: Option<String>,
  pub invite_code: Option<String>,
  pub verification_code: Option<String>,
  pub verification_phone: Option<String>,
  /// Initial account password. May need to meet instance-specific password strength requirements.
  pub password: Option<String>,
  /// DID PLC rotation key (aka, recovery key) to be included in PLC creation operation.
  pub recovery_key: Option<String>,
  /// A signed DID PLC operation to be submitted as part of importing an existing account to this instance. NOTE: this optional field may be updated when full account migration is implemented.
  pub plc_op: Option<serde_json::Value>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Account login session returned on successful account creation.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerCreateAccountOutput {
  pub access_jwt: String,
  pub refresh_jwt: String,
  /// [format: handle]
  pub handle: String,
  /// [format: did] The DID of the new account.
  pub did: String,
  /// Complete DID document.
  pub did_doc: Option<serde_json::Value>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerCreateAppPasswordInput {
  /// A short name for the App Password, to help distinguish them.
  pub name: String,
  /// If an app password has 'privileged' access to possibly sensitive account state. Meant for use with trusted clients.
  pub privileged: Option<bool>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerCreateAppPasswordAppPassword {
  pub name: String,
  pub password: String,
  /// [format: datetime]
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub privileged: Option<bool>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerCreateInviteCodeInput {
  pub use_count: i64,
  /// [format: did]
  pub for_account: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerCreateInviteCodeOutput {
  pub code: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerCreateInviteCodesInput {
  /// [default: 1]
  pub code_count: i64,
  pub use_count: i64,
  pub for_accounts: Option<Vec<String>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerCreateInviteCodesOutput {
  pub codes: Vec<ComAtprotoServerCreateInviteCodesAccountCodes>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerCreateInviteCodesAccountCodes {
  pub account: String,
  pub codes: Vec<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerCreateSessionInput {
  /// Handle or other identifier supported by the server for the authenticating user.
  pub identifier: String,
  pub password: String,
  pub auth_factor_token: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerCreateSessionOutput {
  pub access_jwt: String,
  pub refresh_jwt: String,
  /// [format: handle]
  pub handle: String,
  /// [format: did]
  pub did: String,
  pub did_doc: Option<serde_json::Value>,
  pub email: Option<String>,
  pub email_confirmed: Option<bool>,
  pub email_auth_factor: Option<bool>,
  pub active: Option<bool>,
  /// [known_values: ["takendown", "suspended", "deactivated"]] If active=false, this optional field indicates a possible reason for why the account is not active. If active=false and no status is supplied, then the host makes no claim for why the repository is no longer being hosted.
  pub status: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerDeactivateAccountInput {
  /// [format: datetime] A recommendation to server as to how long they should hold onto the deactivated account before deleting.
  pub delete_after: Option<chrono::DateTime<chrono::Utc>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerDefsInviteCode {
  pub code: String,
  pub available: i64,
  pub disabled: bool,
  pub for_account: String,
  pub created_by: String,
  /// [format: datetime]
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub uses: Vec<ComAtprotoServerDefsInviteCodeUse>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerDefsInviteCodeUse {
  /// [format: did]
  pub used_by: String,
  /// [format: datetime]
  pub used_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerDeleteAccountInput {
  /// [format: did]
  pub did: String,
  pub password: String,
  pub token: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerDescribeServerOutput {
  /// If true, an invite code must be supplied to create an account on this instance.
  pub invite_code_required: Option<bool>,
  /// If true, a phone verification token must be supplied to create an account on this instance.
  pub phone_verification_required: Option<bool>,
  /// List of domain suffixes that can be used in account handles.
  pub available_user_domains: Vec<String>,
  /// URLs of service policy documents.
  pub links: Option<ComAtprotoServerDescribeServerLinks>,
  /// Contact information
  pub contact: Option<ComAtprotoServerDescribeServerContact>,
  /// [format: did]
  pub did: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerDescribeServerLinks {
  /// [format: uri]
  pub privacy_policy: Option<String>,
  /// [format: uri]
  pub terms_of_service: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerDescribeServerContact {
  pub email: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerGetAccountInviteCodesOutput {
  pub codes: Vec<ComAtprotoServerDefsInviteCode>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerGetServiceAuthOutput {
  pub token: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerGetSessionOutput {
  /// [format: handle]
  pub handle: String,
  /// [format: did]
  pub did: String,
  pub email: Option<String>,
  pub email_confirmed: Option<bool>,
  pub email_auth_factor: Option<bool>,
  pub did_doc: Option<serde_json::Value>,
  pub active: Option<bool>,
  /// [known_values: ["takendown", "suspended", "deactivated"]] If active=false, this optional field indicates a possible reason for why the account is not active. If active=false and no status is supplied, then the host makes no claim for why the repository is no longer being hosted.
  pub status: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerListAppPasswordsOutput {
  pub passwords: Vec<ComAtprotoServerListAppPasswordsAppPassword>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerListAppPasswordsAppPassword {
  pub name: String,
  /// [format: datetime]
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub privileged: Option<bool>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerRefreshSessionOutput {
  pub access_jwt: String,
  pub refresh_jwt: String,
  /// [format: handle]
  pub handle: String,
  /// [format: did]
  pub did: String,
  pub did_doc: Option<serde_json::Value>,
  pub active: Option<bool>,
  /// [known_values: ["takendown", "suspended", "deactivated"]] Hosting status of the account. If not specified, then assume 'active'.
  pub status: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerRequestEmailUpdateOutput {
  pub token_required: bool,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerRequestPasswordResetInput {
  pub email: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerReserveSigningKeyInput {
  /// [format: did] The DID to reserve a key for.
  pub did: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerReserveSigningKeyOutput {
  /// The public key for the reserved signing key, in did:key serialization.
  pub signing_key: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerResetPasswordInput {
  pub token: String,
  pub password: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerRevokeAppPasswordInput {
  pub name: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerUpdateEmailInput {
  pub email: String,
  pub email_auth_factor: Option<bool>,
  /// Requires a token from com.atproto.sever.requestEmailUpdate if the account's email has been confirmed.
  pub token: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncGetHeadOutput {
  /// [format: cid]
  pub root: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncGetLatestCommitOutput {
  /// [format: cid]
  pub cid: String,
  pub rev: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncGetRepoStatusOutput {
  /// [format: did]
  pub did: String,
  pub active: bool,
  /// [known_values: ["takendown", "suspended", "deactivated"]] If active=false, this optional field indicates a possible reason for why the account is not active. If active=false and no status is supplied, then the host makes no claim for why the repository is no longer being hosted.
  pub status: Option<String>,
  /// Optional field, the current rev of the repo, if active=true
  pub rev: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncListBlobsOutput {
  pub cursor: Option<String>,
  pub cids: Vec<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncListReposOutput {
  pub cursor: Option<String>,
  pub repos: Vec<ComAtprotoSyncListReposRepo>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncListReposRepo {
  /// [format: did]
  pub did: String,
  /// [format: cid] Current repo commit CID
  pub head: String,
  pub rev: String,
  pub active: Option<bool>,
  /// [known_values: ["takendown", "suspended", "deactivated"]] If active=false, this optional field indicates a possible reason for why the account is not active. If active=false and no status is supplied, then the host makes no claim for why the repository is no longer being hosted.
  pub status: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncNotifyOfUpdateInput {
  /// Hostname of the current service (usually a PDS) that is notifying of update.
  pub hostname: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncRequestCrawlInput {
  /// Hostname of the current service (eg, PDS) that is requesting to be crawled.
  pub hostname: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Represents an update of repository state. Note that empty commits are allowed, which include no repo data changes, but an update to rev and signature.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncSubscribeReposCommit {
  /// The stream sequence number of this message.
  pub seq: i64,
  /// DEPRECATED -- unused
  pub rebase: bool,
  /// Indicates that this commit contained too many ops, or data size was too large. Consumers will need to make a separate request to get missing data.
  pub too_big: bool,
  /// [format: did] The repo this event comes from.
  pub repo: String,
  /// Repo commit object CID.
  pub commit: ciborium::Value,
  /// DEPRECATED -- unused. WARNING -- nullable and optional; stick with optional to ensure golang interoperability.
  pub prev: Option<ciborium::Value>,
  /// The rev of the emitted commit. Note that this information is also in the commit object included in blocks, unless this is a tooBig event.
  pub rev: String,
  /// The rev of the last emitted commit from this repo (if any).
  pub since: Option<String>,
  /// [max_length: 1000000]
  pub blocks: Vec<u8>,
  /// [max_length: 200]
  pub ops: Vec<ComAtprotoSyncSubscribeReposRepoOp>,
  pub blobs: Vec<ciborium::Value>,
  /// [format: datetime] Timestamp of when this message was originally broadcast.
  pub time: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Represents a change to an account's identity. Could be an updated handle, signing key, or pds hosting endpoint. Serves as a prod to all downstream services to refresh their identity cache.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncSubscribeReposIdentity {
  pub seq: i64,
  /// [format: did]
  pub did: String,
  /// [format: datetime]
  pub time: chrono::DateTime<chrono::Utc>,
  /// [format: handle] The current handle for the account, or 'handle.invalid' if validation fails. This field is optional, might have been validated or passed-through from an upstream source. Semantics and behaviors for PDS vs Relay may evolve in the future; see atproto specs for more details.
  pub handle: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Represents a change to an account's status on a host (eg, PDS or Relay). The semantics of this event are that the status is at the host which emitted the event, not necessarily that at the currently active PDS. Eg, a Relay takedown would emit a takedown with active=false, even if the PDS is still active.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncSubscribeReposAccount {
  pub seq: i64,
  /// [format: did]
  pub did: String,
  /// [format: datetime]
  pub time: chrono::DateTime<chrono::Utc>,
  /// Indicates that the account has a repository which can be fetched from the host that emitted this event.
  pub active: bool,
  /// [known_values: ["takendown", "suspended", "deleted", "deactivated"]] If active=false, this optional field indicates a reason for why the account is not active.
  pub status: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// DEPRECATED -- Use #identity event instead
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncSubscribeReposHandle {
  pub seq: i64,
  /// [format: did]
  pub did: String,
  /// [format: handle]
  pub handle: String,
  /// [format: datetime]
  pub time: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// DEPRECATED -- Use #account event instead
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncSubscribeReposMigrate {
  pub seq: i64,
  /// [format: did]
  pub did: String,
  pub migrate_to: Option<String>,
  /// [format: datetime]
  pub time: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// DEPRECATED -- Use #account event instead
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncSubscribeReposTombstone {
  pub seq: i64,
  /// [format: did]
  pub did: String,
  /// [format: datetime]
  pub time: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncSubscribeReposInfo {
  /// [known_values: ["OutdatedCursor"]]
  pub name: String,
  pub message: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// A repo operation, ie a mutation of a single record.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncSubscribeReposRepoOp {
  /// [known_values: ["create", "update", "delete"]]
  pub action: String,
  pub path: String,
  /// For creates and updates, the new record CID. For deletions, null.
  pub cid: Option<ciborium::Value>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoTempCheckSignupQueueOutput {
  pub activated: bool,
  pub place_in_queue: Option<i64>,
  pub estimated_time_ms: Option<i64>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoTempFetchLabelsOutput {
  pub labels: Vec<ComAtprotoLabelDefsLabel>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoTempRequestPhoneVerificationInput {
  pub phone_number: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneCommunicationCreateTemplateInput {
  /// Name of the template.
  pub name: String,
  /// Content of the template, markdown supported, can contain variable placeholders.
  pub content_markdown: String,
  /// Subject of the message, used in emails.
  pub subject: String,
  /// [format: language] Message language.
  pub lang: Option<String>,
  /// [format: did] DID of the user who is creating the template.
  pub created_by: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneCommunicationDefsTemplateView {
  pub id: String,
  /// Name of the template.
  pub name: String,
  /// Content of the template, can contain markdown and variable placeholders.
  pub subject: Option<String>,
  /// Subject of the message, used in emails.
  pub content_markdown: String,
  pub disabled: bool,
  /// [format: language] Message language.
  pub lang: Option<String>,
  /// [format: did] DID of the user who last updated the template.
  pub last_updated_by: String,
  /// [format: datetime]
  pub created_at: chrono::DateTime<chrono::Utc>,
  /// [format: datetime]
  pub updated_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneCommunicationDeleteTemplateInput {
  pub id: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneCommunicationListTemplatesOutput {
  pub communication_templates: Vec<ToolsOzoneCommunicationDefsTemplateView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneCommunicationUpdateTemplateInput {
  /// ID of the template to be updated.
  pub id: String,
  /// Name of the template.
  pub name: Option<String>,
  /// [format: language] Message language.
  pub lang: Option<String>,
  /// Content of the template, markdown supported, can contain variable placeholders.
  pub content_markdown: Option<String>,
  /// Subject of the message, used in emails.
  pub subject: Option<String>,
  /// [format: did] DID of the user who is updating the template.
  pub updated_by: Option<String>,
  pub disabled: Option<bool>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ToolsOzoneModerationDefsModEventViewEventUnion {
  #[serde(rename = "tools.ozone.moderation.defs#modEventTakedown")]
  ToolsOzoneModerationDefsModEventTakedown(Box<ToolsOzoneModerationDefsModEventTakedown>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventReverseTakedown")]
  ToolsOzoneModerationDefsModEventReverseTakedown(
    Box<ToolsOzoneModerationDefsModEventReverseTakedown>,
  ),
  #[serde(rename = "tools.ozone.moderation.defs#modEventComment")]
  ToolsOzoneModerationDefsModEventComment(Box<ToolsOzoneModerationDefsModEventComment>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventReport")]
  ToolsOzoneModerationDefsModEventReport(Box<ToolsOzoneModerationDefsModEventReport>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventLabel")]
  ToolsOzoneModerationDefsModEventLabel(Box<ToolsOzoneModerationDefsModEventLabel>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventAcknowledge")]
  ToolsOzoneModerationDefsModEventAcknowledge(Box<ToolsOzoneModerationDefsModEventAcknowledge>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventEscalate")]
  ToolsOzoneModerationDefsModEventEscalate(Box<ToolsOzoneModerationDefsModEventEscalate>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventMute")]
  ToolsOzoneModerationDefsModEventMute(Box<ToolsOzoneModerationDefsModEventMute>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventUnmute")]
  ToolsOzoneModerationDefsModEventUnmute(Box<ToolsOzoneModerationDefsModEventUnmute>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventMuteReporter")]
  ToolsOzoneModerationDefsModEventMuteReporter(Box<ToolsOzoneModerationDefsModEventMuteReporter>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventUnmuteReporter")]
  ToolsOzoneModerationDefsModEventUnmuteReporter(
    Box<ToolsOzoneModerationDefsModEventUnmuteReporter>,
  ),
  #[serde(rename = "tools.ozone.moderation.defs#modEventEmail")]
  ToolsOzoneModerationDefsModEventEmail(Box<ToolsOzoneModerationDefsModEventEmail>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventResolveAppeal")]
  ToolsOzoneModerationDefsModEventResolveAppeal(Box<ToolsOzoneModerationDefsModEventResolveAppeal>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventDivert")]
  ToolsOzoneModerationDefsModEventDivert(Box<ToolsOzoneModerationDefsModEventDivert>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventTag")]
  ToolsOzoneModerationDefsModEventTag(Box<ToolsOzoneModerationDefsModEventTag>),
  #[serde(rename = "tools.ozone.moderation.defs#accountEvent")]
  ToolsOzoneModerationDefsAccountEvent(Box<ToolsOzoneModerationDefsAccountEvent>),
  #[serde(rename = "tools.ozone.moderation.defs#identityEvent")]
  ToolsOzoneModerationDefsIdentityEvent(Box<ToolsOzoneModerationDefsIdentityEvent>),
  #[serde(rename = "tools.ozone.moderation.defs#recordEvent")]
  ToolsOzoneModerationDefsRecordEvent(Box<ToolsOzoneModerationDefsRecordEvent>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ToolsOzoneModerationDefsModEventViewSubjectUnion {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsRepoRef(Box<ComAtprotoAdminDefsRepoRef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongRef(Box<ComAtprotoRepoStrongRef>),
  #[serde(rename = "chat.bsky.convo.defs#messageRef")]
  ChatBskyConvoDefsMessageRef(Box<ChatBskyConvoDefsMessageRef>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventView {
  pub id: i64,
  pub event: ToolsOzoneModerationDefsModEventViewEventUnion,
  pub subject: ToolsOzoneModerationDefsModEventViewSubjectUnion,
  pub subject_blob_cids: Vec<String>,
  /// [format: did]
  pub created_by: String,
  /// [format: datetime]
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub creator_handle: Option<String>,
  pub subject_handle: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ToolsOzoneModerationDefsModEventViewDetailEventUnion {
  #[serde(rename = "tools.ozone.moderation.defs#modEventTakedown")]
  ToolsOzoneModerationDefsModEventTakedown(Box<ToolsOzoneModerationDefsModEventTakedown>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventReverseTakedown")]
  ToolsOzoneModerationDefsModEventReverseTakedown(
    Box<ToolsOzoneModerationDefsModEventReverseTakedown>,
  ),
  #[serde(rename = "tools.ozone.moderation.defs#modEventComment")]
  ToolsOzoneModerationDefsModEventComment(Box<ToolsOzoneModerationDefsModEventComment>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventReport")]
  ToolsOzoneModerationDefsModEventReport(Box<ToolsOzoneModerationDefsModEventReport>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventLabel")]
  ToolsOzoneModerationDefsModEventLabel(Box<ToolsOzoneModerationDefsModEventLabel>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventAcknowledge")]
  ToolsOzoneModerationDefsModEventAcknowledge(Box<ToolsOzoneModerationDefsModEventAcknowledge>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventEscalate")]
  ToolsOzoneModerationDefsModEventEscalate(Box<ToolsOzoneModerationDefsModEventEscalate>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventMute")]
  ToolsOzoneModerationDefsModEventMute(Box<ToolsOzoneModerationDefsModEventMute>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventUnmute")]
  ToolsOzoneModerationDefsModEventUnmute(Box<ToolsOzoneModerationDefsModEventUnmute>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventMuteReporter")]
  ToolsOzoneModerationDefsModEventMuteReporter(Box<ToolsOzoneModerationDefsModEventMuteReporter>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventUnmuteReporter")]
  ToolsOzoneModerationDefsModEventUnmuteReporter(
    Box<ToolsOzoneModerationDefsModEventUnmuteReporter>,
  ),
  #[serde(rename = "tools.ozone.moderation.defs#modEventEmail")]
  ToolsOzoneModerationDefsModEventEmail(Box<ToolsOzoneModerationDefsModEventEmail>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventResolveAppeal")]
  ToolsOzoneModerationDefsModEventResolveAppeal(Box<ToolsOzoneModerationDefsModEventResolveAppeal>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventDivert")]
  ToolsOzoneModerationDefsModEventDivert(Box<ToolsOzoneModerationDefsModEventDivert>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventTag")]
  ToolsOzoneModerationDefsModEventTag(Box<ToolsOzoneModerationDefsModEventTag>),
  #[serde(rename = "tools.ozone.moderation.defs#accountEvent")]
  ToolsOzoneModerationDefsAccountEvent(Box<ToolsOzoneModerationDefsAccountEvent>),
  #[serde(rename = "tools.ozone.moderation.defs#identityEvent")]
  ToolsOzoneModerationDefsIdentityEvent(Box<ToolsOzoneModerationDefsIdentityEvent>),
  #[serde(rename = "tools.ozone.moderation.defs#recordEvent")]
  ToolsOzoneModerationDefsRecordEvent(Box<ToolsOzoneModerationDefsRecordEvent>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ToolsOzoneModerationDefsModEventViewDetailSubjectUnion {
  #[serde(rename = "tools.ozone.moderation.defs#repoView")]
  ToolsOzoneModerationDefsRepoView(Box<ToolsOzoneModerationDefsRepoView>),
  #[serde(rename = "tools.ozone.moderation.defs#repoViewNotFound")]
  ToolsOzoneModerationDefsRepoViewNotFound(Box<ToolsOzoneModerationDefsRepoViewNotFound>),
  #[serde(rename = "tools.ozone.moderation.defs#recordView")]
  ToolsOzoneModerationDefsRecordView(Box<ToolsOzoneModerationDefsRecordView>),
  #[serde(rename = "tools.ozone.moderation.defs#recordViewNotFound")]
  ToolsOzoneModerationDefsRecordViewNotFound(Box<ToolsOzoneModerationDefsRecordViewNotFound>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventViewDetail {
  pub id: i64,
  pub event: ToolsOzoneModerationDefsModEventViewDetailEventUnion,
  pub subject: ToolsOzoneModerationDefsModEventViewDetailSubjectUnion,
  pub subject_blobs: Vec<ToolsOzoneModerationDefsBlobView>,
  /// [format: did]
  pub created_by: String,
  /// [format: datetime]
  pub created_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ToolsOzoneModerationDefsSubjectStatusViewSubjectUnion {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsRepoRef(Box<ComAtprotoAdminDefsRepoRef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongRef(Box<ComAtprotoRepoStrongRef>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ToolsOzoneModerationDefsSubjectStatusViewHostingUnion {
  #[serde(rename = "tools.ozone.moderation.defs#accountHosting")]
  ToolsOzoneModerationDefsAccountHosting(Box<ToolsOzoneModerationDefsAccountHosting>),
  #[serde(rename = "tools.ozone.moderation.defs#recordHosting")]
  ToolsOzoneModerationDefsRecordHosting(Box<ToolsOzoneModerationDefsRecordHosting>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsSubjectStatusView {
  pub id: i64,
  pub subject: ToolsOzoneModerationDefsSubjectStatusViewSubjectUnion,
  pub hosting: Option<ToolsOzoneModerationDefsSubjectStatusViewHostingUnion>,
  pub subject_blob_cids: Option<Vec<String>>,
  pub subject_repo_handle: Option<String>,
  /// [format: datetime] Timestamp referencing when the last update was made to the moderation status of the subject
  pub updated_at: chrono::DateTime<chrono::Utc>,
  /// [format: datetime] Timestamp referencing the first moderation status impacting event was emitted on the subject
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub review_state: ToolsOzoneModerationDefsSubjectReviewState,
  /// Sticky comment on the subject.
  pub comment: Option<String>,
  /// [format: datetime]
  pub mute_until: Option<chrono::DateTime<chrono::Utc>>,
  /// [format: datetime]
  pub mute_reporting_until: Option<chrono::DateTime<chrono::Utc>>,
  /// [format: did]
  pub last_reviewed_by: Option<String>,
  /// [format: datetime]
  pub last_reviewed_at: Option<chrono::DateTime<chrono::Utc>>,
  /// [format: datetime]
  pub last_reported_at: Option<chrono::DateTime<chrono::Utc>>,
  /// [format: datetime] Timestamp referencing when the author of the subject appealed a moderation action
  pub last_appealed_at: Option<chrono::DateTime<chrono::Utc>>,
  pub takendown: Option<bool>,
  /// True indicates that the a previously taken moderator action was appealed against, by the author of the content. False indicates last appeal was resolved by moderators.
  pub appealed: Option<bool>,
  /// [format: datetime]
  pub suspend_until: Option<chrono::DateTime<chrono::Utc>>,
  pub tags: Option<Vec<String>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolsOzoneModerationDefsSubjectReviewState(pub String);

/// Take down a subject permanently or temporarily
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventTakedown {
  pub comment: Option<String>,
  /// Indicates how long the takedown should be in effect before automatically expiring.
  pub duration_in_hours: Option<i64>,
  /// If true, all other reports on content authored by this account will be resolved (acknowledged).
  pub acknowledge_account_subjects: Option<bool>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Revert take down action on a subject
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventReverseTakedown {
  /// Describe reasoning behind the reversal.
  pub comment: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Resolve appeal on a subject
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventResolveAppeal {
  /// Describe resolution.
  pub comment: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Add a comment to a subject
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventComment {
  pub comment: String,
  /// Make the comment persistent on the subject
  pub sticky: Option<bool>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Report a subject
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventReport {
  pub comment: Option<String>,
  /// Set to true if the reporter was muted from reporting at the time of the event. These reports won't impact the reviewState of the subject.
  pub is_reporter_muted: Option<bool>,
  pub report_type: ComAtprotoModerationDefsReasonType,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Apply/Negate labels on a subject
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventLabel {
  pub comment: Option<String>,
  pub create_label_vals: Vec<String>,
  pub negate_label_vals: Vec<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventAcknowledge {
  pub comment: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventEscalate {
  pub comment: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Mute incoming reports on a subject
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventMute {
  pub comment: Option<String>,
  /// Indicates how long the subject should remain muted.
  pub duration_in_hours: i64,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Unmute action on a subject
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventUnmute {
  /// Describe reasoning behind the reversal.
  pub comment: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Mute incoming reports from an account
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventMuteReporter {
  pub comment: Option<String>,
  /// Indicates how long the account should remain muted. Falsy value here means a permanent mute.
  pub duration_in_hours: Option<i64>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Unmute incoming reports from an account
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventUnmuteReporter {
  /// Describe reasoning behind the reversal.
  pub comment: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Keep a log of outgoing email to a user
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventEmail {
  /// The subject line of the email sent to the user.
  pub subject_line: String,
  /// The content of the email sent to the user.
  pub content: Option<String>,
  /// Additional comment about the outgoing comm.
  pub comment: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Divert a record's blobs to a 3rd party service for further scanning/tagging
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventDivert {
  pub comment: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Add/Remove a tag on a subject
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventTag {
  /// Tags to be added to the subject. If already exists, won't be duplicated.
  pub add: Vec<String>,
  /// Tags to be removed to the subject. Ignores a tag If it doesn't exist, won't be duplicated.
  pub remove: Vec<String>,
  /// Additional comment about added/removed tags.
  pub comment: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Logs account status related events on a repo subject. Normally captured by automod from the firehose and emitted to ozone for historical tracking.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsAccountEvent {
  pub comment: Option<String>,
  /// Indicates that the account has a repository which can be fetched from the host that emitted this event.
  pub active: bool,
  /// [known_values: ["unknown", "deactivated", "deleted", "takendown", "suspended", "tombstoned"]]
  pub status: Option<String>,
  /// [format: datetime]
  pub timestamp: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Logs identity related events on a repo subject. Normally captured by automod from the firehose and emitted to ozone for historical tracking.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsIdentityEvent {
  pub comment: Option<String>,
  /// [format: handle]
  pub handle: Option<String>,
  /// [format: uri]
  pub pds_host: Option<String>,
  pub tombstone: Option<bool>,
  /// [format: datetime]
  pub timestamp: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Logs lifecycle event on a record subject. Normally captured by automod from the firehose and emitted to ozone for historical tracking.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsRecordEvent {
  pub comment: Option<String>,
  /// [known_values: ["create", "update", "delete"]]
  pub op: String,
  /// [format: cid]
  pub cid: Option<String>,
  /// [format: datetime]
  pub timestamp: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsRepoView {
  /// [format: did]
  pub did: String,
  /// [format: handle]
  pub handle: String,
  pub email: Option<String>,
  pub related_records: Vec<serde_json::Value>,
  /// [format: datetime]
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  pub moderation: ToolsOzoneModerationDefsModeration,
  pub invited_by: Option<ComAtprotoServerDefsInviteCode>,
  pub invites_disabled: Option<bool>,
  pub invite_note: Option<String>,
  /// [format: datetime]
  pub deactivated_at: Option<chrono::DateTime<chrono::Utc>>,
  pub threat_signatures: Option<Vec<ComAtprotoAdminDefsThreatSignature>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsRepoViewDetail {
  /// [format: did]
  pub did: String,
  /// [format: handle]
  pub handle: String,
  pub email: Option<String>,
  pub related_records: Vec<serde_json::Value>,
  /// [format: datetime]
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  pub moderation: ToolsOzoneModerationDefsModerationDetail,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  pub invited_by: Option<ComAtprotoServerDefsInviteCode>,
  pub invites: Option<Vec<ComAtprotoServerDefsInviteCode>>,
  pub invites_disabled: Option<bool>,
  pub invite_note: Option<String>,
  /// [format: datetime]
  pub email_confirmed_at: Option<chrono::DateTime<chrono::Utc>>,
  /// [format: datetime]
  pub deactivated_at: Option<chrono::DateTime<chrono::Utc>>,
  pub threat_signatures: Option<Vec<ComAtprotoAdminDefsThreatSignature>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsRepoViewNotFound {
  /// [format: did]
  pub did: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsRecordView {
  /// [format: at-uri]
  pub uri: String,
  /// [format: cid]
  pub cid: String,
  pub value: serde_json::Value,
  pub blob_cids: Vec<String>,
  /// [format: datetime]
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  pub moderation: ToolsOzoneModerationDefsModeration,
  pub repo: ToolsOzoneModerationDefsRepoView,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsRecordViewDetail {
  /// [format: at-uri]
  pub uri: String,
  /// [format: cid]
  pub cid: String,
  pub value: serde_json::Value,
  pub blobs: Vec<ToolsOzoneModerationDefsBlobView>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  /// [format: datetime]
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  pub moderation: ToolsOzoneModerationDefsModerationDetail,
  pub repo: ToolsOzoneModerationDefsRepoView,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsRecordViewNotFound {
  /// [format: at-uri]
  pub uri: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModeration {
  pub subject_status: Option<ToolsOzoneModerationDefsSubjectStatusView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModerationDetail {
  pub subject_status: Option<ToolsOzoneModerationDefsSubjectStatusView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ToolsOzoneModerationDefsBlobViewDetailsUnion {
  #[serde(rename = "tools.ozone.moderation.defs#imageDetails")]
  ToolsOzoneModerationDefsImageDetails(Box<ToolsOzoneModerationDefsImageDetails>),
  #[serde(rename = "tools.ozone.moderation.defs#videoDetails")]
  ToolsOzoneModerationDefsVideoDetails(Box<ToolsOzoneModerationDefsVideoDetails>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsBlobView {
  /// [format: cid]
  pub cid: String,
  pub mime_type: String,
  pub size: i64,
  /// [format: datetime]
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub details: Option<ToolsOzoneModerationDefsBlobViewDetailsUnion>,
  pub moderation: Option<ToolsOzoneModerationDefsModeration>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsImageDetails {
  pub width: i64,
  pub height: i64,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsVideoDetails {
  pub width: i64,
  pub height: i64,
  pub length: i64,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsAccountHosting {
  /// [known_values: ["takendown", "suspended", "deleted", "deactivated", "unknown"]]
  pub status: String,
  /// [format: datetime]
  pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
  /// [format: datetime]
  pub created_at: Option<chrono::DateTime<chrono::Utc>>,
  /// [format: datetime]
  pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
  /// [format: datetime]
  pub deactivated_at: Option<chrono::DateTime<chrono::Utc>>,
  /// [format: datetime]
  pub reactivated_at: Option<chrono::DateTime<chrono::Utc>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsRecordHosting {
  /// [known_values: ["deleted", "unknown"]]
  pub status: String,
  /// [format: datetime]
  pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
  /// [format: datetime]
  pub created_at: Option<chrono::DateTime<chrono::Utc>>,
  /// [format: datetime]
  pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ToolsOzoneModerationEmitEventInputEventUnion {
  #[serde(rename = "tools.ozone.moderation.defs#modEventTakedown")]
  ToolsOzoneModerationDefsModEventTakedown(Box<ToolsOzoneModerationDefsModEventTakedown>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventAcknowledge")]
  ToolsOzoneModerationDefsModEventAcknowledge(Box<ToolsOzoneModerationDefsModEventAcknowledge>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventEscalate")]
  ToolsOzoneModerationDefsModEventEscalate(Box<ToolsOzoneModerationDefsModEventEscalate>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventComment")]
  ToolsOzoneModerationDefsModEventComment(Box<ToolsOzoneModerationDefsModEventComment>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventLabel")]
  ToolsOzoneModerationDefsModEventLabel(Box<ToolsOzoneModerationDefsModEventLabel>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventReport")]
  ToolsOzoneModerationDefsModEventReport(Box<ToolsOzoneModerationDefsModEventReport>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventMute")]
  ToolsOzoneModerationDefsModEventMute(Box<ToolsOzoneModerationDefsModEventMute>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventUnmute")]
  ToolsOzoneModerationDefsModEventUnmute(Box<ToolsOzoneModerationDefsModEventUnmute>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventMuteReporter")]
  ToolsOzoneModerationDefsModEventMuteReporter(Box<ToolsOzoneModerationDefsModEventMuteReporter>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventUnmuteReporter")]
  ToolsOzoneModerationDefsModEventUnmuteReporter(
    Box<ToolsOzoneModerationDefsModEventUnmuteReporter>,
  ),
  #[serde(rename = "tools.ozone.moderation.defs#modEventReverseTakedown")]
  ToolsOzoneModerationDefsModEventReverseTakedown(
    Box<ToolsOzoneModerationDefsModEventReverseTakedown>,
  ),
  #[serde(rename = "tools.ozone.moderation.defs#modEventResolveAppeal")]
  ToolsOzoneModerationDefsModEventResolveAppeal(Box<ToolsOzoneModerationDefsModEventResolveAppeal>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventEmail")]
  ToolsOzoneModerationDefsModEventEmail(Box<ToolsOzoneModerationDefsModEventEmail>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventTag")]
  ToolsOzoneModerationDefsModEventTag(Box<ToolsOzoneModerationDefsModEventTag>),
  #[serde(rename = "tools.ozone.moderation.defs#accountEvent")]
  ToolsOzoneModerationDefsAccountEvent(Box<ToolsOzoneModerationDefsAccountEvent>),
  #[serde(rename = "tools.ozone.moderation.defs#identityEvent")]
  ToolsOzoneModerationDefsIdentityEvent(Box<ToolsOzoneModerationDefsIdentityEvent>),
  #[serde(rename = "tools.ozone.moderation.defs#recordEvent")]
  ToolsOzoneModerationDefsRecordEvent(Box<ToolsOzoneModerationDefsRecordEvent>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ToolsOzoneModerationEmitEventInputSubjectUnion {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsRepoRef(Box<ComAtprotoAdminDefsRepoRef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongRef(Box<ComAtprotoRepoStrongRef>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationEmitEventInput {
  pub event: ToolsOzoneModerationEmitEventInputEventUnion,
  pub subject: ToolsOzoneModerationEmitEventInputSubjectUnion,
  pub subject_blob_cids: Option<Vec<String>>,
  /// [format: did]
  pub created_by: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ToolsOzoneModerationGetRecordsOutputRecordsUnion {
  #[serde(rename = "tools.ozone.moderation.defs#recordViewDetail")]
  ToolsOzoneModerationDefsRecordViewDetail(Box<ToolsOzoneModerationDefsRecordViewDetail>),
  #[serde(rename = "tools.ozone.moderation.defs#recordViewNotFound")]
  ToolsOzoneModerationDefsRecordViewNotFound(Box<ToolsOzoneModerationDefsRecordViewNotFound>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationGetRecordsOutput {
  pub records: Vec<ToolsOzoneModerationGetRecordsOutputRecordsUnion>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ToolsOzoneModerationGetReposOutputReposUnion {
  #[serde(rename = "tools.ozone.moderation.defs#repoViewDetail")]
  ToolsOzoneModerationDefsRepoViewDetail(Box<ToolsOzoneModerationDefsRepoViewDetail>),
  #[serde(rename = "tools.ozone.moderation.defs#repoViewNotFound")]
  ToolsOzoneModerationDefsRepoViewNotFound(Box<ToolsOzoneModerationDefsRepoViewNotFound>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationGetReposOutput {
  pub repos: Vec<ToolsOzoneModerationGetReposOutputReposUnion>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationQueryEventsOutput {
  pub cursor: Option<String>,
  pub events: Vec<ToolsOzoneModerationDefsModEventView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationQueryStatusesOutput {
  pub cursor: Option<String>,
  pub subject_statuses: Vec<ToolsOzoneModerationDefsSubjectStatusView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationSearchReposOutput {
  pub cursor: Option<String>,
  pub repos: Vec<ToolsOzoneModerationDefsRepoView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneServerGetConfigOutput {
  pub appview: Option<ToolsOzoneServerGetConfigServiceConfig>,
  pub pds: Option<ToolsOzoneServerGetConfigServiceConfig>,
  pub blob_divert: Option<ToolsOzoneServerGetConfigServiceConfig>,
  pub chat: Option<ToolsOzoneServerGetConfigServiceConfig>,
  pub viewer: Option<ToolsOzoneServerGetConfigViewerConfig>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneServerGetConfigServiceConfig {
  /// [format: uri]
  pub url: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneServerGetConfigViewerConfig {
  /// [known_values: ["tools.ozone.team.defs#roleAdmin", "tools.ozone.team.defs#roleModerator", "tools.ozone.team.defs#roleTriage"]]
  pub role: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneSetAddValuesInput {
  /// Name of the set to add values to
  pub name: String,
  /// [min_length: 1] [max_length: 1000] Array of string values to add to the set
  pub values: Vec<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneSetDefsSet {
  /// [max_length: 128] [min_length: 3]
  pub name: String,
  /// [max_graphemes: 1024] [max_length: 10240]
  pub description: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneSetDefsSetView {
  /// [max_length: 128] [min_length: 3]
  pub name: String,
  /// [max_graphemes: 1024] [max_length: 10240]
  pub description: Option<String>,
  pub set_size: i64,
  /// [format: datetime]
  pub created_at: chrono::DateTime<chrono::Utc>,
  /// [format: datetime]
  pub updated_at: chrono::DateTime<chrono::Utc>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneSetDeleteSetInput {
  /// Name of the set to delete
  pub name: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolsOzoneSetDeleteSetOutput(pub serde_json::Value);

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneSetDeleteValuesInput {
  /// Name of the set to delete values from
  pub name: String,
  /// [min_length: 1] Array of string values to delete from the set
  pub values: Vec<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneSetGetValuesOutput {
  pub set: ToolsOzoneSetDefsSetView,
  pub values: Vec<String>,
  pub cursor: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneSetQuerySetsOutput {
  pub sets: Vec<ToolsOzoneSetDefsSetView>,
  pub cursor: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneSettingDefsOption {
  /// [format: nsid]
  pub key: String,
  /// [format: did]
  pub did: String,
  pub value: serde_json::Value,
  /// [max_graphemes: 1024] [max_length: 10240]
  pub description: Option<String>,
  /// [format: datetime]
  pub created_at: Option<chrono::DateTime<chrono::Utc>>,
  /// [format: datetime]
  pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
  /// [known_values: ["tools.ozone.team.defs#roleModerator", "tools.ozone.team.defs#roleTriage", "tools.ozone.team.defs#roleAdmin"]]
  pub manager_role: Option<String>,
  /// [known_values: ["instance", "personal"]]
  pub scope: String,
  /// [format: did]
  pub created_by: String,
  /// [format: did]
  pub last_updated_by: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneSettingListOptionsOutput {
  pub cursor: Option<String>,
  pub options: Vec<ToolsOzoneSettingDefsOption>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneSettingRemoveOptionsInput {
  /// [min_length: 1] [max_length: 200]
  pub keys: Vec<String>,
  /// [known_values: ["instance", "personal"]]
  pub scope: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolsOzoneSettingRemoveOptionsOutput(pub serde_json::Value);

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneSettingUpsertOptionInput {
  /// [format: nsid]
  pub key: String,
  /// [known_values: ["instance", "personal"]]
  pub scope: String,
  pub value: serde_json::Value,
  /// [max_length: 2000]
  pub description: Option<String>,
  /// [known_values: ["tools.ozone.team.defs#roleModerator", "tools.ozone.team.defs#roleTriage", "tools.ozone.team.defs#roleAdmin"]]
  pub manager_role: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneSettingUpsertOptionOutput {
  pub option: ToolsOzoneSettingDefsOption,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneSignatureDefsSigDetail {
  pub property: String,
  pub value: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneSignatureFindCorrelationOutput {
  pub details: Vec<ToolsOzoneSignatureDefsSigDetail>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneSignatureFindRelatedAccountsOutput {
  pub cursor: Option<String>,
  pub accounts: Vec<ToolsOzoneSignatureFindRelatedAccountsRelatedAccount>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneSignatureFindRelatedAccountsRelatedAccount {
  pub account: ComAtprotoAdminDefsAccountView,
  pub similarities: Option<Vec<ToolsOzoneSignatureDefsSigDetail>>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneSignatureSearchAccountsOutput {
  pub cursor: Option<String>,
  pub accounts: Vec<ComAtprotoAdminDefsAccountView>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneTeamAddMemberInput {
  /// [format: did]
  pub did: String,
  /// [known_values: ["tools.ozone.team.defs#roleAdmin", "tools.ozone.team.defs#roleModerator", "tools.ozone.team.defs#roleTriage"]]
  pub role: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneTeamDefsMember {
  /// [format: did]
  pub did: String,
  pub disabled: Option<bool>,
  pub profile: Option<AppBskyActorDefsProfileViewDetailed>,
  /// [format: datetime]
  pub created_at: Option<chrono::DateTime<chrono::Utc>>,
  /// [format: datetime]
  pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
  pub last_updated_by: Option<String>,
  /// [known_values: ["#roleAdmin", "#roleModerator", "#roleTriage"]]
  pub role: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneTeamDeleteMemberInput {
  /// [format: did]
  pub did: String,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneTeamListMembersOutput {
  pub cursor: Option<String>,
  pub members: Vec<ToolsOzoneTeamDefsMember>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneTeamUpdateMemberInput {
  /// [format: did]
  pub did: String,
  pub disabled: Option<bool>,
  /// [known_values: ["tools.ozone.team.defs#roleAdmin", "tools.ozone.team.defs#roleModerator", "tools.ozone.team.defs#roleTriage"]]
  pub role: Option<String>,
  #[serde(flatten)]
  pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// arproto client
#[derive(Debug, Clone)]
pub struct Atproto {
  pub client: reqwest::Client,
  pub host: String,
  pub firehose: String,
  pub access_jwt: std::sync::Arc<tokio::sync::RwLock<Option<String>>>,
  pub refresh_jwt: std::sync::Arc<tokio::sync::RwLock<Option<String>>>,
}

impl Default for Atproto {
  fn default() -> Self {
    Self::new(None, None)
  }
}

impl Atproto {
  /// create new arproto client
  pub fn new(host: Option<&str>, firehose: Option<&str>) -> Self {
    Self {
      client: reqwest::Client::new(),
      host: host
        .map(|h| h.to_string())
        .unwrap_or_else(|| String::from("bsky.social")),
      firehose: firehose
        .map(|h| h.to_string())
        .unwrap_or_else(|| String::from("bsky.network")),
      access_jwt: std::sync::Arc::new(tokio::sync::RwLock::new(None)),
      refresh_jwt: std::sync::Arc::new(tokio::sync::RwLock::new(None)),
    }
  }

  /// logging in to use authenticated apis
  pub async fn login(&mut self, id: &str, pw: &str) -> Result<ComAtprotoServerCreateSessionOutput> {
    let output = self
      .com_atproto_server_create_session(ComAtprotoServerCreateSessionInput {
        identifier: id.to_string(),
        password: pw.to_string(),
        auth_factor_token: None,
        extra: std::collections::HashMap::new(),
      })
      .await?;
    {
      let mut lock = self.access_jwt.write().await;
      *lock = Some(output.access_jwt.clone());
    }
    {
      let mut lock = self.refresh_jwt.write().await;
      *lock = Some(output.refresh_jwt.clone());
    }
    Ok(output)
  }

  /// refresh access token
  pub async fn refresh(&mut self) -> Result<()> {
    let access_jwt = self.access_jwt.clone();
    self.access_jwt = self.refresh_jwt.clone();
    let output = match self.com_atproto_server_refresh_session().await {
      Ok(o) => o,
      Err(e) => {
        self.access_jwt = access_jwt;
        return Err(e);
      }
    };
    {
      let mut lock = self.access_jwt.write().await;
      *lock = Some(output.access_jwt.clone());
    }
    {
      let mut lock = self.refresh_jwt.write().await;
      *lock = Some(output.refresh_jwt.clone());
    }
    Ok(())
  }

  /// Get private preferences attached to the current account. Expected use is synchronization between multiple devices, and import/export during account migration. Requires auth.
  pub async fn app_bsky_actor_get_preferences(&self) -> Result<AppBskyActorGetPreferencesOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/app.bsky.actor.getPreferences",
      self.host
    ));
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get detailed profile view of an actor. Does not require auth, but contains relevant metadata with auth.
  ///
  /// # Arguments
  ///
  /// * `actor` - [format: at-identifier] Handle or DID of account to fetch profile of.
  pub async fn app_bsky_actor_get_profile(
    &self,
    actor: &str,
  ) -> Result<AppBskyActorDefsProfileViewDetailed> {
    let mut query_ = Vec::new();
    query_.push((String::from("actor"), actor.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.actor.getProfile",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get detailed profile views of multiple actors.
  ///
  /// # Arguments
  ///
  /// * `actors` - [max_length: 25]
  pub async fn app_bsky_actor_get_profiles(
    &self,
    actors: &[&str],
  ) -> Result<AppBskyActorGetProfilesOutput> {
    let mut query_ = Vec::new();
    query_.append(
      &mut actors
        .iter()
        .map(|i| (String::from("actors"), i.to_string()))
        .collect::<Vec<_>>(),
    );
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.actor.getProfiles",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a list of suggested actors. Expected use is discovery of accounts to follow during new account onboarding.
  ///
  /// # Arguments
  ///
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  pub async fn app_bsky_actor_get_suggestions(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyActorGetSuggestionsOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.actor.getSuggestions",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Set the private preferences attached to the account.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn app_bsky_actor_put_preferences(
    &self,
    body: AppBskyActorPutPreferencesInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/app.bsky.actor.putPreferences",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Find actors (profiles) matching search criteria. Does not require auth.
  ///
  /// # Arguments
  ///
  /// * `term` - DEPRECATED: use 'q' instead.
  /// * `q` - Search query string. Syntax, phrase, boolean, and faceting is unspecified, but Lucene query syntax is recommended.
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 25]
  /// * `cursor`
  pub async fn app_bsky_actor_search_actors(
    &self,
    term: Option<&str>,
    q: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyActorSearchActorsOutput> {
    let mut query_ = Vec::new();
    if let Some(term) = &term {
      query_.push((String::from("term"), term.to_string()));
    }
    if let Some(q) = &q {
      query_.push((String::from("q"), q.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.actor.searchActors",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Find actor suggestions for a prefix search term. Expected use is for auto-completion during text field entry. Does not require auth.
  ///
  /// # Arguments
  ///
  /// * `term` - DEPRECATED: use 'q' instead.
  /// * `q` - Search query prefix; not a full query string.
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 10]
  pub async fn app_bsky_actor_search_actors_typeahead(
    &self,
    term: Option<&str>,
    q: Option<&str>,
    limit: Option<i64>,
  ) -> Result<AppBskyActorSearchActorsTypeaheadOutput> {
    let mut query_ = Vec::new();
    if let Some(term) = &term {
      query_.push((String::from("term"), term.to_string()));
    }
    if let Some(q) = &q {
      query_.push((String::from("q"), q.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.actor.searchActorsTypeahead",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get information about a feed generator, including policies and offered feed URIs. Does not require auth; implemented by Feed Generator services (not App View).
  pub async fn app_bsky_feed_describe_feed_generator(
    &self,
  ) -> Result<AppBskyFeedDescribeFeedGeneratorOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/app.bsky.feed.describeFeedGenerator",
      self.host
    ));
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a list of feeds (feed generator records) created by the actor (in the actor's repo).
  ///
  /// # Arguments
  ///
  /// * `actor` - [format: at-identifier]
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  pub async fn app_bsky_feed_get_actor_feeds(
    &self,
    actor: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetActorFeedsOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("actor"), actor.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getActorFeeds",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a list of posts liked by an actor. Requires auth, actor must be the requesting account.
  ///
  /// # Arguments
  ///
  /// * `actor` - [format: at-identifier]
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  ///
  /// # Errors
  ///
  /// * `BlockedActor`
  /// * `BlockedByActor`
  pub async fn app_bsky_feed_get_actor_likes(
    &self,
    actor: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetActorLikesOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("actor"), actor.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getActorLikes",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a view of an actor's 'author feed' (post and reposts by the author). Does not require auth.
  ///
  /// # Arguments
  ///
  /// * `actor` - [format: at-identifier]
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  /// * `filter` - [known_values: ["posts_with_replies", "posts_no_replies", "posts_with_media", "posts_and_author_threads"]] [default: posts_with_replies] Combinations of post/repost types to include in response.
  /// * `include_pins` - [default: false]
  ///
  /// # Errors
  ///
  /// * `BlockedActor`
  /// * `BlockedByActor`
  pub async fn app_bsky_feed_get_author_feed(
    &self,
    actor: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
    filter: Option<&str>,
    include_pins: Option<bool>,
  ) -> Result<AppBskyFeedGetAuthorFeedOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("actor"), actor.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    if let Some(filter) = &filter {
      query_.push((String::from("filter"), filter.to_string()));
    }
    if let Some(include_pins) = &include_pins {
      query_.push((String::from("include_pins"), include_pins.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getAuthorFeed",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a hydrated feed from an actor's selected feed generator. Implemented by App View.
  ///
  /// # Arguments
  ///
  /// * `feed` - [format: at-uri]
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  ///
  /// # Errors
  ///
  /// * `UnknownFeed`
  pub async fn app_bsky_feed_get_feed(
    &self,
    feed: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetFeedOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("feed"), feed.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!("https://{}/xrpc/app.bsky.feed.getFeed", self.host))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get information about a feed generator. Implemented by AppView.
  ///
  /// # Arguments
  ///
  /// * `feed` - [format: at-uri] AT-URI of the feed generator record.
  pub async fn app_bsky_feed_get_feed_generator(
    &self,
    feed: &str,
  ) -> Result<AppBskyFeedGetFeedGeneratorOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("feed"), feed.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getFeedGenerator",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get information about a list of feed generators.
  ///
  /// # Arguments
  ///
  /// * `feeds`
  pub async fn app_bsky_feed_get_feed_generators(
    &self,
    feeds: &[&str],
  ) -> Result<AppBskyFeedGetFeedGeneratorsOutput> {
    let mut query_ = Vec::new();
    query_.append(
      &mut feeds
        .iter()
        .map(|i| (String::from("feeds"), i.to_string()))
        .collect::<Vec<_>>(),
    );
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getFeedGenerators",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a skeleton of a feed provided by a feed generator. Auth is optional, depending on provider requirements, and provides the DID of the requester. Implemented by Feed Generator Service.
  ///
  /// # Arguments
  ///
  /// * `feed` - [format: at-uri] Reference to feed generator record describing the specific feed being requested.
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  ///
  /// # Errors
  ///
  /// * `UnknownFeed`
  pub async fn app_bsky_feed_get_feed_skeleton(
    &self,
    feed: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetFeedSkeletonOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("feed"), feed.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getFeedSkeleton",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get like records which reference a subject (by AT-URI and CID).
  ///
  /// # Arguments
  ///
  /// * `uri` - [format: at-uri] AT-URI of the subject (eg, a post record).
  /// * `cid` - [format: cid] CID of the subject record (aka, specific version of record), to filter likes.
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  pub async fn app_bsky_feed_get_likes(
    &self,
    uri: &str,
    cid: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetLikesOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("uri"), uri.to_string()));
    if let Some(cid) = &cid {
      query_.push((String::from("cid"), cid.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getLikes",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a feed of recent posts from a list (posts and reposts from any actors on the list). Does not require auth.
  ///
  /// # Arguments
  ///
  /// * `list` - [format: at-uri] Reference (AT-URI) to the list record.
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  ///
  /// # Errors
  ///
  /// * `UnknownList`
  pub async fn app_bsky_feed_get_list_feed(
    &self,
    list: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetListFeedOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("list"), list.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getListFeed",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get posts in a thread. Does not require auth, but additional metadata and filtering will be applied for authed requests.
  ///
  /// # Arguments
  ///
  /// * `uri` - [format: at-uri] Reference (AT-URI) to post record.
  /// * `depth` - [minimum: 0] [maximum: 1000] [default: 6] How many levels of reply depth should be included in response.
  /// * `parent_height` - [minimum: 0] [maximum: 1000] [default: 80] How many levels of parent (and grandparent, etc) post to include.
  ///
  /// # Errors
  ///
  /// * `NotFound`
  pub async fn app_bsky_feed_get_post_thread(
    &self,
    uri: &str,
    depth: Option<i64>,
    parent_height: Option<i64>,
  ) -> Result<AppBskyFeedGetPostThreadOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("uri"), uri.to_string()));
    if let Some(depth) = &depth {
      query_.push((String::from("depth"), depth.to_string()));
    }
    if let Some(parent_height) = &parent_height {
      query_.push((String::from("parent_height"), parent_height.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getPostThread",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Gets post views for a specified list of posts (by AT-URI). This is sometimes referred to as 'hydrating' a 'feed skeleton'.
  ///
  /// # Arguments
  ///
  /// * `uris` - [max_length: 25] List of post AT-URIs to return hydrated views for.
  pub async fn app_bsky_feed_get_posts(&self, uris: &[&str]) -> Result<AppBskyFeedGetPostsOutput> {
    let mut query_ = Vec::new();
    query_.append(
      &mut uris
        .iter()
        .map(|i| (String::from("uris"), i.to_string()))
        .collect::<Vec<_>>(),
    );
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getPosts",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a list of quotes for a given post.
  ///
  /// # Arguments
  ///
  /// * `uri` - [format: at-uri] Reference (AT-URI) of post record
  /// * `cid` - [format: cid] If supplied, filters to quotes of specific version (by CID) of the post record.
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  pub async fn app_bsky_feed_get_quotes(
    &self,
    uri: &str,
    cid: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetQuotesOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("uri"), uri.to_string()));
    if let Some(cid) = &cid {
      query_.push((String::from("cid"), cid.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getQuotes",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a list of reposts for a given post.
  ///
  /// # Arguments
  ///
  /// * `uri` - [format: at-uri] Reference (AT-URI) of post record
  /// * `cid` - [format: cid] If supplied, filters to reposts of specific version (by CID) of the post record.
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  pub async fn app_bsky_feed_get_reposted_by(
    &self,
    uri: &str,
    cid: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetRepostedByOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("uri"), uri.to_string()));
    if let Some(cid) = &cid {
      query_.push((String::from("cid"), cid.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getRepostedBy",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a list of suggested feeds (feed generators) for the requesting account.
  ///
  /// # Arguments
  ///
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  pub async fn app_bsky_feed_get_suggested_feeds(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetSuggestedFeedsOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getSuggestedFeeds",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a view of the requesting account's home timeline. This is expected to be some form of reverse-chronological feed.
  ///
  /// # Arguments
  ///
  /// * `algorithm` - Variant 'algorithm' for timeline. Implementation-specific. NOTE: most feed flexibility has been moved to feed generator mechanism.
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  pub async fn app_bsky_feed_get_timeline(
    &self,
    algorithm: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetTimelineOutput> {
    let mut query_ = Vec::new();
    if let Some(algorithm) = &algorithm {
      query_.push((String::from("algorithm"), algorithm.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getTimeline",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Find posts matching search criteria, returning views of those posts.
  ///
  /// # Arguments
  ///
  /// * `q` - Search query string; syntax, phrase, boolean, and faceting is unspecified, but Lucene query syntax is recommended.
  /// * `sort` - [known_values: ["top", "latest"]] [default: latest] Specifies the ranking order of results.
  /// * `since` - Filter results for posts after the indicated datetime (inclusive). Expected to use 'sortAt' timestamp, which may not match 'createdAt'. Can be a datetime, or just an ISO date (YYYY-MM-DD).
  /// * `until` - Filter results for posts before the indicated datetime (not inclusive). Expected to use 'sortAt' timestamp, which may not match 'createdAt'. Can be a datetime, or just an ISO date (YYY-MM-DD).
  /// * `mentions` - [format: at-identifier] Filter to posts which mention the given account. Handles are resolved to DID before query-time. Only matches rich-text facet mentions.
  /// * `author` - [format: at-identifier] Filter to posts by the given account. Handles are resolved to DID before query-time.
  /// * `lang` - [format: language] Filter to posts in the given language. Expected to be based on post language field, though server may override language detection.
  /// * `domain` - Filter to posts with URLs (facet links or embeds) linking to the given domain (hostname). Server may apply hostname normalization.
  /// * `url` - [format: uri] Filter to posts with links (facet links or embeds) pointing to this URL. Server may apply URL normalization or fuzzy matching.
  /// * `tag` - Filter to posts with the given tag (hashtag), based on rich-text facet or tag field. Do not include the hash (#) prefix. Multiple tags can be specified, with 'AND' matching.
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 25]
  /// * `cursor` - Optional pagination mechanism; may not necessarily allow scrolling through entire result set.
  ///
  /// # Errors
  ///
  /// * `BadQueryString`
  pub async fn app_bsky_feed_search_posts(
    &self,
    q: &str,
    sort: Option<&str>,
    since: Option<&str>,
    until: Option<&str>,
    mentions: Option<&str>,
    author: Option<&str>,
    lang: Option<&str>,
    domain: Option<&str>,
    url: Option<&str>,
    tag: Option<&[&str]>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedSearchPostsOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("q"), q.to_string()));
    if let Some(sort) = &sort {
      query_.push((String::from("sort"), sort.to_string()));
    }
    if let Some(since) = &since {
      query_.push((String::from("since"), since.to_string()));
    }
    if let Some(until) = &until {
      query_.push((String::from("until"), until.to_string()));
    }
    if let Some(mentions) = &mentions {
      query_.push((String::from("mentions"), mentions.to_string()));
    }
    if let Some(author) = &author {
      query_.push((String::from("author"), author.to_string()));
    }
    if let Some(lang) = &lang {
      query_.push((String::from("lang"), lang.to_string()));
    }
    if let Some(domain) = &domain {
      query_.push((String::from("domain"), domain.to_string()));
    }
    if let Some(url) = &url {
      query_.push((String::from("url"), url.to_string()));
    }
    if let Some(tag) = &tag {
      query_.append(
        &mut tag
          .iter()
          .map(|i| (String::from("tag"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.searchPosts",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Send information about interactions with feed items back to the feed generator that served them.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn app_bsky_feed_send_interactions(
    &self,
    body: AppBskyFeedSendInteractionsInput,
  ) -> Result<serde_json::Value> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/app.bsky.feed.sendInteractions",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a list of starter packs created by the actor.
  ///
  /// # Arguments
  ///
  /// * `actor` - [format: at-identifier]
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  pub async fn app_bsky_graph_get_actor_starter_packs(
    &self,
    actor: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetActorStarterPacksOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("actor"), actor.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getActorStarterPacks",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Enumerates which accounts the requesting account is currently blocking. Requires auth.
  ///
  /// # Arguments
  ///
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  pub async fn app_bsky_graph_get_blocks(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetBlocksOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getBlocks",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Enumerates accounts which follow a specified account (actor).
  ///
  /// # Arguments
  ///
  /// * `actor` - [format: at-identifier]
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  pub async fn app_bsky_graph_get_followers(
    &self,
    actor: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetFollowersOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("actor"), actor.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getFollowers",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Enumerates accounts which a specified account (actor) follows.
  ///
  /// # Arguments
  ///
  /// * `actor` - [format: at-identifier]
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  pub async fn app_bsky_graph_get_follows(
    &self,
    actor: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetFollowsOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("actor"), actor.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getFollows",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Enumerates accounts which follow a specified account (actor) and are followed by the viewer.
  ///
  /// # Arguments
  ///
  /// * `actor` - [format: at-identifier]
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  pub async fn app_bsky_graph_get_known_followers(
    &self,
    actor: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetKnownFollowersOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("actor"), actor.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getKnownFollowers",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Gets a 'view' (with additional context) of a specified list.
  ///
  /// # Arguments
  ///
  /// * `list` - [format: at-uri] Reference (AT-URI) of the list record to hydrate.
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  pub async fn app_bsky_graph_get_list(
    &self,
    list: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetListOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("list"), list.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getList",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get mod lists that the requesting account (actor) is blocking. Requires auth.
  ///
  /// # Arguments
  ///
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  pub async fn app_bsky_graph_get_list_blocks(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetListBlocksOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getListBlocks",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Enumerates mod lists that the requesting account (actor) currently has muted. Requires auth.
  ///
  /// # Arguments
  ///
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  pub async fn app_bsky_graph_get_list_mutes(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetListMutesOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getListMutes",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Enumerates the lists created by a specified account (actor).
  ///
  /// # Arguments
  ///
  /// * `actor` - [format: at-identifier] The account (actor) to enumerate lists from.
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  pub async fn app_bsky_graph_get_lists(
    &self,
    actor: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetListsOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("actor"), actor.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getLists",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Enumerates accounts that the requesting account (actor) currently has muted. Requires auth.
  ///
  /// # Arguments
  ///
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  pub async fn app_bsky_graph_get_mutes(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetMutesOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getMutes",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Enumerates public relationships between one account, and a list of other accounts. Does not require auth.
  ///
  /// # Arguments
  ///
  /// * `actor` - [format: at-identifier] Primary account requesting relationships for.
  /// * `others` - [max_length: 30] List of 'other' accounts to be related back to the primary.
  ///
  /// # Errors
  ///
  /// * `ActorNotFound` - the primary actor at-identifier could not be resolved
  pub async fn app_bsky_graph_get_relationships(
    &self,
    actor: &str,
    others: Option<&[&str]>,
  ) -> Result<AppBskyGraphGetRelationshipsOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("actor"), actor.to_string()));
    if let Some(others) = &others {
      query_.append(
        &mut others
          .iter()
          .map(|i| (String::from("others"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getRelationships",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Gets a view of a starter pack.
  ///
  /// # Arguments
  ///
  /// * `starter_pack` - [format: at-uri] Reference (AT-URI) of the starter pack record.
  pub async fn app_bsky_graph_get_starter_pack(
    &self,
    starter_pack: &str,
  ) -> Result<AppBskyGraphGetStarterPackOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("starter_pack"), starter_pack.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getStarterPack",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get views for a list of starter packs.
  ///
  /// # Arguments
  ///
  /// * `uris` - [max_length: 25]
  pub async fn app_bsky_graph_get_starter_packs(
    &self,
    uris: &[&str],
  ) -> Result<AppBskyGraphGetStarterPacksOutput> {
    let mut query_ = Vec::new();
    query_.append(
      &mut uris
        .iter()
        .map(|i| (String::from("uris"), i.to_string()))
        .collect::<Vec<_>>(),
    );
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getStarterPacks",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Enumerates follows similar to a given account (actor). Expected use is to recommend additional accounts immediately after following one account.
  ///
  /// # Arguments
  ///
  /// * `actor` - [format: at-identifier]
  pub async fn app_bsky_graph_get_suggested_follows_by_actor(
    &self,
    actor: &str,
  ) -> Result<AppBskyGraphGetSuggestedFollowsByActorOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("actor"), actor.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getSuggestedFollowsByActor",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Creates a mute relationship for the specified account. Mutes are private in Bluesky. Requires auth.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn app_bsky_graph_mute_actor(&self, body: AppBskyGraphMuteActorInput) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/app.bsky.graph.muteActor",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Creates a mute relationship for the specified list of accounts. Mutes are private in Bluesky. Requires auth.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn app_bsky_graph_mute_actor_list(
    &self,
    body: AppBskyGraphMuteActorListInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/app.bsky.graph.muteActorList",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Mutes a thread preventing notifications from the thread and any of its children. Mutes are private in Bluesky. Requires auth.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn app_bsky_graph_mute_thread(&self, body: AppBskyGraphMuteThreadInput) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/app.bsky.graph.muteThread",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Find starter packs matching search criteria. Does not require auth.
  ///
  /// # Arguments
  ///
  /// * `q` - Search query string. Syntax, phrase, boolean, and faceting is unspecified, but Lucene query syntax is recommended.
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 25]
  /// * `cursor`
  pub async fn app_bsky_graph_search_starter_packs(
    &self,
    q: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphSearchStarterPacksOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("q"), q.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.searchStarterPacks",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Unmutes the specified account. Requires auth.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn app_bsky_graph_unmute_actor(
    &self,
    body: AppBskyGraphUnmuteActorInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/app.bsky.graph.unmuteActor",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Unmutes the specified list of accounts. Requires auth.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn app_bsky_graph_unmute_actor_list(
    &self,
    body: AppBskyGraphUnmuteActorListInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/app.bsky.graph.unmuteActorList",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Unmutes the specified thread. Requires auth.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn app_bsky_graph_unmute_thread(
    &self,
    body: AppBskyGraphUnmuteThreadInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/app.bsky.graph.unmuteThread",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Get information about a list of labeler services.
  ///
  /// # Arguments
  ///
  /// * `dids`
  /// * `detailed` - [default: false]
  pub async fn app_bsky_labeler_get_services(
    &self,
    dids: &[&str],
    detailed: Option<bool>,
  ) -> Result<AppBskyLabelerGetServicesOutput> {
    let mut query_ = Vec::new();
    query_.append(
      &mut dids
        .iter()
        .map(|i| (String::from("dids"), i.to_string()))
        .collect::<Vec<_>>(),
    );
    if let Some(detailed) = &detailed {
      query_.push((String::from("detailed"), detailed.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.labeler.getServices",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Count the number of unread notifications for the requesting account. Requires auth.
  ///
  /// # Arguments
  ///
  /// * `priority`
  /// * `seen_at` - [format: datetime]
  pub async fn app_bsky_notification_get_unread_count(
    &self,
    priority: Option<bool>,
    seen_at: Option<&chrono::DateTime<chrono::Utc>>,
  ) -> Result<AppBskyNotificationGetUnreadCountOutput> {
    let mut query_ = Vec::new();
    if let Some(priority) = &priority {
      query_.push((String::from("priority"), priority.to_string()));
    }
    if let Some(seen_at) = &seen_at {
      query_.push((String::from("seen_at"), seen_at.to_rfc3339()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.notification.getUnreadCount",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Enumerate notifications for the requesting account. Requires auth.
  ///
  /// # Arguments
  ///
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `priority`
  /// * `cursor`
  /// * `seen_at` - [format: datetime]
  pub async fn app_bsky_notification_list_notifications(
    &self,
    limit: Option<i64>,
    priority: Option<bool>,
    cursor: Option<&str>,
    seen_at: Option<&chrono::DateTime<chrono::Utc>>,
  ) -> Result<AppBskyNotificationListNotificationsOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(priority) = &priority {
      query_.push((String::from("priority"), priority.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    if let Some(seen_at) = &seen_at {
      query_.push((String::from("seen_at"), seen_at.to_rfc3339()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.notification.listNotifications",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Set notification-related preferences for an account. Requires auth.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn app_bsky_notification_put_preferences(
    &self,
    body: AppBskyNotificationPutPreferencesInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/app.bsky.notification.putPreferences",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Register to receive push notifications, via a specified service, for the requesting account. Requires auth.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn app_bsky_notification_register_push(
    &self,
    body: AppBskyNotificationRegisterPushInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/app.bsky.notification.registerPush",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Notify server that the requesting account has seen notifications. Requires auth.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn app_bsky_notification_update_seen(
    &self,
    body: AppBskyNotificationUpdateSeenInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/app.bsky.notification.updateSeen",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Get miscellaneous runtime configuration.
  pub async fn app_bsky_unspecced_get_config(&self) -> Result<AppBskyUnspeccedGetConfigOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/app.bsky.unspecced.getConfig",
      self.host
    ));
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// An unspecced view of globally popular feed generators.
  ///
  /// # Arguments
  ///
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  /// * `query`
  pub async fn app_bsky_unspecced_get_popular_feed_generators(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
    query: Option<&str>,
  ) -> Result<AppBskyUnspeccedGetPopularFeedGeneratorsOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    if let Some(query) = &query {
      query_.push((String::from("query"), query.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.unspecced.getPopularFeedGenerators",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a skeleton of suggested actors. Intended to be called and then hydrated through app.bsky.actor.getSuggestions
  ///
  /// # Arguments
  ///
  /// * `viewer` - [format: did] DID of the account making the request (not included for public/unauthenticated queries). Used to boost followed accounts in ranking.
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  /// * `relative_to_did` - [format: did] DID of the account to get suggestions relative to. If not provided, suggestions will be based on the viewer.
  pub async fn app_bsky_unspecced_get_suggestions_skeleton(
    &self,
    viewer: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
    relative_to_did: Option<&str>,
  ) -> Result<AppBskyUnspeccedGetSuggestionsSkeletonOutput> {
    let mut query_ = Vec::new();
    if let Some(viewer) = &viewer {
      query_.push((String::from("viewer"), viewer.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    if let Some(relative_to_did) = &relative_to_did {
      query_.push((String::from("relative_to_did"), relative_to_did.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.unspecced.getSuggestionsSkeleton",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a list of suggestions (feeds and users) tagged with categories
  pub async fn app_bsky_unspecced_get_tagged_suggestions(
    &self,
  ) -> Result<AppBskyUnspeccedGetTaggedSuggestionsOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/app.bsky.unspecced.getTaggedSuggestions",
      self.host
    ));
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Backend Actors (profile) search, returns only skeleton.
  ///
  /// # Arguments
  ///
  /// * `q` - Search query string; syntax, phrase, boolean, and faceting is unspecified, but Lucene query syntax is recommended. For typeahead search, only simple term match is supported, not full syntax.
  /// * `viewer` - [format: did] DID of the account making the request (not included for public/unauthenticated queries). Used to boost followed accounts in ranking.
  /// * `typeahead` - If true, acts as fast/simple 'typeahead' query.
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 25]
  /// * `cursor` - Optional pagination mechanism; may not necessarily allow scrolling through entire result set.
  ///
  /// # Errors
  ///
  /// * `BadQueryString`
  pub async fn app_bsky_unspecced_search_actors_skeleton(
    &self,
    q: &str,
    viewer: Option<&str>,
    typeahead: Option<bool>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyUnspeccedSearchActorsSkeletonOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("q"), q.to_string()));
    if let Some(viewer) = &viewer {
      query_.push((String::from("viewer"), viewer.to_string()));
    }
    if let Some(typeahead) = &typeahead {
      query_.push((String::from("typeahead"), typeahead.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.unspecced.searchActorsSkeleton",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Backend Posts search, returns only skeleton
  ///
  /// # Arguments
  ///
  /// * `q` - Search query string; syntax, phrase, boolean, and faceting is unspecified, but Lucene query syntax is recommended.
  /// * `sort` - [known_values: ["top", "latest"]] [default: latest] Specifies the ranking order of results.
  /// * `since` - Filter results for posts after the indicated datetime (inclusive). Expected to use 'sortAt' timestamp, which may not match 'createdAt'. Can be a datetime, or just an ISO date (YYYY-MM-DD).
  /// * `until` - Filter results for posts before the indicated datetime (not inclusive). Expected to use 'sortAt' timestamp, which may not match 'createdAt'. Can be a datetime, or just an ISO date (YYY-MM-DD).
  /// * `mentions` - [format: at-identifier] Filter to posts which mention the given account. Handles are resolved to DID before query-time. Only matches rich-text facet mentions.
  /// * `author` - [format: at-identifier] Filter to posts by the given account. Handles are resolved to DID before query-time.
  /// * `lang` - [format: language] Filter to posts in the given language. Expected to be based on post language field, though server may override language detection.
  /// * `domain` - Filter to posts with URLs (facet links or embeds) linking to the given domain (hostname). Server may apply hostname normalization.
  /// * `url` - [format: uri] Filter to posts with links (facet links or embeds) pointing to this URL. Server may apply URL normalization or fuzzy matching.
  /// * `tag` - Filter to posts with the given tag (hashtag), based on rich-text facet or tag field. Do not include the hash (#) prefix. Multiple tags can be specified, with 'AND' matching.
  /// * `viewer` - [format: did] DID of the account making the request (not included for public/unauthenticated queries). Used for 'from:me' queries.
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 25]
  /// * `cursor` - Optional pagination mechanism; may not necessarily allow scrolling through entire result set.
  ///
  /// # Errors
  ///
  /// * `BadQueryString`
  pub async fn app_bsky_unspecced_search_posts_skeleton(
    &self,
    q: &str,
    sort: Option<&str>,
    since: Option<&str>,
    until: Option<&str>,
    mentions: Option<&str>,
    author: Option<&str>,
    lang: Option<&str>,
    domain: Option<&str>,
    url: Option<&str>,
    tag: Option<&[&str]>,
    viewer: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyUnspeccedSearchPostsSkeletonOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("q"), q.to_string()));
    if let Some(sort) = &sort {
      query_.push((String::from("sort"), sort.to_string()));
    }
    if let Some(since) = &since {
      query_.push((String::from("since"), since.to_string()));
    }
    if let Some(until) = &until {
      query_.push((String::from("until"), until.to_string()));
    }
    if let Some(mentions) = &mentions {
      query_.push((String::from("mentions"), mentions.to_string()));
    }
    if let Some(author) = &author {
      query_.push((String::from("author"), author.to_string()));
    }
    if let Some(lang) = &lang {
      query_.push((String::from("lang"), lang.to_string()));
    }
    if let Some(domain) = &domain {
      query_.push((String::from("domain"), domain.to_string()));
    }
    if let Some(url) = &url {
      query_.push((String::from("url"), url.to_string()));
    }
    if let Some(tag) = &tag {
      query_.append(
        &mut tag
          .iter()
          .map(|i| (String::from("tag"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(viewer) = &viewer {
      query_.push((String::from("viewer"), viewer.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.unspecced.searchPostsSkeleton",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Backend Starter Pack search, returns only skeleton.
  ///
  /// # Arguments
  ///
  /// * `q` - Search query string; syntax, phrase, boolean, and faceting is unspecified, but Lucene query syntax is recommended.
  /// * `viewer` - [format: did] DID of the account making the request (not included for public/unauthenticated queries).
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 25]
  /// * `cursor` - Optional pagination mechanism; may not necessarily allow scrolling through entire result set.
  ///
  /// # Errors
  ///
  /// * `BadQueryString`
  pub async fn app_bsky_unspecced_search_starter_packs_skeleton(
    &self,
    q: &str,
    viewer: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyUnspeccedSearchStarterPacksSkeletonOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("q"), q.to_string()));
    if let Some(viewer) = &viewer {
      query_.push((String::from("viewer"), viewer.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.unspecced.searchStarterPacksSkeleton",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get status details for a video processing job.
  ///
  /// # Arguments
  ///
  /// * `job_id`
  pub async fn app_bsky_video_get_job_status(
    &self,
    job_id: &str,
  ) -> Result<AppBskyVideoGetJobStatusOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("job_id"), job_id.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.video.getJobStatus",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get video upload limits for the authenticated user.
  pub async fn app_bsky_video_get_upload_limits(
    &self,
  ) -> Result<AppBskyVideoGetUploadLimitsOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/app.bsky.video.getUploadLimits",
      self.host
    ));
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Upload a video to be processed then stored on the PDS.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn app_bsky_video_upload_video(
    &self,
    body: Vec<u8>,
  ) -> Result<AppBskyVideoUploadVideoOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/app.bsky.video.uploadVideo",
        self.host
      ))
      .header("Content-Type", "video/mp4")
      .body(body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  pub async fn chat_bsky_actor_delete_account(&self) -> Result<serde_json::Value> {
    let mut request = self.client.post(&format!(
      "https://{}/xrpc/chat.bsky.actor.deleteAccount",
      self.host
    ));
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  pub async fn chat_bsky_actor_export_account_data(&self) -> Result<Vec<serde_json::Value>> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/chat.bsky.actor.exportAccountData",
      self.host
    ));
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(
      response
        .text()
        .await?
        .split("\n")
        .filter_map(|t| serde_json::from_str(t).ok())
        .collect::<Vec<_>>(),
    )
  }

  /// # Arguments
  ///
  /// * body
  pub async fn chat_bsky_convo_delete_message_for_self(
    &self,
    body: ChatBskyConvoDeleteMessageForSelfInput,
  ) -> Result<ChatBskyConvoDefsDeletedMessageView> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/chat.bsky.convo.deleteMessageForSelf",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * `convo_id`
  pub async fn chat_bsky_convo_get_convo(
    &self,
    convo_id: &str,
  ) -> Result<ChatBskyConvoGetConvoOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("convo_id"), convo_id.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/chat.bsky.convo.getConvo",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * `members` - [min_length: 1] [max_length: 10]
  pub async fn chat_bsky_convo_get_convo_for_members(
    &self,
    members: &[&str],
  ) -> Result<ChatBskyConvoGetConvoForMembersOutput> {
    let mut query_ = Vec::new();
    query_.append(
      &mut members
        .iter()
        .map(|i| (String::from("members"), i.to_string()))
        .collect::<Vec<_>>(),
    );
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/chat.bsky.convo.getConvoForMembers",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * `cursor`
  pub async fn chat_bsky_convo_get_log(
    &self,
    cursor: Option<&str>,
  ) -> Result<ChatBskyConvoGetLogOutput> {
    let mut query_ = Vec::new();
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/chat.bsky.convo.getLog",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * `convo_id`
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  pub async fn chat_bsky_convo_get_messages(
    &self,
    convo_id: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ChatBskyConvoGetMessagesOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("convo_id"), convo_id.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/chat.bsky.convo.getMessages",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * body
  pub async fn chat_bsky_convo_leave_convo(
    &self,
    body: ChatBskyConvoLeaveConvoInput,
  ) -> Result<ChatBskyConvoLeaveConvoOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/chat.bsky.convo.leaveConvo",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  pub async fn chat_bsky_convo_list_convos(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ChatBskyConvoListConvosOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/chat.bsky.convo.listConvos",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * body
  pub async fn chat_bsky_convo_mute_convo(
    &self,
    body: ChatBskyConvoMuteConvoInput,
  ) -> Result<ChatBskyConvoMuteConvoOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/chat.bsky.convo.muteConvo",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * body
  pub async fn chat_bsky_convo_send_message(
    &self,
    body: ChatBskyConvoSendMessageInput,
  ) -> Result<ChatBskyConvoDefsMessageView> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/chat.bsky.convo.sendMessage",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * body
  pub async fn chat_bsky_convo_send_message_batch(
    &self,
    body: ChatBskyConvoSendMessageBatchInput,
  ) -> Result<ChatBskyConvoSendMessageBatchOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/chat.bsky.convo.sendMessageBatch",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * body
  pub async fn chat_bsky_convo_unmute_convo(
    &self,
    body: ChatBskyConvoUnmuteConvoInput,
  ) -> Result<ChatBskyConvoUnmuteConvoOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/chat.bsky.convo.unmuteConvo",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * body
  pub async fn chat_bsky_convo_update_read(
    &self,
    body: ChatBskyConvoUpdateReadInput,
  ) -> Result<ChatBskyConvoUpdateReadOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/chat.bsky.convo.updateRead",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * `actor` - [format: did]
  pub async fn chat_bsky_moderation_get_actor_metadata(
    &self,
    actor: &str,
  ) -> Result<ChatBskyModerationGetActorMetadataOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("actor"), actor.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/chat.bsky.moderation.getActorMetadata",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * `convo_id` - Conversation that the message is from. NOTE: this field will eventually be required.
  /// * `message_id`
  /// * `before` - [default: 5]
  /// * `after` - [default: 5]
  pub async fn chat_bsky_moderation_get_message_context(
    &self,
    convo_id: Option<&str>,
    message_id: &str,
    before: Option<i64>,
    after: Option<i64>,
  ) -> Result<ChatBskyModerationGetMessageContextOutput> {
    let mut query_ = Vec::new();
    if let Some(convo_id) = &convo_id {
      query_.push((String::from("convo_id"), convo_id.to_string()));
    }
    query_.push((String::from("message_id"), message_id.to_string()));
    if let Some(before) = &before {
      query_.push((String::from("before"), before.to_string()));
    }
    if let Some(after) = &after {
      query_.push((String::from("after"), after.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/chat.bsky.moderation.getMessageContext",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * body
  pub async fn chat_bsky_moderation_update_actor_access(
    &self,
    body: ChatBskyModerationUpdateActorAccessInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/chat.bsky.moderation.updateActorAccess",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Delete a user account as an administrator.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_admin_delete_account(
    &self,
    body: ComAtprotoAdminDeleteAccountInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.admin.deleteAccount",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Disable an account from receiving new invite codes, but does not invalidate existing codes.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_admin_disable_account_invites(
    &self,
    body: ComAtprotoAdminDisableAccountInvitesInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.admin.disableAccountInvites",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Disable some set of codes and/or all codes associated with a set of users.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_admin_disable_invite_codes(
    &self,
    body: ComAtprotoAdminDisableInviteCodesInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.admin.disableInviteCodes",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Re-enable an account's ability to receive invite codes.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_admin_enable_account_invites(
    &self,
    body: ComAtprotoAdminEnableAccountInvitesInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.admin.enableAccountInvites",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Get details about an account.
  ///
  /// # Arguments
  ///
  /// * `did` - [format: did]
  pub async fn com_atproto_admin_get_account_info(
    &self,
    did: &str,
  ) -> Result<ComAtprotoAdminDefsAccountView> {
    let mut query_ = Vec::new();
    query_.push((String::from("did"), did.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.admin.getAccountInfo",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get details about some accounts.
  ///
  /// # Arguments
  ///
  /// * `dids`
  pub async fn com_atproto_admin_get_account_infos(
    &self,
    dids: &[&str],
  ) -> Result<ComAtprotoAdminGetAccountInfosOutput> {
    let mut query_ = Vec::new();
    query_.append(
      &mut dids
        .iter()
        .map(|i| (String::from("dids"), i.to_string()))
        .collect::<Vec<_>>(),
    );
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.admin.getAccountInfos",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get an admin view of invite codes.
  ///
  /// # Arguments
  ///
  /// * `sort` - [known_values: ["recent", "usage"]] [default: recent]
  /// * `limit` - [minimum: 1] [maximum: 500] [default: 100]
  /// * `cursor`
  pub async fn com_atproto_admin_get_invite_codes(
    &self,
    sort: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ComAtprotoAdminGetInviteCodesOutput> {
    let mut query_ = Vec::new();
    if let Some(sort) = &sort {
      query_.push((String::from("sort"), sort.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.admin.getInviteCodes",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get the service-specific admin status of a subject (account, record, or blob).
  ///
  /// # Arguments
  ///
  /// * `did` - [format: did]
  /// * `uri` - [format: at-uri]
  /// * `blob` - [format: cid]
  pub async fn com_atproto_admin_get_subject_status(
    &self,
    did: Option<&str>,
    uri: Option<&str>,
    blob: Option<&str>,
  ) -> Result<ComAtprotoAdminGetSubjectStatusOutput> {
    let mut query_ = Vec::new();
    if let Some(did) = &did {
      query_.push((String::from("did"), did.to_string()));
    }
    if let Some(uri) = &uri {
      query_.push((String::from("uri"), uri.to_string()));
    }
    if let Some(blob) = &blob {
      query_.push((String::from("blob"), blob.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.admin.getSubjectStatus",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get list of accounts that matches your search query.
  ///
  /// # Arguments
  ///
  /// * `email`
  /// * `cursor`
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  pub async fn com_atproto_admin_search_accounts(
    &self,
    email: Option<&str>,
    cursor: Option<&str>,
    limit: Option<i64>,
  ) -> Result<ComAtprotoAdminSearchAccountsOutput> {
    let mut query_ = Vec::new();
    if let Some(email) = &email {
      query_.push((String::from("email"), email.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.admin.searchAccounts",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Send email to a user's account email address.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_admin_send_email(
    &self,
    body: ComAtprotoAdminSendEmailInput,
  ) -> Result<ComAtprotoAdminSendEmailOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.admin.sendEmail",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Administrative action to update an account's email.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_admin_update_account_email(
    &self,
    body: ComAtprotoAdminUpdateAccountEmailInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.admin.updateAccountEmail",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Administrative action to update an account's handle.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_admin_update_account_handle(
    &self,
    body: ComAtprotoAdminUpdateAccountHandleInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.admin.updateAccountHandle",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Update the password for a user account as an administrator.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_admin_update_account_password(
    &self,
    body: ComAtprotoAdminUpdateAccountPasswordInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.admin.updateAccountPassword",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Update the service-specific admin status of a subject (account, record, or blob).
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_admin_update_subject_status(
    &self,
    body: ComAtprotoAdminUpdateSubjectStatusInput,
  ) -> Result<ComAtprotoAdminUpdateSubjectStatusOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.admin.updateSubjectStatus",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Describe the credentials that should be included in the DID doc of an account that is migrating to this service.
  pub async fn com_atproto_identity_get_recommended_did_credentials(
    &self,
  ) -> Result<ComAtprotoIdentityGetRecommendedDidCredentialsOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/com.atproto.identity.getRecommendedDidCredentials",
      self.host
    ));
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Request an email with a code to in order to request a signed PLC operation. Requires Auth.
  pub async fn com_atproto_identity_request_plc_operation_signature(&self) -> Result<()> {
    let mut request = self.client.post(&format!(
      "https://{}/xrpc/com.atproto.identity.requestPlcOperationSignature",
      self.host
    ));
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Resolves a handle (domain name) to a DID.
  ///
  /// # Arguments
  ///
  /// * `handle` - [format: handle] The handle to resolve.
  pub async fn com_atproto_identity_resolve_handle(
    &self,
    handle: &str,
  ) -> Result<ComAtprotoIdentityResolveHandleOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("handle"), handle.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.identity.resolveHandle",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Signs a PLC operation to update some value(s) in the requesting DID's document.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_identity_sign_plc_operation(
    &self,
    body: ComAtprotoIdentitySignPlcOperationInput,
  ) -> Result<ComAtprotoIdentitySignPlcOperationOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.identity.signPlcOperation",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Validates a PLC operation to ensure that it doesn't violate a service's constraints or get the identity into a bad state, then submits it to the PLC registry
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_identity_submit_plc_operation(
    &self,
    body: ComAtprotoIdentitySubmitPlcOperationInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.identity.submitPlcOperation",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Updates the current account's handle. Verifies handle validity, and updates did:plc document if necessary. Implemented by PDS, and requires auth.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_identity_update_handle(
    &self,
    body: ComAtprotoIdentityUpdateHandleInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.identity.updateHandle",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Find labels relevant to the provided AT-URI patterns. Public endpoint for moderation services, though may return different or additional results with auth.
  ///
  /// # Arguments
  ///
  /// * `uri_patterns` - List of AT URI patterns to match (boolean 'OR'). Each may be a prefix (ending with '*'; will match inclusive of the string leading to '*'), or a full URI.
  /// * `sources` - Optional list of label sources (DIDs) to filter on.
  /// * `limit` - [minimum: 1] [maximum: 250] [default: 50]
  /// * `cursor`
  pub async fn com_atproto_label_query_labels(
    &self,
    uri_patterns: &[&str],
    sources: Option<&[&str]>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ComAtprotoLabelQueryLabelsOutput> {
    let mut query_ = Vec::new();
    query_.append(
      &mut uri_patterns
        .iter()
        .map(|i| (String::from("uri_patterns"), i.to_string()))
        .collect::<Vec<_>>(),
    );
    if let Some(sources) = &sources {
      query_.append(
        &mut sources
          .iter()
          .map(|i| (String::from("sources"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.label.queryLabels",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Subscribe to stream of labels (and negations). Public endpoint implemented by mod services. Uses same sequencing scheme as repo event stream.
  ///
  /// # Arguments
  ///
  /// * `cursor` - The last known event seq number to backfill from.
  ///
  /// # Messages
  ///
  /// * ComAtprotoLabelSubscribeLabelsLabels
  /// * ComAtprotoLabelSubscribeLabelsInfo
  ///
  /// # Errors
  ///
  /// * `FutureCursor`
  pub async fn com_atproto_label_subscribe_labels(
    &self,
    cursor: Option<i64>,
  ) -> Result<reqwest_websocket::WebSocket> {
    let mut query_ = Vec::new();
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "wss://{}/xrpc/com.atproto.label.subscribeLabels",
        self.firehose
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    Ok(
      reqwest_websocket::RequestBuilderExt::upgrade(request)
        .send()
        .await?
        .into_websocket()
        .await?,
    )
  }

  /// Submit a moderation report regarding an atproto account or record. Implemented by moderation services (with PDS proxying), and requires auth.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_moderation_create_report(
    &self,
    body: ComAtprotoModerationCreateReportInput,
  ) -> Result<ComAtprotoModerationCreateReportOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.moderation.createReport",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Apply a batch transaction of repository creates, updates, and deletes. Requires auth, implemented by PDS.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `InvalidSwap` - Indicates that the 'swapCommit' parameter did not match current commit.
  pub async fn com_atproto_repo_apply_writes(
    &self,
    body: ComAtprotoRepoApplyWritesInput,
  ) -> Result<ComAtprotoRepoApplyWritesOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.repo.applyWrites",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Create a single new repository record. Requires auth, implemented by PDS.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `InvalidSwap` - Indicates that 'swapCommit' didn't match current repo commit.
  pub async fn com_atproto_repo_create_record(
    &self,
    body: ComAtprotoRepoCreateRecordInput,
  ) -> Result<ComAtprotoRepoCreateRecordOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.repo.createRecord",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Delete a repository record, or ensure it doesn't exist. Requires auth, implemented by PDS.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `InvalidSwap`
  pub async fn com_atproto_repo_delete_record(
    &self,
    body: ComAtprotoRepoDeleteRecordInput,
  ) -> Result<ComAtprotoRepoDeleteRecordOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.repo.deleteRecord",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get information about an account and repository, including the list of collections. Does not require auth.
  ///
  /// # Arguments
  ///
  /// * `repo` - [format: at-identifier] The handle or DID of the repo.
  pub async fn com_atproto_repo_describe_repo(
    &self,
    repo: &str,
  ) -> Result<ComAtprotoRepoDescribeRepoOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("repo"), repo.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.repo.describeRepo",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a single record from a repository. Does not require auth.
  ///
  /// # Arguments
  ///
  /// * `repo` - [format: at-identifier] The handle or DID of the repo.
  /// * `collection` - [format: nsid] The NSID of the record collection.
  /// * `rkey` - The Record Key.
  /// * `cid` - [format: cid] The CID of the version of the record. If not specified, then return the most recent version.
  ///
  /// # Errors
  ///
  /// * `RecordNotFound`
  pub async fn com_atproto_repo_get_record(
    &self,
    repo: &str,
    collection: &str,
    rkey: &str,
    cid: Option<&str>,
  ) -> Result<ComAtprotoRepoGetRecordOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("repo"), repo.to_string()));
    query_.push((String::from("collection"), collection.to_string()));
    query_.push((String::from("rkey"), rkey.to_string()));
    if let Some(cid) = &cid {
      query_.push((String::from("cid"), cid.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.repo.getRecord",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Import a repo in the form of a CAR file. Requires Content-Length HTTP header to be set.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_repo_import_repo(&self, body: Vec<u8>) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.repo.importRepo",
        self.host
      ))
      .header("Content-Type", "application/vnd.ipld.car")
      .body(body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Returns a list of missing blobs for the requesting account. Intended to be used in the account migration flow.
  ///
  /// # Arguments
  ///
  /// * `limit` - [minimum: 1] [maximum: 1000] [default: 500]
  /// * `cursor`
  pub async fn com_atproto_repo_list_missing_blobs(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ComAtprotoRepoListMissingBlobsOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.repo.listMissingBlobs",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// List a range of records in a repository, matching a specific collection. Does not require auth.
  ///
  /// # Arguments
  ///
  /// * `repo` - [format: at-identifier] The handle or DID of the repo.
  /// * `collection` - [format: nsid] The NSID of the record type.
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50] The number of records to return.
  /// * `cursor`
  /// * `rkey_start` - DEPRECATED: The lowest sort-ordered rkey to start from (exclusive)
  /// * `rkey_end` - DEPRECATED: The highest sort-ordered rkey to stop at (exclusive)
  /// * `reverse` - Flag to reverse the order of the returned records.
  pub async fn com_atproto_repo_list_records(
    &self,
    repo: &str,
    collection: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
    rkey_start: Option<&str>,
    rkey_end: Option<&str>,
    reverse: Option<bool>,
  ) -> Result<ComAtprotoRepoListRecordsOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("repo"), repo.to_string()));
    query_.push((String::from("collection"), collection.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    if let Some(rkey_start) = &rkey_start {
      query_.push((String::from("rkey_start"), rkey_start.to_string()));
    }
    if let Some(rkey_end) = &rkey_end {
      query_.push((String::from("rkey_end"), rkey_end.to_string()));
    }
    if let Some(reverse) = &reverse {
      query_.push((String::from("reverse"), reverse.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.repo.listRecords",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Write a repository record, creating or updating it as needed. Requires auth, implemented by PDS.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `InvalidSwap`
  pub async fn com_atproto_repo_put_record(
    &self,
    body: ComAtprotoRepoPutRecordInput,
  ) -> Result<ComAtprotoRepoPutRecordOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.repo.putRecord",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Upload a new blob, to be referenced from a repository record. The blob will be deleted if it is not referenced within a time window (eg, minutes). Blob restrictions (mimetype, size, etc) are enforced when the reference is created. Requires auth, implemented by PDS.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_repo_upload_blob(
    &self,
    body: Vec<u8>,
    content_type: &str,
  ) -> Result<ComAtprotoRepoUploadBlobOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.repo.uploadBlob",
        self.host
      ))
      .body(body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    request = request.header("Content-Type", content_type);
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Activates a currently deactivated account. Used to finalize account migration after the account's repo is imported and identity is setup.
  pub async fn com_atproto_server_activate_account(&self) -> Result<()> {
    let mut request = self.client.post(&format!(
      "https://{}/xrpc/com.atproto.server.activateAccount",
      self.host
    ));
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Returns the status of an account, especially as pertaining to import or recovery. Can be called many times over the course of an account migration. Requires auth and can only be called pertaining to oneself.
  pub async fn com_atproto_server_check_account_status(
    &self,
  ) -> Result<ComAtprotoServerCheckAccountStatusOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/com.atproto.server.checkAccountStatus",
      self.host
    ));
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Confirm an email using a token from com.atproto.server.requestEmailConfirmation.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `AccountNotFound`
  /// * `ExpiredToken`
  /// * `InvalidToken`
  /// * `InvalidEmail`
  pub async fn com_atproto_server_confirm_email(
    &self,
    body: ComAtprotoServerConfirmEmailInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.confirmEmail",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Create an account. Implemented by PDS.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `InvalidHandle`
  /// * `InvalidPassword`
  /// * `InvalidInviteCode`
  /// * `HandleNotAvailable`
  /// * `UnsupportedDomain`
  /// * `UnresolvableDid`
  /// * `IncompatibleDidDoc`
  pub async fn com_atproto_server_create_account(
    &self,
    body: ComAtprotoServerCreateAccountInput,
  ) -> Result<ComAtprotoServerCreateAccountOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.createAccount",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Create an App Password.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `AccountTakedown`
  pub async fn com_atproto_server_create_app_password(
    &self,
    body: ComAtprotoServerCreateAppPasswordInput,
  ) -> Result<ComAtprotoServerCreateAppPasswordAppPassword> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.createAppPassword",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Create an invite code.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_server_create_invite_code(
    &self,
    body: ComAtprotoServerCreateInviteCodeInput,
  ) -> Result<ComAtprotoServerCreateInviteCodeOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.createInviteCode",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Create invite codes.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_server_create_invite_codes(
    &self,
    body: ComAtprotoServerCreateInviteCodesInput,
  ) -> Result<ComAtprotoServerCreateInviteCodesOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.createInviteCodes",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Create an authentication session.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `AccountTakedown`
  /// * `AuthFactorTokenRequired`
  pub async fn com_atproto_server_create_session(
    &self,
    body: ComAtprotoServerCreateSessionInput,
  ) -> Result<ComAtprotoServerCreateSessionOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.createSession",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Deactivates a currently active account. Stops serving of repo, and future writes to repo until reactivated. Used to finalize account migration with the old host after the account has been activated on the new host.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_server_deactivate_account(
    &self,
    body: ComAtprotoServerDeactivateAccountInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.deactivateAccount",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Delete an actor's account with a token and password. Can only be called after requesting a deletion token. Requires auth.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `ExpiredToken`
  /// * `InvalidToken`
  pub async fn com_atproto_server_delete_account(
    &self,
    body: ComAtprotoServerDeleteAccountInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.deleteAccount",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Delete the current session. Requires auth.
  pub async fn com_atproto_server_delete_session(&self) -> Result<()> {
    let mut request = self.client.post(&format!(
      "https://{}/xrpc/com.atproto.server.deleteSession",
      self.host
    ));
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Describes the server's account creation requirements and capabilities. Implemented by PDS.
  pub async fn com_atproto_server_describe_server(
    &self,
  ) -> Result<ComAtprotoServerDescribeServerOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/com.atproto.server.describeServer",
      self.host
    ));
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get all invite codes for the current account. Requires auth.
  ///
  /// # Arguments
  ///
  /// * `include_used` - [default: true]
  /// * `create_available` - [default: true] Controls whether any new 'earned' but not 'created' invites should be created.
  ///
  /// # Errors
  ///
  /// * `DuplicateCreate`
  pub async fn com_atproto_server_get_account_invite_codes(
    &self,
    include_used: Option<bool>,
    create_available: Option<bool>,
  ) -> Result<ComAtprotoServerGetAccountInviteCodesOutput> {
    let mut query_ = Vec::new();
    if let Some(include_used) = &include_used {
      query_.push((String::from("include_used"), include_used.to_string()));
    }
    if let Some(create_available) = &create_available {
      query_.push((
        String::from("create_available"),
        create_available.to_string(),
      ));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.server.getAccountInviteCodes",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a signed token on behalf of the requesting DID for the requested service.
  ///
  /// # Arguments
  ///
  /// * `aud` - [format: did] The DID of the service that the token will be used to authenticate with
  /// * `exp` - The time in Unix Epoch seconds that the JWT expires. Defaults to 60 seconds in the future. The service may enforce certain time bounds on tokens depending on the requested scope.
  /// * `lxm` - [format: nsid] Lexicon (XRPC) method to bind the requested token to
  ///
  /// # Errors
  ///
  /// * `BadExpiration` - Indicates that the requested expiration date is not a valid. May be in the past or may be reliant on the requested scopes.
  pub async fn com_atproto_server_get_service_auth(
    &self,
    aud: &str,
    exp: Option<i64>,
    lxm: Option<&str>,
  ) -> Result<ComAtprotoServerGetServiceAuthOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("aud"), aud.to_string()));
    if let Some(exp) = &exp {
      query_.push((String::from("exp"), exp.to_string()));
    }
    if let Some(lxm) = &lxm {
      query_.push((String::from("lxm"), lxm.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.server.getServiceAuth",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get information about the current auth session. Requires auth.
  pub async fn com_atproto_server_get_session(&self) -> Result<ComAtprotoServerGetSessionOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/com.atproto.server.getSession",
      self.host
    ));
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// List all App Passwords.
  ///
  /// # Errors
  ///
  /// * `AccountTakedown`
  pub async fn com_atproto_server_list_app_passwords(
    &self,
  ) -> Result<ComAtprotoServerListAppPasswordsOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/com.atproto.server.listAppPasswords",
      self.host
    ));
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Refresh an authentication session. Requires auth using the 'refreshJwt' (not the 'accessJwt').
  ///
  /// # Errors
  ///
  /// * `AccountTakedown`
  pub async fn com_atproto_server_refresh_session(
    &self,
  ) -> Result<ComAtprotoServerRefreshSessionOutput> {
    let mut request = self.client.post(&format!(
      "https://{}/xrpc/com.atproto.server.refreshSession",
      self.host
    ));
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Initiate a user account deletion via email.
  pub async fn com_atproto_server_request_account_delete(&self) -> Result<()> {
    let mut request = self.client.post(&format!(
      "https://{}/xrpc/com.atproto.server.requestAccountDelete",
      self.host
    ));
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Request an email with a code to confirm ownership of email.
  pub async fn com_atproto_server_request_email_confirmation(&self) -> Result<()> {
    let mut request = self.client.post(&format!(
      "https://{}/xrpc/com.atproto.server.requestEmailConfirmation",
      self.host
    ));
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Request a token in order to update email.
  pub async fn com_atproto_server_request_email_update(
    &self,
  ) -> Result<ComAtprotoServerRequestEmailUpdateOutput> {
    let mut request = self.client.post(&format!(
      "https://{}/xrpc/com.atproto.server.requestEmailUpdate",
      self.host
    ));
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Initiate a user account password reset via email.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_server_request_password_reset(
    &self,
    body: ComAtprotoServerRequestPasswordResetInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.requestPasswordReset",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Reserve a repo signing key, for use with account creation. Necessary so that a DID PLC update operation can be constructed during an account migraiton. Public and does not require auth; implemented by PDS. NOTE: this endpoint may change when full account migration is implemented.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_server_reserve_signing_key(
    &self,
    body: ComAtprotoServerReserveSigningKeyInput,
  ) -> Result<ComAtprotoServerReserveSigningKeyOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.reserveSigningKey",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Reset a user account password using a token.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `ExpiredToken`
  /// * `InvalidToken`
  pub async fn com_atproto_server_reset_password(
    &self,
    body: ComAtprotoServerResetPasswordInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.resetPassword",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Revoke an App Password by name.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_server_revoke_app_password(
    &self,
    body: ComAtprotoServerRevokeAppPasswordInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.revokeAppPassword",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Update an account's email.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `ExpiredToken`
  /// * `InvalidToken`
  /// * `TokenRequired`
  pub async fn com_atproto_server_update_email(
    &self,
    body: ComAtprotoServerUpdateEmailInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.updateEmail",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Get a blob associated with a given account. Returns the full blob as originally uploaded. Does not require auth; implemented by PDS.
  ///
  /// # Arguments
  ///
  /// * `did` - [format: did] The DID of the account.
  /// * `cid` - [format: cid] The CID of the blob to fetch
  ///
  /// # Errors
  ///
  /// * `BlobNotFound`
  /// * `RepoNotFound`
  /// * `RepoTakendown`
  /// * `RepoSuspended`
  /// * `RepoDeactivated`
  pub async fn com_atproto_sync_get_blob(&self, did: &str, cid: &str) -> Result<Vec<u8>> {
    let mut query_ = Vec::new();
    query_.push((String::from("did"), did.to_string()));
    query_.push((String::from("cid"), cid.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.sync.getBlob",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(response.bytes().await?.to_vec())
  }

  /// Get data blocks from a given repo, by CID. For example, intermediate MST nodes, or records. Does not require auth; implemented by PDS.
  ///
  /// # Arguments
  ///
  /// * `did` - [format: did] The DID of the repo.
  /// * `cids`
  ///
  /// # Errors
  ///
  /// * `BlockNotFound`
  /// * `RepoNotFound`
  /// * `RepoTakendown`
  /// * `RepoSuspended`
  /// * `RepoDeactivated`
  pub async fn com_atproto_sync_get_blocks(&self, did: &str, cids: &[&str]) -> Result<Vec<u8>> {
    let mut query_ = Vec::new();
    query_.push((String::from("did"), did.to_string()));
    query_.append(
      &mut cids
        .iter()
        .map(|i| (String::from("cids"), i.to_string()))
        .collect::<Vec<_>>(),
    );
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.sync.getBlocks",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(response.bytes().await?.to_vec())
  }

  /// DEPRECATED - please use com.atproto.sync.getRepo instead
  ///
  /// # Arguments
  ///
  /// * `did` - [format: did] The DID of the repo.
  pub async fn com_atproto_sync_get_checkout(&self, did: &str) -> Result<Vec<u8>> {
    let mut query_ = Vec::new();
    query_.push((String::from("did"), did.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.sync.getCheckout",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(response.bytes().await?.to_vec())
  }

  /// DEPRECATED - please use com.atproto.sync.getLatestCommit instead
  ///
  /// # Arguments
  ///
  /// * `did` - [format: did] The DID of the repo.
  ///
  /// # Errors
  ///
  /// * `HeadNotFound`
  pub async fn com_atproto_sync_get_head(&self, did: &str) -> Result<ComAtprotoSyncGetHeadOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("did"), did.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.sync.getHead",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get the current commit CID & revision of the specified repo. Does not require auth.
  ///
  /// # Arguments
  ///
  /// * `did` - [format: did] The DID of the repo.
  ///
  /// # Errors
  ///
  /// * `RepoNotFound`
  /// * `RepoTakendown`
  /// * `RepoSuspended`
  /// * `RepoDeactivated`
  pub async fn com_atproto_sync_get_latest_commit(
    &self,
    did: &str,
  ) -> Result<ComAtprotoSyncGetLatestCommitOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("did"), did.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.sync.getLatestCommit",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get data blocks needed to prove the existence or non-existence of record in the current version of repo. Does not require auth.
  ///
  /// # Arguments
  ///
  /// * `did` - [format: did] The DID of the repo.
  /// * `collection` - [format: nsid]
  /// * `rkey` - Record Key
  /// * `commit` - [format: cid] DEPRECATED: referenced a repo commit by CID, and retrieved record as of that commit
  ///
  /// # Errors
  ///
  /// * `RecordNotFound`
  /// * `RepoNotFound`
  /// * `RepoTakendown`
  /// * `RepoSuspended`
  /// * `RepoDeactivated`
  pub async fn com_atproto_sync_get_record(
    &self,
    did: &str,
    collection: &str,
    rkey: &str,
    commit: Option<&str>,
  ) -> Result<Vec<u8>> {
    let mut query_ = Vec::new();
    query_.push((String::from("did"), did.to_string()));
    query_.push((String::from("collection"), collection.to_string()));
    query_.push((String::from("rkey"), rkey.to_string()));
    if let Some(commit) = &commit {
      query_.push((String::from("commit"), commit.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.sync.getRecord",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(response.bytes().await?.to_vec())
  }

  /// Download a repository export as CAR file. Optionally only a 'diff' since a previous revision. Does not require auth; implemented by PDS.
  ///
  /// # Arguments
  ///
  /// * `did` - [format: did] The DID of the repo.
  /// * `since` - The revision ('rev') of the repo to create a diff from.
  ///
  /// # Errors
  ///
  /// * `RepoNotFound`
  /// * `RepoTakendown`
  /// * `RepoSuspended`
  /// * `RepoDeactivated`
  pub async fn com_atproto_sync_get_repo(&self, did: &str, since: Option<&str>) -> Result<Vec<u8>> {
    let mut query_ = Vec::new();
    query_.push((String::from("did"), did.to_string()));
    if let Some(since) = &since {
      query_.push((String::from("since"), since.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.sync.getRepo",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(response.bytes().await?.to_vec())
  }

  /// Get the hosting status for a repository, on this server. Expected to be implemented by PDS and Relay.
  ///
  /// # Arguments
  ///
  /// * `did` - [format: did] The DID of the repo.
  ///
  /// # Errors
  ///
  /// * `RepoNotFound`
  pub async fn com_atproto_sync_get_repo_status(
    &self,
    did: &str,
  ) -> Result<ComAtprotoSyncGetRepoStatusOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("did"), did.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.sync.getRepoStatus",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// List blob CIDs for an account, since some repo revision. Does not require auth; implemented by PDS.
  ///
  /// # Arguments
  ///
  /// * `did` - [format: did] The DID of the repo.
  /// * `since` - Optional revision of the repo to list blobs since.
  /// * `limit` - [minimum: 1] [maximum: 1000] [default: 500]
  /// * `cursor`
  ///
  /// # Errors
  ///
  /// * `RepoNotFound`
  /// * `RepoTakendown`
  /// * `RepoSuspended`
  /// * `RepoDeactivated`
  pub async fn com_atproto_sync_list_blobs(
    &self,
    did: &str,
    since: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ComAtprotoSyncListBlobsOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("did"), did.to_string()));
    if let Some(since) = &since {
      query_.push((String::from("since"), since.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.sync.listBlobs",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Enumerates all the DID, rev, and commit CID for all repos hosted by this service. Does not require auth; implemented by PDS and Relay.
  ///
  /// # Arguments
  ///
  /// * `limit` - [minimum: 1] [maximum: 1000] [default: 500]
  /// * `cursor`
  pub async fn com_atproto_sync_list_repos(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ComAtprotoSyncListReposOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.sync.listRepos",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Notify a crawling service of a recent update, and that crawling should resume. Intended use is after a gap between repo stream events caused the crawling service to disconnect. Does not require auth; implemented by Relay.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_sync_notify_of_update(
    &self,
    body: ComAtprotoSyncNotifyOfUpdateInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.sync.notifyOfUpdate",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Request a service to persistently crawl hosted repos. Expected use is new PDS instances declaring their existence to Relays. Does not require auth.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_sync_request_crawl(
    &self,
    body: ComAtprotoSyncRequestCrawlInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.sync.requestCrawl",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Repository event stream, aka Firehose endpoint. Outputs repo commits with diff data, and identity update events, for all repositories on the current server. See the atproto specifications for details around stream sequencing, repo versioning, CAR diff format, and more. Public and does not require auth; implemented by PDS and Relay.
  ///
  /// # Arguments
  ///
  /// * `cursor` - The last known event seq number to backfill from.
  ///
  /// # Messages
  ///
  /// * ComAtprotoSyncSubscribeReposCommit
  /// * ComAtprotoSyncSubscribeReposIdentity
  /// * ComAtprotoSyncSubscribeReposAccount
  /// * ComAtprotoSyncSubscribeReposHandle
  /// * ComAtprotoSyncSubscribeReposMigrate
  /// * ComAtprotoSyncSubscribeReposTombstone
  /// * ComAtprotoSyncSubscribeReposInfo
  ///
  /// # Errors
  ///
  /// * `FutureCursor`
  /// * `ConsumerTooSlow` - If the consumer of the stream can not keep up with events, and a backlog gets too large, the server will drop the connection.
  pub async fn com_atproto_sync_subscribe_repos(
    &self,
    cursor: Option<i64>,
  ) -> Result<reqwest_websocket::WebSocket> {
    let mut query_ = Vec::new();
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "wss://{}/xrpc/com.atproto.sync.subscribeRepos",
        self.firehose
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    Ok(
      reqwest_websocket::RequestBuilderExt::upgrade(request)
        .send()
        .await?
        .into_websocket()
        .await?,
    )
  }

  /// Check accounts location in signup queue.
  pub async fn com_atproto_temp_check_signup_queue(
    &self,
  ) -> Result<ComAtprotoTempCheckSignupQueueOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/com.atproto.temp.checkSignupQueue",
      self.host
    ));
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// DEPRECATED: use queryLabels or subscribeLabels instead -- Fetch all labels from a labeler created after a certain date.
  ///
  /// # Arguments
  ///
  /// * `since`
  /// * `limit` - [minimum: 1] [maximum: 250] [default: 50]
  pub async fn com_atproto_temp_fetch_labels(
    &self,
    since: Option<i64>,
    limit: Option<i64>,
  ) -> Result<ComAtprotoTempFetchLabelsOutput> {
    let mut query_ = Vec::new();
    if let Some(since) = &since {
      query_.push((String::from("since"), since.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.temp.fetchLabels",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Request a verification code to be sent to the supplied phone number
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_temp_request_phone_verification(
    &self,
    body: ComAtprotoTempRequestPhoneVerificationInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.temp.requestPhoneVerification",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Administrative action to create a new, re-usable communication (email for now) template.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `DuplicateTemplateName`
  pub async fn tools_ozone_communication_create_template(
    &self,
    body: ToolsOzoneCommunicationCreateTemplateInput,
  ) -> Result<ToolsOzoneCommunicationDefsTemplateView> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/tools.ozone.communication.createTemplate",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Delete a communication template.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn tools_ozone_communication_delete_template(
    &self,
    body: ToolsOzoneCommunicationDeleteTemplateInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/tools.ozone.communication.deleteTemplate",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Get list of all communication templates.
  pub async fn tools_ozone_communication_list_templates(
    &self,
  ) -> Result<ToolsOzoneCommunicationListTemplatesOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/tools.ozone.communication.listTemplates",
      self.host
    ));
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Administrative action to update an existing communication template. Allows passing partial fields to patch specific fields only.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `DuplicateTemplateName`
  pub async fn tools_ozone_communication_update_template(
    &self,
    body: ToolsOzoneCommunicationUpdateTemplateInput,
  ) -> Result<ToolsOzoneCommunicationDefsTemplateView> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/tools.ozone.communication.updateTemplate",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Take a moderation action on an actor.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `SubjectHasAction`
  pub async fn tools_ozone_moderation_emit_event(
    &self,
    body: ToolsOzoneModerationEmitEventInput,
  ) -> Result<ToolsOzoneModerationDefsModEventView> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/tools.ozone.moderation.emitEvent",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get details about a moderation event.
  ///
  /// # Arguments
  ///
  /// * `id`
  pub async fn tools_ozone_moderation_get_event(
    &self,
    id: i64,
  ) -> Result<ToolsOzoneModerationDefsModEventViewDetail> {
    let mut query_ = Vec::new();
    query_.push((String::from("id"), id.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/tools.ozone.moderation.getEvent",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get details about a record.
  ///
  /// # Arguments
  ///
  /// * `uri` - [format: at-uri]
  /// * `cid` - [format: cid]
  ///
  /// # Errors
  ///
  /// * `RecordNotFound`
  pub async fn tools_ozone_moderation_get_record(
    &self,
    uri: &str,
    cid: Option<&str>,
  ) -> Result<ToolsOzoneModerationDefsRecordViewDetail> {
    let mut query_ = Vec::new();
    query_.push((String::from("uri"), uri.to_string()));
    if let Some(cid) = &cid {
      query_.push((String::from("cid"), cid.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/tools.ozone.moderation.getRecord",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get details about some records.
  ///
  /// # Arguments
  ///
  /// * `uris` - [max_length: 100]
  pub async fn tools_ozone_moderation_get_records(
    &self,
    uris: &[&str],
  ) -> Result<ToolsOzoneModerationGetRecordsOutput> {
    let mut query_ = Vec::new();
    query_.append(
      &mut uris
        .iter()
        .map(|i| (String::from("uris"), i.to_string()))
        .collect::<Vec<_>>(),
    );
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/tools.ozone.moderation.getRecords",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get details about a repository.
  ///
  /// # Arguments
  ///
  /// * `did` - [format: did]
  ///
  /// # Errors
  ///
  /// * `RepoNotFound`
  pub async fn tools_ozone_moderation_get_repo(
    &self,
    did: &str,
  ) -> Result<ToolsOzoneModerationDefsRepoViewDetail> {
    let mut query_ = Vec::new();
    query_.push((String::from("did"), did.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/tools.ozone.moderation.getRepo",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get details about some repositories.
  ///
  /// # Arguments
  ///
  /// * `dids` - [max_length: 100]
  pub async fn tools_ozone_moderation_get_repos(
    &self,
    dids: &[&str],
  ) -> Result<ToolsOzoneModerationGetReposOutput> {
    let mut query_ = Vec::new();
    query_.append(
      &mut dids
        .iter()
        .map(|i| (String::from("dids"), i.to_string()))
        .collect::<Vec<_>>(),
    );
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/tools.ozone.moderation.getRepos",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// List moderation events related to a subject.
  ///
  /// # Arguments
  ///
  /// * `types` - The types of events (fully qualified string in the format of tools.ozone.moderation.defs#modEvent<name>) to filter by. If not specified, all events are returned.
  /// * `created_by` - [format: did]
  /// * `sort_direction` - [default: desc] [enum: ["asc", "desc"]] Sort direction for the events. Defaults to descending order of created at timestamp.
  /// * `created_after` - [format: datetime] Retrieve events created after a given timestamp
  /// * `created_before` - [format: datetime] Retrieve events created before a given timestamp
  /// * `subject` - [format: uri]
  /// * `collections` - [max_length: 20] If specified, only events where the subject belongs to the given collections will be returned. When subjectType is set to 'account', this will be ignored.
  /// * `subject_type` - [known_values: ["account", "record"]] If specified, only events where the subject is of the given type (account or record) will be returned. When this is set to 'account' the 'collections' parameter will be ignored. When includeAllUserRecords or subject is set, this will be ignored.
  /// * `include_all_user_records` - [default: false] If true, events on all record types (posts, lists, profile etc.) or records from given 'collections' param, owned by the did are returned.
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `has_comment` - If true, only events with comments are returned
  /// * `comment` - If specified, only events with comments containing the keyword are returned
  /// * `added_labels` - If specified, only events where all of these labels were added are returned
  /// * `removed_labels` - If specified, only events where all of these labels were removed are returned
  /// * `added_tags` - If specified, only events where all of these tags were added are returned
  /// * `removed_tags` - If specified, only events where all of these tags were removed are returned
  /// * `report_types`
  /// * `cursor`
  pub async fn tools_ozone_moderation_query_events(
    &self,
    types: Option<&[&str]>,
    created_by: Option<&str>,
    sort_direction: Option<&str>,
    created_after: Option<&chrono::DateTime<chrono::Utc>>,
    created_before: Option<&chrono::DateTime<chrono::Utc>>,
    subject: Option<&str>,
    collections: Option<&[&str]>,
    subject_type: Option<&str>,
    include_all_user_records: Option<bool>,
    limit: Option<i64>,
    has_comment: Option<bool>,
    comment: Option<&str>,
    added_labels: Option<&[&str]>,
    removed_labels: Option<&[&str]>,
    added_tags: Option<&[&str]>,
    removed_tags: Option<&[&str]>,
    report_types: Option<&[&str]>,
    cursor: Option<&str>,
  ) -> Result<ToolsOzoneModerationQueryEventsOutput> {
    let mut query_ = Vec::new();
    if let Some(types) = &types {
      query_.append(
        &mut types
          .iter()
          .map(|i| (String::from("types"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(created_by) = &created_by {
      query_.push((String::from("created_by"), created_by.to_string()));
    }
    if let Some(sort_direction) = &sort_direction {
      query_.push((String::from("sort_direction"), sort_direction.to_string()));
    }
    if let Some(created_after) = &created_after {
      query_.push((String::from("created_after"), created_after.to_rfc3339()));
    }
    if let Some(created_before) = &created_before {
      query_.push((String::from("created_before"), created_before.to_rfc3339()));
    }
    if let Some(subject) = &subject {
      query_.push((String::from("subject"), subject.to_string()));
    }
    if let Some(collections) = &collections {
      query_.append(
        &mut collections
          .iter()
          .map(|i| (String::from("collections"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(subject_type) = &subject_type {
      query_.push((String::from("subject_type"), subject_type.to_string()));
    }
    if let Some(include_all_user_records) = &include_all_user_records {
      query_.push((
        String::from("include_all_user_records"),
        include_all_user_records.to_string(),
      ));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(has_comment) = &has_comment {
      query_.push((String::from("has_comment"), has_comment.to_string()));
    }
    if let Some(comment) = &comment {
      query_.push((String::from("comment"), comment.to_string()));
    }
    if let Some(added_labels) = &added_labels {
      query_.append(
        &mut added_labels
          .iter()
          .map(|i| (String::from("added_labels"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(removed_labels) = &removed_labels {
      query_.append(
        &mut removed_labels
          .iter()
          .map(|i| (String::from("removed_labels"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(added_tags) = &added_tags {
      query_.append(
        &mut added_tags
          .iter()
          .map(|i| (String::from("added_tags"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(removed_tags) = &removed_tags {
      query_.append(
        &mut removed_tags
          .iter()
          .map(|i| (String::from("removed_tags"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(report_types) = &report_types {
      query_.append(
        &mut report_types
          .iter()
          .map(|i| (String::from("report_types"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/tools.ozone.moderation.queryEvents",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// View moderation statuses of subjects (record or repo).
  ///
  /// # Arguments
  ///
  /// * `include_all_user_records` - All subjects, or subjects from given 'collections' param, belonging to the account specified in the 'subject' param will be returned.
  /// * `subject` - [format: uri] The subject to get the status for.
  /// * `comment` - Search subjects by keyword from comments
  /// * `reported_after` - [format: datetime] Search subjects reported after a given timestamp
  /// * `reported_before` - [format: datetime] Search subjects reported before a given timestamp
  /// * `reviewed_after` - [format: datetime] Search subjects reviewed after a given timestamp
  /// * `hosting_deleted_after` - [format: datetime] Search subjects where the associated record/account was deleted after a given timestamp
  /// * `hosting_deleted_before` - [format: datetime] Search subjects where the associated record/account was deleted before a given timestamp
  /// * `hosting_updated_after` - [format: datetime] Search subjects where the associated record/account was updated after a given timestamp
  /// * `hosting_updated_before` - [format: datetime] Search subjects where the associated record/account was updated before a given timestamp
  /// * `hosting_statuses` - Search subjects by the status of the associated record/account
  /// * `reviewed_before` - [format: datetime] Search subjects reviewed before a given timestamp
  /// * `include_muted` - By default, we don't include muted subjects in the results. Set this to true to include them.
  /// * `only_muted` - When set to true, only muted subjects and reporters will be returned.
  /// * `review_state` - Specify when fetching subjects in a certain state
  /// * `ignore_subjects`
  /// * `last_reviewed_by` - [format: did] Get all subject statuses that were reviewed by a specific moderator
  /// * `sort_field` - [default: lastReportedAt] [enum: ["lastReviewedAt", "lastReportedAt"]]
  /// * `sort_direction` - [default: desc] [enum: ["asc", "desc"]]
  /// * `takendown` - Get subjects that were taken down
  /// * `appealed` - Get subjects in unresolved appealed status
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `tags`
  /// * `exclude_tags`
  /// * `cursor`
  /// * `collections` - [max_length: 20] If specified, subjects belonging to the given collections will be returned. When subjectType is set to 'account', this will be ignored.
  /// * `subject_type` - [known_values: ["account", "record"]] If specified, subjects of the given type (account or record) will be returned. When this is set to 'account' the 'collections' parameter will be ignored. When includeAllUserRecords or subject is set, this will be ignored.
  pub async fn tools_ozone_moderation_query_statuses(
    &self,
    include_all_user_records: Option<bool>,
    subject: Option<&str>,
    comment: Option<&str>,
    reported_after: Option<&chrono::DateTime<chrono::Utc>>,
    reported_before: Option<&chrono::DateTime<chrono::Utc>>,
    reviewed_after: Option<&chrono::DateTime<chrono::Utc>>,
    hosting_deleted_after: Option<&chrono::DateTime<chrono::Utc>>,
    hosting_deleted_before: Option<&chrono::DateTime<chrono::Utc>>,
    hosting_updated_after: Option<&chrono::DateTime<chrono::Utc>>,
    hosting_updated_before: Option<&chrono::DateTime<chrono::Utc>>,
    hosting_statuses: Option<&[&str]>,
    reviewed_before: Option<&chrono::DateTime<chrono::Utc>>,
    include_muted: Option<bool>,
    only_muted: Option<bool>,
    review_state: Option<&str>,
    ignore_subjects: Option<&[&str]>,
    last_reviewed_by: Option<&str>,
    sort_field: Option<&str>,
    sort_direction: Option<&str>,
    takendown: Option<bool>,
    appealed: Option<bool>,
    limit: Option<i64>,
    tags: Option<&[&str]>,
    exclude_tags: Option<&[&str]>,
    cursor: Option<&str>,
    collections: Option<&[&str]>,
    subject_type: Option<&str>,
  ) -> Result<ToolsOzoneModerationQueryStatusesOutput> {
    let mut query_ = Vec::new();
    if let Some(include_all_user_records) = &include_all_user_records {
      query_.push((
        String::from("include_all_user_records"),
        include_all_user_records.to_string(),
      ));
    }
    if let Some(subject) = &subject {
      query_.push((String::from("subject"), subject.to_string()));
    }
    if let Some(comment) = &comment {
      query_.push((String::from("comment"), comment.to_string()));
    }
    if let Some(reported_after) = &reported_after {
      query_.push((String::from("reported_after"), reported_after.to_rfc3339()));
    }
    if let Some(reported_before) = &reported_before {
      query_.push((
        String::from("reported_before"),
        reported_before.to_rfc3339(),
      ));
    }
    if let Some(reviewed_after) = &reviewed_after {
      query_.push((String::from("reviewed_after"), reviewed_after.to_rfc3339()));
    }
    if let Some(hosting_deleted_after) = &hosting_deleted_after {
      query_.push((
        String::from("hosting_deleted_after"),
        hosting_deleted_after.to_rfc3339(),
      ));
    }
    if let Some(hosting_deleted_before) = &hosting_deleted_before {
      query_.push((
        String::from("hosting_deleted_before"),
        hosting_deleted_before.to_rfc3339(),
      ));
    }
    if let Some(hosting_updated_after) = &hosting_updated_after {
      query_.push((
        String::from("hosting_updated_after"),
        hosting_updated_after.to_rfc3339(),
      ));
    }
    if let Some(hosting_updated_before) = &hosting_updated_before {
      query_.push((
        String::from("hosting_updated_before"),
        hosting_updated_before.to_rfc3339(),
      ));
    }
    if let Some(hosting_statuses) = &hosting_statuses {
      query_.append(
        &mut hosting_statuses
          .iter()
          .map(|i| (String::from("hosting_statuses"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(reviewed_before) = &reviewed_before {
      query_.push((
        String::from("reviewed_before"),
        reviewed_before.to_rfc3339(),
      ));
    }
    if let Some(include_muted) = &include_muted {
      query_.push((String::from("include_muted"), include_muted.to_string()));
    }
    if let Some(only_muted) = &only_muted {
      query_.push((String::from("only_muted"), only_muted.to_string()));
    }
    if let Some(review_state) = &review_state {
      query_.push((String::from("review_state"), review_state.to_string()));
    }
    if let Some(ignore_subjects) = &ignore_subjects {
      query_.append(
        &mut ignore_subjects
          .iter()
          .map(|i| (String::from("ignore_subjects"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(last_reviewed_by) = &last_reviewed_by {
      query_.push((
        String::from("last_reviewed_by"),
        last_reviewed_by.to_string(),
      ));
    }
    if let Some(sort_field) = &sort_field {
      query_.push((String::from("sort_field"), sort_field.to_string()));
    }
    if let Some(sort_direction) = &sort_direction {
      query_.push((String::from("sort_direction"), sort_direction.to_string()));
    }
    if let Some(takendown) = &takendown {
      query_.push((String::from("takendown"), takendown.to_string()));
    }
    if let Some(appealed) = &appealed {
      query_.push((String::from("appealed"), appealed.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(tags) = &tags {
      query_.append(
        &mut tags
          .iter()
          .map(|i| (String::from("tags"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(exclude_tags) = &exclude_tags {
      query_.append(
        &mut exclude_tags
          .iter()
          .map(|i| (String::from("exclude_tags"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    if let Some(collections) = &collections {
      query_.append(
        &mut collections
          .iter()
          .map(|i| (String::from("collections"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(subject_type) = &subject_type {
      query_.push((String::from("subject_type"), subject_type.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/tools.ozone.moderation.queryStatuses",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Find repositories based on a search term.
  ///
  /// # Arguments
  ///
  /// * `term` - DEPRECATED: use 'q' instead
  /// * `q`
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  pub async fn tools_ozone_moderation_search_repos(
    &self,
    term: Option<&str>,
    q: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ToolsOzoneModerationSearchReposOutput> {
    let mut query_ = Vec::new();
    if let Some(term) = &term {
      query_.push((String::from("term"), term.to_string()));
    }
    if let Some(q) = &q {
      query_.push((String::from("q"), q.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/tools.ozone.moderation.searchRepos",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get details about ozone's server configuration.
  pub async fn tools_ozone_server_get_config(&self) -> Result<ToolsOzoneServerGetConfigOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/tools.ozone.server.getConfig",
      self.host
    ));
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Add values to a specific set. Attempting to add values to a set that does not exist will result in an error.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn tools_ozone_set_add_values(&self, body: ToolsOzoneSetAddValuesInput) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/tools.ozone.set.addValues",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Delete an entire set. Attempting to delete a set that does not exist will result in an error.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `SetNotFound` - set with the given name does not exist
  pub async fn tools_ozone_set_delete_set(
    &self,
    body: ToolsOzoneSetDeleteSetInput,
  ) -> Result<serde_json::Value> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/tools.ozone.set.deleteSet",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Delete values from a specific set. Attempting to delete values that are not in the set will not result in an error
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `SetNotFound` - set with the given name does not exist
  pub async fn tools_ozone_set_delete_values(
    &self,
    body: ToolsOzoneSetDeleteValuesInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/tools.ozone.set.deleteValues",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Get a specific set and its values
  ///
  /// # Arguments
  ///
  /// * `name`
  /// * `limit` - [minimum: 1] [maximum: 1000] [default: 100]
  /// * `cursor`
  ///
  /// # Errors
  ///
  /// * `SetNotFound` - set with the given name does not exist
  pub async fn tools_ozone_set_get_values(
    &self,
    name: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ToolsOzoneSetGetValuesOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("name"), name.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/tools.ozone.set.getValues",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Query available sets
  ///
  /// # Arguments
  ///
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  /// * `name_prefix`
  /// * `sort_by` - [default: name] [enum: ["name", "createdAt", "updatedAt"]]
  /// * `sort_direction` - [default: asc] [enum: ["asc", "desc"]] Defaults to ascending order of name field.
  pub async fn tools_ozone_set_query_sets(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
    name_prefix: Option<&str>,
    sort_by: Option<&str>,
    sort_direction: Option<&str>,
  ) -> Result<ToolsOzoneSetQuerySetsOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    if let Some(name_prefix) = &name_prefix {
      query_.push((String::from("name_prefix"), name_prefix.to_string()));
    }
    if let Some(sort_by) = &sort_by {
      query_.push((String::from("sort_by"), sort_by.to_string()));
    }
    if let Some(sort_direction) = &sort_direction {
      query_.push((String::from("sort_direction"), sort_direction.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/tools.ozone.set.querySets",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Create or update set metadata
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn tools_ozone_set_upsert_set(
    &self,
    body: ToolsOzoneSetDefsSet,
  ) -> Result<ToolsOzoneSetDefsSetView> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/tools.ozone.set.upsertSet",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// List settings with optional filtering
  ///
  /// # Arguments
  ///
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  /// * `scope` - [known_values: ["instance", "personal"]] [default: instance]
  /// * `prefix` - Filter keys by prefix
  /// * `keys` - [max_length: 100] Filter for only the specified keys. Ignored if prefix is provided
  pub async fn tools_ozone_setting_list_options(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
    scope: Option<&str>,
    prefix: Option<&str>,
    keys: Option<&[&str]>,
  ) -> Result<ToolsOzoneSettingListOptionsOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    if let Some(scope) = &scope {
      query_.push((String::from("scope"), scope.to_string()));
    }
    if let Some(prefix) = &prefix {
      query_.push((String::from("prefix"), prefix.to_string()));
    }
    if let Some(keys) = &keys {
      query_.append(
        &mut keys
          .iter()
          .map(|i| (String::from("keys"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/tools.ozone.setting.listOptions",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Delete settings by key
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn tools_ozone_setting_remove_options(
    &self,
    body: ToolsOzoneSettingRemoveOptionsInput,
  ) -> Result<serde_json::Value> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/tools.ozone.setting.removeOptions",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Create or update setting option
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn tools_ozone_setting_upsert_option(
    &self,
    body: ToolsOzoneSettingUpsertOptionInput,
  ) -> Result<ToolsOzoneSettingUpsertOptionOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/tools.ozone.setting.upsertOption",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Find all correlated threat signatures between 2 or more accounts.
  ///
  /// # Arguments
  ///
  /// * `dids`
  pub async fn tools_ozone_signature_find_correlation(
    &self,
    dids: &[&str],
  ) -> Result<ToolsOzoneSignatureFindCorrelationOutput> {
    let mut query_ = Vec::new();
    query_.append(
      &mut dids
        .iter()
        .map(|i| (String::from("dids"), i.to_string()))
        .collect::<Vec<_>>(),
    );
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/tools.ozone.signature.findCorrelation",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get accounts that share some matching threat signatures with the root account.
  ///
  /// # Arguments
  ///
  /// * `did` - [format: did]
  /// * `cursor`
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  pub async fn tools_ozone_signature_find_related_accounts(
    &self,
    did: &str,
    cursor: Option<&str>,
    limit: Option<i64>,
  ) -> Result<ToolsOzoneSignatureFindRelatedAccountsOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("did"), did.to_string()));
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/tools.ozone.signature.findRelatedAccounts",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Search for accounts that match one or more threat signature values.
  ///
  /// # Arguments
  ///
  /// * `values`
  /// * `cursor`
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  pub async fn tools_ozone_signature_search_accounts(
    &self,
    values: &[&str],
    cursor: Option<&str>,
    limit: Option<i64>,
  ) -> Result<ToolsOzoneSignatureSearchAccountsOutput> {
    let mut query_ = Vec::new();
    query_.append(
      &mut values
        .iter()
        .map(|i| (String::from("values"), i.to_string()))
        .collect::<Vec<_>>(),
    );
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/tools.ozone.signature.searchAccounts",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Add a member to the ozone team. Requires admin role.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `MemberAlreadyExists` - Member already exists in the team.
  pub async fn tools_ozone_team_add_member(
    &self,
    body: ToolsOzoneTeamAddMemberInput,
  ) -> Result<ToolsOzoneTeamDefsMember> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/tools.ozone.team.addMember",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Delete a member from ozone team. Requires admin role.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `MemberNotFound` - The member being deleted does not exist
  /// * `CannotDeleteSelf` - You can not delete yourself from the team
  pub async fn tools_ozone_team_delete_member(
    &self,
    body: ToolsOzoneTeamDeleteMemberInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/tools.ozone.team.deleteMember",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// List all members with access to the ozone service.
  ///
  /// # Arguments
  ///
  /// * `limit` - [minimum: 1] [maximum: 100] [default: 50]
  /// * `cursor`
  pub async fn tools_ozone_team_list_members(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ToolsOzoneTeamListMembersOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/tools.ozone.team.listMembers",
        self.host
      ))
      .query(&query_);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Update a member in the ozone service. Requires admin role.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `MemberNotFound` - The member being updated does not exist in the team
  pub async fn tools_ozone_team_update_member(
    &self,
    body: ToolsOzoneTeamUpdateMemberInput,
  ) -> Result<ToolsOzoneTeamDefsMember> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/tools.ozone.team.updateMember",
        self.host
      ))
      .json(&body);
    if let Some(token) = { self.access_jwt.read().await.clone() } {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }
}
