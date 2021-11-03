use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Target {
    Http(HttpTarget),
    //S3File { bucket: String, file_name: String },
    //S3Bucket { bucket: String },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HttpTarget {
    pub url: String,
    pub headers: Vec<Header>,
    pub method: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ItemConfig {
    pub file_name: String,
    pub target: Target,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CacheConfig {
    pub item_configs: Vec<ItemConfig>,
}

pub fn parse_config(json: &str) -> Result<CacheConfig> {
    match serde_json::from_str(json) {
        Ok(config) => Ok(config),
        Err(e) => Err(anyhow::Error::from(e)),
    }
}

pub fn parse_config_from_envar() -> Result<CacheConfig> {
    let json = std::env::var("CACHETTE_CONFIG")?;
    parse_config(&json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_deserialize_config() {
        let string_config = r#"{
          "itemConfigs": [
            {
              "fileName": "test",
              "target": {
                "type": "http",
                "url": "http://testurl.com",
                "headers": [],
                "method": "get"
              }
            }
          ]
        }"#;
        let config = CacheConfig {
            item_configs: vec![ItemConfig {
                file_name: "test".to_string(),
                target: Target::Http(HttpTarget {
                    url: "http://testurl.com".to_string(),
                    headers: vec![],
                    method: "get".to_string(),
                }),
            }],
        };
        let config2: CacheConfig = serde_json::from_str(string_config).unwrap();
        assert_eq!(config, config2);
    }
}
