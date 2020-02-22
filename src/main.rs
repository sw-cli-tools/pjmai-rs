use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("emacs pjm1 rtt1");
        std::process::exit(0);
    } else {
        match args[1].as_ref() {
            "emacs" => println!("/home/mike/.emacs.d/mw-emacs"),
            "pjm1" => println!("/home/mike/github/wrightmikea/pjm1"),
            "rtt1" => println!("/home/mike/github/wrightmikea/rtt1"),
            _ => println!("/home/mike"),
        }
        std::process::exit(2)
    }
}
