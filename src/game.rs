use ggez::{
    event, graphics,
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult};
use rand::Rng;

const GRID_SIZE: (i16, i16) = (30, 20);
const GRID_CELL_SIZE: (i16, i16) = (32, 32);

pub const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

const DESIRED_FPS: u32 = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct GridPosition {
    x: i16,
    y: i16,
}

impl GridPosition {
    fn new(x: i16, y: i16) -> Self {
        GridPosition { x, y }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        GridPosition {
            x: rng.gen_range(0..GRID_SIZE.0),
            y: rng.gen_range(0..GRID_SIZE.1),
        }
    }

    pub fn new_for_move(pos: GridPosition, dir: Direction) -> Self {
        match dir {
            Direction::Left => GridPosition::new(pos.x - 1, pos.y),
            Direction::Right => GridPosition::new(pos.x + 1, pos.y),
        }
    }
}

impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * GRID_CELL_SIZE.0 as i32,
            pos.y as i32 * GRID_CELL_SIZE.1 as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        )
    }
}

impl From<(i16, i16)> for GridPosition {
    fn from(pos: (i16, i16)) -> Self {
        GridPosition::new(pos.0, pos.1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match *self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn from_keycode(keycode: KeyCode) -> Option<Self> {
        match keycode {
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Segment {
    pos: GridPosition,
}

impl Segment {
    fn new(pos: GridPosition) -> Self {
        Segment { pos }
    }

    fn update(&mut self, dir: Direction) {
        self.pos = GridPosition::new_for_move(self.pos, dir);
    }
}

struct Obstacle {
    pos: GridPosition,
}

impl Obstacle {
    pub fn new(pos: GridPosition) -> Self {
        Obstacle { pos }
    }

    fn draw(&self, canvas: &mut graphics::Canvas) {
        let color = graphics::Color::new(0.48, 0.39, 0.93, 1.0);

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.pos.into())
                .color(color),
        );
    }
}

#[derive(Clone, Copy, Debug)]
enum PlayerState {
    Alive,
    Dead,
}

struct Player {
    body: Segment,
    dir: Direction,
    state: PlayerState,
}

impl Player {
    pub fn new(pos: GridPosition) -> Self {
        Player {
            body: Segment::new(pos),
            dir: Direction::Left,
            state: PlayerState::Alive,
        }
    }

    fn die(&self) -> bool {
        if self.body.pos == Obstacle::new(GridPosition::random()).pos {
            return true;
        }
        false
    }

    fn update(&mut self, dir: Direction) {
        self.body.update(dir);
        self.dir = dir;

        if self.die() {
            self.state = PlayerState::Dead;
        }
    }

    fn draw(&self, canvas: &mut graphics::Canvas) {
        let color = graphics::Color::WHITE;

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.body.pos.into())
                .color(color),
        );
    }

}

pub struct GameState {
    player: Player,
    obstacles: Vec<Obstacle>, 
    die: bool,
    //score: u32,
}

impl GameState {
    pub fn new() -> Self {
        let player = Player::new(GridPosition::new(15, 10));
        let obstacles_pos = GridPosition::random();
        let obstacles = vec![Obstacle::new(obstacles_pos)];
        GameState { 
            player,
            obstacles,
            die: false,
            //score: 0,
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while ctx.time.check_update_time(DESIRED_FPS) {
            if self.die {
                println!("Game Over");
                std::process::exit(0);
            }
            self.player.update(self.player.dir);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Create a canvas
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        self.player.draw(&mut canvas);
        for obstacle in &self.obstacles {
            obstacle.draw(&mut canvas);
        }

        canvas.finish(ctx)?;
        
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        if let Some(dir) = input.keycode.and_then(Direction::from_keycode) {
            if self.player.dir != dir.opposite() {
                self.player.dir= dir;   
            }
        }
        
        Ok(())
    }
}
