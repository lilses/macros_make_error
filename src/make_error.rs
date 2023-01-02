#[macro_export]
macro_rules! make_error {
    ($name: ident) => {
        #[derive(Debug, Clone)]
        pub enum $name {
            GeneralError(String),
        }

        impl $name {
            pub fn from_general<T>(x: T) -> Self
            where
                T: ToString,
            {
                $name::GeneralError(x.to_string())
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                match self {
                    $name::GeneralError(s) => {
                        write!(f, "{}: {}", stringify!($name), s)
                    },
                }
            }
        }
    };
}
