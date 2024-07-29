use lexer::tokens::TokenKind;

mod cli;
mod diagnostics;
mod lexer;
mod parser;
mod utils;

fn main() {
  let matches = cli::command_line();
  match matches.subcommand() {
    Some(("check", matches)) => {
      let path_name = matches.get_one::<String>("file").unwrap();
      check_wasm(path_name);
    }
    Some(("compile", matches)) => {
      let path_name = matches.get_one::<String>("file").unwrap();
      compile_wasm(path_name);
    }
    Some(("run", matches)) => {
      let path_name = matches.get_one::<String>("file").unwrap();
      run_wasm(path_name);
    }
    _ => {}
  }
}

fn run_wasm(file_name: &str) {
  let contents = std::fs::read_to_string(file_name).unwrap();
  let mut lexer = lexer::Lexer::new(&contents, file_name);
  // let mut parser = parser::Parser::new(lexer);
  // parser.parse_program().unwrap();
  loop {
    let token = lexer.next_token();
    println!("{:#?}", token);
    if token.kind == TokenKind::EOF {
      break;
    }
  }
}

fn compile_wasm(file_name: &str) {
  let contents = std::fs::read_to_string(file_name).unwrap();
  let mut lexer = lexer::Lexer::new(&contents, file_name);
  // let mut parser = parser::Parser::new(lexer);
  // parser.parse_program().unwrap();
  loop {
    let token = lexer.next_token();
    println!("{:#?}", token);
    if token.kind == TokenKind::EOF {
      break;
    }
  }
}

fn check_wasm(file_name: &str) {
  let contents = std::fs::read_to_string(file_name).unwrap();
  let mut lexer = lexer::Lexer::new(&contents, file_name);
  loop {
    let token = lexer.next_token();
    println!("{:#?}", token);
    if token.kind == TokenKind::EOF {
      break;
    }
  }
}
