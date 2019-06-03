use crate::error::Error;
use crate::ser::to_string;
use serde_derive::*;
use serde_json::*;
use std::{i64, u64};

use std::collections::btree_map::BTreeMap;

#[test]
fn test_struct() {
    #[derive(Serialize)]
    struct Test {
        int: u32,
        seq: Vec<&'static str>,
    }

    let test = Test {
        int: 1,
        seq: vec!["a", "b"],
    };
    let expected = r#"{"int":1,"seq":["a","b"]}"#;
    assert_encode(&test, expected);
}

#[test]
fn test_enum() {
    #[derive(Serialize)]
    enum E {
        Unit,
        Newtype(u32),
        Tuple(u32, u32),
        Struct { a: u32 },
    }

    let u = E::Unit;
    let expected = r#""Unit""#;
    assert_eq!(to_string(&u).unwrap(), expected);

    let n = E::Newtype(1);
    let expected = r#"{"Newtype":1}"#;
    assert_encode(&n, expected);

    let t = E::Tuple(1, 2);
    let expected = r#"{"Tuple":[1,2]}"#;
    assert_encode(&t, expected);

    let s = E::Struct { a: 1 };
    let expected = r#"{"Struct":{"a":1}}"#;
    assert_encode(&s, expected);
}

fn assert_encode<T>(value: &T, expected: &str)
where
    T: serde::ser::Serialize,
{
    assert_eq!(to_string(&value).unwrap(), expected);
}

fn assert_encode_ok<T>(errors: &[(T, &str)])
where
    T: serde::ser::Serialize,
{
    for &(ref value, expected) in errors {
        assert_eq!(to_string(&value).unwrap(), expected);
    }
}

fn assert_encode_err<T>(val: T)
where
    T: serde::ser::Serialize,
{
    match to_string(&val).unwrap_err() {
        Error::Syntax(_, _, _) => (),
        _ => panic!("this should error out"),
    }
}

#[test]
fn write_null() {
    let tests = &[((), "null")];
    assert_encode_ok(tests)
}

#[test]
fn write_u64() {
    let tests = &[(3u64, "3"), (u64::MAX, &u64::MAX.to_string())];
    assert_encode_ok(tests);
}

#[test]
fn write_i64() {
    let tests = &[
        (3i64, "3"),
        (-2i64, "-2"),
        (-1234i64, "-1234"),
        (i64::MIN, &i64::MIN.to_string()),
    ];
    assert_encode_ok(tests);
}

// TODO - Radu M
// error out on nonfinit float values
#[test]
fn encode_nonfinite_float_yields_err() {
    let v = std::f64::NAN;
    assert_encode_err(&v);

    let v = std::f64::INFINITY;
    assert_encode_err(&v);

    let v = std::f32::NAN;
    assert_encode_err(&v);

    let v = std::f32::INFINITY;
    assert_encode_err(&v);
}

// TODO - Radu M
// correctly escape strings
#[test]
fn write_str() {
    let tests = &[
        ("", "\"\""),
        ("foo", "\"foo\""),
        ("\\", "\"\\\\\""),
        ("\"", "\"\\\"\""),
        ("\n", "\"\n\""),
        ("\r", "\"\r\""),
        ("\t", "\"\t\""),
        ("\u{2603}", "\"\u{2603}\""),
    ];
    assert_encode_ok(tests);
}

#[test]
fn write_bool() {
    let tests = &[(true, "true"), (false, "false")];
    assert_encode_ok(tests);
}

#[test]
fn write_list() {
    assert_encode_ok(&[
        (vec![], "[]"),
        (vec![true], "[true]"),
        (vec![true, false], "[true,false]"),
    ]);

    assert_encode_ok(&[
        (vec![vec![], vec![], vec![]], "[[],[],[]]"),
        (vec![vec![1, 2, 3], vec![], vec![]], "[[1,2,3],[],[]]"),
        (vec![vec![], vec![1, 2, 3], vec![]], "[[],[1,2,3],[]]"),
        (vec![vec![], vec![], vec![1, 2, 3]], "[[],[],[1,2,3]]"),
    ]);

    let long_test_list = Value::Array(vec![Value::Bool(false), Value::Null]);

    assert_encode_ok(&[(long_test_list.clone(), "[false,null]")]);
}

#[test]
fn write_tuple() {
    assert_encode_ok(&[((5,), "[5]")]);

    assert_encode_ok(&[((5, (6, "abc")), "[5,[6,\"abc\"]]")]);
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
enum Animal {
    Dog,
    Frog(String, Vec<isize>),
    Cat { age: usize, name: String },
    AntHive(Vec<String>),
}

#[test]
fn write_enum() {
    assert_encode_ok(&[
        (Animal::Dog, "\"Dog\""),
        (
            Animal::Frog("Henry".to_string(), vec![]),
            "{\"Frog\":[\"Henry\",[]]}",
        ),
        (
            Animal::Frog("Henry".to_string(), vec![349]),
            "{\"Frog\":[\"Henry\",[349]]}",
        ),
        (
            Animal::Frog("Henry".to_string(), vec![349, 102]),
            "{\"Frog\":[\"Henry\",[349,102]]}",
        ),
        (
            Animal::Cat {
                age: 5,
                name: "Kate".to_string(),
            },
            "{\"Cat\":{\"age\":5,\"name\":\"Kate\"}}",
        ),
        (
            Animal::AntHive(vec!["Bob".to_string(), "Stuart".to_string()]),
            "{\"AntHive\":[\"Bob\",\"Stuart\"]}",
        ),
    ]);
}

#[test]
fn write_option() {
    assert_encode_ok(&[(None, "null"), (Some("jodhpurs"), "\"jodhpurs\"")]);

    assert_encode_ok(&[
        (None, "null"),
        (Some(vec!["foo", "bar"]), "[\"foo\",\"bar\"]"),
    ]);
}

macro_rules! treemap {
    () => {
        BTreeMap::new()
    };
    ($($k:expr => $v:expr),+) => {
        {
            let mut m = BTreeMap::new();
            $(m.insert($k, $v);)+
            m
        }
    };
}

#[test]
fn write_object() {
    assert_encode_ok(&[
        (treemap!(), "{}"),
        (treemap!("a".to_string() => true), "{\"a\":true}"),
        (
            treemap!(
                "a".to_string() => true,
                "b".to_string() => false
            ),
            "{\"a\":true,\"b\":false}",
        ),
    ]);

    assert_encode_ok(&[
        (
            treemap![
                "a".to_string() => treemap![],
                "b".to_string() => treemap![],
                "c".to_string() => treemap![]
            ],
            "{\"a\":{},\"b\":{},\"c\":{}}",
        ),
        (
            treemap![
                "a".to_string() => treemap![
                    "a".to_string() => treemap!["a" => vec![1,2,3]],
                    "b".to_string() => treemap![],
                    "c".to_string() => treemap![]
                ],
                "b".to_string() => treemap![],
                "c".to_string() => treemap![]
            ],
            "{\"a\":{\"a\":{\"a\":[1,2,3]},\"b\":{},\"c\":{}},\"b\":{},\"c\":{}}",
        ),
        (
            treemap![
                "a".to_string() => treemap![],
                "b".to_string() => treemap![
                    "a".to_string() => treemap!["a" => vec![1,2,3]],
                    "b".to_string() => treemap![],
                    "c".to_string() => treemap![]
                ],
                "c".to_string() => treemap![]
            ],
            "{\"a\":{},\"b\":{\"a\":{\"a\":[1,2,3]},\"b\":{},\"c\":{}},\"c\":{}}",
        ),
        (
            treemap![
                "a".to_string() => treemap![],
                "b".to_string() => treemap![],
                "c".to_string() => treemap![
                    "a".to_string() => treemap!["a" => vec![1,2,3]],
                    "b".to_string() => treemap![],
                    "c".to_string() => treemap![]
                ]
            ],
            "{\"a\":{},\"b\":{},\"c\":{\"a\":{\"a\":[1,2,3]},\"b\":{},\"c\":{}}}",
        ),
    ]);

    let complex_obj = treemap!(
        "b".to_string() => vec![
            treemap!("c".to_string() => String::from("\x0c\x1f\r")),
            treemap!("d".to_string() => String::from(""))
        ]
    );

    assert_encode_ok(&[(
        complex_obj.clone(),
        "{\
         \"b\":[\
         {\"c\":\"\x0c\x1f\r\"},\
         {\"d\":\"\"}\
         ]\
         }",
    )]);
}

#[test]
fn write_newtype_struct() {
    #[derive(Serialize, PartialEq, Debug)]
    struct Newtype(BTreeMap<String, i32>);

    let inner = Newtype(treemap!(String::from("inner") => 123));
    let outer = treemap!(String::from("outer") => &inner);

    assert_encode_ok(&[(&inner, r#"{"inner":123}"#)]);
    assert_encode_ok(&[(outer, r#"{"outer":{"inner":123}}"#)]);
}

// TODO - Radu M
// should the serializer error out on unsorted structs, or should it sort them?
#[test]
fn write_unsorted_struct() {
    #[derive(Serialize, PartialEq, Debug)]
    struct UnsortedStruct {
        z: i64,
        a: i64,
    };

    #[derive(Serialize, PartialEq, Debug)]
    enum UnsortedEnum {
        Boo { z: i64, a: i64 },
    };

    assert_encode_err(UnsortedStruct { z: 1, a: 2 });

    assert_encode_err(&UnsortedEnum::Boo { z: 1, a: 2 });
}
