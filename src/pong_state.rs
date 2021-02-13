use crate::game_constants::{
    PADDLE_WIDTH,
    PADDLE_HEIGHT,
    HALF_PADDLE_WIDTH,
    BALL_WIDTH,
    BALL_HEIGHT,
    GAME_WIDTH,
    GAME_HEIGHT,
    PLAYER_MOVE_UNIT,
    P1_X_POS,
    P2_X_POS,
};
use ggez::graphics::Rect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlayerMovement {
    Up,
    Down,
    Still,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PongState {
    // The horizontal position of player 1.
    pub p1: i32,

    // The latest known move by Player 1
    pub p1_move: Option<PlayerMovement>,

    // The horizontal position of player 2.
    pub p2: i32,

    // The latest known move by Player 2
    pub p2_move: Option<PlayerMovement>,

    // (x, y) coordinates
    pub ball: (i32, i32),

    // velocity vector
    pub ball_vel: (i32, i32),
}

impl PongState {
    pub fn new() -> Self {
        Self {
            p1: 0,
            p1_move: None,

            p2: 0,
            p2_move: None,

            ball: (GAME_WIDTH/2, GAME_HEIGHT/2),
            ball_vel: (5, 7),
        }
    }

    // We override the saved movement state for player 1 based provided (local)
    // information. This will make the clients own paddle be more up to date.
    pub fn extrapolate_p1(&mut self, p1_move: PlayerMovement) {

        // p1_movement will stay the same. So if it moves up/down, it will
        // continue to do so.
        self.tick(Some(p1_move), self.p2_move);
    }

    // We override the saved movement state for player 2 based provided (local)
    // information. This will make the clients own paddle be more up to date.
    pub fn extrapolate_p2(&mut self, p2_move: PlayerMovement) {
        self.tick(self.p1_move, Some(p2_move));
    }

    pub fn tick(
        &mut self,
        p1_move: Option<PlayerMovement>,
        p2_move: Option<PlayerMovement>
    ) {
        self.update_player_movements(p1_move, p2_move);
        self.update_ball_movement();

        if self.ball_passed_a_paddle() {
            self.reset();
        }

        if self.ball_touching_top_or_bottom() {
            self.vertically_bounce_ball();
        }

        if self.ball_touching_any_paddle() {
            self.horizontally_bounce_ball();
        }
    }

    fn update_player_movements(
        &mut self,
        p1_move: Option<PlayerMovement>,
        p2_move: Option<PlayerMovement>
    ) {
        // Override info about whether players are moving up/down and then
        // update their positions.

        self.p1_move = p1_move;
        match self.p1_move {
            Some(PlayerMovement::Up) => {
                self.p1 = std::cmp::max(0, self.p1 - PLAYER_MOVE_UNIT)
            }
            Some(PlayerMovement::Down) => {
                self.p1 = std::cmp::min(GAME_HEIGHT - PADDLE_HEIGHT, self.p1 + PLAYER_MOVE_UNIT)
            }
            _ => {}
        }

        self.p2_move = p2_move;
        match self.p2_move {
            Some(PlayerMovement::Up) => {
                self.p2 = std::cmp::max(0, self.p2 - PLAYER_MOVE_UNIT)
            }
            Some(PlayerMovement::Down) => {
                self.p2 = std::cmp::min(GAME_HEIGHT - PADDLE_HEIGHT, self.p2 + PLAYER_MOVE_UNIT)
            }
            _ => {}
        }
    }

    fn reset(&mut self) {
        self.ball = (GAME_WIDTH/2, GAME_HEIGHT/2);
    }

    fn ball_passed_a_paddle(&self) -> bool {
        self.ball.0 > GAME_WIDTH || (self.ball.0 + BALL_WIDTH) < 0
    }

    fn ball_touching_top_or_bottom(&self) -> bool {
        (self.ball.1 + BALL_HEIGHT) > GAME_HEIGHT || self.ball.1 < 0
    }

    fn ball_touching_any_paddle(&mut self) -> bool {
        // Cheats alert:
        // Use ggez Rect in order to make use of their collision detection
        let ball = Rect::new(
            self.ball.0 as f32,
            self.ball.1 as f32,
            BALL_WIDTH as f32,
            BALL_HEIGHT as f32,
        );

        let left_paddle = Rect::new(
            P1_X_POS as f32,
            self.p1 as f32,
            PADDLE_WIDTH as f32,
            PADDLE_HEIGHT as f32,
        );

        let right_paddle = Rect::new(
            P2_X_POS as f32,
            self.p2 as f32,
            PADDLE_WIDTH as f32,
            PADDLE_HEIGHT as f32,
        );

        // Is the ball is moving towards a paddle and but hasn't yet passed it?
        let towards_left_paddle =
            self.ball_vel.0 < 0 && self.ball.0 > HALF_PADDLE_WIDTH;

        // Is the ball is moving towards a paddle and but hasn't yet passed it?
        let towards_right_paddle =
            self.ball_vel.0 > 0 && self.ball.0 < (GAME_WIDTH-HALF_PADDLE_WIDTH);

        // Need to make sure that we aren't inside/passed the paddle, because
        // then we don't want collisions.
        (towards_left_paddle && ball.overlaps(&left_paddle)) ||
            (towards_right_paddle && ball.overlaps(&right_paddle))
    }

    fn vertically_bounce_ball(&mut self) {
        self.ball_vel.1 *= -1;
    }

    fn horizontally_bounce_ball(&mut self) {
        self.ball_vel.0 *= -1;
    }

    fn update_ball_movement(&mut self) {
        self.ball.0 += self.ball_vel.0;
        self.ball.1 += self.ball_vel.1;
    }
}

#[cfg(test)]
mod player_tests {
    use super::*;

    #[test]
    fn test_new_returns_a_state_with_players_on_top() {
        let state = PongState::new();
        assert_eq!(0, state.p1);
        assert_eq!(0, state.p2);
    }

    #[test]
    fn test_tick_resets_state_when_ball_reached_offscreen() {
        let mut state = PongState::new();

        state.ball.0 = -100;
        assert_ne!(state, PongState::new());


        state.tick(None, None);
        assert_eq!(state, PongState::new());
    }

    #[test]
    fn test_tick_bounces_ball_vertically() {
        let mut state = PongState::new();

        assert!(state.ball_vel.1 > 0);

        state.ball.1 = -10;
        state.tick(None, None);

        assert!(state.ball_vel.1 < 0);

        state.ball.1 = 10000;
        state.tick(None, None);

        assert!(state.ball_vel.1 > 0);
    }

    #[test]
    fn test_tick_bounces_horizontal_bounce() {
        let mut state = PongState::new();

        assert!(state.ball_vel.1 > 0);

        // Moving towars from p1 and colliding with p1
        state.ball_vel.0 = -1;
        state.ball = (PADDLE_WIDTH - 1, state.p1);
        state.tick(None, None);

        assert_eq!(state.ball_vel.0, 1);
    }

    #[test]
    fn test_tick_ignores_horizontal_bounce_if_moving_away() {
        let mut state = PongState::new();

        assert!(state.ball_vel.1 > 0);

        // Moving away from p1, but colliding with p1
        state.ball_vel.0 = 1;
        state.ball = (PADDLE_WIDTH - 1, state.p1);
        state.tick(None, None);

        assert_eq!(state.ball_vel.0, 1);

        // Moving away from p2, but colliding with p2
        state.ball_vel.0 = -1;
        state.ball = (GAME_WIDTH - PADDLE_WIDTH + 1, state.p2);
        state.tick(None, None);

        assert_eq!(state.ball_vel.0, -1);
    }

    #[test]
    fn test_updates_player_movement() {
        let mut state = PongState::new();

        // Player 1
        let mut before = state.p1;
        state.tick(Some(PlayerMovement::Down), None);
        assert!(before < state.p1);

        before = state.p1;
        state.tick(None, None);
        assert_eq!(before, state.p1);

        before = state.p1;
        state.tick(Some(PlayerMovement::Up), None);
        assert!(before > state.p1);

        // Player 2
        before = state.p2;
        state.tick(None, Some(PlayerMovement::Down));
        assert!(before < state.p2);

        before = state.p2;
        state.tick(None, None);
        assert_eq!(before, state.p2);

        before = state.p2;
        state.tick(None, Some(PlayerMovement::Up));
        assert!(before > state.p2);
    }
}
