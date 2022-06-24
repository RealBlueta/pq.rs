use pq_rs::lexer;

fn main() {
    let tokens = lexer::lex("1 + 3");

    println!("{:?}", tokens);
}
