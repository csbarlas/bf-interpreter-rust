mod input;
mod program;

fn main() {
    let mut prog = input::read_program();
    println!("{:?}", prog);
    prog.run();
    println!("{:?}", prog);
}
