mod parseargs;

fn main() {
    println!("Hello World");
    let mut args = parseargs::ConfigOptions::new();
    args.parse_cmd_args();
}
