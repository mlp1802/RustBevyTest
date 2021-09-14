pub enum Ent {
    Player,
    Monster
}


#[derive(Debug, Copy, Clone)]
pub enum GameStatus {
    Running,
    Lost,
    Won,
}
pub struct Game {
    pub points: u32,
    pub dots: u32,
    pub status: GameStatus
}
pub struct Config {
    pub start_x: f32,
    pub start_y: f32,
    pub gab: f32,
    pub lab_size: u32,
}

pub struct Player {}
pub struct Monster {}
pub struct Wall {}
pub struct Plane {}

pub struct Dot {}
