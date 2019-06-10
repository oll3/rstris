use crate::player::*;

use rstris::figure_pos::*;
use rstris::movement::*;
use rstris::playfield::*;
use rstris::pos_dir::*;

//
// Try to place players next figure in playfield
//
pub fn try_place_new_figure(player: &mut dyn Player, ticks: u64, pf: &mut Playfield) -> bool {
    let figure = player.common().next_figure().clone();
    let pos = PosDir::new(((pf.width() / 2 - 1) as i32, 0, 0));
    if figure.test_collision(pf, &pos) {
        println!("Figure collided");
        return true;
    }
    let fig_pos = FigurePos::new(figure, pos);
    player.common_mut().gen_next_figure();
    let next_down = ticks + player.common().force_down_time;
    player.common_mut().add_move(Movement::MoveDown, next_down);

    println!(
        "{}: Placed figure {} in playfield (next is {})",
        player.common().get_name(),
        fig_pos.get_figure().get_name(),
        player.common().next_figure().get_name()
    );
    fig_pos.place(pf);
    player.common_mut().set_figure(Some(fig_pos.clone()));

    player.new_figure_event(ticks, pf, &fig_pos);

    false
}

//
// Move player current figure according to the given movement.
//
pub fn execute_move(player: &mut dyn Player, pf: &mut Playfield, move_and_time: MoveAndTime) {
    let (fig_move, move_time) = (move_and_time.movement, move_and_time.time);
    let mut fig_pos = player.common().get_figure().unwrap();
    fig_pos.remove(pf);
    let test_pos = PosDir::apply_move(fig_pos.get_position(), &fig_move);

    let collision = fig_pos.get_figure().test_collision(pf, &test_pos);
    if collision && fig_move == Movement::MoveDown {
        fig_pos.place(pf);
        player.common_mut().set_figure(None);
    } else {
        if !collision {
            fig_pos.set_position(&test_pos);
        }
        fig_pos.place(pf);
        player.common_mut().set_figure(Some(fig_pos.clone()));

        player.figure_move_event(move_time, pf, &fig_pos, &fig_move);
    }
}
