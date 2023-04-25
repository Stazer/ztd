use quote::quote;
use trybuild::TestCases;
use ztd_record::Record;
use ztd_record_macro::Macro;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn r#struct() {
    #[derive(Record)]
    struct Struct {
        first: String,
    }

    assert!(
        (Struct {
            first: String::from("foo"),
        })
        .into_record()
        .first
            == String::from("foo"),
    )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn struct_with_lifetime() {
    #[derive(Record)]
    struct StructWithLifetime<'a> {
        first: &'a String,
    }

    let first = String::from("foo");

    assert!((StructWithLifetime { first: &first }).into_record().first == &first)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn struct_flatten() {
    #[derive(Record)]
    struct Struct2 {
        name: &'static str,
    }

    #[derive(Record)]
    struct Struct {
        #[Record(flatten)]
        child: Struct2,
    }

    assert!(
        (Struct {
            child: Struct2 { name: "foo" }
        })
        .into_record()
        .child
        .name
            == "foo",
    )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn tuple_struct_flatten() {
    #[derive(Record)]
    struct TupleStruct2(&'static str, &'static str);

    #[derive(Record)]
    struct TupleStruct(#[Record(flatten)] TupleStruct2, &'static str);

    assert!(
        TupleStruct(TupleStruct2("foo", "bar"), "hello")
            .into_record()
            .0
             .1
            == "bar",
    )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn tuple_struct_skip_first() {
    #[derive(Record)]
    struct TupleStruct(#[Record(skip)] &'static str, &'static str);

    assert!(TupleStruct("foo", "bar").into_record().0 == "bar");
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn tuple_struct_skip_second() {
    #[derive(Record)]
    struct TupleStruct(&'static str, #[Record(skip)] &'static str);

    assert!(TupleStruct("foo", "bar").into_record().0 == "foo");
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn unit_struct() {
    #[derive(Record)]
    struct UnitStruct;

    assert!(matches!(UnitStruct.into_record(), UnitStructRecord))
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
#[should_panic(expected = "Cannot flatten Type::Tuple { paren_token: Paren, elems: [] }")]
fn invalid_flatten_on_tuple_type() {
    Macro::handle(quote!(
        #[derive(Record)]
        struct Struct {
            #[Record(flatten)]
            _first: (),
        }
    ));
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn ui_struct_skip() {
    Macro::handle(quote!(
        #[derive(Record)]
        struct Struct {
            #[Record(skip)]
            field: String,
        }
    ));
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn ui() {
    let cases = TestCases::new();
    cases.compile_fail("ui/struct_skip.rs")
}
