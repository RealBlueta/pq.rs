use pq_rs::lexer as Lexer;

fn main() {
    let tokens = Lexer::lex(b"1 + 3");
    for token in tokens {
        println!("{:?}", token);
    }
}
