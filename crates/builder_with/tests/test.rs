use builder_with::With;

#[derive(With, Debug, Default, PartialEq, Clone)]
struct MyStruct {
    number: f64,
    string: String,
}

#[test]
fn test() {
    assert_eq!(
        MyStruct {
            number: 10.,
            string: String::from("hello")
        },
        MyStruct::default()
            .with_number(10.)
            .with_string(String::from("hello"))
            .to_owned()
    );
}
