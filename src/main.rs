extern crate ncurses;
use ncurses::*;
use std::env;
use std::fs::File;
use buffer::Buffer;

mod buffer;
mod view;

fn main() {

    env::set_var("ESCDELAY", "25");
    let mut args = env::args();
    args.next(); // consume first
    let file = File::open(args.next().unwrap()).unwrap();
    let buf = Buffer::new(file);

    initscr();
    use_default_colors();
    start_color();
    cbreak();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_VERY_VISIBLE);
        
    keypad(stdscr(), true);

    init_pair(1, -1, -1);

    let mut view = view::View {
        buf,
        win: stdscr(),
    };

    view.update();
    let mut ch = getch();
    while ch != 27 {
        match ch {
            KEY_RESIZE => {
                view.update();
            },
            _ => {
                view.input(ch);
            },
        }
        ch = getch();
    }
    
    
    endwin();
}
