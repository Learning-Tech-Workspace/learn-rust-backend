use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct LoginResponse {
	pub msg: String,
	pub token: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
	pub email: String,
	pub password: String,
}