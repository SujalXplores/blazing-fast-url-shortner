use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures::future::{ready, Ready, LocalBoxFuture};
use std::rc::Rc;
use url::Url;

pub struct UrlValidator;

impl UrlValidator {
    pub fn new() -> Self {
        UrlValidator
    }
}

impl<S, B> Transform<S, ServiceRequest> for UrlValidator
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = UrlValidatorMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(UrlValidatorMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct UrlValidatorMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for UrlValidatorMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            if req.path() == "/api/v1/shorten" && req.method() == actix_web::http::Method::POST {
                let mut payload = req.extract::<actix_web::web::Json<serde_json::Value>>().await;
                if let Ok(body) = &mut payload {
                    if let Some(url) = body.get("url").and_then(|v| v.as_str()) {
                        if Url::parse(url).is_err() {
                            let response = HttpResponse::BadRequest()
                                .json("Invalid URL format");
                            return Ok(ServiceResponse::new(
                                req.into_parts().0,
                                response,
                            ));
                        }
                    }
                }
            }
            
            svc.call(req).await
        })
    }
} 