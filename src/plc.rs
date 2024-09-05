#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidDocumentVerificationMethod {
  pub id: String,
  #[serde(rename = "type")]
  pub type_: String,
  pub controller: String,
  pub public_key_multibase: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidDocumentService {
  pub id: String,
  #[serde(rename = "type")]
  pub type_: String,
  pub service_endpoint: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidDocument {
  pub id: String,
  pub also_known_as: Vec<String>,
  pub verification_method: Vec<DidDocumentVerificationMethod>,
  pub service: Vec<DidDocumentService>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlcOperation {
  rotation_keys: Vec<String>,
  verification_methods: DidDocumentVerificationMethod,
  also_known_as: Vec<String>,
  services: serde_json::Value,
  prev: Option<String>,
  sig: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlcTombstone {
  prev: String,
  sig: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlcCreate {
  signing_key: String,
  recovery_key: String,
  handle: String,
  service: String,
  prev: Option<String>,
  sig: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PlcDidOperation {
  PlcOperation(PlcOperation),
  PlcTombstone(PlcTombstone),
  Create(PlcCreate),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlcEntry {
  did: String,
  operation: PlcDidOperation,
  cid: String,
  nullified: bool,
  created_at: String,
}

pub struct Plc {
  pub host: String,
  pub client: reqwest::Client,
}

impl Default for Plc {
  fn default() -> Self {
    Self::new("plc.directory")
  }
}

impl Plc {
  pub fn new(host: &str) -> Self {
    Self {
      host: host.to_string(),
      client: reqwest::Client::new(),
    }
  }

  pub async fn resolve_did(&self, did: &str) -> anyhow::Result<DidDocument> {
    let text = self
      .client
      .get(format!("https://{}/{did}", self.host))
      .send()
      .await?
      .text()
      .await?;
    Ok(serde_json::from_str(&text)?)
  }

  pub async fn export(&self) -> anyhow::Result<Vec<PlcEntry>> {
    let text = self
      .client
      .get(format!("https://{}/export", self.host))
      .send()
      .await?
      .text()
      .await?;
    Ok(
      text
        .split("\n")
        .filter_map(|t| serde_json::from_str(t).ok())
        .collect::<Vec<_>>(),
    )
  }
}
