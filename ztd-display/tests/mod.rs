use ztd_display::Display;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn enum_unit_variant() {
    #[derive(Display)]
    enum Enum {
        Variant,
    }

    assert!(format!("{}", Enum::Variant) == "Variant")
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn enum_unit_variant_with_message() {
    #[derive(Display)]
    enum Enum {
        #[Display("foobar")]
        Variant,
    }

    assert!(format!("{}", Enum::Variant) == "foobar")
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn enum_unnamed_variant_with_one_field() {
    #[derive(Display)]
    enum DisplayEnum {
        Variant(String),
    }

    #[derive(Debug)]
    enum DebugEnum {
        Variant(String),
    }

    let left = format!("{}", DisplayEnum::Variant(String::from("foo")));
    let right = format!("{:?}", DebugEnum::Variant(String::from("foo")));

    assert!(left == right);
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn enum_unnamed_variant_with_two_fields() {
    #[derive(Display)]
    enum DisplayEnum {
        Variant(String, String),
    }

    #[derive(Debug)]
    enum DebugEnum {
        Variant(String, String),
    }

    let left = format!(
        "{}",
        DisplayEnum::Variant(String::from("foo"), String::from("bar"))
    );
    let right = format!(
        "{:?}",
        DebugEnum::Variant(String::from("foo"), String::from("bar"))
    );

    assert!(left == right);
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn enum_unnamed_variant_with_one_field_and_message() {
    #[derive(Display)]
    enum Enum {
        #[Display("foo{value}")]
        Variant(String),
    }

    let left = format!("{}", Enum::Variant(String::from("bar")));

    assert!(left == "foobar");
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn enum_unnamed_variant_with_two_fields_and_message() {
    #[derive(Display)]
    enum Enum {
        #[Display("{value1}foo{value0}")]
        Variant(String, String),
    }

    let left = format!(
        "{}",
        Enum::Variant(String::from("bar"), String::from("hello"))
    );

    assert!(left == "hellofoobar");
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn enum_named_variant() {
    #[derive(Display)]
    enum DisplayEnum {
        Variant { _first: String, _second: String },
    }

    #[derive(Debug)]
    enum DebugEnum {
        Variant { _first: String, _second: String },
    }

    let left = format!(
        "{}",
        DisplayEnum::Variant {
            _first: String::from("foo"),
            _second: String::from("bar"),
        },
    );

    let right = format!(
        "{:?}",
        DebugEnum::Variant {
            _first: String::from("foo"),
            _second: String::from("bar"),
        },
    );

    assert!(left == right);
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn enum_named_variant_with_message() {
    #[derive(Display)]
    enum Enum {
        #[Display("{first} and {second}")]
        Variant { first: String, second: String },
    }

    let format = format!(
        "{}",
        Enum::Variant {
            first: String::from("foo"),
            second: String::from("bar"),
        },
    );

    assert!(format == "foo and bar");
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn r#struct() {
    #[derive(Debug, Display)]
    struct Struct {
        first: String,
        second: String,
    }

    let instance = Struct {
        first: String::from("foo"),
        second: String::from("bar"),
    };

    assert!(format!("{:?}", &instance) == format!("{}", &instance));
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn struct_with_zero_fields() {
    #[derive(Debug, Display)]
    struct Struct {}

    let instance = Struct {};

    assert!(format!("{:?}", &instance) == format!("{}", &instance));
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn struct_with_message() {
    #[derive(Debug, Display)]
    #[Display("{first} and then {second}")]
    struct Struct {
        first: String,
        second: String,
    }

    let instance = Struct {
        first: String::from("foo"),
        second: String::from("bar"),
    };

    assert!(format!("{}", &instance) == "foo and then bar");
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn tuple_struct() {
    #[derive(Debug, Display)]
    struct TupleStruct(String, String);

    let instance = TupleStruct(String::from("foo"), String::from("bar"));

    assert!(format!("{:?}", &instance) == format!("{}", &instance));
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn tuple_struct_with_zero_fields() {
    #[derive(Debug, Display)]
    struct TupleStruct(());

    let instance = TupleStruct(());

    assert!(format!("{:?}", &instance) == format!("{}", &instance));
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn tuple_struct_with_message() {
    #[derive(Display)]
    #[Display("{value0} and then {value1}")]
    struct TupleStruct(String, String);

    let format = format!("{}", TupleStruct(String::from("foo"), String::from("bar")));

    assert!(format == "foo and then bar");
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn newtype_struct() {
    #[derive(Debug, Display)]
    struct TupleStruct(String);

    let instance = TupleStruct(String::from("foo"));

    assert!(format!("{:?}", &instance) == format!("{}", &instance));
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn newtype_struct_with_message() {
    #[derive(Display)]
    #[Display("{value}bar")]
    struct TupleStruct(String);

    let format = format!("{}", TupleStruct(String::from("foo")));

    assert!(format == "foobar");
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn unit_struct() {
    #[derive(Display)]
    struct Struct;

    assert!(format!("{}", Struct) == "Struct");
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn unit_struct_with_message() {
    #[derive(Display)]
    #[Display("foobar")]
    struct Struct;

    assert!(format!("{}", Struct) == "foobar");
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn struct_with_closure() {
    #[derive(Display)]
    //#[Display(|first| format!("Hello {}", first))]
    struct Struct {
        first: String,
    }
}
