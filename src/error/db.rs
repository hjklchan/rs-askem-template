use axum::response::IntoResponse;
use thiserror::Error;

/// 数据库错误
#[derive(Error, Debug)]
#[error(transparent)]
pub struct DatabaseError(#[from] sqlx::Error);

/// 为数据库错误实现 IntoResponse 特征
impl IntoResponse for DatabaseError {
    fn into_response(self) -> axum::response::Response {
        self.0.to_string().into_response()
    }
}

type new_error = sqlx::Error;
