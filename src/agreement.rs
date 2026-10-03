use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::digest;
use ring::rand::{SecureRandom, SystemRandom};
use ring::signature as ring_sig;

#[napi(object)]
pub struct EncapsulateResult {
  pub shared_key: Buffer,
  pub ciphertext: Buffer,
}

#[napi]
pub fn encapsulate(
  #[napi(ts_arg_type = "Uint8Array")] public_key: Uint8Array,
) -> Result<EncapsulateResult> {
  let rng = SystemRandom::new();
  let mut shared_key = vec![0u8; 32];
  let mut ciphertext = vec![0u8; 32];

  rng
    .fill(&mut shared_key)
    .map_err(|_| Error::new(Status::GenericFailure, "Random generation failed"))?;

  let digest = digest::digest(&digest::SHA256, public_key.as_ref());
  ciphertext.copy_from_slice(digest.as_ref());

  Ok(EncapsulateResult {
    shared_key: Buffer::from(shared_key),
    ciphertext: Buffer::from(ciphertext),
  })
}

#[napi]
pub fn decapsulate(
  #[napi(ts_arg_type = "Uint8Array")] private_key: Uint8Array,
  #[napi(ts_arg_type = "Uint8Array")] ciphertext: Uint8Array,
) -> Result<Buffer> {
  let mut input = Vec::new();
  input.extend_from_slice(private_key.as_ref());
  input.extend_from_slice(ciphertext.as_ref());

  let digest = digest::digest(&digest::SHA256, &input);
  Ok(Buffer::from(digest.as_ref().to_vec()))
}

#[napi]
pub struct Verify {
  data: Vec<u8>,
  #[allow(dead_code)]
  algorithm: String,
}

#[napi]
impl Verify {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Self {
    Self {
      data: Vec::new(),
      algorithm,
    }
  }

  #[napi]
  pub fn update(
    &mut self,
    #[napi(ts_arg_type = "string | Uint8Array")] data: Either<String, Uint8Array>,
  ) {
    match data {
      Either::A(s) => self.data.extend_from_slice(s.as_bytes()),
      Either::B(b) => self.data.extend_from_slice(b.as_ref()),
    }
  }

  #[napi]
  pub fn verify(
    &self,
    #[napi(ts_arg_type = "Uint8Array")] public_key: Uint8Array,
    #[napi(ts_arg_type = "Uint8Array")] signature: Uint8Array,
  ) -> bool {
    let peer_public_key = ring_sig::UnparsedPublicKey::new(
      &ring_sig::ED25519,
      public_key.as_ref(),
    );
    peer_public_key
      .verify(&self.data, signature.as_ref())
      .is_ok()
  }
}

#[napi]
pub fn create_verify(algorithm: String) -> Verify {
  Verify::new(algorithm)
}
