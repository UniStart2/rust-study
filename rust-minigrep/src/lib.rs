use std::{
    env,
    fmt::{self, Display},
    fs, process,
};

#[derive(Debug)]
pub struct CmdArgs {
    query_string: String,
    file_path: String,
}

impl CmdArgs {
    fn new(args: Vec<String>) -> CmdArgs {
        let query_string = args[1].clone();
        let file_path = args[2].clone();

        CmdArgs {
            query_string,
            file_path,
        }
    }
}

fn check(args: &Vec<String>) {
    if args.len() != 3 {
        eprintln!("必须传入两个参数：");
        eprintln!("格式：cargo run arg1[待查询字符串], arg2[待查询文本路径，该路径必须存在] ");
        process::exit(1);
    }

    let read_failed = fs::read(&args[2]).is_err();
    if read_failed {
        eprintln!("file not found in the path[arg2]");
        process::exit(1);
    }
}

pub fn parse_cmd_args() -> CmdArgs {
    let args: Vec<String> = env::args().collect();
    check(&args);

    CmdArgs::new(args)
}

pub fn search(cmd_args: &CmdArgs) {
    let file = fs::read_to_string(&cmd_args.file_path);

    match file {
        Ok(file) => {
            for line in file.lines() {
                if line.contains(&cmd_args.query_string) {
                    println!("{}", line);
                }
            }
        }
        Err(e) => {
            print!("{:?}", e);
            process::exit(1);
        }
    }
}
