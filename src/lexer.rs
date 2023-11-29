// let x = 5;

// TokenType : Number
// TokenValue : byte[]
// "abc" = byte[4*char_size]
// 

enum TokenType{
    Let,
    Identifier,
    Number
}

// Let : 'let'
// Number : 'regex for numbers'

pub struct Token{
    m_value: String,
    m_type: TokenType
}

fn PrintToken(tok: Token){
    println!("value : {}",tok.m_value);
}

pub fn tokenize(m_code : String){
    let tokens : Vec<Token> = vec![];
    let src: Vec<&str> = m_code.split(';').collect();
    for line in src {
        let splitted: Vec<&str> = line.split(" ").collect();
        for key in splitted{
            print!("{}:",key)
        }
    }
    
}