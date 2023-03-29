use ztd_method::Method;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Method)]
#[Method(setters)]
struct StructSkipAllSetter {
    #[Method(skip_all)]
    first: String,
    _second: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    let mut instance = StructSkipAllSetter {
        first: String::from("foo"),
        _second: String::from("bar"),
    };

    instance.set_first(String::from("foobar"));
}
