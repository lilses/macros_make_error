/// remove name argument
#[macro_export]
macro_rules! make_error30 {
    (@s actix) => {
        impl actix_web::ResponseError for Error {
            fn error_response(&self) -> actix_web::HttpResponse {
                actix_web::HttpResponse::build(self.status_code())
                    .insert_header(actix_web::http::header::ContentType::html())
                    .body(serde_json::json!({"error":self.to_string()}).to_string())
            }
            fn status_code(&self) -> actix_web::http::StatusCode {
                match *self {
                    Error::GeneralError(_) => {
                        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
                    },
                }
            }
        }
    };
    ([$($plugin:ident),*]) => {
        #[derive(Debug, Clone)]
        pub enum Error {
            GeneralError(String),
        }

        impl Error {
            pub fn from_general<T>(x: T) -> Self
            where
                T: ToString,
            {
                Error::GeneralError(x.to_string())
            }

            pub fn from_general_with_trace<T>(x: T) -> Self
            where
                T: ToString,
            {
                tracing::error!("{:?}", Error::GeneralError(x.to_string()));
                Error::GeneralError(x.to_string())
            }
        }

        impl std::fmt::Display for Error {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                match self {
                    Error::GeneralError(s) => {
                        write!(f, "{}: {}", file!(), s)
                    },
                }
            }
        }

        $(
            make_error30!(@s $plugin);
        )*

    };
}
