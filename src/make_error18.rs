/// add plugins
#[macro_export]
macro_rules! make_error18 {
    (@s actix) => {
        impl actix_web::ResponseError for $name {
            fn error_response(&self) -> actix_web::HttpResponse {
                actix_web::HttpResponse::build(self.status_code())
                    .insert_header(actix_web::http::header::ContentType::html())
                    .body(serde_json::json!({"error":self.to_string()}).to_string())
            }
            fn status_code(&self) -> actix_web::http::StatusCode {
                match *self {
                    $name::GeneralError(_) => {
                        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
                    },
                }
            }
        }
    };
    ($name: ident, [$($p:ident),*]) => {
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

            pub fn from_general_with_trace<T>(x: T) -> Self
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

        $(
            make_error18!(@s $p);
        )*

    };
}
