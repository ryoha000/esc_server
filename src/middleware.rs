use actix_web::{Error, HttpRequest, FromRequest};
use actix_web::dev::Payload;
use futures::future::{ok, Ready};
use anyhow::{Result};
use std::ops::DerefMut;

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

pub struct Me {
    pub user_id: String,
    pub header: String,
    pub session_id: String,
}

pub fn check_user(auth: Authorized, redis_conn: &mut diesel::r2d2::PooledConnection<r2d2_redis::RedisConnectionManager>) -> Option<Me> {
    let mut user_id: Option<String> = None;
    let mut header: Option<String> = None;
    if let Some(session_id) = &auth.session_id {
        println!("{}", session_id);
        match r2d2_redis::redis::cmd("GET").arg(&format!("session_header:{}", session_id)).query(redis_conn.deref_mut()) {
            Ok(res) => header = Some(res),
            _ => {}
        }
        match r2d2_redis::redis::cmd("GET").arg(&format!("session_user:{}", session_id)).query(redis_conn.deref_mut()) {
            Ok(res) => user_id = Some(res),
            _ => {}
        }
    }
    if user_id != None && header != None {
        let me = Me {
            user_id: user_id.unwrap(),
            header: header.unwrap(),
            session_id: auth.session_id.unwrap(),
        };
        return Some(me)
    }
    None
}
