use ztd_error::Error;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn r#struct() {
    #[derive(Debug, Error)]
    struct Struct;

    impl std::fmt::Display for Struct {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "")
        }
    }

    fn test<T>(_value: T)
    where
        T: std::error::Error,
    {
    }

    test(Struct)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn r#enum() {
    #[derive(Debug, Error)]
    enum Enum {
        Case,
    }

    impl std::fmt::Display for Enum {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "")
        }
    }

    fn test<T>(_value: T)
    where
        T: std::error::Error,
    {
    }

    test(Enum::Case)
}
