use actix_web::middleware::{ErrorHandlers, ErrorHandlerResponse};
use actix_web::{web, http, dev, App, HttpRequest, HttpResponse, Result};

pub fn render_500<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut()
        .headers_mut()
        .insert(http::header::CONTENT_TYPE, http::HeaderValue::from_static("Error"));
    Ok(ErrorHandlerResponse::Response(res))
}
