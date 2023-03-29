use ztd_method::Method;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Method)]
#[Method(setters)]
struct StructSkipSetter {
    #[Method(skip_setter)]
    first: String,
    _second: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    let mut instance = StructSkipSetter {
        first: String::from("foo"),
        _second: String::from("bar"),
    };

    instance.set_first(String::from("foobar"));
}
