extern crate rand;

use rstris::position::*;
use rstris::figure::*;
use rstris::playfield::*;

#[derive(Debug)]
pub struct Player<'a> {
    player_name: String,
    current_pos: Position,
    avail_figures: &'a Vec<Figure>,
    next_figure: Figure,
    current_figure: Option<Figure>,
}


impl <'a> Player<'a> {
    pub fn new(name: String, figures: &Vec<Figure>) -> Player {
        Player{player_name: name,
               avail_figures: figures,
               current_pos: Position::new(-1, -1, -1),
               current_figure: None,
               next_figure: Player::get_rand_figure(figures),
        }
    }

    //
    // Get a random figure from array of figures
    //
    fn get_rand_figure(avail_figures: &Vec<Figure>) -> Figure {
        let next_figure = (rand::random::<u8>() %
                           avail_figures.len() as u8) as usize;
        let figure = avail_figures[next_figure].clone();
        return figure;
    }

    pub fn get_next_figure(&self) -> Figure {
        return self.next_figure.clone();
    }

    //
    // Place the next figure in playfield.
    // Game is over if this function returns false.
    //
    pub fn place_next_figure(&mut self, pf: &mut Playfield) -> bool {
        self.current_pos = Position::new((pf.width() / 2 - 1) as i32, 0, 0);
        self.current_figure = Some(self.next_figure.clone());

        let figure = self.current_figure.clone().unwrap();
        if !figure.test_figure(pf, &self.current_pos) {
            return false;
        } else {
            self.next_figure.place_figure(pf, &self.current_pos);
            println!("{}: Placed figure {} in playfield",
                     self.player_name, self.next_figure.get_name());

            self.next_figure = Player::get_rand_figure(self.avail_figures);
            println!("{}: Next figure is {}", self.player_name,
                     self.next_figure.get_name());
        }
        return true;
    }

    //
    // Move figure to new (relative) position.
    // If the move is downwards and fails false is returned, else true.
    //
    pub fn move_figure(&mut self, pf: &mut Playfield,
                       rel_pos: &Position) -> bool {

        let figure = self.current_figure.clone().unwrap();
        let mut new_pos = Position::add_pos(&self.current_pos, &rel_pos);
        new_pos.normalize_dir(figure.dir.len());
        let result = figure.move_figure(pf, &self.current_pos, &new_pos);
        if result {
            self.current_pos = new_pos;
            return true;
        } else {
            return rel_pos.get_y() == 0;
        }
    }
}
