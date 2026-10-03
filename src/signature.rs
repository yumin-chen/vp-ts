use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::digest;

#[napi]
pub struct Sign {
  pub algorithm: String,
  data: Vec<u8>,
}

#[napi]
impl Sign {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Result<Self> {
    Ok(Self {
      algorithm: algorithm.to_lowercase(),
      data: Vec::new(),
    })
  }

  #[napi]
  pub fn update(&mut self, data: Either<String, Buffer>) -> &Self {
    match data {
      Either::A(s) => self.data.extend_from_slice(s.as_bytes()),
      Either::B(b) => self.data.extend_from_slice(b.as_ref()),
    }
    self
  }

  #[napi]
  pub fn sign(&mut self, _private_key: Buffer, encoding: Option<String>) -> Either<String, Buffer> {
    let algo = match self.algorithm.as_str() {
      "sha256" => &digest::SHA256,
      "sha384" => &digest::SHA384,
      "sha512" => &digest::SHA512,
      "sha1" => &digest::SHA1_FOR_LEGACY_USE_ONLY,
      _ => &digest::SHA256,
    };
    let mut ctx = digest::Context::new(algo);
    ctx.update(&self.data);
    let result = ctx.finish();
    let bytes = result.as_ref();

    let enc = encoding.unwrap_or_else(|| "buffer".to_string());
    match enc.as_str() {
      "hex" => Either::A(bytes.iter().map(|b| format!("{:02x}", b)).collect()),
      "buffer" | "" => Either::B(Buffer::from(bytes.to_vec())),
      _ => Either::B(Buffer::from(bytes.to_vec())),
    }
  }
}

#[napi]
pub fn create_sign(algorithm: String) -> Result<Sign> {
  Sign::new(algorithm)
}

#[napi]
pub fn sign(_algorithm: Option<String>, data: Buffer, private_key: Buffer) -> Buffer {
  let mut s = Sign::new("sha256".to_string()).unwrap();
  s.update(Either::B(data));
  match s.sign(private_key, None) {
    Either::A(str_val) => Buffer::from(str_val.into_bytes()),
    Either::B(buf) => buf,
  }
}
