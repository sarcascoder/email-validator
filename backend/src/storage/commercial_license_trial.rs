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

use crate::config::{BackendConfig, CommercialLicenseTrialConfig};
use crate::http::ApiResponseError;
use crate::worker::do_work::TaskError;
use email_validator_core::{CheckEmailOutput, LOG_TARGET};
use std::sync::Arc;
use tracing::debug;
use warp::http::StatusCode;

/// Optionally, send the result to an external webhook if configured.
pub async fn send_to_webhook(
	config: Arc<BackendConfig>,
	email: &str,
	worker_output: &Result<CheckEmailOutput, TaskError>,
) -> Result<(), ApiResponseError> {
	if let Some(CommercialLicenseTrialConfig { api_token, url }) = &config.commercial_license_trial
	{
		let res = reqwest::Client::new()
			.post(url)
			.header("Authorization", api_token)
			.json(worker_output)
			.send()
			.await?;

		// Error if not 2xx status code
		if !res.status().is_success() {
			let status = StatusCode::from_u16(res.status().as_u16())?;
			let body: serde_json::Value = res.json().await?;

			// Extract error message from the "error" field, if it exists, or
			// else just return the whole body.
			let error_body = body.get("error").unwrap_or(&body).to_owned();

			return Err(ApiResponseError::new(status, error_body));
		}

		let res = res.text().await?;
		debug!(target: LOG_TARGET, email=email, res=res, "Sent result to configured webhook");
	}

	Ok(())
}
