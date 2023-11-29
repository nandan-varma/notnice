use crate::lexer::{self, tokenize};

pub struct runtime{
    tokens : Vec<lexer::Token> ,
}

pub struct runtime_options{
}
impl runtime_options{
    pub fn new()-> runtime_options{
        runtime_options {  }
    }
}

impl runtime{
    // impl. runtime options
    pub fn new(options:runtime_options)-> runtime{
        let m_runner: runtime = runtime { tokens: Vec::new() };
        println!("Runtime initiated");
        m_runner
    }

    pub fn run(r: runtime) {
        tokenize("let x = 5;let y = 6;".to_string());
    }
}