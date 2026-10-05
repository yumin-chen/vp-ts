use aes_gcm::aead::{Aead, KeyInit, Payload};
use aes_gcm::{Aes128Gcm, Aes256Gcm, Nonce as GcmNonce};
use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::hmac::{decode_input, encode_output};

#[napi(js_name = "Cipheriv")]
pub struct Cipheriv {
  algorithm: String,
  key: Vec<u8>,
  iv: Vec<u8>,
  buffer: Vec<u8>,
  aad: Vec<u8>,
  auth_tag: Option<Vec<u8>>,
  auto_padding: bool,
}

#[napi]
impl Cipheriv {
  #[napi(constructor, ts_args_type = "algorithm: string, key: string | Buffer, iv: string | Buffer")]
  pub fn new(algorithm: String, key: Either<String, Buffer>, iv: Either<String, Buffer>) -> Self {
    Cipheriv {
      algorithm,
      key: decode_input(&key, None),
      iv: decode_input(&iv, None),
      buffer: Vec::new(),
      aad: Vec::new(),
      auth_tag: None,
      auto_padding: true,
    }
  }

  #[napi(ts_args_type = "data: string | Buffer, inputEncoding?: string, outputEncoding?: string", ts_return_type = "string | Buffer")]
  pub fn update(&mut self, data: Either<String, Buffer>, input_encoding: Option<String>, output_encoding: Option<String>) -> Either<String, Buffer> {
    let bytes = decode_input(&data, input_encoding.as_deref());
    self.buffer.extend_from_slice(&bytes);
    encode_output(&[], output_encoding.as_deref())
  }

  #[napi(ts_args_type = "buffer: string | Buffer")]
  pub fn set_aad(&mut self, buffer: Either<String, Buffer>) {
    let bytes = decode_input(&buffer, None);
    self.aad = bytes;
  }

  #[napi]
  pub fn set_auto_padding(&mut self, auto_padding: Option<bool>) {
    self.auto_padding = auto_padding.unwrap_or(true);
  }

  #[napi(js_name = "final", ts_args_type = "outputEncoding?: string", ts_return_type = "string | Buffer")]
  pub fn final_cipher(&mut self, output_encoding: Option<String>) -> napi::Result<Either<String, Buffer>> {
    let algo = self.algorithm.to_lowercase().replace("-", "");
    match algo.as_str() {
      "aes128gcm" => {
        if self.key.len() < 16 || self.iv.len() < 12 {
          return Err(napi::Error::from_reason("Invalid key or IV length for AES-128-GCM"));
        }
        let key = aes_gcm::Key::<Aes128Gcm>::from_slice(&self.key[..16]);
        let cipher = Aes128Gcm::new(key);
        let nonce = GcmNonce::from_slice(&self.iv[..12]);
        let payload = Payload {
          msg: self.buffer.as_slice(),
          aad: self.aad.as_slice(),
        };
        let ciphertext = cipher
          .encrypt(nonce, payload)
          .map_err(|_| napi::Error::from_reason("AES-128-GCM encryption failed"))?;

        if ciphertext.len() >= 16 {
          let split_idx = ciphertext.len() - 16;
          self.auth_tag = Some(ciphertext[split_idx..].to_vec());
          Ok(encode_output(&ciphertext[..split_idx], output_encoding.as_deref()))
        } else {
          Ok(encode_output(&ciphertext, output_encoding.as_deref()))
        }
      }
      "aes256gcm" => {
        if self.key.len() < 32 || self.iv.len() < 12 {
          return Err(napi::Error::from_reason("Invalid key or IV length for AES-256-GCM"));
        }
        let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&self.key[..32]);
        let cipher = Aes256Gcm::new(key);
        let nonce = GcmNonce::from_slice(&self.iv[..12]);
        let payload = Payload {
          msg: self.buffer.as_slice(),
          aad: self.aad.as_slice(),
        };
        let ciphertext = cipher
          .encrypt(nonce, payload)
          .map_err(|_| napi::Error::from_reason("AES-256-GCM encryption failed"))?;

        if ciphertext.len() >= 16 {
          let split_idx = ciphertext.len() - 16;
          self.auth_tag = Some(ciphertext[split_idx..].to_vec());
          Ok(encode_output(&ciphertext[..split_idx], output_encoding.as_deref()))
        } else {
          Ok(encode_output(&ciphertext, output_encoding.as_deref()))
        }
      }
      _ => Err(napi::Error::from_reason(format!("Unsupported cipher algorithm: {}", self.algorithm))),
    }
  }

  #[napi]
  pub fn get_auth_tag(&self) -> napi::Result<Buffer> {
    if let Some(tag) = &self.auth_tag {
      Ok(Buffer::from(tag.clone()))
    } else {
      Err(napi::Error::from_reason("Auth tag not ready. Call final() first."))
    }
  }
}

#[napi]
pub fn create_cipheriv(
  algorithm: String,
  key: Either<String, Buffer>,
  iv: Either<String, Buffer>,
) -> Cipheriv {
  Cipheriv::new(algorithm, key, iv)
}

#[napi(js_name = "Decipheriv")]
pub struct Decipheriv {
  algorithm: String,
  key: Vec<u8>,
  iv: Vec<u8>,
  buffer: Vec<u8>,
  aad: Vec<u8>,
  auth_tag: Vec<u8>,
  auto_padding: bool,
}

#[napi]
impl Decipheriv {
  #[napi(constructor, ts_args_type = "algorithm: string, key: string | Buffer, iv: string | Buffer")]
  pub fn new(algorithm: String, key: Either<String, Buffer>, iv: Either<String, Buffer>) -> Self {
    Decipheriv {
      algorithm,
      key: decode_input(&key, None),
      iv: decode_input(&iv, None),
      buffer: Vec::new(),
      aad: Vec::new(),
      auth_tag: Vec::new(),
      auto_padding: true,
    }
  }

  #[napi(ts_args_type = "data: string | Buffer, inputEncoding?: string, outputEncoding?: string", ts_return_type = "string | Buffer")]
  pub fn update(&mut self, data: Either<String, Buffer>, input_encoding: Option<String>, output_encoding: Option<String>) -> Either<String, Buffer> {
    let bytes = decode_input(&data, input_encoding.as_deref());
    self.buffer.extend_from_slice(&bytes);
    encode_output(&[], output_encoding.as_deref())
  }

  #[napi(ts_args_type = "buffer: string | Buffer")]
  pub fn set_aad(&mut self, buffer: Either<String, Buffer>) {
    let bytes = decode_input(&buffer, None);
    self.aad = bytes;
  }

  #[napi(ts_args_type = "tag: Buffer")]
  pub fn set_auth_tag(&mut self, tag: Buffer) {
    self.auth_tag = tag.to_vec();
  }

  #[napi]
  pub fn set_auto_padding(&mut self, auto_padding: Option<bool>) {
    self.auto_padding = auto_padding.unwrap_or(true);
  }

  #[napi(js_name = "final", ts_args_type = "outputEncoding?: string", ts_return_type = "string | Buffer")]
  pub fn final_cipher(&mut self, output_encoding: Option<String>) -> napi::Result<Either<String, Buffer>> {
    let algo = self.algorithm.to_lowercase().replace("-", "");
    match algo.as_str() {
      "aes128gcm" => {
        if self.key.len() < 16 || self.iv.len() < 12 {
          return Err(napi::Error::from_reason("Invalid key or IV length for AES-128-GCM"));
        }
        let key = aes_gcm::Key::<Aes128Gcm>::from_slice(&self.key[..16]);
        let cipher = Aes128Gcm::new(key);
        let nonce = GcmNonce::from_slice(&self.iv[..12]);

        let mut payload_msg = self.buffer.clone();
        payload_msg.extend_from_slice(&self.auth_tag);

        let payload = Payload {
          msg: payload_msg.as_slice(),
          aad: self.aad.as_slice(),
        };

        let plaintext = cipher
          .decrypt(nonce, payload)
          .map_err(|_| napi::Error::from_reason("AES-128-GCM decryption failed"))?;

        Ok(encode_output(&plaintext, output_encoding.as_deref()))
      }
      "aes256gcm" => {
        if self.key.len() < 32 || self.iv.len() < 12 {
          return Err(napi::Error::from_reason("Invalid key or IV length for AES-256-GCM"));
        }
        let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&self.key[..32]);
        let cipher = Aes256Gcm::new(key);
        let nonce = GcmNonce::from_slice(&self.iv[..12]);

        let mut payload_msg = self.buffer.clone();
        payload_msg.extend_from_slice(&self.auth_tag);

        let payload = Payload {
          msg: payload_msg.as_slice(),
          aad: self.aad.as_slice(),
        };

        let plaintext = cipher
          .decrypt(nonce, payload)
          .map_err(|_| napi::Error::from_reason("AES-256-GCM decryption failed"))?;

        Ok(encode_output(&plaintext, output_encoding.as_deref()))
      }
      _ => Err(napi::Error::from_reason(format!("Unsupported cipher algorithm: {}", self.algorithm))),
    }
  }
}

#[napi]
pub fn create_decipheriv(
  algorithm: String,
  key: Either<String, Buffer>,
  iv: Either<String, Buffer>,
) -> Decipheriv {
  Decipheriv::new(algorithm, key, iv)
}
