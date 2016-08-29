use rstris::playfield::*;
use rstris::position::*;
use rstris::figure::*;
use rstris::figure_pos::*;
use rstris::find::*;

use player::*;

pub struct ComputerPlayer {
    common: PlayerCommon,
    last_fig: String,
    avail_pos: Vec<Position>,
}

impl Player for ComputerPlayer {
    fn common(&self) -> &PlayerCommon {
        &self.common
    }

    fn common_mut(&mut self) -> &mut PlayerCommon {
        &mut self.common
    }

    fn update(&mut self, current_ticks: u64, pf: &Playfield) {
        match self.common.get_figure() {
            Some(ref fig_pos) => {
                let fig = fig_pos.get_figure();
                if self.last_fig != *fig.get_name() {
                    self.last_fig = fig.get_name().clone();
                    self.handle_new_figure(fig_pos, pf);
                }
            }
            None => {}
        }
    }

    fn get_moves(&mut self, current_ticks: u64) -> Vec<(Movement, u64)> {
        Vec::new()
    }
}

impl ComputerPlayer {
    pub fn new(common: PlayerCommon) -> Self {
        ComputerPlayer {
            common: common,
            avail_pos: Vec::new(),
            last_fig: "".to_string(),
        }
    }

    fn handle_new_figure(&mut self, fig_pos: &FigurePos, pf: &Playfield) {
        let mut tmp_pf = pf.clone();
        fig_pos.remove(&mut tmp_pf);
        self.avail_pos =
            get_valid_placing(&tmp_pf, fig_pos);
        println!("New figure ({}) - {} available placings",
                 self.last_fig, self.avail_pos.len());
    }
}
