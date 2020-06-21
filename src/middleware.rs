use actix_web::{web, Error, HttpResponse, HttpRequest, FromRequest};
use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use futures::future::{ok, err, Ready};
use reqwest::header;
use anyhow::{Context, Result};

pub struct Authorized {
    pub session_id: Option<String>
}

impl FromRequest for Authorized {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth = Authorized {
            session_id: is_authorized(req)
        };
        ok(auth)
    }
}

fn is_authorized(req: &HttpRequest) -> Option<String> {
    if let Some(value) = req.headers().get("cookie") {
        // actual implementation that checks header here
        dbg!(value);
        let c: &str = value.to_str().unwrap();
        if let Some(v) = c.split("=").nth(1) {
            Some(v.to_string())
        } else {
            None
        }
    } else {
        None
    }
}
