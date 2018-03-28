extern crate ncurses;
use ncurses::*;

use buffer::Buffer;

pub struct View {
    pub buf: Buffer,
    pub win: WINDOW,
}

impl View {


    /// Creates an editable region within the given window
    pub fn input(&mut self, ch: i32) {

        werase(self.win);    

        refresh();
        wmove(self.win, self.buf.pos.1 as i32, self.buf.pos.0 as i32);
        wrefresh(self.win);

        // need scrollability
        let mut view_pos = 0; // the beginning of the view


        if self.buf.take_input(ch) == Ok(()) {
            let (mut row, mut col): (i32, i32) = (0, 0);
            getmaxyx(self.win, &mut row, &mut col); 

            let view_size = row as usize;

            if self.buf.pos.1 < view_pos {
                view_pos = self.buf.pos.1;
            }
            else if self.buf.pos.1 > view_pos + (view_size - 1) {
                view_pos = self.buf.pos.1 - (view_size - 1);
            }

            let sub = view_pos as i32;

            let mut ax = 0;
            let mut ay = 0;

            let mut extra: i32 = 0;
            for (i, line) in self.buf.lines.iter().enumerate() {

                let mut gain = 0;
                if col > 0 && line.len() > 0 {
                    gain = line.len() as i32 / col;
                }
                if i == self.buf.pos.1 {
                    ax = self.buf.pos.0;
                    ay = i as i32 + extra; // now at start of line

                    if gain > 0 {
                        if self.buf.pos.1 as i32 > col {
                            let minigain = self.buf.pos.1 as i32 / col;
                            ay += minigain; // now at start of line
                            ax = self.buf.pos.0 - (minigain * col) as usize;
                        }
                    }
                }
                for l in 0..gain+1 {
                    clrprintw(self.win, i as i32 + extra + l + 1 - sub, 0, "");
                }
                clrprintw(self.win, i as i32 + extra - sub, 0, line);
                extra += gain;
            }

            refresh();

            wmove(self.win, ay as i32 - sub, ax as i32);

            wrefresh(self.win);
        }
    }
}

fn clrprintw(window: WINDOW, y: i32, x: i32, string: &str) {
    wmove(window, y, x);
    wclrtoeol(window);
    mvwprintw(window, y, x, string);
}
