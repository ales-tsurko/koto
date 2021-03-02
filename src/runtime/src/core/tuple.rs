use crate::{
    external_error, value::deep_copy_value, value_iterator::ValueIterator, Operator, RuntimeError,
    Value, ValueList, ValueMap, Vm,
};

pub fn make_module() -> ValueMap {
    use Value::*;

    let mut result = ValueMap::new();

    result.add_fn("contains", |vm, args| match vm.get_args(args) {
        [Tuple(t), value] => Ok(Bool(t.data().contains(value))),
        _ => external_error!("tuple.contains: Expected tuple and value as arguments"),
    });

    result.add_fn("deep_copy", |vm, args| match vm.get_args(args) {
        [value @ Tuple(_)] => Ok(deep_copy_value(value)),
        _ => external_error!("tuple.deep_copy: Expected tuple as argument"),
    });

    result.add_fn("first", |vm, args| match vm.get_args(args) {
        [Tuple(t)] => match t.data().first() {
            Some(value) => Ok(value.clone()),
            None => Ok(Value::Empty),
        },
        _ => external_error!("tuple.first: Expected tuple as argument"),
    });

    result.add_fn("get", |vm, args| match vm.get_args(args) {
        [Tuple(t), Number(n)] => {
            if *n < 0.0 {
                return external_error!("tuple.get: Negative indices aren't allowed");
            }
            let index: usize = n.into();
            match t.data().get(index) {
                Some(value) => Ok(value.clone()),
                None => Ok(Value::Empty),
            }
        }
        _ => external_error!("tuple.get: Expected tuple and number as arguments"),
    });

    result.add_fn("iter", |vm, args| match vm.get_args(args) {
        [Tuple(t)] => Ok(Iterator(ValueIterator::with_tuple(t.clone()))),
        _ => external_error!("tuple.iter: Expected tuple as argument"),
    });

    result.add_fn("last", |vm, args| match vm.get_args(args) {
        [Tuple(t)] => match t.data().last() {
            Some(value) => Ok(value.clone()),
            None => Ok(Value::Empty),
        },
        _ => external_error!("tuple.last: Expected tuple as argument"),
    });

    result.add_fn("size", |vm, args| match vm.get_args(args) {
        [Tuple(t)] => Ok(Number(t.data().len().into())),
        _ => external_error!("tuple.size: Expected tuple as argument"),
    });

    result.add_fn("sort_copy", |vm, args| match vm.get_args(args) {
        [Tuple(t)] => {
            let mut result = t.data().to_vec();

            fn is_less(vm: &mut Vm, a: Value, b: Value) -> Result<bool, RuntimeError> {
                match vm.run_binary_op(Operator::Less, a, b)? {
                    Bool(val) => Ok(val),
                    _ => unreachable!(),
                }
            }

            fn swap(arr: &mut [Value], i: usize, j: usize) {
                let temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;
            }

            fn partition(
                vm: &mut Vm,
                arr: &mut [Value],
                start: usize,
                end: usize,
            ) -> Result<usize, RuntimeError> {
                let mut pivot = arr[end];
                let mut index = start;
                let mut i = start;

                while i < end {
                    if is_less(vm, arr[i], pivot)? {
                        swap(arr, i, index);
                        index += 1;
                    }

                    i += 1;
                }

                swap(arr, index, end);

                Ok(index)
            }

            fn quick_sort(
                vm: &mut Vm,
                arr: &mut [Value],
                start: usize,
                end: usize,
            ) -> Result<(), RuntimeError> {
                if start >= end {
                    return Ok(());
                }

                let pivot = partition(vm, arr, start, end)?;

                quick_sort(vm, arr, start, (pivot - 1) as usize)?;
                quick_sort(vm, arr, (pivot + 1) as usize, end)?;

                Ok(())
            }

            quick_sort(vm, &mut result, 0, result.len())?;

            Ok(Tuple(result.into()))
        }
        _ => external_error!("tuple.sort_copy: Expected tuple as argument"),
    });

    result.add_fn("to_list", |vm, args| match vm.get_args(args) {
        [Tuple(t)] => Ok(List(ValueList::from_slice(t.data()))),
        _ => external_error!("tuple.to_list: Expected tuple as argument"),
    });

    result
}
