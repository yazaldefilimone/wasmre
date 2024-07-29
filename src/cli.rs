use clap::{Arg, Command};

pub fn command_line() -> clap::ArgMatches {
  let matches = Command::new("wasmre")
    .about("WebAssembly Runtime Engine")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .author("Yazalde Filimone <yazaldefilimon@gmail.com>")
    .subcommand(
      Command::new("check")
        .about("analyze a wasm file.")
        .arg(Arg::new("file").help("the wasm file to check.").required(true)),
    )
    .subcommand(
      Command::new("compile")
        .about("compile a wasm file.")
        .arg(Arg::new("file").help("the wasm file to compile.").required(true)),
    )
    .subcommand(
      Command::new("run").about("run a wasm file.").arg(Arg::new("file").help("the wasm file to run.").required(true)),
    )
    .get_matches();

  return matches;
}
