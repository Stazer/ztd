use ztd_method::Method;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Method)]
#[Method(mutators)]
struct StructSkipMutator {
    #[Method(skip_mutator)]
    first: String,
    _second: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    let mut instance = StructSkipMutator {
        first: String::from("foo"),
        _second: String::from("bar"),
    };

    instance.first_mut();
}
