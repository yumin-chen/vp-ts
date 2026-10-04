use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::aead::{self, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey, UnboundKey, NONCE_LEN};

struct OneNonceSequence(Option<[u8; NONCE_LEN]>);

impl NonceSequence for OneNonceSequence {
  fn advance(&mut self) -> std::result::Result<Nonce, ring::error::Unspecified> {
    self
      .0
      .take()
      .map(Nonce::assume_unique_for_key)
      .ok_or(ring::error::Unspecified)
  }
}

#[napi]
pub struct Cipher {
  algorithm: String,
  key: Vec<u8>,
  iv: Vec<u8>,
  aad: Vec<u8>,
  plain_text: Vec<u8>,
  auth_tag: Option<Vec<u8>>,
}

#[napi]
impl Cipher {
  #[napi(constructor)]
  pub fn new(algorithm: String, key: Buffer, iv: Buffer) -> Result<Self> {
    Ok(Cipher {
      algorithm,
      key: key.as_ref().to_vec(),
      iv: iv.as_ref().to_vec(),
      aad: Vec::new(),
      plain_text: Vec::new(),
      auth_tag: None,
    })
  }

  #[napi]
  pub fn set_aad(&mut self, buffer: Buffer) -> Result<()> {
    self.aad.extend_from_slice(buffer.as_ref());
    Ok(())
  }

  #[napi]
  pub fn update(&mut self, data: Either<Buffer, String>) -> Result<Buffer> {
    let bytes = match data {
      Either::A(b) => b.as_ref().to_vec(),
      Either::B(s) => s.as_bytes().to_vec(),
    };
    self.plain_text.extend_from_slice(&bytes);
    Ok(Buffer::from(vec![]))
  }

  #[napi]
  pub fn final_(&mut self) -> Result<Buffer> {
    let alg_str = self.algorithm.to_lowercase().replace('-', "");
    let ring_alg = match alg_str.as_str() {
      "aes128gcm" => &aead::AES_128_GCM,
      "aes256gcm" => &aead::AES_256_GCM,
      "chacha20poly1305" => &aead::CHACHA20_POLY1305,
      _ => &aead::AES_256_GCM,
    };

    let unbound_key = UnboundKey::new(ring_alg, &self.key)
      .map_err(|_| Error::from_reason("Invalid key length for cipher"))?;

    let mut nonce_bytes = [0u8; NONCE_LEN];
    let len = self.iv.len().min(NONCE_LEN);
    nonce_bytes[..len].copy_from_slice(&self.iv[..len]);

    let nonce_seq = OneNonceSequence(Some(nonce_bytes));
    let mut sealing_key = SealingKey::new(unbound_key, nonce_seq);

    let mut in_out = self.plain_text.clone();
    let aad = aead::Aad::from(&self.aad);

    let tag = sealing_key
      .seal_in_place_separate_tag(aad, &mut in_out)
      .map_err(|_| Error::from_reason("Encryption failed"))?;

    self.auth_tag = Some(tag.as_ref().to_vec());
    Ok(Buffer::from(in_out))
  }

  #[napi]
  pub fn get_auth_tag(&self) -> Result<Buffer> {
    match &self.auth_tag {
      Some(tag) => Ok(Buffer::from(tag.clone())),
      None => Err(Error::from_reason("Auth tag not generated yet. Call final() first.")),
    }
  }
}

#[napi]
pub struct Decipher {
  algorithm: String,
  key: Vec<u8>,
  iv: Vec<u8>,
  aad: Vec<u8>,
  cipher_text: Vec<u8>,
  auth_tag: Vec<u8>,
}

#[napi]
impl Decipher {
  #[napi(constructor)]
  pub fn new(algorithm: String, key: Buffer, iv: Buffer) -> Result<Self> {
    Ok(Decipher {
      algorithm,
      key: key.as_ref().to_vec(),
      iv: iv.as_ref().to_vec(),
      aad: Vec::new(),
      cipher_text: Vec::new(),
      auth_tag: Vec::new(),
    })
  }

  #[napi]
  pub fn set_aad(&mut self, buffer: Buffer) -> Result<()> {
    self.aad.extend_from_slice(buffer.as_ref());
    Ok(())
  }

  #[napi]
  pub fn set_auth_tag(&mut self, buffer: Buffer) -> Result<()> {
    self.auth_tag = buffer.as_ref().to_vec();
    Ok(())
  }

  #[napi]
  pub fn update(&mut self, data: Either<Buffer, String>) -> Result<Buffer> {
    let bytes = match data {
      Either::A(b) => b.as_ref().to_vec(),
      Either::B(s) => s.as_bytes().to_vec(),
    };
    self.cipher_text.extend_from_slice(&bytes);
    Ok(Buffer::from(vec![]))
  }

  #[napi]
  pub fn final_(&mut self) -> Result<Buffer> {
    let alg_str = self.algorithm.to_lowercase().replace('-', "");
    let ring_alg = match alg_str.as_str() {
      "aes128gcm" => &aead::AES_128_GCM,
      "aes256gcm" => &aead::AES_256_GCM,
      "chacha20poly1305" => &aead::CHACHA20_POLY1305,
      _ => &aead::AES_256_GCM,
    };

    let unbound_key = UnboundKey::new(ring_alg, &self.key)
      .map_err(|_| Error::from_reason("Invalid key length for decipher"))?;

    let mut nonce_bytes = [0u8; NONCE_LEN];
    let len = self.iv.len().min(NONCE_LEN);
    nonce_bytes[..len].copy_from_slice(&self.iv[..len]);

    let nonce_seq = OneNonceSequence(Some(nonce_bytes));
    let mut opening_key = OpeningKey::new(unbound_key, nonce_seq);

    let mut in_out = self.cipher_text.clone();
    in_out.extend_from_slice(&self.auth_tag);
    let aad = aead::Aad::from(&self.aad);

    let decrypted = opening_key
      .open_in_place(aad, &mut in_out)
      .map_err(|_| Error::from_reason("Decryption/authentication failed"))?;

    Ok(Buffer::from(decrypted.to_vec()))
  }
}

#[napi(js_name = "createCipheriv")]
pub fn create_cipheriv(algorithm: String, key: Buffer, iv: Buffer) -> Result<Cipher> {
  Cipher::new(algorithm, key, iv)
}

#[napi(js_name = "createDecipheriv")]
pub fn create_decipheriv(algorithm: String, key: Buffer, iv: Buffer) -> Result<Decipher> {
  Decipher::new(algorithm, key, iv)
}
