use cgp::prelude::*;

#[test]
pub fn test_string_getter() {
    #[cgp_getter]
    pub trait HasFoo {
        fn foo(&self) -> &str;
    }

    #[cgp_context]
    #[derive(HasField)]
    pub struct App {
        pub bar: String,
    }

    delegate_components! {
        AppComponents {
            FooGetterComponent: UseField<symbol!("bar")>,
        }
    }

    let context = App {
        bar: "abc".to_owned(),
    };

    assert_eq!(context.foo(), "abc");
}

#[test]
pub fn test_string_auto_getter() {
    #[cgp_auto_getter]
    pub trait HasFoo {
        fn foo(&self) -> &str;
    }

    #[cgp_context]
    #[derive(HasField)]
    pub struct App {
        pub foo: String,
    }

    let context = App {
        foo: "abc".to_owned(),
    };

    assert_eq!(context.foo(), "abc");
}
