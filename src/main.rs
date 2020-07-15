use actix::prelude::*;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use esc_server::ws_actor::WsActor;

use esc_server::api;
use esc_server::ws_actor;
use esc_server;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use actix_web::middleware::Logger;
use env_logger;
use env_logger::Env;

extern crate redis;
extern crate r2d2_redis;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

struct WsSession {
    id: u32,
    hb: Instant,
    addr: Addr<WsActor>,
}

impl WsSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting!");
                act.addr
                    .do_send(esc_server::ws_actor::Disconnect { id: act.id });
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.addr
            .send(esc_server::ws_actor::Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr
            .do_send(esc_server::ws_actor::Disconnect { id: self.id });
        Running::Stop
    }
}

impl Handler<esc_server::ws_actor::Message> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: esc_server::ws_actor::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let m = text.trim();
                println!("Get message: {:?}", m.to_string());
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(_) => {
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

pub async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<WsActor>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WsSession {
            id: 0,
            hb: Instant::now(),
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let db_url: String = esc_server::get_db_url();
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create postgres pool.");

    let redis_url = esc_server::get_redis_url();
    let redis_manager = r2d2_redis::RedisConnectionManager::new(redis_url).unwrap();
    let redis_pool = r2d2_redis::r2d2::Pool::builder()
        .build(redis_manager)
        .expect("Failed to create redis pool.");
    
    let ws_server = ws_actor::WsActor::new().start();

    let pools = esc_server::Pools {
        db: pool,
        redis: redis_pool,
    };
    
    // esc_server::db_setup(&pools).await;
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    println!("Hello, world!");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(web::scope("/api")
                .data(pools.clone())
                .data(ws_server.clone())

                .route("/", web::get().to(api::hello_world::hello_world))

                .route("/me", web::get().to(api::users::me))
                .route("/me/follows", web::get().to(api::follows::get_my_follow_request))

                .route("/users/{user_id}", web::get().to(api::users::get_user_detail))
                .route("/users", web::get().to(api::users::get_users))
                .route("/users", web::post().to(api::users::signup))
                .route("/users", web::put().to(api::users::edit_user))
                .route("/users/{user_id}/followers", web::get().to(api::follows::get_followers))
                .route("/users/{user_id}/followees", web::get().to(api::follows::get_followees))
                .route("/users/{follower_id}/follows", web::post().to(api::follows::post_follows))
                .route("/users/{user_id}/messages", web::post().to(api::messages::post_messages))
                .route("/users/{user_id}/lists", web::get().to(api::lists::get_lists_by_user_id))
                .route("/login", web::post().to(api::users::login))

                .route("/brands", web::get().to(api::brands::get_brands))
                .route("/brands/{brand_id}", web::get().to(api::brands::get_brand))
                // for test
                .route("/brands", web::patch().to(api::brands::update_all_brands))
                // for test
                .route("brands/{brand_id}", web::post().to(api::brands::add_id_brand))

                .route("/games", web::get().to(api::games::get_minimal_games))
                .route("/games", web::post().to(api::games::get_games))
                .route("/games", web::patch().to(api::games::update_all_games))
                .route("/games/{game_id}", web::get().to(api::games::get_game))
                // for test
                .route("games/{game_id}", web::post().to(api::games::add_id_game))
                
                .route("/timelines/{timeline_id}", web::get().to(api::timelines::get_timeline))
                .route("/timelines", web::get().to(api::timelines::get_timelines))

                .route("/play/{game_id}", web::post().to(api::play::post_play))

                .route("/lists", web::get().to(api::lists::get_lists))
                .route("/lists", web::post().to(api::lists::post_list))
                .route("/lists/{list_id}", web::get().to(api::lists::get_list))
                .route("/lists/{list_id}", web::post().to(api::listmaps::add_game_list))
                .route("/lists/{list_id}", web::put().to(api::lists::put_list))
                .route("/lists/{list_id}", web::patch().to(api::listmaps::delete_game_list))
                .route("/lists/{list_id}", web::delete().to(api::lists::delete_list))

                .route("/reviews", web::post().to(api::reviews::add_recent_reviews))
                .route("/reviews", web::get().to(api::reviews::get_reviews))

                .route("/messages", web::get().to(api::messages::get_messages))

                .route("/follows", web::get().to(api::follows::get_follow_request))
                .route("/follows/{follow_id}", web::post().to(api::follows::handle_follow_request))

                .route("/campaigns", web::get().to(api::campaigns::get_campaigns))
                .route("/campaigns", web::post().to(api::campaigns::set_campaigns))

                .route("/recentgames", web::get().to(api::games::get_recent_games))
                .route("/recentgames", web::post().to(api::games::add_game))

                .service(web::resource("/ws/").to(ws_route))
        )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
