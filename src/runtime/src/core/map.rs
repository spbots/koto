use {
    crate::{
        runtime_error, value_iterator::ValueIteratorOutput as Output, value_sort::compare_values,
        MetaKey, RuntimeResult, Value, ValueHashMap, ValueIterator, ValueKey, ValueMap, Vm,
    },
    std::{cmp::Ordering, ops::Deref},
};

pub fn make_module() -> ValueMap {
    use Value::*;

    let mut result = ValueMap::new();

    result.add_fn("clear", |vm, args| match vm.get_args(args) {
        [Map(m)] => {
            m.contents_mut().data.clear();
            Ok(Empty)
        }
        _ => runtime_error!("map.clear: Expected map as argument"),
    });

    result.add_fn("contains_key", |vm, args| match vm.get_args(args) {
        [Map(m), key] if key.is_immutable() => Ok(Bool(
            m.contents().data.contains_key(&ValueKey::from(key.clone())),
        )),
        [other_a, other_b, ..] => runtime_error!(
            "map.contains_key: Expected map and key as arguments, found '{}' and '{}'",
            other_a.type_as_string(),
            other_b.type_as_string()
        ),
        _ => runtime_error!("map.contains_key: Expected map and key as arguments"),
    });

    result.add_fn("copy", |vm, args| match vm.get_args(args) {
        [Map(m)] => Ok(Map(ValueMap::with_data(m.contents().data.clone()))),
        _ => runtime_error!("map.copy: Expected map as argument"),
    });

    result.add_fn("deep_copy", |vm, args| match vm.get_args(args) {
        [value @ Map(_)] => Ok(value.deep_copy()),
        _ => runtime_error!("map.deep_copy: Expected map as argument"),
    });

    result.add_fn("get", |vm, args| match vm.get_args(args) {
        [Map(m), key] => match m.contents().data.get(&ValueKey::from(key.clone())) {
            Some(value) => Ok(value.clone()),
            None => Ok(Empty),
        },
        [other_a, other_b, ..] => runtime_error!(
            "map.get: Expected map and key as arguments, found '{}' and '{}'",
            other_a.type_as_string(),
            other_b.type_as_string()
        ),
        _ => runtime_error!("map.get: Expected map and key as arguments"),
    });

    result.add_fn("get_index", |vm, args| match vm.get_args(args) {
        [Map(m), Number(n)] => {
            if *n < 0.0 {
                return runtime_error!("map.get_index: Negative indices aren't allowed");
            }
            match m.contents().data.get_index(n.into()) {
                Some((key, value)) => Ok(Tuple(vec![key.deref().clone(), value.clone()].into())),
                None => Ok(Empty),
            }
        }
        _ => runtime_error!("map.get_index: Expected map and index as arguments"),
    });

    result.add_fn("help", |vm, args| {
        let result = match vm.get_args(args) {
            [Map(m)] if m.contents().meta.contains_key(&MetaKey::Help) => {
                let help = "\
Call this function with the name of the item that you would like help with.

For example:
  import number
  number.help \"abs\"
"
                .to_string();

                match m.contents().meta.get(&MetaKey::SelfHelp) {
                    Some(Value::Str(self_help)) => {
                        Value::Str(format!("{}\n\n========\n\n{}", self_help, help).into())
                    }
                    _ => Value::Str(help.to_string().into()),
                }
            }
            [Map(m)] if m.contents().meta.contains_key(&MetaKey::SelfHelp) => {
                match m.contents().meta.get(&MetaKey::SelfHelp) {
                    Some(self_help @ Value::Str(_)) => self_help.clone(),
                    Some(unexpected) => {
                        return runtime_error!(
                            "map.help: Expected string for self help, found '{}'",
                            unexpected.type_as_string()
                        )
                    }
                    None => unreachable!(),
                }
            }
            [Map(_)] => Value::Str("map.help: No help found".into()),
            [Map(m), Str(s)] => {
                let result = match m.contents().meta.get(&MetaKey::Help) {
                    Some(Value::Map(help)) => {
                        help.contents().data.get_with_string(&s.as_str()).cloned()
                    }
                    _ => None,
                };

                result.unwrap_or_else(|| Value::Str(format!("Help not found for '{}'", s).into()))
            }
            _ => return runtime_error!("map.help: Expected map and string as arguments"),
        };

        Ok(result)
    });

    result.add_fn("insert", |vm, args| match vm.get_args(args) {
        [Map(m), key] if key.is_immutable() => {
            match m.contents_mut().data.insert(key.clone().into(), Empty) {
                Some(old_value) => Ok(old_value),
                None => Ok(Empty),
            }
        }
        [Map(m), key, value] if key.is_immutable() => {
            match m
                .contents_mut()
                .data
                .insert(key.clone().into(), value.clone())
            {
                Some(old_value) => Ok(old_value),
                None => Ok(Empty),
            }
        }
        [other_a, other_b, ..] => runtime_error!(
            "map.insert: Expected map and key as arguments, found '{}' and '{}'",
            other_a.type_as_string(),
            other_b.type_as_string()
        ),
        _ => runtime_error!("map.insert: Expected map and key as arguments"),
    });

    result.add_fn("is_empty", |vm, args| match vm.get_args(args) {
        [Map(m)] => Ok(Bool(m.contents().data.is_empty())),
        [other, ..] => runtime_error!(
            "map.is_empty: Expected map as argument, found '{}'",
            other.type_as_string(),
        ),
        _ => runtime_error!("map.contains_key: Expected map and key as arguments"),
    });

    result.add_fn("iter", |vm, args| match vm.get_args(args) {
        [Map(m)] => Ok(Iterator(ValueIterator::with_map(m.clone()))),
        [other, ..] => runtime_error!(
            "map.iter: Expected map as argument, found '{}'",
            other.type_as_string(),
        ),
        _ => runtime_error!("map.iter: Expected map as argument"),
    });

    result.add_fn("keys", |vm, args| match vm.get_args(args) {
        [Map(m)] => {
            let mut iter = ValueIterator::with_map(m.clone()).map(|output| match output {
                Ok(Output::ValuePair(key, _)) => Ok(Output::Value(key)),
                Ok(_) => unreachable!(),
                Err(e) => Err(e),
            });

            Ok(Iterator(ValueIterator::make_external(move || iter.next())))
        }
        [other, ..] => runtime_error!(
            "map.keys: Expected map as argument, found '{}'",
            other.type_as_string(),
        ),
        _ => runtime_error!("map.keys: Expected map as argument"),
    });

    result.add_fn("remove", |vm, args| match vm.get_args(args) {
        [Map(m), key] if key.is_immutable() => {
            match m
                .contents_mut()
                .data
                .shift_remove(&ValueKey::from(key.clone()))
            {
                Some(old_value) => Ok(old_value),
                None => Ok(Empty),
            }
        }
        [other_a, other_b, ..] => runtime_error!(
            "map.remove: Expected map and key as arguments, found '{}' and '{}'",
            other_a.type_as_string(),
            other_b.type_as_string()
        ),
        _ => runtime_error!("map.remove: Expected map and key as arguments"),
    });

    result.add_fn("sort", |vm, args| match vm.get_args(args) {
        [Map(m)] => {
            m.contents_mut().data.sort_keys();
            Ok(Empty)
        }
        [Map(l), f] if f.is_callable() => {
            let m = l.clone();
            let f = f.clone();
            let vm = vm.child_vm();
            let mut error = None;

            let get_sort_key = |vm: &mut Vm,
                                cache: &mut ValueHashMap,
                                key: &Value,
                                value: &Value|
             -> RuntimeResult {
                let value = vm.run_function(f.clone(), &[key.clone(), value.clone()])?;
                cache.insert(key.clone().into(), value.clone());
                Ok(value)
            };

            let mut cache = ValueHashMap::with_capacity(m.len());
            m.contents_mut()
                .data
                .sort_by(|key_a, value_a, key_b, value_b| {
                    if error.is_some() {
                        return Ordering::Equal;
                    }

                    let value_a = match cache.get(key_a) {
                        Some(value) => value.clone(),
                        None => match get_sort_key(vm, &mut cache, key_a, value_a) {
                            Ok(val) => val,
                            Err(e) => {
                                error.get_or_insert(Err(e.with_prefix("map.sort")));
                                Empty
                            }
                        },
                    };
                    let value_b = match cache.get(key_b) {
                        Some(value) => value.clone(),
                        None => match get_sort_key(vm, &mut cache, key_b, value_b) {
                            Ok(val) => val,
                            Err(e) => {
                                error.get_or_insert(Err(e.with_prefix("map.sort")));
                                Empty
                            }
                        },
                    };

                    match compare_values(vm, &value_a, &value_b) {
                        Ok(ordering) => ordering,
                        Err(e) => {
                            error.get_or_insert(Err(e));
                            Ordering::Equal
                        }
                    }
                });

            if let Some(error) = error {
                error
            } else {
                Ok(Empty)
            }
        }
        _ => runtime_error!("map.sort: Expected map as argument"),
    });

    result.add_fn("size", |vm, args| match vm.get_args(args) {
        [Map(m)] => Ok(Number(m.len().into())),
        [other, ..] => runtime_error!(
            "map.size: Expected map as argument, found '{}'",
            other.type_as_string(),
        ),
        _ => runtime_error!("map.contains_key: Expected map and key as arguments"),
    });

    result.add_fn("update", |vm, args| match vm.get_args(args) {
        [Map(m), key, f] if key.is_immutable() && f.is_callable() => do_map_update(
            m.clone(),
            key.clone().into(),
            Empty,
            f.clone(),
            vm.child_vm(),
        ),
        [Map(m), key, default, f] if key.is_immutable() && f.is_callable() => do_map_update(
            m.clone(),
            key.clone().into(),
            default.clone(),
            f.clone(),
            vm.child_vm(),
        ),
        _ => runtime_error!("map.update: Expected map, key, and function as arguments"),
    });

    result.add_fn("values", |vm, args| match vm.get_args(args) {
        [Map(m)] => {
            let mut iter = ValueIterator::with_map(m.clone()).map(|output| match output {
                Ok(Output::ValuePair(_, value)) => Ok(Output::Value(value)),
                Ok(_) => unreachable!(),
                Err(e) => Err(e),
            });

            Ok(Iterator(ValueIterator::make_external(move || iter.next())))
        }
        [other, ..] => runtime_error!(
            "map.values: Expected map as argument, found '{}'",
            other.type_as_string(),
        ),
        _ => runtime_error!("map.values: Expected map as argument"),
    });

    result
}

fn do_map_update(
    map: ValueMap,
    key: ValueKey,
    default: Value,
    f: Value,
    vm: &mut Vm,
) -> RuntimeResult {
    if !map.contents().data.contains_key(&key) {
        map.contents_mut().data.insert(key.clone(), default);
    }
    let value = map.contents().data.get(&key).cloned().unwrap();
    match vm.run_function(f, &[value]) {
        Ok(new_value) => {
            map.contents_mut().data.insert(key, new_value.clone());
            Ok(new_value)
        }
        Err(error) => Err(error.with_prefix("map.update")),
    }
}
