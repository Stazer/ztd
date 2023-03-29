use ztd_method::Method;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Method)]
#[Method(mutators)]
struct StructSkipAllMutator {
    #[Method(skip_all)]
    first: String,
    _second: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    let mut instance = StructSkipAllMutator {
        first: String::from("foo"),
        _second: String::from("bar"),
    };

    instance.first_mut();
}
