macro_rules! get_env {
    ($name:ident) => {
        pub static $name: once_cell::sync::Lazy<&'static str> = once_cell::sync::Lazy::new(|| {
            Box::leak(
                std::env::var(stringify!($name))
                    .expect(concat!("Missing env ", stringify!($name)))
                    .into_boxed_str(),
            )
        });
    };
}


get_env!(DATABASE_URL);
get_env!(FRONTEND_URL);
