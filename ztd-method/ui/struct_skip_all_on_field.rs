use ztd_method::Method;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Method)]
#[Method(all)]
struct StructSkipAllOnField {
    #[Method(skip_all)]
    first: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    let mut instance = StructSkipAllOnField {
        first: String::from("foo"),
    };

    instace.set_first(String::from("bar"));
    *instance.first_mut() = String::from("bar");
    instance.first();
}
