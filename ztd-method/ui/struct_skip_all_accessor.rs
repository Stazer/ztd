use ztd_method::Method;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Method)]
#[Method(accessors)]
struct StructSkipAllAccessor {
    #[Method(skip_all)]
    first: String,
    _second: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    let mut instance = StructSkipAllAccessor {
        first: String::from("foo"),
        _second: String::from("bar"),
    };

    instance.first();
}
