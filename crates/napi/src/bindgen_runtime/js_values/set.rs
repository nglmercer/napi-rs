use std::collections::{BTreeSet, HashSet};
use std::hash::{BuildHasher, Hash};

#[cfg(feature = "object_indexmap")]
use indexmap::IndexSet;

use crate::bindgen_prelude::*;

impl<V, S> TypeName for HashSet<V, S> {
  fn type_name() -> &'static str {
    "HashSet"
  }

  fn value_type() -> ValueType {
    ValueType::Object
  }
}

impl<V: FromNapiValue + ValidateNapiValue, S> ValidateNapiValue for HashSet<V, S> {
  unsafe fn validate(env: sys::napi_env, napi_val: sys::napi_value) -> Result<sys::napi_value> {
    let mut is_set = false;
    let mut global = std::ptr::null_mut();
    check_status!(
      unsafe { sys::napi_get_global(env, &mut global) },
      "Failed to get global object"
    )?;
    let mut set_constructor = std::ptr::null_mut();
    check_status!(
      unsafe {
        sys::napi_get_named_property(
          env,
          global,
          c"Set".as_ptr().cast(),
          &mut set_constructor,
        )
      },
      "Failed to get Set constructor"
    )?;
    check_status!(
      unsafe { sys::napi_instanceof(env, napi_val, set_constructor, &mut is_set) },
      "Failed to check if value is an instance of Set"
    )?;
    if !is_set {
      return Err(Error::new(
        Status::InvalidArg,
        "Expected a Set object".to_owned(),
      ));
    }
    Ok(std::ptr::null_mut())
  }

  unsafe fn validate_recursive(
    env: sys::napi_env,
    napi_val: sys::napi_value,
  ) -> Result<sys::napi_value> {
    Self::validate(env, napi_val)?;
    let obj = Object::from_raw(env, napi_val);
    let iter_creator: Function<'_, (), Object> = obj.get_named_property("values")?;
    let iter = iter_creator.apply(obj, ())?;
    let next: Function<'_, (), Object> = iter.get_named_property("next")?;
    while {
      let o: Object = next.apply(iter, ())?;
      let done: bool = o.get_named_property("done")?;
      if !done {
        let v = o.get_named_property_unchecked::<Unknown>("value")?;
        V::validate_recursive(env, v.0.value)?;
      }
      !done
    } {}
    Ok(std::ptr::null_mut())
  }
}

impl<V, S> ToNapiValue for HashSet<V, S>
where
  V: ToNapiValue,
{
  unsafe fn to_napi_value(raw_env: sys::napi_env, val: Self) -> Result<sys::napi_value> {
    let env = Env::from(raw_env);
    let obj = env.get_global()?;
    let set_class = obj.get_named_property_unchecked::<Function<'_, Array, ()>>("Set")?;
    let set = set_class.new_instance(Array::from_vec(&env, val.into_iter().collect())?)?;

    Ok(set.0.value)
  }
}

impl<V, S> FromNapiValue for HashSet<V, S>
where
  V: FromNapiValue + PartialEq + Eq + Hash,
  S: Default + BuildHasher,
{
  unsafe fn from_napi_value(env: sys::napi_env, napi_val: sys::napi_value) -> Result<Self> {
    let obj = Object::from_raw(env, napi_val);
    let mut set = HashSet::default();
    let iter_creator: Function<'_, (), Object> = obj.get_named_property("values")?;
    let iter = iter_creator.apply(obj, ())?;
    let next: Function<'_, (), Object> = iter.get_named_property("next")?;
    while {
      let o: Object = next.apply(iter, ())?;
      let done: bool = o.get_named_property("done")?;
      if !done {
        let v = o.get_named_property_unchecked::<V>("value")?;
        set.insert(v);
      }
      !done
    } {}
    Ok(set)
  }
}

impl<V> TypeName for BTreeSet<V> {
  fn type_name() -> &'static str {
    "BTreeSet"
  }

  fn value_type() -> ValueType {
    ValueType::Object
  }
}

impl<V: FromNapiValue + ValidateNapiValue> ValidateNapiValue for BTreeSet<V> {
  unsafe fn validate(env: sys::napi_env, napi_val: sys::napi_value) -> Result<sys::napi_value> {
    HashSet::<V, std::collections::hash_map::RandomState>::validate(env, napi_val)
  }

  unsafe fn validate_recursive(
    env: sys::napi_env,
    napi_val: sys::napi_value,
  ) -> Result<sys::napi_value> {
    HashSet::<V, std::collections::hash_map::RandomState>::validate_recursive(env, napi_val)
  }
}

impl<V> ToNapiValue for BTreeSet<V>
where
  V: ToNapiValue,
{
  unsafe fn to_napi_value(raw_env: sys::napi_env, val: Self) -> Result<sys::napi_value> {
    let env = Env::from(raw_env);
    let obj = env.get_global()?;
    let set_class = obj.get_named_property_unchecked::<Function<'_, Array, ()>>("Set")?;
    let set = set_class.new_instance(Array::from_vec(&env, val.into_iter().collect())?)?;

    Ok(set.0.value)
  }
}

impl<V> FromNapiValue for BTreeSet<V>
where
  V: FromNapiValue + Ord,
{
  unsafe fn from_napi_value(env: sys::napi_env, napi_val: sys::napi_value) -> Result<Self> {
    let obj = unsafe { Object::from_napi_value(env, napi_val)? };
    let mut set = BTreeSet::default();
    let iter_creator: Function<'_, (), Object> = obj.get_named_property("values")?;
    let iter = iter_creator.apply(obj, ())?;
    let next: Function<'_, (), Object> = iter.get_named_property("next")?;
    while {
      let o: Object = next.apply(iter, ())?;
      let done: bool = o.get_named_property("done")?;
      if !done {
        let v = o.get_named_property_unchecked::<V>("value")?;
        set.insert(v);
      }
      !done
    } {}
    Ok(set)
  }
}

#[cfg(feature = "object_indexmap")]
impl<V, S> TypeName for IndexSet<V, S> {
  fn type_name() -> &'static str {
    "IndexSet"
  }

  fn value_type() -> ValueType {
    ValueType::Object
  }
}
#[cfg(feature = "object_indexmap")]
impl<V: FromNapiValue + ValidateNapiValue, S> ValidateNapiValue for IndexSet<V, S> {
  unsafe fn validate(env: sys::napi_env, napi_val: sys::napi_value) -> Result<sys::napi_value> {
    HashSet::<V, S>::validate(env, napi_val)
  }

  unsafe fn validate_recursive(
    env: sys::napi_env,
    napi_val: sys::napi_value,
  ) -> Result<sys::napi_value> {
    HashSet::<V, S>::validate_recursive(env, napi_val)
  }
}
#[cfg(feature = "object_indexmap")]
impl<V, S> ToNapiValue for IndexSet<V, S>
where
  V: ToNapiValue,
{
  unsafe fn to_napi_value(raw_env: sys::napi_env, val: Self) -> Result<sys::napi_value> {
    let env = Env::from(raw_env);
    let obj = env.get_global()?;
    let set_class = obj.get_named_property_unchecked::<Function<'_, Array, ()>>("Set")?;
    let set = set_class.new_instance(Array::from_vec(&env, val.into_iter().collect())?)?;

    Ok(set.0.value)
  }
}
#[cfg(feature = "object_indexmap")]
impl<V, S> FromNapiValue for IndexSet<V, S>
where
  V: FromNapiValue + PartialEq + Eq + Hash,
  S: Default + BuildHasher,
{
  unsafe fn from_napi_value(env: sys::napi_env, napi_val: sys::napi_value) -> Result<Self> {
    let obj = Object::from_raw(env, napi_val);
    let mut set = IndexSet::default();
    let iter_creator: Function<'_, (), Object> = obj.get_named_property("values")?;
    let iter = iter_creator.apply(obj, ())?;
    let next: Function<'_, (), Object> = iter.get_named_property("next")?;
    while {
      let o: Object = next.apply(iter, ())?;
      let done: bool = o.get_named_property("done")?;
      if !done {
        let v = o.get_named_property_unchecked::<V>("value")?;
        set.insert(v);
      }
      !done
    } {}
    Ok(set)
  }
}
