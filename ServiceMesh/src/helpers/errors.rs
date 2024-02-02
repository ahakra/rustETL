use warp::reject::Reject;
use warp::{reject::Rejection, reply::Reply};
use warp::http::StatusCode;


#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    QuestionNotFound,
    DuplicateKey,
    SQLError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => write!(f, "Cannot parse parameter: {}", err),
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::QuestionNotFound => write!(f, "Question not found"),
            Error::DuplicateKey => write!(f, "Question not found"),
            Error::SQLError => write!(f, "Question not found"),
        }
    }
}
impl Reject for Error {}

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    //println!("{:?}",r);
    if let Some(error) = r.find::<Error>()  {
        match error {
            Error::DuplicateKey => {
                Ok(warp::reply::with_status("Duplicate parameter", StatusCode::BAD_REQUEST))
            }
            Error::QuestionNotFound => {
                Ok(warp::reply::with_status("Question not found", StatusCode::NOT_FOUND))
            }
            _ => {
                // Handle other errors if needed
                Err(r)
            }
        }
    } else {
        Ok(warp::reply::with_status(
            "Route not found",
            StatusCode::NOT_FOUND,
            ))
    }

  
}
    