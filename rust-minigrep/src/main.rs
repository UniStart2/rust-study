use rust_minigrep::{parse_cmd_args, search};

fn main() {
    let args = parse_cmd_args();

    search(&args);
}
