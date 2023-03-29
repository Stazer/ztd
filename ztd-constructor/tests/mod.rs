use ztd_constructor::Constructor;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn r#struct() {
    #[derive(Constructor)]
    struct Struct {
        _first: String,
        second: String,
    }

    assert!(Struct::new(String::from("foo"), String::from("bar")).second == String::from("bar"));
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn struct_with_lifetimes() {
    #[derive(Constructor)]
    struct StructWithLifetimes<'a> {
        _first: &'a mut String,
        second: &'a String,
        _third: String,
    }

    let second = String::from("bar");

    assert!(
        StructWithLifetimes::new(&mut String::from("foo"), &second, String::from("foobar"),).second
            == &second
    )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn tuple_struct() {
    #[derive(Constructor)]
    struct TupleStruct(String, String);

    assert!(TupleStruct::new(String::from("foo"), String::from("bar")).1 == String::from("bar"));
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn tuple_struct_with_lifetimes() {
    #[derive(Constructor)]
    struct TupleStructWithLifetimes<'a>(&'a String, &'a mut String);

    let first = String::from("foo");

    assert!(TupleStructWithLifetimes::new(&first, &mut String::from("bar")).0 == &first)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn unit_struct() {
    #[derive(Constructor)]
    struct UnitStruct;

    assert!(matches!(UnitStruct::new(), UnitStruct))
}
