#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PoolMode {
  Nano,
  Smart,
}
