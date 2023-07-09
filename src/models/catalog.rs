use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Response {
    pub status: String,
    pub data: Vec<Product>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub product_id: u32,
    pub games: Vec<Game>,
    pub purchasable: bool,
    pub catalog_visibility: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Game {
    pub game_id: String,
    pub game_name: String,
    pub game_description: String,
    pub game_coverart: String,
    pub game_installed_size: String,
    pub installer_uuid: String,
}
