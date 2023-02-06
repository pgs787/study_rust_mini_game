use std::{
    cmp::min,
    time::{Duration, Instant},
};

use rand::Rng;
use bracket_terminal::prelude::*;

#[derive(Debug)]
struct UserState {
    position: Position,
    direction: Option<VirtualKeyCode>,
}

#[derive(Debug)]
pub(crate) struct Control {
    isGameOver: bool,
    recordTime: Instant,
    userPosition: Vec<UserState>,
    targetPosition: Position
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Position {
    x: u8,
    y: u8,
}

impl Position {
    fn random() -> Self {
        let mut range = rand::thread_rng();
        Self {
            x: range.gen_range(0..GAME_WIDTH),
            y: range.gen_range(0..GAME_HEIGHT),
        }
    }
}

const GAME_WIDTH: u8 = 80;
const GAME_HEIGHT: u8 = 50;

impl GameState for Control {
    fn tick(&mut self, context: &mut bracket_terminal::prelude::BTerm) {
        if self.isGameOver {
            context.print_centered(GAME_HEIGHT / 2 - 1, "GAME OVER");
            context.print_centered(GAME_HEIGHT / 2 + 1, "QUIT: Q, START: SPACE");
            match context.key {
                Some(VirtualKeyCode::Space) => {
                    self.reset();
                    context.cls();
                }
                Some(VirtualKeyCode::Q) => context.quit(),
                _ => {}
            }
            return;
        }
        self.handleControlKey(context);

        let now = Instant::now();

        // 속도 조절
        if now.duration_since(self.recordTime) < Duration::from_millis(50) {
            return;
        }
        self.recordTime = now;

        context.print_centered(0, format!("SCORE: {}", self.userPosition.len()));

        context.print(self.targetPosition.x, self.targetPosition.y, "*");

        self.handleDrawTarget(context);

        if self.targetPosition == self.userPosition.first().unwrap().position {
            self.targetPosition = Position::random();
            let last = self.userPosition.last().unwrap();
            let mut position = self.userPosition.last().unwrap().position;

            match last.direction {
                Some(VirtualKeyCode::Up) => position.y = min(position.y + 1, GAME_HEIGHT - 1),
                Some(VirtualKeyCode::Down) => position.y = position.y.saturating_sub(1),
                Some(VirtualKeyCode::Left) => position.x = min(position.x + 1, GAME_WIDTH - 1),
                Some(VirtualKeyCode::Right) => position.x = position.x.saturating_sub(1),
                _ => {}
            }
            let userState = UserState {
                position,
                direction: last.direction,
            };
            self.userPosition.push(userState);
        }
    }
}

impl Control {
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn new() -> Self {
        Self {
            isGameOver: false,
            recordTime: Instant::now(),
            userPosition: vec![UserState {
                position: Position { x: 30, y: 25 },
                direction: None,
            }],
            targetPosition: Position::random(),
        }
    }

    fn handleDrawTarget(&mut self, context: &mut BTerm) {
        for UserState { position, direction } in &mut self.userPosition {
            context.print(position.x, position.y, format!(" "));
            match direction {
                Some(VirtualKeyCode::Up) => position.y = position.y.saturating_sub(1),
                Some(VirtualKeyCode::Down) => position.y = min(position.y + 1, GAME_HEIGHT - 1),
                Some(VirtualKeyCode::Left) => position.x = position.x.saturating_sub(1),
                Some(VirtualKeyCode::Right) => position.x = min(position.x + 1, GAME_WIDTH - 1),
                _ => {}
            }
            context.print(position.x, position.y, "#");
        }

        let startPosition = self.userPosition.first().unwrap().position;

        // 1개 일때는 죽지 않음
        if self.userPosition.iter().skip(1).any(|userState| startPosition == userState.position) {
            self.isGameOver = true;
            return;
        }

        let mut i = self.userPosition.len() - 1;
        while i > 0 {
            self.userPosition[i].direction = self.userPosition[i - 1].direction;
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }

    fn handleControlKey(&mut self, context: &mut BTerm) {
        let userPositionCount = self.userPosition.len();
        let direction = &mut self.userPosition.first_mut().unwrap().direction;
        match context.key {
            d @ Some(
                VirtualKeyCode::Up
                | VirtualKeyCode::Down
                | VirtualKeyCode::Left
                | VirtualKeyCode::Right,
            ) if userPositionCount == 1 => *direction = d,
            d @ Some(VirtualKeyCode::Up) => {
                if !matches!(direction, Some(VirtualKeyCode::Down)) {
                    *direction = d;
                }
            }
            d @ Some(VirtualKeyCode::Down) => {
                if !matches!(direction, Some(VirtualKeyCode::Up)) {
                    *direction = d;
                }
            }
            d @ Some(VirtualKeyCode::Left) => {
                if !matches!(direction, Some(VirtualKeyCode::Right)) {
                    *direction = d;
                }
            }
            d @ Some(VirtualKeyCode::Right) => {
                if !matches!(direction, Some(VirtualKeyCode::Left)) {
                    *direction = d;
                }
            }
            Some(VirtualKeyCode::Q) => context.quit(),
            _ => {}
        }
    }
}