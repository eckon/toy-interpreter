mod lexer;
mod repl;
mod token;

fn main() {
    println!("Hello to the this toy interpreter!");
    println!("Feel free to type in commands");

    repl::start();
}
