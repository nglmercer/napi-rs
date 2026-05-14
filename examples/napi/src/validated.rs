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
  pub list: Vec<u32>,
}

#[napi]
pub fn accept_validated_object(obj: Validated<MainObject>) -> Result<u32> {
  let name: String = obj.get("name")?;
  let nested: Validated<NestedObject> = obj.get("nested")?;
  let count: u32 = nested.get("count")?;
  let optional: Option<u32> = obj.get("optional")?;
  let list: Vec<u32> = obj.get("list")?;

  Ok(count + optional.unwrap_or(0) + name.len() as u32 + list.iter().sum::<u32>())
}

#[napi]
pub fn create_validated_object(env: Env) -> Result<Validated<MainObject>> {
  let mut obj = Validated::<MainObject>::new_in(&env)?;
  obj.set("name", "hello")?;

  let mut nested = Validated::<NestedObject>::new_in(&env)?;
  nested.set("count", 42u32)?;
  obj.set("nested", nested)?;

  obj.set("list", vec![1, 2, 3])?;

  Ok(obj)
}

#[napi(string_enum)]
pub enum Status {
  Active,
  Inactive,
}

#[napi(object)]
pub struct StructuredObject {
  pub status: Status,
}

#[napi]
pub fn test_validated_enum(obj: Validated<StructuredObject>) -> Result<String> {
  let status: Status = obj.get("status")?;
  match status {
    Status::Active => Ok("active".to_string()),
    Status::Inactive => Ok("inactive".to_string()),
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_validated_compile() {
    // This test ensures that the code in this file compiles
    // and that the macro-generated validation logic is present.
  }
}
