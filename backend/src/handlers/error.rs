use actix_web::middleware::{ErrorHandlerResponse};
use actix_web::{ http, dev, Result};

pub fn render_500<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut()
        .headers_mut()
        .insert(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("error"));
    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}
