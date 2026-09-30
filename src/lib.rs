use napi::bindgen_prelude::*;
use napi::sys;
use napi_derive::napi;
use std::ptr;

#[napi]
pub struct Extension {
  pub type_: i8,
  pub data: Uint8Array,
}

#[napi]
impl Extension {
  #[napi(constructor)]
  pub fn new(type_: i8, data: Uint8Array) -> Self {
    Self { type_, data }
  }

  #[napi(getter, js_name = "type")]
  pub fn get_type(&self) -> i8 {
    self.type_
  }

  #[napi(setter, js_name = "type")]
  pub fn set_type(&mut self, type_: i8) {
    self.type_ = type_;
  }
}

#[napi(ts_args_type = "object: ValueType", ts_return_type = "Uint8Array")]
pub fn encode(env: Env, object: Unknown) -> Result<Uint8Array> {
  let rmp_val = js_to_rmp_value(&env, object)?;
  let mut buf = Vec::new();
  rmpv::encode::write_value(&mut buf, &rmp_val)
    .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to encode MessagePack: {}", e)))?;
  Ok(Uint8Array::new(buf))
}

#[napi(ts_args_type = "data: Uint8Array", ts_return_type = "ValueType")]
pub fn decode<'a>(env: &'a Env, data: Uint8Array) -> Result<Unknown<'a>> {
  let slice: &[u8] = &data;
  let mut cursor = slice;
  let rmp_val = rmpv::decode::read_value(&mut cursor)
    .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to decode MessagePack: {}", e)))?;
  rmp_value_to_js(env, &rmp_val)
}

fn js_to_rmp_value(env: &Env, val: Unknown) -> Result<rmpv::Value> {
  let value_type = val.get_type()?;
  match value_type {
    ValueType::Null | ValueType::Undefined => Ok(rmpv::Value::Nil),
    ValueType::Boolean => {
      let b = val.coerce_to_bool()?;
      Ok(rmpv::Value::Boolean(b))
    }
    ValueType::Number => {
      let num = val.coerce_to_number()?;
      let f = num.get_double()?;
      if f.fract() == 0.0 && !f.is_nan() && !f.is_infinite() {
        if f >= 0.0 && f <= u64::MAX as f64 {
          Ok(rmpv::Value::Integer((f as u64).into()))
        } else if f < 0.0 && f >= i64::MIN as f64 {
          Ok(rmpv::Value::Integer((f as i64).into()))
        } else {
          Ok(rmpv::Value::F64(f))
        }
      } else {
        Ok(rmpv::Value::F64(f))
      }
    }
    ValueType::BigInt => {
      let bigint_str = val.coerce_to_string()?.into_utf8()?;
      let str_ref = bigint_str.as_str()?;
      if let Ok(i) = str_ref.parse::<i64>() {
        Ok(rmpv::Value::Integer(i.into()))
      } else if let Ok(u) = str_ref.parse::<u64>() {
        Ok(rmpv::Value::Integer(u.into()))
      } else {
        Err(Error::new(Status::GenericFailure, "BigInt out of range for MessagePack integer".to_string()))
      }
    }
    ValueType::String => {
      let s = val.coerce_to_string()?.into_utf8()?;
      let str_ref = s.as_str()?;
      Ok(rmpv::Value::String(str_ref.into()))
    }
    ValueType::Object => {
      let obj: Object = unsafe { Object::from_napi_value(env.raw(), val.raw())? };

      // 1. Buffer / Uint8Array / TypedArray / ArrayBuffer
      if val.is_buffer()? || val.is_typedarray()? || val.is_arraybuffer()? {
        if let Ok(u8a) = Uint8Array::from_unknown(val) {
          return Ok(rmpv::Value::Binary(u8a.to_vec()));
        }
      }

      // 2. Date
      if obj.has_named_property("getTime")? {
        if let Ok(date_fn_unk) = obj.get_named_property::<Unknown>("getTime") {
          if date_fn_unk.get_type()? == ValueType::Function {
            let mut raw_res = ptr::null_mut();
            let status = unsafe {
              sys::napi_call_function(env.raw(), obj.raw(), date_fn_unk.raw(), 0, ptr::null(), &mut raw_res)
            };
            if status == sys::Status::napi_ok {
              let ms_val = unsafe { Unknown::from_napi_value(env.raw(), raw_res)? };
              if let Ok(num) = ms_val.coerce_to_number() {
                if let Ok(ms) = num.get_double() {
                  let tv_sec = (ms / 1000.0).floor() as i64;
                  let tv_nsec = (((ms % 1000.0) + 1000.0) % 1000.0 * 1_000_000.0) as u32;

                  if (tv_sec >> 34) == 0 && tv_sec >= 0 {
                    let data64: u64 = ((tv_nsec as u64) << 34) | (tv_sec as u64);
                    if (data64 & 0xffff_ffff_0000_0000) == 0 {
                      let data32 = data64 as u32;
                      return Ok(rmpv::Value::Ext(-1, data32.to_be_bytes().to_vec()));
                    } else {
                      return Ok(rmpv::Value::Ext(-1, data64.to_be_bytes().to_vec()));
                    }
                  } else {
                    let mut bytes = Vec::with_capacity(12);
                    bytes.extend_from_slice(&tv_nsec.to_be_bytes());
                    bytes.extend_from_slice(&tv_sec.to_be_bytes());
                    return Ok(rmpv::Value::Ext(-1, bytes));
                  }
                }
              }
            }
          }
        }
      }

      // 3. Extension ({ type: number, data: Uint8Array })
      if obj.has_named_property("type")? && obj.has_named_property("data")? {
        let type_prop: Unknown = obj.get_named_property("type")?;
        let data_prop: Unknown = obj.get_named_property("data")?;
        if type_prop.get_type()? == ValueType::Number {
          let type_id = type_prop.coerce_to_number()?.get_int32()? as i8;
          if let Ok(u8a) = Uint8Array::from_unknown(data_prop) {
            return Ok(rmpv::Value::Ext(type_id, u8a.to_vec()));
          }
        }
      }

      // 4. Array
      if val.is_array()? {
        let len = obj.get_array_length()?;
        let mut vec = Vec::with_capacity(len as usize);
        for i in 0..len {
          let elem: Unknown = obj.get_element(i)?;
          vec.push(js_to_rmp_value(env, elem)?);
        }
        return Ok(rmpv::Value::Array(vec));
      }

      // 5. Map
      let global = env.get_global()?;
      let map_ctor_unk: Unknown = global.get_named_property("Map")?;
      if val.instanceof(map_ctor_unk)? {
        let entries_fn_unk: Unknown = obj.get_named_property("entries")?;
        let mut raw_iterator = ptr::null_mut();
        unsafe {
          sys::napi_call_function(env.raw(), obj.raw(), entries_fn_unk.raw(), 0, ptr::null(), &mut raw_iterator);
        }
        let iterator_obj: Object = unsafe { Object::from_napi_value(env.raw(), raw_iterator)? };
        let next_fn_unk: Unknown = iterator_obj.get_named_property("next")?;
        let mut pairs = Vec::new();
        loop {
          let mut raw_step = ptr::null_mut();
          unsafe {
            sys::napi_call_function(env.raw(), iterator_obj.raw(), next_fn_unk.raw(), 0, ptr::null(), &mut raw_step);
          }
          let step_obj: Object = unsafe { Object::from_napi_value(env.raw(), raw_step)? };
          let done: bool = step_obj.get_named_property("done")?;
          if done {
            break;
          }
          let entry: Object = step_obj.get_named_property("value")?;
          let k: Unknown = entry.get_element(0)?;
          let v: Unknown = entry.get_element(1)?;
          pairs.push((js_to_rmp_value(env, k)?, js_to_rmp_value(env, v)?));
        }
        return Ok(rmpv::Value::Map(pairs));
      }

      // 6. Plain Object
      let keys = obj.get_property_names()?;
      let len = keys.get_array_length()?;
      let mut pairs = Vec::with_capacity(len as usize);
      for i in 0..len {
        let key_js: String = keys.get_element(i)?;
        let val_js: Unknown = obj.get_named_property(&key_js)?;
        pairs.push((rmpv::Value::String(key_js.into()), js_to_rmp_value(env, val_js)?));
      }
      Ok(rmpv::Value::Map(pairs))
    }
    _ => Ok(rmpv::Value::Nil),
  }
}

fn rmp_value_to_js<'a>(env: &'a Env, val: &rmpv::Value) -> Result<Unknown<'a>> {
  match val {
    rmpv::Value::Nil => Null.into_unknown(env),
    rmpv::Value::Boolean(b) => b.into_unknown(env),
    rmpv::Value::Integer(i) => {
      if i.is_i64() {
        let v = i.as_i64().unwrap();
        if v >= -9007199254740991 && v <= 9007199254740991 {
          (v as f64).into_unknown(env)
        } else {
          BigInt {
            sign_bit: v < 0,
            words: vec![v.unsigned_abs()],
          }.into_unknown(env)
        }
      } else if i.is_u64() {
        let v = i.as_u64().unwrap();
        if v <= 9007199254740991 {
          (v as f64).into_unknown(env)
        } else {
          BigInt {
            sign_bit: false,
            words: vec![v],
          }.into_unknown(env)
        }
      } else {
        i.as_f64().unwrap_or(0.0).into_unknown(env)
      }
    }
    rmpv::Value::F32(f) => (*f as f64).into_unknown(env),
    rmpv::Value::F64(f) => f.into_unknown(env),
    rmpv::Value::String(s) => {
      if let Some(utf8) = s.as_str() {
        utf8.to_string().into_unknown(env)
      } else {
        Uint8Array::new(s.as_bytes().to_vec()).into_unknown(env)
      }
    }
    rmpv::Value::Binary(bytes) => {
      Uint8Array::new(bytes.clone()).into_unknown(env)
    }
    rmpv::Value::Array(arr) => {
      let mut js_arr = Vec::with_capacity(arr.len());
      for elem in arr {
        js_arr.push(rmp_value_to_js(env, elem)?);
      }
      js_arr.into_unknown(env)
    }
    rmpv::Value::Map(pairs) => {
      let all_str_keys = pairs.iter().all(|(k, _)| k.is_str());
      if all_str_keys {
        let mut js_obj = Object::new(env)?;
        for (k, v) in pairs {
          let k_str = k.as_str().unwrap_or("");
          js_obj.set_named_property(k_str, rmp_value_to_js(env, v)?)?;
        }
        js_obj.into_unknown(env)
      } else {
        let global = env.get_global()?;
        let map_ctor_unk: Unknown = global.get_named_property("Map")?;
        let mut raw_map_instance = ptr::null_mut();
        unsafe {
          sys::napi_new_instance(env.raw(), map_ctor_unk.raw(), 0, ptr::null(), &mut raw_map_instance);
        }
        let js_map_obj: Object = unsafe { Object::from_napi_value(env.raw(), raw_map_instance)? };
        let set_fn_unk: Unknown = js_map_obj.get_named_property("set")?;
        for (k, v) in pairs {
          let js_k = rmp_value_to_js(env, k)?;
          let js_v = rmp_value_to_js(env, v)?;
          let mut raw_call_res = ptr::null_mut();
          let args = [js_k.raw(), js_v.raw()];
          unsafe {
            sys::napi_call_function(env.raw(), js_map_obj.raw(), set_fn_unk.raw(), 2, args.as_ptr(), &mut raw_call_res);
          }
        }
        js_map_obj.into_unknown(env)
      }
    }
    rmpv::Value::Ext(type_id, bytes) => {
      if *type_id == -1 {
        let global = env.get_global()?;
        let date_ctor_unk: Unknown = global.get_named_property("Date")?;
        let millis = if bytes.len() == 4 {
          let sec = u32::from_be_bytes(bytes[0..4].try_into().unwrap()) as i64;
          (sec * 1000) as f64
        } else if bytes.len() == 8 {
          let data64 = u64::from_be_bytes(bytes[0..8].try_into().unwrap());
          let nsec = (data64 >> 34) as i64;
          let sec = (data64 & 0x0000_0003_ffff_ffff) as i64;
          (sec * 1000 + nsec / 1_000_000) as f64
        } else if bytes.len() == 12 {
          let nsec = u32::from_be_bytes(bytes[0..4].try_into().unwrap()) as i64;
          let sec = i64::from_be_bytes(bytes[4..12].try_into().unwrap());
          (sec * 1000 + nsec / 1_000_000) as f64
        } else {
          0.0
        };
        let js_millis = millis.into_unknown(env)?;
        let mut raw_date_instance = ptr::null_mut();
        let args = [js_millis.raw()];
        unsafe {
          sys::napi_new_instance(env.raw(), date_ctor_unk.raw(), 1, args.as_ptr(), &mut raw_date_instance);
          Unknown::from_napi_value(env.raw(), raw_date_instance)
        }
      } else {
        let ext_struct = Extension {
          type_: *type_id,
          data: Uint8Array::new(bytes.clone()),
        };
        ext_struct.into_instance(env)?.into_unknown(env)
      }
    }
  }
}
