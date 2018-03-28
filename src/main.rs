extern crate ncurses;
use ncurses::*;

mod buffer;
mod view;

fn main() {

    std::env::set_var("ESCDELAY", "25");
    initscr();
    use_default_colors();
    start_color();
    cbreak();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_VERY_VISIBLE);
        
    keypad(stdscr(), true);

    init_pair(1, -1, -1);

    let mut view = view::View {
        buf: buffer::Buffer::new(),
        win: stdscr(),
    };

    let mut ch = getch();
    while ch != 27 {
        match ch {
            KEY_RESIZE => {
                view.input(27);            
            },
            _ => view.input(ch),
        }
        ch = getch();
    }
    
    
    endwin();
}
