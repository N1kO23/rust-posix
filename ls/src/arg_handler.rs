use std::path::Path;

use anyhow::{anyhow, Result};
pub struct ArgHandler {
    args: Args,
    env_args: Vec<String>,
    index: usize,
    args_parsed: bool,
    is_path_set: bool,
}

#[derive(Clone)]
pub struct Args {
    pub path: Box<Path>,
    pub long: bool,
    pub all: bool,
}

impl ArgHandler {
    pub fn new() -> ArgHandler {
        let env_args: Vec<String> = std::env::args().collect();
        let args = Args {
            path: Path::new(".").into(),
            long: false,
            all: false,
        };
        return Self {
            args,
            env_args,
            index: 1,
            args_parsed: false,
            is_path_set: false,
        };
    }

    pub fn get_args(&mut self) -> Result<Args> {
        if !self.args_parsed {
            while self.has_next() {
                match self.execute_next() {
                    Ok(args) => args,
                    Err(err) => {
                        return Err(anyhow!(
                            "An error occurred while parsing arguments: {}",
                            err
                        ))
                    }
                }
            }
        }

        return Ok(self.args.clone());
    }

    fn execute_next(&mut self) -> Result<()> {
        let command = self.env_args[self.index].clone();
        match command.as_str() {
            "-l" => {
                // let mod_id_vec = self.args_into_i32()?;
                self.index += 1;
            }
            "-a" => {
                self.index += 1;
                // if self.index >= self.args.len() {
                //     std::process::exit(0);
                // } else {
                //     let mod_name = self.join_args_string();
                //     self.index += 1;
                // }
            }
            "help" => {
                self.index += 1;
                self.help();
                std::process::exit(0);
            }
            any => {
                if self.is_path_set {
                    self.help();
                    return Err(anyhow!("Path is already set!"));
                } else {
                    self.args.path = Path::new(any).into();
                    self.index += 1;
                }
            }
        }
        Ok(())
    }

    fn help(&self) {
        println!("{}", "Help");
        println!(
            "{}",
            "install <mod_id> - Installs the specified mod into prefix"
        );
        println!("{}", "update - Updates the local list of available mods");
        println!(
            "{}",
            "list - Lists all the mods that are currently downloaded and available"
        );
        println!(
            "{}",
            "search <mod_id> - Searches for the specified mod on curseforge"
        );
        println!("{}", "help - Displays this help message and exits program");
    }

    fn has_next(&self) -> bool {
        return self.index < self.env_args.len();
    }

    fn join_args_string(&mut self) -> String {
        let mut joined_args = String::new();
        while self.has_next() {
            joined_args.push_str(&self.env_args[self.index]);
            self.index += 1;
            if self.has_next() {
                joined_args.push_str(" ");
            }
        }
        return joined_args;
    }

    fn args_into_i32(&mut self) -> Result<Vec<i32>> {
        let mut args_i32 = Vec::new();
        while self.has_next() {
            let num = self.env_args[self.index].clone().parse::<i32>()?;
            args_i32.push(num);
            self.index += 1;
        }
        return Ok(args_i32);
    }
}
