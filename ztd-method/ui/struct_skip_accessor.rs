use ztd_method::Method;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Method)]
#[Method(accessors)]
struct StructSkipAccessor {
    #[Method(skip_accessor)]
    first: String,
    _second: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    let mut instance = StructSkipAccessor {
        first: String::from("foo"),
        _second: String::from("bar"),
    };

    instance.first();
}
