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
