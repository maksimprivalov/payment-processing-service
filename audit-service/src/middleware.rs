// use axum::{
//     extract::{State, Request},
//     middleware::Next,
//     response::Response,
// };
// use crate::{auth::validate_token, error::AppError};
// use uuid::Uuid;
// pub async fn auth_middleware(
//     State((_, secret)): State<(sqlx::PgPool, String)>,
//     mut req: Request,
//     next: Next,
// ) -> Result<Response, AppError> {
//
//     let auth_header = req
//         .headers()
//         .get("Authorization")
//         .and_then(|h| h.to_str().ok())
//         .ok_or(AppError::Unauthorized)?;
//
//     let token = auth_header
//         .strip_prefix("Bearer ")
//         .ok_or(AppError::Unauthorized)?;
//
//     let claims = validate_token(token, &secret)
//         .map_err(|_| AppError::Unauthorized)?;
//
//     let user_id = Uuid::parse_str(&claims.sub)
//         .map_err(|_| AppError::Unauthorized)?;
//
//     req.extensions_mut().insert(user_id);
//
//     Ok(next.run(req).await)
// }
