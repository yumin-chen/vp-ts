use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn argon2_sync(password: Either<String, Buffer>, _salt: Either<String, Buffer>) -> Result<String> {
  use argon2::{Argon2, PasswordHasher};

  let pwd_bytes = match &password {
    Either::A(s) => s.as_bytes(),
    Either::B(b) => b.as_ref(),
  };

  let argon2 = Argon2::default();
  match argon2.hash_password(pwd_bytes) {
    Ok(parsed) => Ok(parsed.to_string()),
    Err(e) => Err(Error::new(Status::GenericFailure, format!("Argon2 error: {}", e))),
  }
}

pub struct Argon2Task {
  password: Vec<u8>,
  salt: Vec<u8>,
}

impl Task for Argon2Task {
  type Output = String;
  type JsValue = String;

  fn compute(&mut self) -> Result<Self::Output> {
    argon2_sync(Either::B(Buffer::from(self.password.clone())), Either::B(Buffer::from(self.salt.clone())))
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
pub fn argon2(password: Either<String, Buffer>, salt: Either<String, Buffer>) -> AsyncTask<Argon2Task> {
  let pwd_bytes = match password {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.as_ref().to_vec(),
  };
  let salt_bytes = match salt {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.as_ref().to_vec(),
  };

  AsyncTask::new(Argon2Task {
    password: pwd_bytes,
    salt: salt_bytes,
  })
}
