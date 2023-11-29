
mod run;
mod lexer;
use run::runtime_options;
use run::runtime;



fn main() {
    let options = runtime_options::new();
    let m_runner = runtime::new(options);
    runtime::run(m_runner);
    // println!("Hello, world!");
}
