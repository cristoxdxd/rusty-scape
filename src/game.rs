use ggez::{
    event, graphics,
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult, glam::Vec2
};

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
            Direction::Right => GridPosition::new(pos.x + 1, pos.y), // add speed
            Direction::None => pos,
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
    None,
}

impl Direction {
    pub fn from_keycode(keycode: KeyCode) -> Option<Self> {
        match keycode {
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Particle {
    pos: (f32, f32),
    velocity: (f32, f32),
    life: f32,
    color: graphics::Color,
}

impl Particle {
    fn new(pos: GridPosition) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Particle {
            pos: (pos.x as f32, pos.y as f32),
            velocity: (rng.gen_range(-2.0..2.0), rng.gen_range(-3.0..-1.0)),
            life: 1.0,
            color: graphics::Color::new(1.0, rng.gen_range(0.5..1.0), 0.0, 1.0),
        }
    }

    fn update(&mut self) {
        self.pos.0 += self.velocity.0;
        self.pos.1 += self.velocity.1;
        self.velocity.1 += 0.1; // gravity
        self.life -= 0.02;
        self.color.a = self.life;
    }

    fn draw(&self, canvas: &mut graphics::Canvas) {
        if self.life > 0.0 {
            let rect = graphics::Rect::new(
                self.pos.0 * GRID_CELL_SIZE.0 as f32,
                self.pos.1 * GRID_CELL_SIZE.1 as f32,
                GRID_CELL_SIZE.0 as f32 / 2.0,
                GRID_CELL_SIZE.1 as f32 / 2.0,
            );
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(rect)
                    .color(self.color),
            );
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

    fn draw(&self, canvas: &mut graphics::Canvas, difficulty_level: f32) {
        // Change obstacle color based on difficulty for visual feedback
        let red_intensity = (difficulty_level / 5.0).min(1.0);
        let color = graphics::Color::new(0.48 + red_intensity * 0.5, 0.39 - red_intensity * 0.2, 0.93 - red_intensity * 0.2, 1.0);
        
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

#[derive(Clone, Copy, Debug)]
enum GamePhase {
    Menu,
    Playing,
    GameOver,
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
            dir: Direction::None,
            state: PlayerState::Alive,
        }
    }

    fn die(&self, obstacles: &[Obstacle], tolerance: i16) -> bool {
        fn is_approx_equal(pos1: GridPosition, pos2: GridPosition, tolerance: i16) -> bool {
            let dx = (pos1.x - pos2.x).abs();
            let dy = (pos1.y - pos2.y).abs();
            
            dx <= tolerance && dy <= tolerance
        }

        for obstacle in obstacles {
            if is_approx_equal(self.body.pos, obstacle.pos, tolerance) {
                return true;
            }
        }
        false
    }

    fn update(&mut self, dir: Direction, obstacles: &[Obstacle]) -> bool {
        self.body.update(dir);
        self.dir = dir;

        if self.die(obstacles, 1) {
            self.state = PlayerState::Dead;
            return true; // Return true if player died
        }
        false
    }

    fn draw(&self, canvas: &mut graphics::Canvas) {
        let color = match self.dir {
            Direction::Left => graphics::Color::new(0.8, 0.8, 1.0, 1.0),  // Slightly blue when moving left
            Direction::Right => graphics::Color::new(1.0, 0.8, 0.8, 1.0), // Slightly red when moving right
            Direction::None => graphics::Color::WHITE,
        };

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
    score: f32,
    difficulty_level: f32,
    game_phase: GamePhase,
    high_score: f32,
    particles: Vec<Particle>,
}

impl GameState {
    pub fn new() -> Self {
        let player = Player::new(GridPosition::new(GRID_SIZE.0 / 2, GRID_SIZE.1 - 1));
        let obstacles_pos = GridPosition::random();
        let obstacles = vec![Obstacle::new(obstacles_pos)];
        GameState { 
            player,
            obstacles,
            score: 0.0,
            difficulty_level: 1.0,
            game_phase: GamePhase::Menu,
            high_score: 0.0,
            particles: Vec::new(),
        }
    }

    fn start_game(&mut self) {
        self.player = Player::new(GridPosition::new(GRID_SIZE.0 / 2, GRID_SIZE.1 - 1));
        let obstacles_pos = GridPosition::random();
        self.obstacles = vec![Obstacle::new(obstacles_pos)];
        self.score = 0.0;
        self.difficulty_level = 1.0;
        self.game_phase = GamePhase::Playing;
        self.particles.clear();
    }

    fn restart(&mut self) {
        if self.score > self.high_score {
            self.high_score = self.score;
        }
        self.start_game();
    }

    fn create_explosion(&mut self, pos: GridPosition) {
        for _ in 0..8 {
            self.particles.push(Particle::new(pos));
        }
    }

    fn update_obstacle(&mut self) {
        // Move obstacles down at increasing speed based on difficulty
        let fall_speed = (self.difficulty_level / 2.0).max(1.0) as i16;
        for obstacle in &mut self.obstacles {
            obstacle.pos.y += fall_speed;
        }

        self.obstacles.retain(|obstacle| obstacle.pos.y < GRID_SIZE.1);

        // Increase spawn rate with difficulty
        let spawn_rate = (0.1 * self.difficulty_level).min(0.3);
        if rand::random::<f32>() < spawn_rate {
            let new_obstacle = Obstacle::new(GridPosition::new(rand::thread_rng().gen_range(0..GRID_SIZE.0), 0));
            self.obstacles.push(new_obstacle);
        }
    }    
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.game_phase {
            GamePhase::Menu => {
                // No updates needed in menu
            }
            GamePhase::Playing => {
                while ctx.time.check_update_time(DESIRED_FPS) {
                    if let PlayerState::Alive = self.player.state {
                        let player_died = self.player.update(self.player.dir, &self.obstacles);
                        if player_died {
                            self.create_explosion(self.player.body.pos);
                        }
                        
                        self.update_obstacle();

                        // Improved bounds checking - keep player within screen bounds
                        if self.player.body.pos.x < 0 {
                            self.player.body.pos.x = 0;
                        }
                        if self.player.body.pos.x >= GRID_SIZE.0 {
                            self.player.body.pos.x = GRID_SIZE.0 - 1;
                        }

                        self.score += 0.1;
                        
                        // Increase difficulty every 50 points
                        self.difficulty_level = 1.0 + (self.score / 50.0);
                    } else {
                        // Player died, transition to game over
                        self.game_phase = GamePhase::GameOver;
                    }

                    // Update particles
                    for particle in &mut self.particles {
                        particle.update();
                    }
                    self.particles.retain(|p| p.life > 0.0);
                }
            }
            GamePhase::GameOver => {
                // No updates needed in game over
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Create a canvas
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        match self.game_phase {
            GamePhase::Menu => {
                // Title screen
                let title = graphics::Text::new("RUSTY SCAPE");
                let title_dest = Vec2::new((SCREEN_SIZE.0 / 2.0) - 60.0, SCREEN_SIZE.1 / 3.0);
                canvas.draw(
                    &title, 
                    graphics::DrawParam::from(title_dest).color(graphics::Color::new(1.0, 0.8, 0.2, 1.0))
                );

                let subtitle = graphics::Text::new("Dodge the falling blocks!");
                let subtitle_dest = Vec2::new((SCREEN_SIZE.0 / 2.0) - 80.0, SCREEN_SIZE.1 / 3.0 + 40.0);
                canvas.draw(
                    &subtitle, 
                    graphics::DrawParam::from(subtitle_dest).color(graphics::Color::WHITE)
                );

                let instructions = graphics::Text::new("Press SPACE to start\nArrow keys to move");
                let inst_dest = Vec2::new((SCREEN_SIZE.0 / 2.0) - 70.0, SCREEN_SIZE.1 / 2.0 + 20.0);
                canvas.draw(
                    &instructions, 
                    graphics::DrawParam::from(inst_dest).color(graphics::Color::new(0.8, 0.8, 0.8, 1.0))
                );

                if self.high_score > 0.0 {
                    let high_score_text = graphics::Text::new(format!("High Score: {}", self.high_score.trunc()));
                    let hs_dest = Vec2::new((SCREEN_SIZE.0 / 2.0) - 50.0, SCREEN_SIZE.1 / 2.0 + 80.0);
                    canvas.draw(
                        &high_score_text, 
                        graphics::DrawParam::from(hs_dest).color(graphics::Color::new(0.2, 1.0, 0.2, 1.0))
                    );
                }
            }
            GamePhase::Playing => {
                self.player.draw(&mut canvas);
                for obstacle in &self.obstacles {
                    obstacle.draw(&mut canvas, self.difficulty_level);
                }

                // Draw particles
                for particle in &self.particles {
                    particle.draw(&mut canvas);
                }

                let text = graphics::Text::new(format!("Score: {} | Level: {:.1}", self.score.trunc(), self.difficulty_level));
                let dest_point = Vec2::new(10.0, 10.0);
                canvas.draw(
                    &text, 
                    graphics::DrawParam::from(dest_point).color(graphics::Color::WHITE)
                );
            }
            GamePhase::GameOver => {
                let is_new_high_score = self.score > self.high_score;
                let high_score_msg = if is_new_high_score { "\nNEW HIGH SCORE!" } else { "" };
                
                let text = graphics::Text::new(format!("Game Over!{}\nScore: {}\nHigh Score: {}\n\nPress SPACE to restart\nPress ESC for menu", 
                    high_score_msg, self.score.trunc(), self.high_score.max(self.score).trunc()));
                let dest_point = Vec2::new((SCREEN_SIZE.0 / 2.0) - 100.0, SCREEN_SIZE.1 / 2.0 - 70.0);
                let color = if is_new_high_score { 
                    graphics::Color::new(0.2, 1.0, 0.2, 1.0) 
                } else { 
                    graphics::Color::WHITE 
                };
                canvas.draw(
                    &text, 
                    graphics::DrawParam::from(dest_point).color(color)
                );
            }
        }

        canvas.finish(ctx)?;
        
        Ok(())
    }
    
    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        match self.game_phase {
            GamePhase::Menu => {
                if let Some(KeyCode::Space) = input.keycode {
                    self.start_game();
                }
            }
            GamePhase::Playing => {
                match input.keycode {
                    Some(KeyCode::Left) | Some(KeyCode::Right) => {
                        if let Some(dir) = Direction::from_keycode(input.keycode.unwrap()) {
                            self.player.dir = dir;
                        }
                    }
                    Some(KeyCode::Escape) => {
                        self.game_phase = GamePhase::Menu;
                    }
                    _ => {}
                }
            }
            GamePhase::GameOver => {
                match input.keycode {
                    Some(KeyCode::Space) => {
                        self.restart();
                    }
                    Some(KeyCode::Escape) => {
                        if self.score > self.high_score {
                            self.high_score = self.score;
                        }
                        self.game_phase = GamePhase::Menu;
                    }
                    _ => {}
                }
            }
        }
        
        Ok(())
    }
}
