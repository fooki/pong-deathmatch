use crate::client_connection::ClientConnection;
use crate::net_messages::{ServerMsg, PlayerOrder};
use crate::pong_state::{PongState, PlayerMovement};
use crate::game_constants::{
    PADDLE_WIDTH,
    PADDLE_HEIGHT,
    BALL_WIDTH,
    BALL_HEIGHT,
    GAME_WIDTH,
    GAME_HEIGHT,
    P1_X_POS,
    P2_X_POS,
};
use crate::game_constants::{
    MS_PER_UPDATE,
};

use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::{graphics, Context, ContextBuilder, GameResult};

use std::time::{Duration, Instant};

pub fn run(addr: &str, cpu: bool) {
    let (mut ctx, mut event_loop) = ContextBuilder::new("PONG", "Karl Johansson")
        .window_mode(ggez::conf::WindowMode::default().dimensions(
            GAME_WIDTH as f32, GAME_HEIGHT as f32)
        )
        .build()
        .expect("Could not create ggez context!");

    let connection = ClientConnection::connect(&addr).expect("Can't send any packets");
    let mut client_game = ClientGame::new(&mut ctx, connection, cpu);
    event::run(&mut ctx, &mut event_loop, &mut client_game).expect("Game crashed");
}

struct ClientGame {
    // How I talk to the server.
    connection: ClientConnection,

    // My currenct view of the game, which might be nonexistant or extarpolated
    // locally (i e not what the server sees).
    pong_state: Option<PongState>,

    // Am I player 1 or 2, could be fixed with some nice polymorophism instead.
    player: Option<PlayerOrder>,

    // Keeps track of key presses
    up: bool,
    down: bool,

    // Dictates whether this client is human or machine.
    cpu: bool,

    // Helps keeping track of a good update rate.
    last_update: Instant
}


impl ClientGame {
    pub fn new(_ctx: &mut Context, connection: ClientConnection, cpu: bool) -> Self {
        Self {
            connection,
            cpu,
            player: None,
            pong_state: None,
            up: false,
            down: false,
            last_update: Instant::now(),
        }
    }
}

impl ClientGame {
    fn game_loop(&mut self) -> GameResult<()> {
        if Instant::now() - self.last_update < Duration::from_millis(MS_PER_UPDATE) {
            return Ok(());
        }

        self.poll_server_events();
        self.extrapolate();

        if self.game_has_started() {
            self.update_cpu_movement();

            if self.moving_up() {
                self.connection.send_move_up().expect("Failed to send movement");
            } else if self.moving_down() {
                self.connection.send_move_down().expect("Failed to send movement");
            }
        }

        self.last_update = Instant::now();
        Ok(())
    }


    fn moving_up(&self) -> bool {
        self.up && !self.down
    }

    fn moving_down(&self) -> bool {
        self.down && !self.up
    }

    fn movement(&self) -> PlayerMovement {
        if self.moving_up() {
            PlayerMovement::Up
        } else if self.moving_down() {
            PlayerMovement::Down
        } else {
            PlayerMovement::Still
        }
    }

    fn game_has_started(&self) -> bool {
        self.pong_state.is_some()
    }

    fn abort_game(&mut self) {
        self.pong_state = None;
        self.player = None;
    }

    fn update_cpu_movement(&mut self) {
        if !self.cpu {
            return;
        }

        if !self.is_ball_moving_away() {
            self.up = self.is_ball_above_me();
            self.down = !self.is_ball_above_me();
        }
    }

    fn is_ball_moving_away(&self) -> bool {
        if let Some(pong_state) = self.pong_state {
            match self.player {
                Some(PlayerOrder::P1) => pong_state.ball_vel.0 > 0,
                Some(PlayerOrder::P2) => pong_state.ball_vel.0 < 0,
                None => false
            }
        } else {
            // Technically correct..
            false
        }
    }

    fn is_ball_above_me(&self) -> bool {
        if let Some(pong_state) = self.pong_state {
            match self.player {
                Some(PlayerOrder::P1) => pong_state.ball.1 < pong_state.p1,
                Some(PlayerOrder::P2) => pong_state.ball.1 < pong_state.p2,
                None => false
            }
        } else {
            // Technically correct..
            false
        }
    }

    fn poll_server_events(&mut self) {
        while let Some(event) = self.connection.receive() {
            match event {
                ServerMsg::Start(order) => {
                    // Am I Player 1 or Player 2?
                    self.player = Some(order);
                }

                ServerMsg::State(state) => {
                    self.pong_state = Some(state);
                }

                ServerMsg::Abort => {
                    self.abort_game();
                    self.connection.greet_server().expect("Failed to greet server");
                }

                ServerMsg::Ping => {
                    self.connection.pong().expect("Failed to ping server");
                }

                _ => {}
            }
        }
    }

    fn extrapolate(&mut self) {
        // Fetch our own movement so we can use it when extrapolating. It will
        // make our own movement smoother.
        let movement = self.movement();

        if let Some(state) = &mut self.pong_state {
            match self.player {
                Some(PlayerOrder::P1) => {
                    state.extrapolate_p1(movement);
                }

                Some(PlayerOrder::P2) => {
                    state.extrapolate_p2(movement);
                }

                _ => {}
            }
        }
    }
}

// Below is mostly ggez stuffs

impl EventHandler for ClientGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.game_loop()
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        if let Some(state) = self.pong_state {
            let color = [1.0, 1.0, 1.0, 1.0].into();
            let player_rect =
                graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(0.0, 0.0, PADDLE_WIDTH as f32, PADDLE_HEIGHT as f32),
                    color
                )?;

            let ball_rect =
                graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(0.0, 0.0, BALL_WIDTH as f32, BALL_HEIGHT as f32),
                    color
                )?;

            let p1_y = state.p1;
            let p2_y = state.p2;
            let (ball_x, ball_y) = state.ball;

            graphics::draw(
                ctx,
                &player_rect,
                (ggez::mint::Point2 { x: P1_X_POS as f32, y: p1_y as f32 },)
            )?;

            graphics::draw(
                ctx,
                &player_rect,
                (ggez::mint::Point2 { x: P2_X_POS as f32, y: p2_y as f32 },)
            )?;

            graphics::draw(
                ctx,
                &ball_rect,
                (ggez::mint::Point2 { x: ball_x as f32, y: ball_y as f32 },)
            )?;
        }

        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        key: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        if self.cpu {
            return
        }

        match key {
            KeyCode::Up => { self.up = true }
            KeyCode::Down => { self.down = true }
            _ => {}
        };
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        key: KeyCode,
        _keymod: KeyMods
    ) {
        if self.cpu {
            return
        }

        match key {
            KeyCode::Up => { self.up = false }
            KeyCode::Down => { self.down = false }
            _ => {}
        };
    }
}
