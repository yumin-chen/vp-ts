use napi_derive::napi;
use serde_json::Value;

#[napi]
pub fn parse(text: String) -> napi::Result<Value> {
  let val: Value = toml::from_str(&text)
    .map_err(|e| napi::Error::from_reason(e.to_string()))?;
  Ok(val)
}

#[napi]
pub fn stringify(value: Value) -> napi::Result<String> {
  let text = toml::to_string(&value)
    .map_err(|e| napi::Error::from_reason(e.to_string()))?;
  Ok(text)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_stringify() {
    let toml_str = "title = \"TOML Example\"\n";
    let parsed = parse(toml_str.to_string()).unwrap();
    let stringified = stringify(parsed).unwrap();
    assert_eq!(stringified, "title = \"TOML Example\"\n");
  }
}
