extern crate rand;
extern crate sdl2;
extern crate time;
extern crate rstris;

mod draw;

use draw::*;
use rstris::block::*;
use rstris::playfield::*;
use rstris::player::*;
use rstris::figure::*;
use rstris::position::*;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;


static PF_WIDTH: u32 = 10;
static PF_HEIGHT: u32 = 20;
static BLOCK_SIZE: u32 = 20;
static BLOCK_SPACING: u32 = 1;
static FRAME_COLOR: Color = Color::RGB(200, 64, 64);
static FILL_COLOR: Color = Color::RGB(98, 204, 244);
static BG_COLOR: Color = Color::RGB(101, 208, 246);
static DELAY_FIRST_STEP_DOWN: u64 = 1 * 1000 * 1000 * 1000;

struct PlayerKeys {
    step_left: Option<Keycode>,
    step_right: Option<Keycode>,
    step_down: Option<Keycode>,
    rot_cw: Option<Keycode>,
    rot_ccw: Option<Keycode>,
}

struct PlayerStats {
    line_count: usize,
}

struct PlayerContext {
    name: String,
    key_map: PlayerKeys,
    stats: PlayerStats,
    time_last_move: HashMap<Movement, u64>,
    avail_figures: Vec<Figure>,
    next_figure: Figure,
    player: Player,
    delay_first_step_down: u64,
}

struct PlayfieldContext {
    pf: Playfield,
    player_ctx: Vec<PlayerContext>,
    game_over: bool,
    lines_to_throw: Vec<usize>,
}


impl PlayerContext {
    pub fn new(name: &str, player: Player, key_map: PlayerKeys,
               figures: Vec<Figure>) -> Self {
        PlayerContext{
            name: name.to_owned(),
            key_map: key_map,
            stats: PlayerStats{
                line_count: 0,
            },
            time_last_move: HashMap::new(),
            next_figure: PlayerContext::get_rand_figure(&figures).clone(),
            avail_figures: figures,
            player: player,
            delay_first_step_down: 0,
        }
    }
    fn get_rand_figure(figures: &Vec<Figure>) -> &Figure {
        let next_figure = (rand::random::<u8>() %
                           figures.len() as u8) as usize;
        return &figures[next_figure];
    }
    pub fn get_next_figure(&self) -> &Figure {
        &self.next_figure
    }
    pub fn gen_next_figure(&mut self) {
        self.next_figure =
            PlayerContext::get_rand_figure(&self.avail_figures).clone();
    }
}

impl PlayfieldContext {
    pub fn new(pf: Playfield) -> Self {
        PlayfieldContext{pf: pf,
                         player_ctx: Vec::new(),
                         game_over: false,
                         lines_to_throw: Vec::new()}
    }

    pub fn add_player(&mut self, player: PlayerContext) {
        self.player_ctx.push(player);
    }

    // Test if there are figures currently being played
    pub fn figures_in_play(&self) -> bool {
        for player in &self.player_ctx {
            if player.player.figure_in_play() {
                return true;
            }
        }
        return false;
    }
}

//
// Build list of figures
//
fn init_figures() -> Vec<Figure> {
    let mut figure_list: Vec<Figure> = Vec::new();
    figure_list.push(Figure::
                     new_from_face("1",
                                   &[&[0, 0, 0],
                                     &[1, 1, 1],
                                     &[0, 1, 0]]));
    figure_list.push(Figure::
                     new_from_face("2",
                                   &[&[0, 0, 0],
                                     &[2, 2, 2],
                                     &[0, 0, 2]]));
    figure_list.push(Figure::
                     new_from_face("3",
                                   &[&[0, 0, 3],
                                     &[3, 3, 3],
                                     &[0, 0, 0]]));
    figure_list.push(Figure::
                     new_from_face("4",
                                   &[&[4, 4],
                                     &[4, 4]]));
    figure_list.push(Figure::
                     new_from_face("5",
                                   &[&[0, 5, 5],
                                     &[5, 5, 0]]));
    figure_list.push(Figure::
                     new_from_face("6",
                                   &[&[6, 6, 0],
                                     &[0, 6, 6]]));
    figure_list.push(Figure::
                     new_from_face("7",
                                   &[&[0, 7, 0],
                                     &[0, 7, 0],
                                     &[0, 7, 0],
                                     &[0, 7, 0]]));
    return figure_list;
}

fn get_max_figure_dimensions(figure_list: &Vec<Figure>)
                             -> (u32, u32) {
    let mut max_width: u32 = 0;
    let mut max_height: u32 = 0;
    for fig in figure_list {
        for dir in fig.dir.clone() {
            if dir.get_width() as u32 > max_width {
                max_width = dir.get_width() as u32;
            }
            if dir.get_height() as u32 > max_height {
                max_height = dir.get_height() as u32;
            }
        }
    }
    return (max_width, max_height);
}


//
// Try to place a new figure in playfield
//
fn place_new_figure(player_ctx: &mut PlayerContext,
                    pf: &mut Playfield) -> bool {

    let current_ticks = time::precise_time_ns();
    // Place new figure in playfield
    let figure = player_ctx.get_next_figure().clone();
    let figure_pos = Position::new((pf.width() / 2 - 1) as i32, 0, 0);
    if figure.collide_locked(pf, &figure_pos) {
        println!("{}: Game over!", player_ctx.name);
        return false;
    }
    if figure.collide_blocked(pf, &figure_pos) {
        return true;
    }
    player_ctx.gen_next_figure();
    println!("{}: Placed figure {} in playfield (next is {})",
             player_ctx.name, figure.get_name(),
             player_ctx.get_next_figure().get_name());
    player_ctx.player.place_figure(pf, figure, figure_pos);
    player_ctx.delay_first_step_down =
        current_ticks + DELAY_FIRST_STEP_DOWN;
    return true;
}

//
// Move player current figure according to the given movements.
// If movement caused full lines being created then return those
// line indexes.
//
fn handle_player_moves(player_ctx: &mut PlayerContext, pf: &mut Playfield,
                       moves: Vec<Movement>) -> Vec<usize> {

    let current_ticks = time::precise_time_ns();
    for fig_move in moves {
        player_ctx.delay_first_step_down = 0;

        player_ctx.time_last_move.insert(fig_move.clone(), current_ticks);
        if !player_ctx.player.move_figure(pf, fig_move) {

            // Figure couldn't be moved downwards -
            // Test for full lines and return them if there are
            // any.

            let full_lines = pf.find_full_lines();
            player_ctx.stats.line_count += full_lines.len();
            return full_lines;
        }
    }
    return Vec::new();
}

fn handle_player_input(key_map: &PlayerKeys, pressed_keys:
                       &mut HashMap<Keycode, (u64, u64)>) -> Vec<Movement> {
    let current_ticks = time::precise_time_ns();
    let mut moves: Vec<Movement> = vec![];
    let keys = pressed_keys.clone();
    for (key, (time, this_delay)) in keys {

        if time <= current_ticks {

            let next_delay = (this_delay * 2) / 5 + 20000000;
            let delay = current_ticks + this_delay;

            if !key_map.step_left.is_none() &&
                key == key_map.step_left.unwrap()
            {
                moves.push(Movement::MoveLeft);
                pressed_keys.insert(key, (delay, next_delay));
            }
            else if !key_map.step_right.is_none() &&
                key == key_map.step_right.unwrap()
            {
                moves.push(Movement::MoveRight);
                pressed_keys.insert(key, (delay, next_delay));
            }
            else if !key_map.step_down.is_none() &&
                key == key_map.step_down.unwrap()
            {
                moves.push(Movement::MoveDown);
                pressed_keys.insert(key, (delay, next_delay));
            }
            else if !key_map.rot_cw.is_none() &&
                key == key_map.rot_cw.unwrap()
            {
                moves.push(Movement::RotateCW);
                pressed_keys.insert(key, (delay, next_delay));
            }
            else if !key_map.rot_ccw.is_none() &&
                key == key_map.rot_ccw.unwrap()
            {
                moves.push(Movement::RotateCCW);
                pressed_keys.insert(key, (delay, next_delay));
            }
        }
    }
    return moves;
}


fn move_every(time_last_move: &HashMap<Movement, u64>, movement: Movement,
              every_ns: u64) -> Vec<Movement> {
    let mut moves: Vec<Movement> = vec![];
    let current_ticks = time::precise_time_ns();
    let last_move = time_last_move.get(&movement);
    if last_move.is_none() ||
        (last_move.unwrap() + every_ns) < current_ticks {
            moves.push(movement);
        }
    return moves;
}


fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let figure_list = init_figures();
    let (figure_max_width, figure_max_height) =
        get_max_figure_dimensions(&figure_list);
    println!("Max figure dimensions: {} x {}",
             figure_max_width, figure_max_height);

    let window_width: u32 = (PF_WIDTH + 2 + figure_max_width + 3) *
        (BLOCK_SIZE + BLOCK_SPACING);
    let window_height: u32 = (PF_HEIGHT + 1) * (BLOCK_SIZE + BLOCK_SPACING);
    let window = video_subsystem.window("rust-sdl2 demo: Video",
                                        window_width, window_height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut draw = DrawContext::new(window, BLOCK_SIZE,
                                    BLOCK_SPACING, FRAME_COLOR,
                                    FILL_COLOR);

    let player1_key_map = PlayerKeys {
        step_left: Some(Keycode::Left),
        step_right: Some(Keycode::Right),
        step_down: Some(Keycode::Down),
        rot_cw: Some(Keycode::Up),
        rot_ccw: None
    };

    let pf1 = Playfield::new("Playfield 1",
                             PF_WIDTH as usize, PF_HEIGHT as usize);
    let mut pf_ctx = PlayfieldContext::new(pf1);
    pf_ctx.add_player(PlayerContext::new("Player 1",
                                         Player::new(),
                                         player1_key_map,
                                         figure_list));

    let mut pause = false;

    let mut pressed_keys: HashMap<Keycode, (u64, u64)> = HashMap::new();
    let mut events = sdl_context.event_pump().unwrap();
    'running: loop {
        let current_ticks = time::precise_time_ns();
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {
                    keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    pause = !pause;
                    if pause {
                        println!("Paused");
                    } else {
                        println!("Continued");
                    }
                },Event::KeyDown {
                    keycode: Some(key), .. } => {
                    if !pressed_keys.contains_key(&key) {
                        pressed_keys.insert(key, (current_ticks, 140000000));
                    }
                },Event::KeyUp {
                    keycode: Some(key), .. } => {
                    if pressed_keys.contains_key(&key) {
                        pressed_keys.remove(&key);
                    }
                },

                _ => {}
            }
        }

        if pause || pf_ctx.game_over {
            continue;
        }


        // Handle movement and figure creation
        for pl_ctx in &mut pf_ctx.player_ctx {
            let mut moves = handle_player_input(&pl_ctx.key_map,
                                                &mut pressed_keys);
            if current_ticks > pl_ctx.delay_first_step_down {
                moves.append(&mut move_every(&mut pl_ctx.time_last_move,
                                             Movement::MoveDown,
                                             500000000 /* ns */));
            }

            if pl_ctx.player.figure_in_play() {
                // Player has a figure in game
                let mut lines =
                    handle_player_moves(pl_ctx, &mut pf_ctx.pf, moves);
                pf_ctx.pf.set_lines(&lines, &Block::new_locked(10));
                pf_ctx.lines_to_throw.append(&mut lines);

            } else if pf_ctx.lines_to_throw.len() == 0 {
                if !place_new_figure(pl_ctx, &mut pf_ctx.pf) {
                    pf_ctx.game_over = true;
                }
            }
        }

        // Throw full lines (when there are no figures being played)
        if pf_ctx.lines_to_throw.len() > 0 && !pf_ctx.figures_in_play() {
            println!("Throw away {} lines!", pf_ctx.lines_to_throw.len());
            for line in &pf_ctx.lines_to_throw {
                pf_ctx.pf.throw_line(*line);
            }
            pf_ctx.lines_to_throw.clear();
        }

        /* Render graphics */
        draw.clear(BG_COLOR);
        draw.draw_playfield(&pf_ctx.pf);
        let mut pi = 0;
        for player in &mut pf_ctx.player_ctx {
            draw.draw_next_figure(&player.get_next_figure(), PF_WIDTH + 3,
                                  (figure_max_height + 2) * pi,
                                  figure_max_width, figure_max_height);

            pi += 1;
        }
        draw.present();
    }
}
