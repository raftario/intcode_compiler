use std::io::Write;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{self, Command, Stdio},
};
use structopt::StructOpt;

mod error;
mod interpreter;
mod parser;
mod transpiler;

fn read_to_string<P: AsRef<Path>>(path: P) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| {
        println!("{}", e);
        process::exit(3);
    })
}

#[derive(StructOpt)]
enum Opt {
    /// Interactively runs an Intcode program with the built-in interpreter
    Run {
        /// Intcode file to run
        #[structopt(name = "FILE")]
        file: PathBuf,
    },

    /// Compiles an Intcode program to a standalone binary
    Compile {
        /// Intcode file to run
        #[structopt(name = "FILE")]
        file: PathBuf,

        /// Inputs to pass to the program, formatted the same way as Intcode
        #[structopt(name = "INPUT")]
        input: Option<PathBuf>,

        /// File to write binary to
        #[structopt(short, long, name = "OUTPUT")]
        output: Option<PathBuf>,

        /// Transpiles the Intcode program to Rust and prints in without compiling
        #[structopt(short = "T", long)]
        transpile_only: bool,

        /// LLVM optimisation level
        #[structopt(short = "O", long = "opt-level", name = "LEVEL")]
        optimisation_level: Option<char>,
    },
}

impl Opt {
    fn run(self) -> Result<(), error::Error> {
        match self {
            Opt::Run { file } => {
                let contents = read_to_string(file);
                let mut code = parser::parse(&contents)?;
                interpreter::run(&mut code, 0)?;
            }
            Opt::Compile {
                file,
                input,
                output,
                transpile_only,
                optimisation_level,
            } => {
                let contents = read_to_string(&file);
                let code = parser::parse(&contents)?;
                let input = match input {
                    None => vec![],
                    Some(i) => {
                        let contents = read_to_string(i);
                        parser::parse(&contents)?
                    }
                };

                let transpiled = transpiler::transpile(code, input)?;
                if transpile_only {
                    print!("{}", transpiled);
                    return Ok(());
                }

                let output = output.unwrap_or_else(|| {
                    PathBuf::from({
                        let file_stem = file.file_stem().unwrap().to_str().unwrap();
                        if cfg!(windows) {
                            format!("{}.exe", file_stem)
                        } else {
                            file_stem.to_owned()
                        }
                    })
                });
                let optimisation_level = match optimisation_level {
                    None => '2',
                    Some(l) => match l {
                        '0'..='3' | 's' | 'z' => optimisation_level.unwrap(),
                        _ => {
                            println!("Invalid optimisation level");
                            process::exit(2);
                        }
                    },
                };

                let mut child = Command::new("rustc")
                    .args(&[
                        "-",
                        "-o",
                        output.to_str().unwrap(),
                        "-C",
                        &format!("opt-level={}", optimisation_level),
                    ])
                    .stdin(Stdio::piped())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .unwrap_or_else(|e| {
                        println!("{}", e);
                        process::exit(4);
                    });
                child
                    .stdin
                    .as_mut()
                    .unwrap()
                    .write_all(transpiled.as_bytes())
                    .unwrap_or_else(|e| {
                        println!("{}", e);
                        process::exit(4);
                    });
                let status = child.wait().unwrap_or_else(|e| {
                    println!("{}", e);
                    process::exit(4);
                });

                if !status.success() {
                    process::exit(status.code().unwrap());
                }
            }
        }
        Ok(())
    }
}

fn main() {
    let opt = Opt::from_args();
    let result = opt.run();
    if let Err(e) = result {
        println!("{}", e);
        process::exit(1);
    }
}
