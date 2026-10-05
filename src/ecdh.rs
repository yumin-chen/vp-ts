use napi::bindgen_prelude::*;
use napi_derive::napi;
use num_bigint_dig::BigUint;
use p256::SecretKey;
use ring::rand::SecureRandom;

use crate::hmac::{decode_input, encode_output};

fn random_key_bytes(len: usize) -> Vec<u8> {
  let rng = ring::rand::SystemRandom::new();
  let mut buf = vec![0u8; len];
  let _ = rng.fill(&mut buf);
  buf
}

#[napi(js_name = "ECDH")]
pub struct ECDH {
  pub curve_name: String,
  pub private_bytes: Vec<u8>,
  pub public_bytes: Vec<u8>,
}

#[napi]
impl ECDH {
  #[napi(constructor)]
  pub fn new(curve_name: String) -> Self {
    let priv_b = random_key_bytes(32);
    let secret = SecretKey::from_slice(&priv_b).unwrap_or_else(|_| SecretKey::from_slice(&[1u8; 32]).unwrap());
    let public_key = secret.public_key();

    ECDH {
      curve_name,
      private_bytes: secret.to_bytes().to_vec(),
      public_bytes: public_key.to_sec1_bytes().to_vec(),
    }
  }

  #[napi(ts_args_type = "key: string | Buffer, curve: string", ts_return_type = "Buffer")]
  pub fn convert_key(key: Either<String, Buffer>, _curve: String) -> Buffer {
    let bytes = decode_input(&key, None);
    Buffer::from(bytes)
  }

  #[napi(ts_args_type = "encoding?: string, format?: string", ts_return_type = "string | Buffer")]
  pub fn generate_keys(&mut self, encoding: Option<String>, _format: Option<String>) -> Either<String, Buffer> {
    let priv_b = random_key_bytes(32);
    let secret = SecretKey::from_slice(&priv_b).unwrap_or_else(|_| SecretKey::from_slice(&[1u8; 32]).unwrap());
    let public_key = secret.public_key();

    self.private_bytes = secret.to_bytes().to_vec();
    self.public_bytes = public_key.to_sec1_bytes().to_vec();

    encode_output(&self.public_bytes, encoding.as_deref())
  }

  #[napi(ts_args_type = "encoding?: string, format?: string", ts_return_type = "string | Buffer")]
  pub fn get_public_key(&self, encoding: Option<String>, _format: Option<String>) -> Either<String, Buffer> {
    encode_output(&self.public_bytes, encoding.as_deref())
  }

  #[napi(ts_args_type = "encoding?: string", ts_return_type = "string | Buffer")]
  pub fn get_private_key(&self, encoding: Option<String>) -> Either<String, Buffer> {
    encode_output(&self.private_bytes, encoding.as_deref())
  }

  #[napi(ts_args_type = "privateKey: string | Buffer, encoding?: string")]
  pub fn set_private_key(&mut self, private_key: Either<String, Buffer>, encoding: Option<String>) {
    let bytes = decode_input(&private_key, encoding.as_deref());
    if let Ok(secret) = SecretKey::from_slice(&bytes) {
      self.private_bytes = secret.to_bytes().to_vec();
      self.public_bytes = secret.public_key().to_sec1_bytes().to_vec();
    } else {
      self.private_bytes = bytes;
    }
  }

  #[napi(ts_args_type = "publicKey: string | Buffer, encoding?: string")]
  pub fn set_public_key(&mut self, public_key: Either<String, Buffer>, encoding: Option<String>) {
    let bytes = decode_input(&public_key, encoding.as_deref());
    self.public_bytes = bytes;
  }

  #[napi(ts_args_type = "otherPublicKey: string | Buffer, inputEncoding?: string, outputEncoding?: string", ts_return_type = "string | Buffer")]
  pub fn compute_secret(&self, other_public_key: Either<String, Buffer>, input_encoding: Option<String>, output_encoding: Option<String>) -> napi::Result<Either<String, Buffer>> {
    let other_bytes = decode_input(&other_public_key, input_encoding.as_deref());
    let secret_key = SecretKey::from_slice(&self.private_bytes)
      .map_err(|_| napi::Error::from_reason("Invalid private key"))?;
    let other_pk = p256::PublicKey::from_sec1_bytes(&other_bytes)
      .map_err(|_| napi::Error::from_reason("Invalid public key"))?;

    let shared = p256::ecdh::diffie_hellman(secret_key.to_nonzero_scalar(), other_pk.as_affine());
    Ok(encode_output(shared.raw_secret_bytes().as_slice(), output_encoding.as_deref()))
  }
}

#[napi]
pub fn create_ecdh(curve_name: String) -> ECDH {
  ECDH::new(curve_name)
}

#[napi(js_name = "DiffieHellman")]
pub struct DiffieHellman {
  pub group_or_prime: String,
  prime: BigUint,
  generator: BigUint,
  private_key: BigUint,
  public_key: BigUint,
}

#[napi]
impl DiffieHellman {
  #[napi(constructor)]
  pub fn new(group_or_prime: String) -> Self {
    let p = BigUint::from_bytes_be(&[
      0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xC9, 0x0F, 0xDA, 0xA2, 0x21, 0x68, 0xC2, 0x34,
      0xC4, 0xC6, 0x62, 0x8B, 0x80, 0xDC, 0x1C, 0xD1, 0x29, 0x02, 0x4E, 0x08, 0x8A, 0x67, 0xCC, 0x74,
    ]);
    let g = BigUint::from(2u32);
    let priv_bytes = random_key_bytes(32);
    let priv_key = BigUint::from_bytes_be(&priv_bytes);
    let pub_key = g.modpow(&priv_key, &p);

    DiffieHellman {
      group_or_prime,
      prime: p,
      generator: g,
      private_key: priv_key,
      public_key: pub_key,
    }
  }

  #[napi(ts_args_type = "encoding?: string", ts_return_type = "string | Buffer")]
  pub fn generate_keys(&mut self, encoding: Option<String>) -> Either<String, Buffer> {
    let priv_bytes = random_key_bytes(32);
    self.private_key = BigUint::from_bytes_be(&priv_bytes);
    self.public_key = self.generator.modpow(&self.private_key, &self.prime);
    encode_output(&self.public_key.to_bytes_be(), encoding.as_deref())
  }

  #[napi(ts_args_type = "encoding?: string", ts_return_type = "string | Buffer")]
  pub fn get_prime(&self, encoding: Option<String>) -> Either<String, Buffer> {
    encode_output(&self.prime.to_bytes_be(), encoding.as_deref())
  }

  #[napi(ts_args_type = "encoding?: string", ts_return_type = "string | Buffer")]
  pub fn get_generator(&self, encoding: Option<String>) -> Either<String, Buffer> {
    encode_output(&self.generator.to_bytes_be(), encoding.as_deref())
  }

  #[napi(ts_args_type = "encoding?: string", ts_return_type = "string | Buffer")]
  pub fn get_public_key(&self, encoding: Option<String>) -> Either<String, Buffer> {
    encode_output(&self.public_key.to_bytes_be(), encoding.as_deref())
  }

  #[napi(ts_args_type = "encoding?: string", ts_return_type = "string | Buffer")]
  pub fn get_private_key(&self, encoding: Option<String>) -> Either<String, Buffer> {
    encode_output(&self.private_key.to_bytes_be(), encoding.as_deref())
  }

  #[napi(ts_args_type = "privateKey: string | Buffer, encoding?: string")]
  pub fn set_private_key(&mut self, private_key: Either<String, Buffer>, encoding: Option<String>) {
    let bytes = decode_input(&private_key, encoding.as_deref());
    self.private_key = BigUint::from_bytes_be(&bytes);
    self.public_key = self.generator.modpow(&self.private_key, &self.prime);
  }

  #[napi(ts_args_type = "publicKey: string | Buffer, encoding?: string")]
  pub fn set_public_key(&mut self, public_key: Either<String, Buffer>, encoding: Option<String>) {
    let bytes = decode_input(&public_key, encoding.as_deref());
    self.public_key = BigUint::from_bytes_be(&bytes);
  }

  #[napi(getter)]
  pub fn verify_error(&self) -> u32 {
    0
  }

  #[napi(ts_args_type = "otherPublicKey: string | Buffer, inputEncoding?: string, outputEncoding?: string", ts_return_type = "string | Buffer")]
  pub fn compute_secret(&self, other_public_key: Either<String, Buffer>, input_encoding: Option<String>, output_encoding: Option<String>) -> Either<String, Buffer> {
    let other_bytes = decode_input(&other_public_key, input_encoding.as_deref());
    let other_pub = BigUint::from_bytes_be(&other_bytes);
    let secret = other_pub.modpow(&self.private_key, &self.prime);
    encode_output(&secret.to_bytes_be(), output_encoding.as_deref())
  }
}

#[napi]
pub fn create_diffie_hellman(group_or_prime: String) -> DiffieHellman {
  DiffieHellman::new(group_or_prime)
}

#[napi]
pub fn create_diffie_hellman_group(group_name: String) -> DiffieHellman {
  DiffieHellman::new(group_name)
}

#[napi]
pub fn get_diffie_hellman(group_name: String) -> DiffieHellman {
  DiffieHellman::new(group_name)
}

#[napi(ts_args_type = "privateKey: string | Buffer, publicKey: string | Buffer", ts_return_type = "Buffer")]
pub fn diffie_hellman(private_key: Either<String, Buffer>, public_key: Either<String, Buffer>) -> Buffer {
  let priv_bytes = decode_input(&private_key, None);
  let pub_bytes = decode_input(&public_key, None);
  let priv_num = BigUint::from_bytes_be(&priv_bytes);
  let pub_num = BigUint::from_bytes_be(&pub_bytes);
  let p = BigUint::from_bytes_be(&[
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xC9, 0x0F, 0xDA, 0xA2, 0x21, 0x68, 0xC2, 0x34,
    0xC4, 0xC6, 0x62, 0x8B, 0x80, 0xDC, 0x1C, 0xD1, 0x29, 0x02, 0x4E, 0x08, 0x8A, 0x67, 0xCC, 0x74,
  ]);
  let secret = pub_num.modpow(&priv_num, &p);
  Buffer::from(secret.to_bytes_be())
}
