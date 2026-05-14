use std::marker::PhantomData;

use crate::bindgen_prelude::*;
use crate::{sys, Result, Value, ValueType};

pub struct Validated<T> {
  value: Value,
  _phantom: PhantomData<T>,
}

impl<T> Validated<T> {
  pub fn new(value: Value) -> Self {
    Self {
      value,
      _phantom: PhantomData,
    }
  }

  pub fn value(&self) -> &Value {
    &self.value
  }
}

impl<T: ValidateNapiValue> FromNapiValue for Validated<T> {
  unsafe fn from_napi_value(env: sys::napi_env, napi_val: sys::napi_value) -> Result<Self> {
    Ok(Self {
      value: Value {
        env,
        value: napi_val,
        value_type: ValueType::Object,
      },
      _phantom: PhantomData,
    })
  }
}

impl<T: ValidateNapiValue> JsValue<'_> for Validated<T> {
  fn value(&self) -> Value {
    self.value
  }
}

impl<T: ValidateNapiValue> JsObjectValue<'_> for Validated<T> {}

impl<T: TypeName> TypeName for Validated<T> {
  fn type_name() -> &'static str {
    T::type_name()
  }

  fn value_type() -> ValueType {
    ValueType::Object
  }
}

impl<T: ValidateNapiValue> ValidateNapiValue for Validated<T> {
  unsafe fn validate(env: sys::napi_env, napi_val: sys::napi_value) -> Result<sys::napi_value> {
    T::validate(env, napi_val)
  }
}

impl<T> Validated<T>
where
  T: ValidateNapiValue,
{
  /// Create a new validated object in the given environment.
  pub fn new_in(env: &Env) -> Result<Self> {
    let obj = Object::new(env)?;
    Ok(Self {
      value: obj.0,
      _phantom: PhantomData,
    })
  }

  /// Convert the validated object into the inner type.
  /// This will perform a full conversion (copying data).
  pub fn into_inner(self) -> Result<T>
  where
    T: FromNapiValue,
  {
    unsafe { T::from_napi_value(self.value.env, self.value.value) }
  }

  /// Get a property from the object.
  /// This will validate the property type.
  pub fn get<V>(&self, key: &str) -> Result<V>
  where
    V: FromNapiValue + ValidateNapiValue,
  {
    self.get_named_property(key)
  }

  /// Set a property on the object.
  pub fn set<V>(&mut self, key: &str, value: V) -> Result<()>
  where
    V: ToNapiValue,
  {
    self.set_named_property(key, value)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // We cannot easily run full integration tests here without a JS environment,
  // but we can at least ensure it compiles and the traits are implemented.
  #[test]
  fn test_validated_trait_impls() {
    // This just ensures that if we have a type that implements ValidateNapiValue,
    // Validated<T> also implements it.
    fn assert_validate<T: ValidateNapiValue>() {}
    assert_validate::<Validated<i32>>();
    assert_validate::<Validated<String>>();
  }
}
