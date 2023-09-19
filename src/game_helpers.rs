
use uuid::Uuid;
use serde_json::{to_string, Result};
use std::fs::File;
use std::io::{Write, Read};
use std::fmt;

use crate::security;

#[derive(Deserialize)]
pub struct create_game_struct {
    game_name: String,
    map_select: String,
    pub username: String,
    password: String,
}
#[derive(Deserialize)]
pub struct create_user_struct {
    pub username: String,
    password: String,
}
#[derive(Serialize, Deserialize)]
pub struct player_game {
    Name: String,
    Players: Vec<player>,
    Map: String,
    Current_runner: i32 // player array index
}
#[derive(Serialize, Deserialize)]
pub struct player {
    Username: String,
    Password_hash: String,
    Password_salt: String,
    Icon: String,
    Coins: i32,
    is_runner: bool,
    is_admin: bool,
}

pub fn create_game(user_data: &create_game_struct) ->  String{
    let uuid = Uuid::new_v4().to_string();
    let mut game = player_game {
        Name: user_data.game_name.clone(),
        Map: user_data.map_select.clone(),
        Players: vec![ create_player1(&user_data, true)],
        Current_runner: 0,
    };
    write_game(game, &uuid);
    return uuid
}
fn create_player1(user_data: &create_game_struct, is_admin: bool) -> player{
    return create_player(
        &create_user_struct {
            username: user_data.username.clone(),
            password: user_data.password.clone(),
        }, is_admin,
    )
}
pub fn add_player(play: &create_user_struct, game: &String) {
    let mut game = read_game(game);
    let player = create_player(&create_user_struct{
        username: play.username.clone(),
        password: play.password.clone(),
    }, false);
    game.Players.insert(0, player);
} 
pub fn write_game(game: player_game, uuid: &String ){
    let game_serial = to_string(&game).unwrap();
    let mut file = File::create(format!("./Data/games/{}", &uuid)).expect("Unable to create file");
    file.write_all(game_serial.as_bytes()).expect("Unable to write to file");
    
}
pub fn read_game(uuid: &String) -> player_game {
    let mut file = File::open(format!("./Data/games/{}", uuid)).unwrap();
    let mut file_str: String = String::new();
    file.read_to_string(&mut file_str);
    let mut parsed: player_game = serde_json::from_str(&file_str).unwrap();
    return parsed;
}
fn create_player(user_data: &create_user_struct, is_admin: bool) -> player {
    let salt = security::create_salt();
    let hash = security::hash_pwd(&user_data.password, salt.clone());
    return player {
        Username: user_data.username.clone(),
        Password_hash: hash,
        Password_salt: salt.to_string(),
        Icon: "".to_string(),
        is_admin: is_admin,
        Coins: 0,
        is_runner: false
    }
}
fn upload_img(file: actix_multipart::Multipart) {
    let name_uuid = Uuid::new_v4();
    let path = format!("./Static/Public/User/Icon/{}", name_uuid);
    while let Some(mut field) = file.try_next().await? {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap_or_default();
        let filepath = format!("{}/{}", upload_dir, filename);
        let mut file = File::create(filepath).await?;

        while let Some(chunk) = field.next().await {
            let data = chunk?;
            file.write_all(&data).await?;
        }
    }
}