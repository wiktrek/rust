use ncurses::*;
fn main() {
  initscr();
    addstr("Hello, world!");
    refresh();
      getch();
  endwin();
}
