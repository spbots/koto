use crate::{external_error, Value, ValueIterator, ValueMap};

pub fn make_module() -> ValueMap {
    use Value::*;

    let mut result = ValueMap::new();

    result.add_fn("contains", |vm, args| match vm.get_args(args) {
        [Range(r), Number(n)] => Ok(Bool(*n >= r.start as f64 && n.ceil() < r.end as f64)),
        _ => external_error!("range.contains: Expected range and number as arguments"),
    });

    result.add_fn("end", |vm, args| match vm.get_args(args) {
        [Range(r)] => Ok(Number(r.end as f64)),
        _ => external_error!("range.end: Expected range as argument"),
    });

    result.add_fn("iter", |vm, args| match vm.get_args(args) {
        [Range(r)] => Ok(Iterator(ValueIterator::with_range(*r))),
        _ => external_error!("range.iter: Expected range as argument"),
    });

    result.add_fn("size", |vm, args| match vm.get_args(args) {
        [Range(r)] => Ok(Number((r.end - r.start) as f64)),
        _ => external_error!("range.size: Expected range as argument"),
    });

    result.add_fn("start", |vm, args| match vm.get_args(args) {
        [Range(r)] => Ok(Number(r.start as f64)),
        _ => external_error!("range.start: Expected range as argument"),
    });

    result
}
