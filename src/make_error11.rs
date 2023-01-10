/// trace error
#[macro_export]
macro_rules! make_error11 {
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
                tracing::error!("{:?}", $name::GeneralError(x.to_string()));
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
