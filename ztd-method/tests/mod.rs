use quote::quote;
use trybuild::TestCases;
use ztd_method::Method;
use ztd_method_macro::Macro;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn struct_with_accessors() {
    #[derive(Method)]
    #[Method(accessors)]
    struct StructWithAccessors {
        first: String,
        _second: String,
    }

    assert!(
        (StructWithAccessors {
            first: String::from("foo"),
            _second: String::from("bar")
        })
        .first()
            == &String::from("foo")
    )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn struct_with_all0() {
    #[derive(Method)]
    #[Method(all)]
    struct StructWithAll0 {
        first: String,
    }

    let mut instance = StructWithAll0 {
        first: String::from("foo"),
    };

    *instance.first_mut() = String::from("foobar");

    assert!(instance.first() == &String::from("foobar"))
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn struct_with_all1() {
    #[derive(Method)]
    #[Method(all)]
    struct StructWithAll0 {
        first: String,
    }

    let mut instance = StructWithAll0 {
        first: String::from("foo"),
    };

    instance.set_first(String::from("foobar"));

    assert!(instance.first() == &String::from("foobar"))
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn struct_with_single_accessor() {
    #[derive(Method)]
    struct StructWithSingleAccessor {
        #[Method(accessor)]
        first: String,
    }

    assert!(
        (StructWithSingleAccessor {
            first: String::from("foo")
        })
        .first()
            == &String::from("foo")
    )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn struct_with_mutators() {
    #[derive(Method)]
    #[Method(mutators)]
    struct StructWithMutators {
        _first: String,
        second: String,
    }

    let mut instance = StructWithMutators {
        _first: String::from("foo"),
        second: String::from("bar"),
    };
    *instance.second_mut() = String::from("foobar");

    assert!(instance.second == String::from("foobar"))
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn struct_with_single_mutator() {
    #[derive(Method)]
    struct StructWithSingleMutator {
        #[Method(mutator)]
        first: String,
    }

    let mut instance = StructWithSingleMutator {
        first: String::from("foo"),
    };
    *instance.first_mut() = String::from("foobar");

    assert!(instance.first == String::from("foobar"))
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn struct_with_setters() {
    #[derive(Method)]
    #[Method(setters)]
    struct StructWithSetters {
        first: String,
    }

    let mut instance = StructWithSetters {
        first: String::from("foo"),
    };
    instance.set_first(String::from("foobar"));

    assert!(instance.first == String::from("foobar"))
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn struct_with_single_setter() {
    #[derive(Method)]
    struct StructWithSingleSetter {
        #[Method(setter)]
        first: String,
    }

    let mut instance = StructWithSingleSetter {
        first: String::from("foo"),
    };
    instance.set_first(String::from("foobar"));

    assert!(instance.first == String::from("foobar"))
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn struct_with_all_on_field0() {
    #[derive(Method)]
    struct StructWithAllForField0 {
        #[Method(all)]
        first: String,
    }

    let mut instance = StructWithAllForField0 {
        first: String::from("foo"),
    };

    instance.set_first(String::from("bar"));

    assert!(instance.first() == &String::from("bar"))
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn tuple_struct_with_all() {
    #[derive(Method)]
    struct TupleStructWithSetter(#[Method(all)] String);

    let mut instance = TupleStructWithSetter(String::from("foo"));

    instance.set_value0(String::from("bar"));

    assert!(instance.value0() == &String::from("bar"))
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn struct_with_all_on_field1() {
    #[derive(Method)]
    struct StructWithAllForField1 {
        #[Method(all)]
        first: String,
    }

    let mut instance = StructWithAllForField1 {
        first: String::from("foo"),
    };

    *instance.first_mut() = String::from("bar");

    assert!(instance.first() == &String::from("bar"))
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn accessors_return_automatic() {
    #[derive(Method)]
    #[Method(accessors, accessors_return_automatic)]
    struct Struct {
        first: u32,
    }

    let instance = Struct { first: 100 };

    assert!(instance.first() == 100)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn is_copy() {
    #[derive(Method)]
    #[Method(accessors)]
    struct _Struct {
        _i8: i8,
        _i16: i16,
        _i32: i32,
        _i64: i64,
        _i128: i128,
        _isize: isize,

        _u8: u8,
        _u16: u16,
        _u32: u32,
        _u64: u64,
        _u128: u128,
        _usize: usize,

        _f32: f32,
        _f64: f64,

        _char: char,

        _bool: bool,

        _empty: (),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn is_single_copy_tuple_copy() {
    #[derive(Method)]
    #[Method(accessors)]
    struct Struct {
        value: (bool,),
    }

    let instance = Struct { value: (true,) };

    assert!(instance.value() == (true,));
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn is_single_non_copy_tuple_non_copy() {
    #[derive(Method)]
    #[Method(accessors)]
    struct Struct {
        value: (String,),
    }

    let instance = Struct {
        value: (String::from("foo"),),
    };

    assert!(instance.value() == &(String::from("foo"),));
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn is_slice_not_copy() {
    #[derive(Method)]
    #[Method(accessors)]
    struct Struct {
        value: [usize; 2],
    }

    let instance = Struct { value: [2, 4] };

    assert!(instance.value() == &[2, 4]);
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn is_multiplie_tuple_not_copy() {
    #[derive(Method)]
    #[Method(accessors)]
    struct Struct {
        value: (bool, bool),
    }

    let instance = Struct {
        value: (true, false),
    };

    assert!(instance.value() == &(true, false));
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn accessor_return_automatic() {
    #[derive(Method)]
    #[Method(accessors)]
    struct Struct {
        #[Method(accessor_return_automatic)]
        first: u32,
    }

    let instance = Struct { first: 100 };

    assert!(instance.first() == 100)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn accessors_return_reference() {
    #[derive(Method)]
    #[Method(accessors, accessors_return_reference)]
    struct Struct {
        first: u32,
    }

    let instance = Struct { first: 100 };

    assert!(*instance.first() == 100)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn accessor_return_reference() {
    #[derive(Method)]
    #[Method(accessors)]
    struct Struct {
        #[Method(accessor_return_reference)]
        first: u32,
    }

    let instance = Struct { first: 100 };

    assert!(*instance.first() == 100)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn accessors_return_copy() {
    #[derive(Clone, Copy, PartialEq)]
    struct Tag;

    #[derive(Method)]
    #[Method(accessors, accessors_return_copy)]
    struct Struct {
        first: Tag,
    }

    let instance = Struct { first: Tag };

    assert!(instance.first() == Tag)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn accessor_return_copy() {
    #[derive(Clone, Copy, PartialEq)]
    struct Tag;

    #[derive(Method)]
    #[Method(accessors)]
    struct Struct {
        #[Method(accessor_return_copy)]
        first: Tag,
    }

    let instance = Struct { first: Tag };

    assert!(instance.first() == Tag)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn ui_struct_skip_accessor() {
    Macro::handle(quote!(
        #[derive(Method)]
        #[Method(accessors)]
        struct StructSkipAccessor {
            #[Method(skip_accessor)]
            first: String,
            _second: String,
        }
    ));
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn ui_struct_skip_all_accessor() {
    Macro::handle(quote!(
        #[derive(Method)]
        #[Method(accessors)]
        struct StructSkipAllAccessor {
            #[Method(skip_all)]
            first: String,
            _second: String,
        }
    ));
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn ui_struct_skip_all_mutator() {
    Macro::handle(quote!(
        #[derive(Method)]
        #[Method(mutators)]
        struct StructSkipAllMutator {
            #[Method(skip_all)]
            first: String,
            _second: String,
        }
    ));
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn ui_struct_skip_all_on_field() {
    Macro::handle(quote!(
        #[derive(Method)]
        #[Method(all)]
        struct StructSkipAllOnField {
            #[Method(skip_all)]
            first: String,
        }
    ));
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn ui_struct_skip_all_setter() {
    Macro::handle(quote!(
        #[derive(Method)]
        #[Method(setters)]
        struct StructSkipAllSetter {
            #[Method(skip_all)]
            first: String,
            _second: String,
        }
    ));
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn ui_struct_skip_mutator() {
    Macro::handle(quote!(
        #[derive(Method)]
        #[Method(mutators)]
        struct StructSkipMutator {
            #[Method(skip_mutator)]
            first: String,
            _second: String,
        }
    ));
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn ui_struct_skip_setter() {
    Macro::handle(quote!(
        #[derive(Method)]
        #[Method(setters)]
        struct StructSkipSetter {
            #[Method(skip_setter)]
            first: String,
            _second: String,
        }
    ));
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn ui() {
    let cases = TestCases::new();

    cases.compile_fail("ui/struct_skip_accessor.rs");
    cases.compile_fail("ui/struct_skip_all_accessor.rs");
    cases.compile_fail("ui/struct_skip_all_mutator.rs");
    cases.compile_fail("ui/struct_skip_all_on_field.rs");
    cases.compile_fail("ui/struct_skip_all_setter.rs");
    cases.compile_fail("ui/struct_skip_mutator.rs");
    cases.compile_fail("ui/struct_skip_setter.rs");
}
