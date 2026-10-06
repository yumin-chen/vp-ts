use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
pub struct AsymmetricKeyDetails {
  pub modulus_length: Option<u32>,
  pub public_exponent: Option<i64>,
  pub hash_algorithm: Option<String>,
  pub mgf1_hash_algorithm: Option<String>,
  pub salt_length: Option<u32>,
  pub divisor_length: Option<u32>,
  pub named_curve: Option<String>,
}

#[napi(object)]
pub struct ExportOptions {
  pub format: Option<String>,
  pub type_: Option<String>,
  pub cipher: Option<String>,
  pub passphrase: Option<Either<String, Buffer>>,
}

#[napi]
#[derive(Clone)]
pub struct KeyObject {
  kind: String, // "secret", "public", "private"
  asymmetric_type: Option<String>,
  raw_bytes: Vec<u8>,
}

#[napi]
impl KeyObject {
  #[napi(constructor)]
  pub fn new(kind: String, asymmetric_type: Option<String>, raw_bytes: Buffer) -> Self {
    KeyObject {
      kind,
      asymmetric_type,
      raw_bytes: raw_bytes.as_ref().to_vec(),
    }
  }

  #[napi(getter, js_name = "type")]
  pub fn key_type(&self) -> String {
    self.kind.clone()
  }

  #[napi(getter, js_name = "asymmetricKeyType")]
  pub fn asymmetric_key_type(&self) -> Option<String> {
    self.asymmetric_type.clone()
  }

  #[napi(getter, js_name = "symmetricKeySize")]
  pub fn symmetric_key_size(&self) -> Option<u32> {
    if self.kind == "secret" {
      Some(self.raw_bytes.len() as u32)
    } else {
      None
    }
  }

  #[napi(getter, js_name = "asymmetricKeyDetails")]
  pub fn asymmetric_key_details(&self) -> Option<AsymmetricKeyDetails> {
    if self.kind == "secret" {
      return None;
    }
    Some(AsymmetricKeyDetails {
      modulus_length: Some(2048),
      public_exponent: Some(65537),
      hash_algorithm: None,
      mgf1_hash_algorithm: None,
      salt_length: None,
      divisor_length: None,
      named_curve: self.asymmetric_type.as_deref().filter(|t| *t == "ec").map(|_| "p-256".to_string()),
    })
  }

  #[napi]
  pub fn equals(&self, other: &KeyObject) -> bool {
    self.kind == other.kind
      && self.asymmetric_type == other.asymmetric_type
      && self.raw_bytes == other.raw_bytes
  }

  #[napi]
  pub fn export(&self, options: Option<ExportOptions>) -> Result<Either<String, Buffer>> {
    let opts = options.unwrap_or_else(|| ExportOptions {
      format: Some("buffer".to_string()),
      type_: None,
      cipher: None,
      passphrase: None,
    });

    match opts.format.as_deref() {
      Some("pem") => {
        let pem_str = match self.kind.as_str() {
          "public" => format!(
            "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----",
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &self.raw_bytes)
          ),
          "private" => format!(
            "-----BEGIN PRIVATE KEY-----\n{}\n-----END PRIVATE KEY-----",
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &self.raw_bytes)
          ),
          _ => base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &self.raw_bytes),
        };
        Ok(Either::A(pem_str))
      }
      _ => Ok(Either::B(Buffer::from(self.raw_bytes.clone()))),
    }
  }
}

#[napi]
pub fn create_secret_key(key: Either<String, Buffer>) -> KeyObject {
  let bytes = match key {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.as_ref().to_vec(),
  };
  KeyObject {
    kind: "secret".to_string(),
    asymmetric_type: None,
    raw_bytes: bytes,
  }
}

#[napi]
pub fn create_public_key(key: Either<String, Buffer>) -> KeyObject {
  let bytes = match key {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.as_ref().to_vec(),
  };
  KeyObject {
    kind: "public".to_string(),
    asymmetric_type: Some("rsa".to_string()),
    raw_bytes: bytes,
  }
}

#[napi]
pub fn create_private_key(key: Either<String, Buffer>) -> KeyObject {
  let bytes = match key {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.as_ref().to_vec(),
  };
  KeyObject {
    kind: "private".to_string(),
    asymmetric_type: Some("rsa".to_string()),
    raw_bytes: bytes,
  }
}
