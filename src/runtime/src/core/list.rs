use crate::{external_error, value, Value, ValueIterator, ValueList, ValueMap};

pub fn make_module() -> ValueMap {
    use Value::*;

    let mut result = ValueMap::new();

    result.add_fn("contains", |vm, args| match vm.get_args(args) {
        [List(l), value] => Ok(Bool(l.data().contains(value))),
        _ => external_error!("list.contains: Expected list and value as arguments"),
    });

    result.add_fn("fill", |vm, args| match vm.get_args(args) {
        [List(l), value] => {
            for v in l.data_mut().iter_mut() {
                *v = value.clone();
            }
            Ok(Value::Empty)
        }
        _ => external_error!("list.fill: Expected list and value as arguments"),
    });

    result.add_fn("first", |vm, args| match vm.get_args(args) {
        [List(l)] => match l.data().first() {
            Some(value) => Ok(value.clone()),
            None => Ok(Value::Empty),
        },
        _ => external_error!("list.first: Expected list as argument"),
    });

    result.add_fn("get", |vm, args| match vm.get_args(args) {
        [List(l), Number(n)] => {
            if *n < 0.0 {
                return external_error!("list.get: Negative indices aren't allowed");
            }
            let index = *n as usize;
            match l.data().get(index) {
                Some(value) => Ok(value.clone()),
                None => Ok(Value::Empty),
            }
        }
        _ => external_error!("list.get: Expected list and number as arguments"),
    });

    result.add_fn("insert", |vm, args| match vm.get_args(args) {
        [List(l), Number(n), value] => {
            if *n < 0.0 {
                return external_error!("list.insert: Negative indices aren't allowed");
            }
            let index = *n as usize;
            if index > l.data().len() {
                return external_error!("list.insert: Index out of bounds");
            }

            l.data_mut().insert(index, value.clone());
            Ok(Value::Empty)
        }
        _ => external_error!("list.insert: Expected list, number, and value as arguments"),
    });

    result.add_fn("is_empty", |vm, args| match vm.get_args(args) {
        [List(l)] => Ok(Bool(l.data().is_empty())),
        _ => external_error!("list.is_empty: Expected list as argument"),
    });

    result.add_fn("iter", |vm, args| match vm.get_args(args) {
        [List(l)] => Ok(Iterator(ValueIterator::with_list(l.clone()))),
        _ => external_error!("list.iter: Expected list as argument"),
    });

    result.add_fn("last", |vm, args| match vm.get_args(args) {
        [List(l)] => match l.data().last() {
            Some(value) => Ok(value.clone()),
            None => Ok(Value::Empty),
        },
        _ => external_error!("list.last: Expected list as argument"),
    });

    result.add_fn("pop", |vm, args| match vm.get_args(args) {
        [List(l)] => match l.data_mut().pop() {
            Some(value) => Ok(value),
            None => Ok(Value::Empty),
        },
        _ => external_error!("list.pop: Expected list as argument"),
    });

    result.add_fn("push", |vm, args| match vm.get_args(args) {
        [List(l), value] => {
            l.data_mut().push(value.clone());
            Ok(Value::Empty)
        }
        _ => external_error!("list.push: Expected list and value as arguments"),
    });

    result.add_fn("remove", |vm, args| match vm.get_args(args) {
        [List(l), Number(n)] => {
            if *n < 0.0 {
                return external_error!("list.remove: Negative indices aren't allowed");
            }
            let index = *n as usize;
            if index >= l.data().len() {
                return external_error!(
                    "list.remove: Index out of bounds - \
                     the index is {} but the List only has {} elements",
                    index,
                    l.data().len(),
                );
            }

            Ok(l.data_mut().remove(index))
        }
        _ => external_error!("list.remove: Expected list and index as arguments"),
    });

    result.add_fn("resize", |vm, args| match vm.get_args(args) {
        [List(l), Number(n), value] => {
            if *n < 0.0 {
                return external_error!("list.resize: Negative sizes aren't allowed");
            }
            l.data_mut().resize(*n as usize, value.clone());
            Ok(Value::Empty)
        }
        _ => external_error!("list.resize: Expected list, number, and value as arguments"),
    });

    result.add_fn("retain", |vm, args| {
        match vm.get_args(args) {
            [List(l), Function(f)] => {
                let l = l.clone();
                let f = f.clone();
                let mut vm = vm.spawn_shared_vm();

                if f.arg_count != 1 {
                    return external_error!(
                        "The function passed to list.retain must have a \
                         single argument, found '{}'",
                        f.arg_count,
                    );
                }
                let mut write_index = 0;
                for read_index in 0..l.len() {
                    let value = l.data()[read_index].clone();
                    match vm.run_function(&f, &[value.clone()])? {
                        Bool(result) => {
                            if result {
                                l.data_mut()[write_index] = value;
                                write_index += 1;
                            }
                        }
                        unexpected => {
                            return external_error!(
                                "list.retain expects a Bool to be returned from the \
                                 predicate, found '{}'",
                                value::type_as_string(&unexpected),
                            );
                        }
                    }
                }
                l.data_mut().resize(write_index, Value::Empty);
            }
            [List(l), value] => {
                l.data_mut().retain(|x| x == value);
            }
            _ => {
                return external_error!(
                    "list.retain: Expected list and function or value as arguments"
                )
            }
        }

        Ok(Value::Empty)
    });

    result.add_fn("reverse", |vm, args| match vm.get_args(args) {
        [List(l)] => {
            l.data_mut().reverse();
            Ok(Value::Empty)
        }
        _ => external_error!("list.reverse: Expected list as argument"),
    });

    result.add_fn("size", |vm, args| match vm.get_args(args) {
        [List(l)] => Ok(Number(l.len() as f64)),
        _ => external_error!("list.size: Expected list as argument"),
    });

    result.add_fn("sort", |vm, args| match vm.get_args(args) {
        [List(l)] => {
            l.data_mut().sort();
            Ok(Value::Empty)
        }
        _ => external_error!("list.sort: Expected list as argument"),
    });

    result.add_fn("sort_copy", |vm, args| match vm.get_args(args) {
        [List(l)] => {
            let mut result = l.data().clone();
            result.sort();
            Ok(List(ValueList::with_data(result)))
        }
        _ => external_error!("list.sort_copy: Expected list as argument"),
    });

    result.add_fn("to_tuple", |vm, args| match vm.get_args(args) {
        [List(l)] => Ok(Value::Tuple(l.data().as_slice().into())),
        _ => external_error!("list.to_tuple expects a list as argument"),
    });

    result.add_fn("transform", |vm, args| match vm.get_args(args) {
        [List(l), Function(f)] => {
            let l = l.clone();
            let f = f.clone();
            let mut vm = vm.spawn_shared_vm();

            if f.arg_count != 1 {
                return external_error!(
                    "The function passed to list.transform must have a \
                         single argument, found '{}'",
                    f.arg_count,
                );
            }

            for value in l.data_mut().iter_mut() {
                *value = vm.run_function(&f, &[value.clone()])?;
            }

            Ok(Value::Empty)
        }
        _ => external_error!("list.transform expects a list and function as arguments"),
    });

    result.add_fn("with_size", |vm, args| match vm.get_args(args) {
        [Number(n), value] => {
            if *n < 0.0 {
                return external_error!("list.with_size: Negative sizes aren't allowed");
            }

            let result = smallvec::smallvec![value.clone(); *n as usize];
            Ok(Value::List(ValueList::with_data(result)))
        }
        _ => external_error!("list.with_size: Expected number and value as arguments"),
    });

    result
}
