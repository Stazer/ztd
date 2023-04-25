use quote::quote;
use trybuild::TestCases;
use ztd_constructor::Constructor;
use ztd_constructor_macro::Macro;

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

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn visibility() {
    #[derive(Constructor)]
    #[Constructor(visibility = pub(crate))]
    pub struct _Struct {
        _first: usize,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
#[should_panic(expected = "expected `=`")]
fn visiblity_should_panic_without_value() {
    Macro::handle(
        quote!(
            #[derive(Constructor)]
            #[Constructor(visibility)]
            pub struct _Struct {
                _first: usize,
            }
        )
        .into(),
    );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
#[should_panic(expected = "Unknown visibility")]
fn visiblity_should_panic_without_visibility_syntax() {
    Macro::handle(
        quote!(
            #[derive(Constructor)]
            #[Constructor(visibility = foobar)]
            pub struct _Struct {
                _first: usize,
            }
        )
        .into(),
    );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
#[should_panic(expected = "Unknown attribute")]
fn should_panic_with_unknown_attribute() {
    Macro::handle(
        quote!(
            #[derive(Constructor)]
            #[Constructor(foo = bar)]
            pub struct _Struct {
                _first: usize,
            }
        )
        .into(),
    );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn ui_visibility() {
    Macro::handle(quote!(
        #[derive(Constructor)]
        #[Constructor(visibility = pub(self))]
        pub struct Struct {
            _first: String,
        }
    ));
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn ui() {
    let cases = TestCases::new();

    cases.compile_fail("ui/visibility.rs");
}
