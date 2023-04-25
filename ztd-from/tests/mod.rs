use quote::quote;
use ztd_from::From;
use ztd_from_macro::Macro;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn enum_named_variant_with_two_fields() {
    #[derive(From)]
    enum Enum {
        #[From]
        Case { _first: String, _second: String },
    }

    fn test<T>(_value: T)
    where
        T: From<(String, String)>,
    {
    }

    test(Enum::Case {
        _first: String::default(),
        _second: String::default(),
    });
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn enum_named_variant_with_named() {
    #[derive(From)]
    #[From(named)]
    enum Enum {
        Case { _first: String, _second: String },
    }

    fn test<T>(_value: T)
    where
        T: From<(String, String)>,
    {
    }

    test(Enum::Case {
        _first: String::default(),
        _second: String::default(),
    });
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn enum_unnamed_variant_with_zero_fields() {
    #[derive(From)]
    enum Enum {
        #[From]
        Error(),
    }

    fn test<T>(_value: T)
    where
        T: From<()>,
    {
    }

    test(Enum::Error());
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn enum_unnamed_variant_with_one_field() {
    #[derive(From)]
    enum Enum {
        #[From]
        Error(std::io::Error),
    }

    fn test<T>(_value: T)
    where
        T: From<std::io::Error>,
    {
    }

    test(Enum::Error(std::io::Error::new(
        std::io::ErrorKind::Other,
        "",
    )))
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn enum_unnamed_variant_with_two_fields() {
    #[derive(From)]
    enum Enum {
        #[From]
        Error(std::io::Error, String),
    }

    fn test<T>(_value: T)
    where
        T: From<(std::io::Error, String)>,
    {
    }

    test(Enum::Error(
        std::io::Error::new(std::io::ErrorKind::Other, ""),
        String::from("foo"),
    ))
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn enum_unnamed_variant_with_unnamed() {
    #[derive(From)]
    #[From(unnamed)]
    enum Enum {
        Error(std::io::Error, String),
    }

    fn test<T>(_value: T)
    where
        T: From<(std::io::Error, String)>,
    {
    }

    test(Enum::Error(
        std::io::Error::new(std::io::ErrorKind::Other, ""),
        String::from("foo"),
    ))
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn enum_unit_variant() {
    #[derive(From)]
    enum Enum {
        #[From]
        Case,
    }

    fn test<T>(_value: T)
    where
        T: From<()>,
    {
    }

    test(Enum::Case)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn enum_unit_variant_with_all() {
    #[derive(From)]
    #[From(all)]
    enum Enum {
        Case,
    }

    fn test<T>(_value: T)
    where
        T: From<()>,
    {
    }

    test(Enum::Case)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn enum_unit_variant_with_unit() {
    #[derive(From)]
    #[From(unit)]
    enum Enum {
        Case,
    }

    fn test<T>(_value: T)
    where
        T: From<()>,
    {
    }

    test(Enum::Case)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn enum_unit_variant_with_explicit_enable() {
    #[derive(From)]
    enum Enum {
        #[From(enable)]
        Case0,
        _Case1,
    }

    fn test<T>(_value: T)
    where
        T: From<()>,
    {
    }

    test(Enum::Case0)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn enum_unit_variant_with_skip() {
    #[derive(From)]
    #[From(all)]
    enum Enum {
        #[From(skip)]
        _Case0,
        Case1,
    }

    fn test<T>(_value: T)
    where
        T: From<()>,
    {
    }

    test(Enum::Case1)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn unit_struct() {
    #[derive(From)]
    struct Struct;

    fn test<T>(_value: T)
    where
        T: From<()>,
    {
    }

    test(Struct)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn tuple_struct_with_zero_fields() {
    #[derive(From)]
    struct Struct();
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn tuple_struct_with_one_field() {
    #[derive(From)]
    struct Struct(String);

    fn test<T>(_value: T)
    where
        T: From<String>,
    {
    }

    test(Struct(String::default()))
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn tuple_struct_with_two_fields() {
    #[derive(From)]
    struct Struct(String, String);

    fn test<T>(_value: T)
    where
        T: From<(String, String)>,
    {
    }

    test(Struct(String::default(), String::default()))
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn r#struct() {
    #[derive(From)]
    struct Struct {
        _first: String,
        _second: String,
    }

    fn test<T>(_value: T)
    where
        T: From<(String, String)>,
    {
    }

    test(Struct {
        _first: String::default(),
        _second: String::default(),
    })
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn multiple_enum_variants() {
    #[derive(From)]
    #[From(all)]
    enum Enum {
        Case0,
        Case1(std::io::Error),
        Case2 { _first: String, _second: String },
    }

    fn test0<T>(_value: T)
    where
        T: From<()>,
    {
    }

    fn test1<T>(_value: T)
    where
        T: From<std::io::Error>,
    {
    }

    fn test2<T>(_value: T)
    where
        T: From<(String, String)>,
    {
    }

    test0(Enum::Case0);
    test1(Enum::Case1(std::io::Error::new(
        std::io::ErrorKind::Other,
        "",
    )));
    test2(Enum::Case2 {
        _first: String::default(),
        _second: String::default(),
    });
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
#[should_panic(expected = "Unsupported item")]
fn union_should_panic() {
    Macro::handle(
        quote!(
            #[derive(From)]
            union Union {
            }
        )
        .into(),
    );
}
