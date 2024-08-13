use askama::Template;
use axum::body::Body;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    AnyError(String),
    ConfigError(String),
    RootConfigError(String),
    ManifestError(String),
    ValidationError(String),
    BadRequest(String),
    Forbidden(String),
    JsonParseError(String),
    ServiceError(String),
}

#[derive(Clone)]
pub struct ErrorInfo {
    pub status_code: StatusCode,
    pub title: String,
    pub message: String,
    pub description: String,
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorData {
    error: ErrorInfo,
}

/// Allow string slices to be converted to Error
impl From<&str> for Error {
    fn from(val: &str) -> Self {
        Self::AnyError(val.to_string())
    }
}

/// Allow errors to be displayed as string
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::AnyError(val) => write!(f, "{}", val),
            Self::ConfigError(val) => write!(f, "{}", val),
            Self::RootConfigError(val) => write!(f, "{}", val),
            Self::ManifestError(val) => write!(f, "{}", val),
            Self::ValidationError(val) => write!(f, "{}", val),
            Self::BadRequest(val) => write!(f, "{}", val),
            Self::Forbidden(val) => write!(f, "{}", val),
            Self::JsonParseError(val) => write!(f, "{}", val),
            Self::ServiceError(val) => write!(f, "{}", val),
        }
    }
}

/// Allow Error to be converted to StatusCode
impl From<Error> for StatusCode {
    fn from(err: Error) -> Self {
        match err {
            Error::AnyError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::RootConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ManifestError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ValidationError(_) => StatusCode::BAD_REQUEST,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::Forbidden(_) => StatusCode::FORBIDDEN,
            Error::JsonParseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ServiceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// Allow error to be converted to response
impl IntoResponse for Error {
    fn into_response(self) -> Response<Body> {
        let error: ErrorInfo = self.into();

        let status_code = error.status_code;
        let tpl = ErrorData { error };

        Response::builder()
            .status(status_code)
            .body(Body::from(tpl.render().unwrap()))
            .unwrap()
    }
}

impl ErrorInfo {
    /// Creates a generic internal server error
    pub fn new(message: String) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            title: "Internal Server Error".to_string(),
            message: message.clone(),
            description: message,
        }
    }
}

impl From<Error> for ErrorInfo {
    fn from(e: Error) -> Self {
        match e {
            Error::AnyError(msg) => Self {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                title: "Internal Server Error".to_string(),
                message: msg.clone(),
                description: msg,
            },
            Error::ConfigError(msg) => Self {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                title: "Configuration Error".to_string(),
                message: msg.clone(),
                description: msg,
            },
            Error::RootConfigError(msg) => Self {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                title: "Root Configuration Error".to_string(),
                message: msg.clone(),
                description: msg,
            },
            Error::ManifestError(msg) => Self {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                title: "Manifest Error".to_string(),
                message: msg.clone(),
                description: msg,
            },
            Error::ValidationError(msg) => Self {
                status_code: StatusCode::BAD_REQUEST,
                title: "Validation Error".to_string(),
                message: msg.clone(),
                description: msg,
            },
            Error::BadRequest(msg) => Self {
                status_code: StatusCode::BAD_REQUEST,
                title: "Bad Request".to_string(),
                message: msg.clone(),
                description: msg,
            },
            Error::Forbidden(msg) => Self {
                status_code: StatusCode::FORBIDDEN,
                title: "Forbidden".to_string(),
                message: msg.clone(),
                description: msg,
            },
            Error::JsonParseError(msg) => Self {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                title: "JSON Parse Error".to_string(),
                message: msg.clone(),
                description: msg,
            },
            Error::ServiceError(msg) => Self {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                title: "Service Error".to_string(),
                message: msg.clone(),
                description: msg,
            },
        }
    }
}
