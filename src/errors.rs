use actix_web::error::BlockingError;
use actix_web::web::HttpResponse;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::{DatabaseError, NotFound};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    RecordAlreadyExists,
    RecordNotFound,
    DatabaseError(diesel::result::Error),
    OperatonCanceled
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::RecordAlreadyExists => write!(f, "This record violates a unique constraint"),
            AppError::RecordNotFound => write!(f, "This record doesn't exist"),
            AppError::DatabaseError(e) => write!(f, "Database error: {:?}", e),
            AppError::OperatonCanceled => write!(f, "The running operation was canceled")
        }
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            DatabaseError(UniqueViolation, _) => AppError::RecordAlreadyExists,
            NotFound => AppError::RecordNotFound,
            _ => AppError::DatabaseError(error)
        }
    }
}

impl From<BlockingError<AppError>> for AppError {
    fn from(error: BlockingError<AppError>) -> Self {
        match error {
            BlockingError::Error(inner) => inner,
            BlockingError::Canceled => AppError::OperatonCanceled
        }
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String
}

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error = format!("{}", self);
        let mut builder = match self {
            AppError::RecordAlreadyExists => HttpResponse::BadRequest(),
            AppError::RecordNotFound => HttpResponse::NotFound(),
            _ => HttpResponse::InternalServerError(),
        };
        builder.json(ErrorResponse { error })
    }

    fn render_response(&self) -> HttpResponse {
        self.error_response()
    }
}
