use crate::{external_error, type_as_string, Value, ValueMap};

pub fn make_module() -> ValueMap {
    use Value::*;

    let mut result = ValueMap::new();

    macro_rules! number_fn {
        ($fn:ident) => {
            number_fn!(stringify!($fn), $fn)
        };
        ($name:expr, $fn:ident) => {
            result.add_fn($name, |vm, args| match vm.get_args(args) {
                [Number(n)] => Ok(Number(n.$fn())),
                [other] => external_error!(
                    "number.{}: Expected Number as argument, found '{}'",
                    $name,
                    type_as_string(other)
                ),
                _ => external_error!("number.{} expects a Number as argument", $name),
            });
        };
    }

    macro_rules! number_f64_fn {
        ($fn:ident) => {
            number_f64_fn!(stringify!($fn), $fn)
        };
        ($name:expr, $fn:ident) => {
            result.add_fn($name, |vm, args| match vm.get_args(args) {
                [Number(n)] => Ok(Number(f64::from(n).$fn().into())),
                [other] => external_error!(
                    "number.{} expects a Number as argument, found {}",
                    $name,
                    type_as_string(other),
                ),
                _ => external_error!("number.{} expects a Number as argument", $name),
            })
        };
    }

    number_fn!(abs);
    number_f64_fn!(acos);
    number_f64_fn!(asin);
    number_f64_fn!(atan);
    number_fn!(ceil);

    result.add_fn("clamp", |vm, args| match vm.get_args(args) {
        [Number(x), Number(a), Number(b)] => Ok(Number(*a.max(b.min(x)))),
        _ => external_error!("number.clamp: Expected three numbers as arguments"),
    });

    number_f64_fn!(cos);
    number_f64_fn!(cosh);
    number_f64_fn!("degrees", to_degrees);

    result.add_value("e", Number(std::f64::consts::E.into()));

    number_f64_fn!(exp);
    number_f64_fn!(exp2);
    number_fn!(floor);

    result.add_value("infinity", Number(std::f64::INFINITY.into()));

    result.add_fn("is_nan", |vm, args| match vm.get_args(args) {
        [Number(n)] => Ok(Bool(n.is_nan())),
        _ => external_error!("number.is_nan: Expected Number as argument"),
    });

    number_f64_fn!(ln);
    number_f64_fn!(log2);
    number_f64_fn!(log10);

    result.add_fn("max", |vm, args| match vm.get_args(args) {
        [Number(a), Number(b)] => Ok(Number(*a.max(b))),
        _ => external_error!("number.max: Expected two numbers as arguments"),
    });

    result.add_fn("min", |vm, args| match vm.get_args(args) {
        [Number(a), Number(b)] => Ok(Number(*a.min(b))),
        _ => external_error!("number.min: Expected two numbers as arguments"),
    });

    result.add_value("nan", Number(std::f64::NAN.into()));
    result.add_value("negative_infinity", Number(std::f64::NEG_INFINITY.into()));
    result.add_value("pi", Number(std::f64::consts::PI.into()));

    result.add_fn("pow", |vm, args| match vm.get_args(args) {
        [Number(a), Number(b)] => Ok(Number(a.pow(*b))),
        _ => external_error!("number.pow: Expected two numbers as arguments"),
    });

    number_f64_fn!("radians", to_radians);
    number_f64_fn!(recip);
    number_f64_fn!(sin);
    number_f64_fn!(sinh);
    number_f64_fn!(sqrt);
    number_f64_fn!(tan);
    number_f64_fn!(tanh);

    result.add_fn("to_float", |vm, args| match vm.get_args(args) {
        [Number(n)] => Ok(Number(f64::from(n).into())),
        _ => external_error!("number.to_float: Expected Number as argument"),
    });

    result.add_fn("to_int", |vm, args| match vm.get_args(args) {
        [Number(n)] => Ok(Number(i64::from(n).into())),
        _ => external_error!("number.to_int: Expected Number as argument"),
    });

    result.add_value("tau", Number(std::f64::consts::TAU.into()));

    result
}
