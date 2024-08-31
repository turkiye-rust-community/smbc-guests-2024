use std::thread::sleep;
use std::time::{Duration, Instant};

pub struct Player {
    pub id: usize,
    pub nick_name: String,
    pub health: Option<u8>,
}

impl Player {
    pub fn new(id: usize, nick_name: String) -> Self {
        Self {
            id,
            nick_name,
            health: Some(3),
        }
    }
}

pub struct Tower;

pub struct Scoreboard {
    pub player_point: u16,
    pub current_health: u8,
    pub level: Option<Level>,
}

pub enum Level {
    Beginner,
    Medium,
    Pro(u8),
}

pub enum GameState {
    Playing,
    Paused,
    Lose,
}

pub struct TowerDefenseGame {
    pub player: Player,
    pub towers: Vec<Tower>,
    pub scoreboard: Scoreboard,
}

impl TowerDefenseGame {
    pub fn new(player: Player) -> Self {
        Self {
            player,
            towers: vec![],
            scoreboard: Scoreboard {
                level: None,
                current_health: 3,
                player_point: 0,
            },
        }
    }
}

impl GameObject for TowerDefenseGame {
    fn draw(&self) {
        sleep(Duration::from_secs(2));
        println!("Tower defense drawing");
    }

    fn update(&mut self) -> MainState {
        sleep(Duration::from_secs(2));
        println!("Tower defense updating");
        MainState::Running
    }
}

fn main() -> Result<(), String> {
    let player = Player::new(1903, "Bacari Bonzai".to_string());
    let my_game = TowerDefenseGame::new(player);
    let mut game = GameEngineBuilder::new()?
        .add_game(Box::new(my_game))
        .change_fps(30)
        .build()?;

    game.run()?;

    Ok(())
}

pub enum MainState {
    Initial,
    Running,
    //PreExit,
    Exit,
}

pub trait GameObject {
    fn draw(&self);
    fn update(&mut self) -> MainState;
}

pub struct GameEngine {
    pub game_object: Option<Box<dyn GameObject>>,
    pub fps: u32,
    // pub window:Option<Window>
}

impl Default for GameEngine {
    fn default() -> Self {
        Self {
            game_object: None,
            fps: 60,
        }
    }
}

impl GameEngine {
    pub fn run(&mut self) -> Result<(), String> {
        let mut state = MainState::Initial;
        let mut last_update_time = Instant::now();
        let frame_duration = Duration::new(0, 1_000_000_000u32 / 60);

        loop {
            match state {
                MainState::Initial => {
                    // Do Something
                    state = MainState::Running;
                    continue;
                }
                MainState::Running => {
                    let now = Instant::now();
                    let delta = now.duration_since(last_update_time);

                    if let Some(game_object) = &mut self.game_object {
                        game_object.draw();
                        state = game_object.update();
                    }

                    if frame_duration > delta {
                        sleep(frame_duration - delta);
                    }

                    last_update_time = now;
                }
                MainState::Exit => {
                    break;
                }
            }
        }

        Ok(())
    }
}

pub struct GameEngineBuilder {
    game_engine: GameEngine,
}

impl GameEngineBuilder {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            game_engine: GameEngine::default(),
        })
    }

    pub fn change_fps(mut self, fps: u32) -> Self {
        self.game_engine.fps = fps;
        self
    }

    pub fn add_game(mut self, game_object: Box<dyn GameObject>) -> Self {
        self.game_engine.game_object = Some(game_object);
        self
    }

    // pub fn setup_window(mut self,window:Window) -> Result<GameEngine, String> {
    //     self.window=window;
    //     self
    // }

    pub fn build(self) -> Result<GameEngine, String> {
        Ok(self.game_engine)
    }
}

// pub struct Window {
//     pub width:u32,
//     pub height:u32,
//     pub title:String,
// }