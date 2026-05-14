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
