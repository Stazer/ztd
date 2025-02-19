use quote::quote;
use trybuild::TestCases;
use ztd_inner::Inner;
use ztd_inner_macro::Macro;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn r#struct() {
    #[derive(Inner)]
    struct Struct {
        first: String,
    }

    assert!(
        (Struct {
            first: String::from("foo"),
        })
        .into_inner()
        .first
            == String::from("foo"),
    )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn struct_with_lifetime() {
    #[derive(Inner)]
    struct StructWithLifetime<'a> {
        first: &'a String,
    }

    let first = String::from("foo");

    assert!((StructWithLifetime { first: &first }).into_inner().first == &first)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn struct_flatten() {
    #[derive(Inner)]
    struct Struct2 {
        name: &'static str,
    }

    #[derive(Inner)]
    struct Struct {
        #[Inner(flatten)]
        child: Struct2,
    }

    assert!(
        (Struct {
            child: Struct2 { name: "foo" }
        })
        .into_inner()
        .child
        .name
            == "foo",
    )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn tuple_struct_flatten() {
    #[derive(Inner)]
    struct TupleStruct2(&'static str, &'static str);

    #[derive(Inner)]
    struct TupleStruct(#[Inner(flatten)] TupleStruct2, &'static str);

    assert!(
        TupleStruct(TupleStruct2("foo", "bar"), "hello")
            .into_inner()
            .0
             .1
            == "bar",
    )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn tuple_struct_skip_first() {
    #[derive(Inner)]
    struct TupleStruct(#[Inner(skip)] &'static str, &'static str);

    assert!(TupleStruct("foo", "bar").into_inner().0 == "bar");
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn tuple_struct_skip_second() {
    #[derive(Inner)]
    struct TupleStruct(&'static str, #[Inner(skip)] &'static str);

    assert!(TupleStruct("foo", "bar").into_inner().0 == "foo");
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn unit_struct() {
    #[derive(Inner)]
    struct UnitStruct;

    assert!(matches!(UnitStruct.into_inner(), UnitStructInner))
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
#[should_panic(expected = "Cannot flatten Type::Tuple { paren_token: Paren, elems: [] }")]
fn invalid_flatten_on_tuple_type() {
    Macro::handle(quote!(
        #[derive(Inner)]
        struct Struct {
            #[Inner(flatten)]
            _first: (),
        }
    ));
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn ui_struct_skip() {
    Macro::handle(quote!(
        #[derive(Inner)]
        struct Struct {
            #[Inner(skip)]
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
