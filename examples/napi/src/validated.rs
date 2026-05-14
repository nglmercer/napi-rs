use napi::bindgen_prelude::*;

#[napi(object)]
pub struct NestedObject {
  pub count: u32,
}

#[napi(object)]
pub struct MainObject {
  pub name: String,
  pub nested: NestedObject,
  pub optional: Option<u32>,
}

#[napi]
pub fn accept_validated_object(obj: Validated<MainObject>) -> Result<u32> {
  // Using zero-copy access with manual validation
  let name: String = obj.get("name")?;
  let nested: Validated<NestedObject> = obj.get("nested")?;
  let count: u32 = nested.get("count")?;
  let optional: Option<u32> = obj.get("optional")?;

  Ok(count + optional.unwrap_or(0) + name.len() as u32)
}

#[napi(object)]
pub struct DeepObject {
  pub list: Vec<u32>,
  pub map: std::collections::HashMap<String, NestedObject>,
}

#[napi]
pub fn accept_deep_validated_object(obj: Validated<DeepObject>) -> Result<u32> {
  let list: Vec<u32> = obj.get("list")?;
  let map: std::collections::HashMap<String, NestedObject> = obj.get("map")?;

  Ok(list.len() as u32 + map.len() as u32)
}

#[napi(object)]
pub struct UserSchema {
  pub id: u32,
  pub profile: ProfileSchema,
  pub tags: Vec<String>,
}

#[napi(object)]
pub struct ProfileSchema {
  pub username: String,
  pub email: String,
}

/// A hypothetical "CustomDB" that saves a user.
/// By using `Validated<UserSchema>`, we ensure the JS object matches the schema
/// without copying the entire object into a Rust struct.
/// This is "zero-copy" in the sense that we only read the fields we need.
#[napi]
pub fn save_user_to_custom_db(user: Validated<UserSchema>) -> Result<String> {
  // We can extract fields lazily. If we only need the ID and username to "save",
  // we don't need to copy the 'tags' array or the 'email' string.
  let id: u32 = user.get("id")?;

  let profile: Validated<ProfileSchema> = user.get("profile")?;
  let username: String = profile.get("username")?;

  // Hypothetically "save" to DB
  Ok(format!("Saved user {} with ID {}", username, id))
}

#[napi]
pub fn validate_and_process_dynamic(input: Unknown) -> Result<String> {
  // Use .parse() to dynamically validate the input against a schema.
  // This is useful when the type is not known at compile time or when
  // receiving `unknown` from JS.
  let user = Validated::<UserSchema>::parse(input)?;

  let id: u32 = user.get("id")?;
  Ok(format!("Dynamic validation successful for user ID {}", id))
}

#[napi]
pub enum DynamicSchemaType {
  String,
  Number,
  Boolean,
}

#[napi]
pub struct DynamicSchema {
  #[napi(skip)]
  pub fields: std::collections::HashMap<String, DynamicSchemaType>,
}

#[napi]
impl DynamicSchema {
  #[napi(constructor)]
  pub fn new(fields: std::collections::HashMap<String, DynamicSchemaType>) -> Self {
    Self { fields }
  }
}

/// A dynamically validated object whose schema is defined at runtime in Node.js.
/// This allows for zero-copy access even when the schema is not known at Rust compile-time.
#[napi]
pub struct DynamicValidated {
  inner: ObjectRef<false>,
}

#[napi]
impl DynamicValidated {
  #[napi(factory)]
  pub fn parse(input: Object, schema: &DynamicSchema) -> Result<Self> {
    for (name, ty) in &schema.fields {
      let val = input.get_inner(name.as_str())?.ok_or_else(|| {
        Error::new(Status::InvalidArg, format!("Field {} is missing", name))
      })?;

      let received = type_of!(input.value().env, val)?;

      match ty {
        DynamicSchemaType::String => {
          if received != ValueType::String {
            return Err(Error::new(
              Status::InvalidArg,
              format!("Field {} expect String, received {}", name, received),
            ));
          }
        }
        DynamicSchemaType::Number => {
          if received != ValueType::Number {
            return Err(Error::new(
              Status::InvalidArg,
              format!("Field {} expect Number, received {}", name, received),
            ));
          }
        }
        DynamicSchemaType::Boolean => {
          if received != ValueType::Boolean {
            return Err(Error::new(
              Status::InvalidArg,
              format!("Field {} expect Boolean, received {}", name, received),
            ));
          }
        }
      }
    }
    Ok(Self {
      inner: input.create_ref::<false>()?,
    })
  }

  #[napi]
  pub fn get_string(&self, env: Env, name: String) -> Result<String> {
    self.inner.get_value(&env)?.get_named_property_unchecked(&name)
  }

  #[napi]
  pub fn get_number(&self, env: Env, name: String) -> Result<f64> {
    self.inner.get_value(&env)?.get_named_property_unchecked(&name)
  }

  #[napi]
  pub fn get_boolean(&self, env: Env, name: String) -> Result<bool> {
    self.inner.get_value(&env)?.get_named_property_unchecked(&name)
  }
}

#[napi]
pub fn save_to_dynamic_db(env: Env, data: &DynamicValidated) -> Result<String> {
  // Access data lazily and zero-copy
  let name = data.get_string(env, "name".to_owned())?;
  let age = data.get_number(env, "age".to_owned())?;

  Ok(format!("Saved {} (age {}) to dynamic DB", name, age))
}

#[napi]
pub fn save_to_dynamic_db_js(data: Object, schema: &DynamicSchema) -> Result<String> {
  // 1. Dynamic validation (zero-copy)
  for (name, ty) in &schema.fields {
    let val = data.get_inner(name.as_str())?.ok_or_else(|| {
      Error::new(Status::InvalidArg, format!("Field {} is missing", name))
    })?;

    let received = type_of!(data.value().env, val)?;
    match ty {
      DynamicSchemaType::String => {
        if received != ValueType::String {
          return Err(Error::new(Status::InvalidArg, format!("Field {} expect String", name)));
        }
      }
      DynamicSchemaType::Number => {
        if received != ValueType::Number {
          return Err(Error::new(Status::InvalidArg, format!("Field {} expect Number", name)));
        }
      }
      DynamicSchemaType::Boolean => {
        if received != ValueType::Boolean {
          return Err(Error::new(Status::InvalidArg, format!("Field {} expect Boolean", name)));
        }
      }
    }
  }

  // 2. Lazy access
  let name: String = data.get_named_property_unchecked("name")?;
  let id: f64 = data.get_named_property_unchecked("id")?;

  Ok(format!("Saved {} (id {}) to dynamic DB JS", name, id))
}

#[napi(object)]
pub struct SmallUserSchema {
  pub id: u32,
  pub name: String,
}

#[napi]
pub fn process_small_user_validated(user: Validated<SmallUserSchema>) -> Result<String> {
  let name: String = user.get("name")?;
  let id: u32 = user.get("id")?;
  Ok(format!("Processed {} ({})", name, id))
}

#[napi]
pub fn process_serde_json(input: serde_json::Value) -> Result<String> {
  let name = input.get("name").and_then(|v| v.as_str()).unwrap_or("unknown");
  let id = input.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
  Ok(format!("Processed {} ({})", name, id))
}

#[napi]
pub fn create_validated_object(env: Env) -> Result<Validated<MainObject>> {
  let mut obj = Validated::<MainObject>::new_in(&env)?;
  obj.set("name", "hello")?;

  let mut nested = Validated::<NestedObject>::new_in(&env)?;
  nested.set("count", 42u32)?;
  obj.set("nested", nested)?;

  Ok(obj)
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_validated_compile() {
    // This test ensures that the code in this file compiles
  }
}
