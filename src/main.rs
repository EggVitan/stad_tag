use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, web, HttpRequest, http::header::{self, ExtendedValue}, dev::ServiceRequest, cookie::Cookie};
use actix_files::Files;
use actix_multipart;
#[macro_use]
extern crate serde_derive;
extern crate uuid;
mod templating;
use templating::{templating, template_single_val};
mod game_helpers;
mod security;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body(templating("root.html.tera", templating::get_json_by_file("Static/Json_data/root.json")))
}
#[get("/game/create_game")]
async fn create_game() -> impl Responder {
    HttpResponse::Ok().body(templating("create_game.html.tera", templating::get_json_by_file("Static/Json_data/test_create_game.json")))
}
#[post("/game/create_game")]
async fn create_game_post(form_data: web::Form<game_helpers::create_game_struct>, req: HttpRequest) -> impl Responder {
    let create_game_struct: game_helpers::create_game_struct = form_data.into_inner();
    let uuid = game_helpers::create_game(&create_game_struct);
    HttpResponse::Found()
        .header(header::LOCATION, format!("/game/create_game/share/?game_uuid={}", uuid))
        .cookie(
            Cookie::new("login", security::create_login_jwt(create_game_struct.username))
        )
        .finish()
}
#[derive(Deserialize)]
struct share_game_qp {
    game_uuid: String,
}
#[get("/game/create_game/share")]
async fn share_game(qp: web::Query<share_game_qp>) -> impl Responder {
    HttpResponse::Ok().body(template_single_val("share_game.html.tera", "url", format!("/game/join_game/?game_uuid={}", qp.game_uuid)))
}
#[derive(Deserialize)]
struct join_game_qp {
    game_uuid: String,
}
#[get("/game/join_game")]
async fn join_game(qp: web::Query<join_game_qp>) -> impl Responder {
    HttpResponse::Ok().body(templating("join_game.html.tera", templating::get_json_by_file(format!("Data/games/{}", qp.game_uuid).as_str())))
}
#[post("/game/join_game")]
async fn join_game_post(
    form_data: web::Form<game_helpers::create_user_struct>, 
    req: HttpRequest, 
    qp: web::Query<join_game_qp>,
    mut payload: actix_multipart::Multipart,
) -> impl Responder {
    let create_user: game_helpers::create_user_struct = form_data.into_inner();
    game_helpers::add_player(&create_user, &qp.game_uuid);
    HttpResponse::Found()
        .header(header::LOCATION, format!("/game/?game_uuid={}", qp.game_uuid))
        .cookie(
            Cookie::build("login", security::create_login_jwt(create_user.username)).path("/game").finish()
        )
        .finish()
}
#[derive(Deserialize)]
struct game_qp {
    game_uuid: String,
}
#[get("/game")]
async fn game(req: HttpRequest, qp: web::Query<game_qp>) -> impl Responder {
    let cookie_raw = req.cookie("login");
    let cookie: String; 
    match cookie_raw {
        Some(e) => {
            cookie = e.value().to_owned();
        }
        None => {
            return HttpResponse::Ok().body("You are not loged in");
        }
    }
    match security::verify_jwt(cookie.as_str()) {
        Ok(j) => {
            return HttpResponse::Ok().body(templating("game.html.tera", serde_json::json!{game_helpers::read_game(&qp.game_uuid)}));
        }
        Err(e) => {
            return HttpResponse::Ok().body("Something whent wrong, try login in again");
        }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::NormalizePath::default())
            .service(root)
            .service(create_game)
            .service(create_game_post)
            .service(join_game_post)
            .service(join_game)
            .service(share_game)
            .service(game)
            .service(Files::new("/static", "./Static/Public").prefer_utf8(true))
        
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
