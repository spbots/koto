use crate::{external_error, Value, ValueIterator, ValueMap};

pub fn make_module() -> ValueMap {
    use Value::*;

    let mut result = ValueMap::new();

    result.add_fn("contains", |_, args| match args {
        [Range(r), Number(n)] => Ok(Bool(*n >= r.start as f64 && n.ceil() < r.end as f64)),
        _ => external_error!("range.contains: Expected range and number as arguments"),
    });

    result.add_fn("iter", |_, args| match args {
        [Range(r)] => Ok(Iterator(ValueIterator::with_range(*r))),
        _ => external_error!("range.iter: Expected range as argument"),
    });

    result.add_fn("size", |_, args| match args {
        [Range(r)] => Ok(Number((r.end - r.start) as f64)),
        _ => external_error!("range.size: Expected range as argument"),
    });

    result
}