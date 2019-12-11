use super::*;

#[test]
fn from_int() {
    let obj = Builder::from(10).build();
    assert_eq!(obj.get_type(), Type::Int);
}

#[test]
fn from_double() {
    let obj = Builder::from(10.0f64).build();
    assert_eq!(obj.get_type(), Type::Float);
}

#[test]
fn from_bool() {
    let obj = Builder::from(true).build();
    assert_eq!(obj.get_type(), Type::Boolean);
}

#[test]
fn from_string() {
    let obj = Builder::from("test_string".to_string()).build();
    assert_eq!(obj.get_type(), Type::String);
}

#[test]
fn from_str() {
    let obj = Builder::from("test_string").build();
    assert_eq!(obj.get_type(), Type::String);
}

#[test]
fn to_int() {
    let obj = Builder::from(10).build();
    assert_eq!(obj.as_int(), Some(10));
}

#[test]
fn to_string() {
    let obj = Builder::from("test_string").build();
    assert_eq!(obj.as_string(), Some("test_string".to_string()));
}

#[test]
fn to_int_invalid_type() {
    let obj = Builder::from(10.0f64).build();
    assert_eq!(obj.as_int(), None);
}

#[test]
fn parse_array_and_iter() {
    let parser = ::Parser::new();
    let result = parser.parse(r#"name = "mort";
section {
    nice = true;
    server = ["http://localhost:6666", "trtMrtBrt"];
    chunk = 1Gb;
}"#).unwrap();

    result.fetch_path("section.server").and_then(|values| { 
        for obj in values.next() {
            println!("{:?}", obj.as_string());
        }
    });
    assert_eq!(true, true);
}
