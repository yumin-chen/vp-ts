use napi_derive::napi;
use sqids::Sqids;
use std::collections::HashSet;

#[napi(object)]
#[derive(Default)]
pub struct NativeSqidsOptions {
  pub alphabet: Option<String>,
  pub min_length: Option<u32>,
  pub blocklist: Option<Vec<String>>,
}

#[napi]
pub struct NativeSqids {
  inner: Sqids,
}

#[napi]
impl NativeSqids {
  #[napi(constructor)]
  pub fn new(options: Option<NativeSqidsOptions>) -> napi::Result<Self> {
    let mut builder = Sqids::builder();
    if let Some(opts) = options {
      if let Some(alphabet) = opts.alphabet {
        builder = builder.alphabet(alphabet.chars().collect());
      }
      if let Some(min_length) = opts.min_length {
        if min_length > 255 {
          return Err(napi::Error::from_reason(
            "Minimum length has to be between 0 and 255",
          ));
        }
        builder = builder.min_length(min_length as u8);
      }
      if let Some(blocklist) = opts.blocklist {
        let set: HashSet<String> = blocklist.into_iter().collect();
        builder = builder.blocklist(set);
      }
    }
    let inner = builder
      .build()
      .map_err(|err| napi::Error::from_reason(err.to_string()))?;
    Ok(NativeSqids { inner })
  }

  #[napi]
  pub fn encode(&self, numbers: Vec<f64>) -> napi::Result<String> {
    let max_value = 9007199254740991f64;
    let mut u64_numbers = Vec::with_capacity(numbers.len());
    for &num in &numbers {
      if num < 0.0 || num > max_value || num.fract() != 0.0 {
        return Err(napi::Error::from_reason(
          "Encoding supports numbers between 0 and 9007199254740991",
        ));
      }
      u64_numbers.push(num as u64);
    }
    self
      .inner
      .encode(&u64_numbers)
      .map_err(|err| napi::Error::from_reason(err.to_string()))
  }

  #[napi]
  pub fn decode(&self, id: String) -> Vec<f64> {
    let numbers = self.inner.decode(&id);
    numbers.into_iter().map(|n| n as f64).collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_native_sqids_default() {
    let sqids = NativeSqids::new(None).unwrap();
    let id = sqids.encode(vec![1.0, 2.0, 3.0]).unwrap();
    assert_eq!(id, "86Rf07");
    let decoded = sqids.decode(id);
    assert_eq!(decoded, vec![1.0, 2.0, 3.0]);
  }

  #[test]
  fn test_native_sqids_min_length() {
    let opts = NativeSqidsOptions {
      min_length: Some(10),
      ..Default::default()
    };
    let sqids = NativeSqids::new(Some(opts)).unwrap();
    let id = sqids.encode(vec![1.0, 2.0, 3.0]).unwrap();
    assert_eq!(id, "86Rf07xd4z");
    let decoded = sqids.decode(id);
    assert_eq!(decoded, vec![1.0, 2.0, 3.0]);
  }

  #[test]
  fn test_native_sqids_alphabet() {
    let opts = NativeSqidsOptions {
      alphabet: Some(
        "FxnXM1kBN6cuhsAvjW3Co7l2RePyY8DwaU04Tzt9fHQrqSVKdpimLGIJOgb5ZE"
          .to_string(),
      ),
      ..Default::default()
    };
    let sqids = NativeSqids::new(Some(opts)).unwrap();
    let id = sqids.encode(vec![1.0, 2.0, 3.0]).unwrap();
    assert_eq!(id, "B4aajs");
    let decoded = sqids.decode(id);
    assert_eq!(decoded, vec![1.0, 2.0, 3.0]);
  }

  #[test]
  fn test_native_sqids_blocklist() {
    let opts = NativeSqidsOptions {
      blocklist: Some(vec!["86Rf07".to_string()]),
      ..Default::default()
    };
    let sqids = NativeSqids::new(Some(opts)).unwrap();
    let id = sqids.encode(vec![1.0, 2.0, 3.0]).unwrap();
    assert_eq!(id, "se8ojk");
    let decoded = sqids.decode(id);
    assert_eq!(decoded, vec![1.0, 2.0, 3.0]);
  }
}
