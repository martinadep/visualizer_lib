use robotics_lib::world::environmental_conditions::WeatherType;
use tetra::{Context, TetraError};
use tetra::graphics::Texture;
use robotics_lib::world::tile::{Content, TileType};
use robotics_lib::world::tile::TileType::*;

pub(crate) trait Texturizable {
    ///matches an object with its texture
    fn get_texture(&self, ctx: &mut Context) -> Texture;
}
impl Texturizable for TileType{
    fn get_texture(&self, ctx: &mut Context) -> Texture {
        let tmp = match self {
            DeepWater => {Texture::new(ctx, "./utils/tiletype/DeepWater.png")}
            ShallowWater => {Texture::new(ctx, "./utils/tiletype/ShallowWater.png")}
            Sand => {Texture::new(ctx, "./utils/tiletype/Sand.png")}
            Grass => {Texture::new(ctx, "./utils/tiletype/Grass.png")}
            Street => {Texture::new(ctx, "./utils/tiletype/Street.png")}
            Hill => {Texture::new(ctx, "./utils/tiletype/Hill.png")}
            Mountain => {Texture::new(ctx, "./utils/tiletype/Mountain.png")}
            Snow => {Texture::new(ctx, "./utils/tiletype/Snow.png")}
            Lava => {Texture::new(ctx, "./utils/tiletype/Lava.png")}
            Teleport(_) => {Texture::new(ctx, "./utils/tiletype/Teleport.png")}
            Wall => {Texture::new(ctx, "./utils/tiletype/Wall.png")}
        };
        tmp.expect("failed to upload tile type texture")
    }
}

impl Texturizable for Content{
    fn get_texture(&self, ctx: &mut Context) -> Texture {
        let tmp = match self {
            Content::Rock(_) => {Texture::new(ctx, "./utils/content/Rock.png")}
            Content::Tree(_) => {Texture::new(ctx, "./utils/content/Tree.png")}
            Content::Garbage(_) => {Texture::new(ctx, "./utils/content/Garbage.png")}
            Content::Fire => {Texture::new(ctx, "./utils/content/Fire.png")}
            Content::Coin(_) => {Texture::new(ctx, "./utils/content/Coin.png")}
            Content::Bin(_) => {Texture::new(ctx, "./utils/content/Bin.png")}
            Content::Crate(_) => {Texture::new(ctx, "./utils/content/Crate.png")}
            Content::Bank(_) => {Texture::new(ctx, "./utils/content/Bank.png")}
            Content::Market(_) => {Texture::new(ctx, "./utils/content/Market.png")}
            Content::Fish(_) => {Texture::new(ctx, "./utils/content/Fish.png")}
            Content::Building => {Texture::new(ctx, "./utils/content/Building.png")}
            Content::Bush(_) => {Texture::new(ctx, "./utils/content/Bush.png")}
            Content::Scarecrow => {Texture::new(ctx, "./utils/content/Scarecrow.png")}
            _ => {Texture::new(ctx, "./utils/content/None.png")}
        };
        tmp.expect("failed to upload tile type texture")
    }
}

impl Texturizable for WeatherType {
    fn get_texture(&self, ctx: &mut Context) -> Texture {
        let tmp = match self {
            WeatherType::Sunny => {Texture::new(ctx, "./utils/sun.png")}
            WeatherType::Foggy => {Texture::new(ctx, "./utils/fog.png")}
            WeatherType::TrentinoSnow => {Texture::new(ctx, "./utils/snow.png")}
            _ => {Texture::new(ctx, "./utils/drop.png")}
        };
        tmp.expect("failed to upload weather texture")
    }
}

