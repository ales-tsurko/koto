mod io;
mod list;
mod math;

use koto_runtime::{
    value,
    value::{deref_value, type_as_string},
    Error, Runtime, Value, ValueList, ValueMap,
};
use std::rc::Rc;

#[macro_export]
macro_rules! make_builtin_error {
    ($message:expr) => {{
        let error = Error::BuiltinError { message: $message };
        #[cfg(panic_on_runtime_error)]
        {
            panic!();
        }
        error
    }};
}

#[macro_export]
macro_rules! builtin_error {
    ($error:expr) => {
        Err(crate::make_builtin_error!(String::from($error)))
    };
    ($error:expr, $($y:expr),+) => {
        Err(crate::make_builtin_error!(format!($error, $($y),+)))
    };
}

#[macro_export]
macro_rules! single_arg_fn {
    ($map_name: ident, $fn_name: expr, $type: ident, $match_name: ident, $body: block) => {
        $map_name.add_fn($fn_name, |_, args| {
            if args.len() == 1 {
                match deref_value(&args[0]) {
                    $type($match_name) => $body
                    unexpected => {
                        crate::builtin_error!(
                            "{}.{} only accepts a {} as its argument, found {}",
                            stringify!($map_name),
                            $fn_name,
                            stringify!($type),
                            value::type_as_string(&unexpected)
                        )
                    }
                }
            } else {
                crate::builtin_error!("{}.{} expects a single argument, found {}",
                    stringify!($map_name),
                    $fn_name,
                    args.len()
                )
            }
        });
    }
}

pub fn register<'a>(runtime: &mut Runtime<'a>) {
    use Value::*;

    let global = runtime.global_mut();

    io::register(global);
    list::register(global);
    math::register(global);

    {
        let mut map = ValueMap::new();

        single_arg_fn!(map, "keys", Map, m, {
            Ok(List(Rc::new(ValueList::with_data(
                m.0.keys().map(|k| Str(k.clone())).collect::<Vec<_>>(),
            ))))
        });

        global.add_map("map", map);
    }

    {
        let mut string = ValueMap::new();

        single_arg_fn!(string, "escape", Str, s, {
            Ok(Str(Rc::new(s.escape_default().to_string())))
        });

        single_arg_fn!(string, "lines", Str, s, {
            Ok(List(Rc::new(ValueList::with_data(
                s.lines()
                    .map(|line| Str(Rc::new(line.to_string())))
                    .collect::<Vec<_>>(),
            ))))
        });

        global.add_map("string", string);
    }

    global.add_fn("assert", |_, args| {
        for value in args.iter() {
            match value {
                Bool(b) => {
                    if !b {
                        return builtin_error!("Assertion failed");
                    }
                }
                _ => return builtin_error!("assert only expects booleans as arguments"),
            }
        }
        Ok(Empty)
    });

    global.add_fn("assert_eq", |_, args| {
        if args.len() != 2 {
            builtin_error!("assert_eq expects two arguments, found {}", args.len())
        } else if args[0] == args[1] {
            Ok(Empty)
        } else {
            builtin_error!(
                "Assertion failed, '{}' is not equal to '{}'",
                args[0],
                args[1]
            )
        }
    });

    global.add_fn("assert_ne", |_, args| {
        if args.len() != 2 {
            builtin_error!("assert_ne expects two arguments, found {}", args.len())
        } else if args[0] != args[1] {
            Ok(Empty)
        } else {
            builtin_error!(
                "Assertion failed, '{}' should not be equal to '{}'",
                args[0],
                args[1]
            )
        }
    });

    global.add_fn("assert_near", |_, args| {
        if args.len() != 3 {
            builtin_error!("assert_eq expects three arguments, found {}", args.len())
        } else {
            match (&args[0], &args[1], &args[2]) {
                (Number(a), Number(b), Number(allowed_diff)) => {
                    if (a - b).abs() <= *allowed_diff {
                        Ok(Empty)
                    } else {
                        builtin_error!(
                            "Assertion failed, '{}' and '{}' are not within {} of each other",
                            a,
                            b,
                            allowed_diff
                        )
                    }
                }
                (a, b, c) => builtin_error!(
                    "assert_near expects Numbers as arguments, found '{}', '{}', and '{}'",
                    type_as_string(&a),
                    type_as_string(&b),
                    type_as_string(&c)
                ),
            }
        }
    });

    global.add_fn("size", |_, args| {
        let mut arg_iter = args.iter();
        let first_arg_value = match arg_iter.next() {
            Some(arg) => arg,
            None => {
                return builtin_error!("Missing list as first argument for size");
            }
        };

        match first_arg_value {
            Empty => Ok(Number(0.0)),
            List(list) => Ok(Number(list.data().len() as f64)),
            Range { start, end } => {
                println!("size: start: {} end: {}", start, end);

                Ok(Number(if end >= start {
                    end - start
                } else {
                    start - end
                } as f64))
            }
            unexpected => builtin_error!(
                "size is only supported for lists and ranges, found {}",
                unexpected
            ),
        }
    });

    global.add_fn("number", |_, args| {
        let mut arg_iter = args.iter();
        let first_arg_value = match arg_iter.next() {
            Some(arg) => arg,
            None => {
                return builtin_error!("Missing list as first argument for size");
            }
        };

        match first_arg_value {
            Number(_) => Ok(first_arg_value.clone()),
            Str(s) => match s.parse::<f64>() {
                Ok(n) => Ok(Number(n)),
                Err(_) => builtin_error!("Failed to convert '{}' into a Number", s),
            },
            unexpected => builtin_error!(
                "number is only supported for numbers and strings, found {}",
                unexpected
            ),
        }
    });

    global.add_fn("print", |_, args| {
        for value in args.iter() {
            print!("{}", value);
        }
        println!();
        Ok(Empty)
    });
}