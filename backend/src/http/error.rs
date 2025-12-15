// Reacher - Email Verification
// Copyright (C) 2018-2023 Reacher

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::storage::error::StorageError;
use email_validator_core::{CheckEmailInputBuilderError, LOG_TARGET};
use serde::ser::SerializeStruct;
use serde::Serialize;
use std::fmt;
use std::fmt::Debug;
use tracing::error;
use warp::{http::StatusCode, reject};

/// Trait combining Display and Debug.
pub trait DisplayDebug: fmt::Display + Debug + Sync + Send {}
impl<T: fmt::Display + Debug + Sync + Send> DisplayDebug for T {}

/// Struct describing an error response.
#[derive(Debug, thiserror::Error)]
pub struct ApiResponseError {
	pub code: StatusCode,
	pub error: Box<dyn DisplayDebug>,
}

impl reject::Reject for ApiResponseError {}

impl Serialize for ApiResponseError {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let mut state = serializer.serialize_struct("ApiResponseError", 1)?;
		state.serialize_field("error", &self.error.to_string())?;
		state.end()
	}
}

impl fmt::Display for ApiResponseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.error)
	}
}

impl ApiResponseError {
	pub fn new<T: DisplayDebug + 'static>(code: StatusCode, error: T) -> Self {
		Self {
			code,
			error: Box::new(error),
		}
	}
}

impl From<CheckEmailInputBuilderError> for ApiResponseError {
	fn from(e: CheckEmailInputBuilderError) -> Self {
		Self {
			code: StatusCode::BAD_REQUEST,
			error: Box::new(e),
		}
	}
}

impl From<serde_json::Error> for ApiResponseError {
	fn from(e: serde_json::Error) -> Self {
		ApiResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, e)
	}
}

impl From<lapin::Error> for ApiResponseError {
	fn from(e: lapin::Error) -> Self {
		ApiResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, e)
	}
}

impl From<sqlx::Error> for ApiResponseError {
	fn from(e: sqlx::Error) -> Self {
		ApiResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, e)
	}
}

impl From<csv::Error> for ApiResponseError {
	fn from(e: csv::Error) -> Self {
		ApiResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, e)
	}
}

impl From<csv::IntoInnerError<csv::Writer<Vec<u8>>>> for ApiResponseError {
	fn from(e: csv::IntoInnerError<csv::Writer<Vec<u8>>>) -> Self {
		ApiResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, e)
	}
}

impl From<warp::http::status::InvalidStatusCode> for ApiResponseError {
	fn from(e: warp::http::status::InvalidStatusCode) -> Self {
		ApiResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, e)
	}
}

impl From<anyhow::Error> for ApiResponseError {
	fn from(e: anyhow::Error) -> Self {
		ApiResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, e)
	}
}

impl From<StorageError> for ApiResponseError {
	fn from(e: StorageError) -> Self {
		ApiResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, e)
	}
}

impl From<reqwest::Error> for ApiResponseError {
	fn from(e: reqwest::Error) -> Self {
		ApiResponseError::new(
			e.status()
				.map(|s| s.as_u16())
				.map(StatusCode::from_u16)
				.and_then(Result::ok)
				.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
			e,
		)
	}
}

/// This function receives a `Rejection` and tries to return a custom value,
/// otherwise simply passes the rejection along.
pub async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
	if let Some(err) = err.find::<ApiResponseError>() {
		error!(target: LOG_TARGET, code=?err.code, message=?err.to_string(), "Request rejected");
		Ok((warp::reply::with_status(warp::reply::json(err), err.code),))
	} else {
		Err(err)
	}
}
