mod input;
mod program;

fn main() {
    let mut prog = input::read_program();
    prog.run();
}
