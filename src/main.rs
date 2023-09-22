mod input;
mod program;

fn main() {
    let prog = input::read_program();
    dbg!(prog.data);
}
