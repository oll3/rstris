use position::*;
use figure::*;
use playfield::*;

#[derive(Debug)]
pub struct Player {
    current_pos: Position,
    current_figure: Option<Figure>,
}


impl Player {
    pub fn new() -> Self {
        Player{current_pos: Position::new(-1, -1, -1),
               current_figure: None,
        }
    }
    pub fn get_current_pos(&self) -> &Position {
        return &self.current_pos;
    }
    pub fn figure_in_play(&self) -> bool {
        self.current_figure.is_some()
    }
    //
    // Place the next figure in playfield.
    // Game is over if this function returns false.
    //
    pub fn place_figure(&mut self, pf: &mut Playfield,
                        figure: Figure, position: Position) {
        self.current_pos = position;
        figure.place(pf, &self.current_pos);
        self.current_figure = Some(figure);
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
        let new_pos_locked = figure.collide_locked(pf, &new_pos);
        let new_pos_blocked = figure.collide_blocked(pf, &new_pos);
        if !new_pos_locked && !new_pos_blocked {
            self.current_pos = new_pos;
            figure.place(pf, &self.current_pos);
            return true;
        } else if movement == Movement::MoveDown && new_pos_locked {
            // Figure couldn't be moved down further because of collision with
            // locked block(s) - Mark figure blocks as locked in its current
            // position.
            figure.lock(pf, &self.current_pos);
            self.current_figure = None;
            return false;
        } else {
            // Move is not valid
            figure.place(pf, &self.current_pos);
            return true;
        }
    }
}
