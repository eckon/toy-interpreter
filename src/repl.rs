use crate::{lexer::Lexer, token::TokenType};

pub fn start() {
    let mut buffer = String::new();

    loop {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        let mut lexer = Lexer::new(buffer.clone());
        loop {
            let token = lexer.next_token();
            println!("{:?}", token);

            if token.get_type() == TokenType::Eof {
                break;
            }
        }
    }
}
