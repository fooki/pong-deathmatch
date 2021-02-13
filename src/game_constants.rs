pub const GAME_WIDTH: i32 = 640;
pub const GAME_HEIGHT: i32 = 480;

pub const PADDLE_WIDTH: i32 = 25;
pub const PADDLE_HEIGHT: i32 = 100;

pub const HALF_PADDLE_WIDTH: i32 = PADDLE_WIDTH/2;

pub const BALL_WIDTH: i32 = 25;
pub const BALL_HEIGHT: i32 = 25;

pub const PLAYER_MOVE_UNIT: i32 = 5;

// Horizontal positions
pub const P1_X_POS: i32 = 0;
pub const P2_X_POS: i32 = GAME_WIDTH - PADDLE_WIDTH;

// Stolen from ggez examples.
pub const CLIENT_UPDATES_PER_SECONDS: f64 = 60.0;
pub const MS_PER_UPDATE: u64 = (1000.0 / CLIENT_UPDATES_PER_SECONDS) as u64;

pub const MS_PER_PING: u64 = 500;
pub const MS_PER_STATE_BROADCAST: u64 = 50;
