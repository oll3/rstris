extern crate rand;

use position::*;
use figure::*;
use playfield::*;

#[derive(Debug)]
pub struct Player<'a> {
    player_name: String,
    current_pos: Position,
    avail_figures: &'a Vec<Figure>,
    next_figure: Figure,
    current_figure: Option<Figure>,
}


impl <'a> Player<'a> {
    pub fn new(name: &str, figures: &'a Vec<Figure>) -> Self {
        Player{player_name: name.to_owned(),
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
    pub fn get_current_figure(&self) -> Figure {
        return self.current_figure.clone().unwrap();
    }
    pub fn get_current_pos(&self) -> Position {
        return self.current_pos.clone();
    }
    //
    // Place the next figure in playfield.
    // Game is over if this function returns false.
    //
    pub fn place_next_figure(&mut self, pf: &mut Playfield) -> bool {
        self.current_pos = Position::new((pf.width() / 2 - 1) as i32, 0, 0);
        self.current_figure = Some(self.next_figure.clone());

        let figure = self.current_figure.clone().unwrap();
        if !figure.test(pf, &self.current_pos) {
            return false;
        } else {
            self.next_figure.place(pf, &self.current_pos);
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
                       movement: Movement) -> bool {

        let figure = self.current_figure.clone().unwrap();
        let mut new_pos = Position::apply_move(&self.current_pos, &movement);
        new_pos.normalize_dir(figure.dir.len());

        figure.remove(pf, &self.current_pos);
        if figure.test(pf, &new_pos) {
            self.current_pos = new_pos;
            figure.place(pf, &self.current_pos);
            return true;
        } else if movement != Movement::MoveDown {
            figure.place(pf, &self.current_pos);
            return true;
        } else {
            /* Figure couldn't be moved down - Mark figure blocks
            as locked in its current position */
            figure.lock(pf, &self.current_pos);
            return false;
        }
    }
}
