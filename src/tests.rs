use crate::error::Error;
use crate::ser::to_string;
use serde_derive::*;
use serde_json::Value;
use std::{f32, i16, i32, i64, i8, u16, u32, u64, u8};

use std::collections::{BTreeMap, HashMap};

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
        Error::Custom(_) => (),
        _ => panic!("this should error out"),
    }
}

#[test]
fn write_null() {
    let tests = &[((), "null")];
    assert_encode_ok(tests)
}

#[test]
fn write_bool() {
    let tests = &[(true, "true"), (false, "false")];
    assert_encode_ok(tests);
}

#[test]
fn write_i8() {
    let tests = &[
        (3i8, "3"),
        (i8::MIN, &i8::MIN.to_string()),
        (i8::MAX, &i8::MAX.to_string()),
    ];
    assert_encode_ok(tests);
}

#[test]
fn write_i16() {
    let tests = &[
        (3i16, "3"),
        (i16::MIN, &i16::MIN.to_string()),
        (i16::MAX, &i16::MAX.to_string()),
    ];
    assert_encode_ok(tests);
}

#[test]
fn write_i32() {
    let tests = &[
        (3i32, "3"),
        (46i32, "46"),
        (-1933i32, "-1933"),
        (i32::MIN, &i32::MIN.to_string()),
        (i32::MAX, &i32::MAX.to_string()),
    ];
    assert_encode_ok(tests);
}

#[test]
fn write_i64() {
    let tests = &[
        (3i64, "3"),
        (-2i64, "-2"),
        (-1234i64, "-1234"),
        (i64::MIN, &i64::MIN.to_string()),
        (i64::MAX, &i64::MAX.to_string()),
    ];
    assert_encode_ok(tests);
}

#[test]
fn write_u8() {
    let tests = &[
        (3u8, "3"),
        (46u8, "46"),
        (254u8, "254"),
        (u8::MIN, &u8::MIN.to_string()),
        (u8::MAX, &u8::MAX.to_string()),
    ];
    assert_encode_ok(tests);
}

#[test]
fn write_u16() {
    let tests = &[
        (3u16, "3"),
        (46u16, "46"),
        (254u16, "254"),
        (u16::MIN, &u16::MIN.to_string()),
        (u16::MAX, &u16::MAX.to_string()),
    ];
    assert_encode_ok(tests);
}

#[test]
fn write_u32() {
    let tests = &[
        (3u32, "3"),
        (46u32, "46"),
        (254u32, "254"),
        (u32::MIN, &u32::MIN.to_string()),
        (u32::MAX, &u32::MAX.to_string()),
    ];
    assert_encode_ok(tests);
}

#[test]
fn write_u64() {
    let tests = &[
        (3u64, "3"),
        (46u64, "46"),
        (254u64, "254"),
        (u64::MIN, &u64::MIN.to_string()),
        (u64::MAX, &u64::MAX.to_string()),
    ];
    assert_encode_ok(tests);
}

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

#[test]
fn encode_f32_ne_int() {
    let v = 3.1f32;
    assert_encode_err(&v);

    let v = -1.3f32;
    assert_encode_err(&v);
}

#[test]
fn write_f32() {
    let tests = &[(3.0f32, "3"), (46.0f32, "46"), (-254.0f32, "-254")];
    assert_encode_ok(tests);
}

#[test]
fn encode_f64_ne_int() {
    let v = 3.1f64;
    assert_encode_err(&v);

    let v = -1.3f64;
    assert_encode_err(&v);
}

#[test]
fn write_f64() {
    let tests = &[
        (3.0f64, "3"),
        (46.0f64, "46"),
        (-254.0f64, "-254"),
        (1234567f64, "1234567"),
    ];
    assert_encode_ok(tests);
}

// TODO - Radu M
// port string escape tests from the Go implementation
// https://github.com/docker/go/blob/master/canonical/json/encode_test.go#L489-L525

#[test]
fn write_string() {
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
fn write_bytes() {
    let buf: Vec<i32> = vec![];
    assert_encode(&buf, "[]");

    let buf = vec![1, 2, 3];
    assert_encode(&buf, "[1,2,3]");
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
fn write_map() {
    let mut map: BTreeMap<&str, BTreeMap<&str, &str>> = BTreeMap::new();
    map.insert("a", BTreeMap::new());
    map.insert("b", BTreeMap::new());
    assert_encode_ok(&[(map.clone(), "{\"a\":{},\"b\":{}}")]);
}

#[test]
fn order_btree_map() {
    let mut map: BTreeMap<&str, BTreeMap<&str, &str>> = BTreeMap::new();
    map.insert("z", BTreeMap::new());
    map.insert("a", BTreeMap::new());
    assert_encode_ok(&[(map.clone(), r#"{"a":{},"z":{}}"#)]);
}

#[test]
fn order_hash_map() {
    let mut map: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
    map.insert("b", HashMap::new());
    map.insert("a", HashMap::new());
    assert_encode_ok(&[(map.clone(), r#"{"a":{},"b":{}}"#)]);
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

#[test]
fn write_struct() {
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
fn write_struct_ordered() {
    #[derive(Serialize)]
    struct Test {
        b: u32,
        a: u32,
    }

    let test = Test { b: 2, a: 1 };

    let expected = r#"{"a":1,"b":2}"#;
    assert_encode(&test, expected);
}

#[test]
fn write_enum_derive() {
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
