


use connect_four::{
    run, command_line::{seq, bits},
};


fn main() {
    if let Some(arg) = std::env::args().nth(1) {
        match &arg[..] {
            "seq" => {
                seq();
            },
            "play" => {
                let pretty = if let Some(play_arg) = std::env::args().nth(2) {
                    if play_arg.eq("pretty") {
                        true
                    } else if play_arg.eq("debug") {
                        false
                    } else {
                        true
                    }
                } else if !std::env::var("PRETTY_OFF").is_err() {
                    false
                } else {
                    true
                };
                println!("sequence: {}", run(pretty));
            }
            "bits" => {
                bits();
            },
            _ => {
                println!("argument not recognized");
            }
        }
    } else {
        let pretty = if !std::env::var("PRETTY_OFF").is_err() {
            false
        } else {
            true
        };
        println!("sequence: {}", run(pretty));
    };
}

