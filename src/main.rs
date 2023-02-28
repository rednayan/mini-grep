use std::env;
use std::process;
use mini_grep::Config;

fn main(){
    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments:{err}");
        process::exit(1);
    });
  if let Err(e) = mini_grep::run(config) {
    eprintln!("Application error : {e}");
    process::exit(1);
  }
}


