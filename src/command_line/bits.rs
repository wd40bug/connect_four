pub fn bits(){
    if let Some(sub_arg) = std::env::args().nth(2) {
        if let Ok(num) = sub_arg.parse() {
            let bits = u64_to_bit_array(num);
            print_bits(bits);
        } else {
            match &sub_arg[..] {
                "seq" => {
                    if let Some(seq) = std::env::args().nth(3) {
                        print_seq(&seq);
                    } else {
                        let mut seq = String::new();
                        std::io::stdin().read_line(&mut seq).unwrap();
                        print_seq(&seq);
                    }
                }
                "eval" => {
                    let (num1, num2) =
                        if let Some(sub_sub_sub_arg) = std::env::args().nth(4) {
                            if let Ok(num1) = sub_sub_sub_arg.trim().parse() {
                                if let Some(num2_string) = std::env::args().nth(5) {
                                    if let Ok(num2) = num2_string.parse() {
                                        (num1, num2)
                                    } else {
                                        println!(
                                            "must provide a number for second argument"
                                        );
                                        std::process::exit(1)
                                    }
                                } else {
                                    println!("must provide a second argument");
                                    std::process::exit(1)
                                }
                            } else {
                                let mut num1 = String::new();
                                std::io::stdin().read_line(&mut num1).unwrap();
                                let mut num2 = String::new();
                                std::io::stdin().read_line(&mut num2).unwrap();
                                let num1: u64 = num1.trim().parse().unwrap();
                                let num2: u64 = num2.trim().parse().unwrap();
                                (num1, num2)
                            }
                        } else {
                            let mut num1 = String::new();
                            std::io::stdin().read_line(&mut num1).unwrap();
                            let mut num2 = String::new();
                            std::io::stdin().read_line(&mut num2).unwrap();
                            let num1: u64 = num1.trim().parse().unwrap();
                            let num2: u64 = num2.trim().parse().unwrap();
                            (num1, num2)
                        };
                    if let Some(sub_sub_arg) = std::env::args().nth(3) {
                        match &sub_sub_arg[..] {
                            "rs" => println!("{}", num1 >> num2),
                            "ls" => println!("{}", num1 << num2),
                            "xor" => println!("{}", num1 ^ num2),
                            "and" => println!("{}", num1 & num2),
                            "or" => println!("{}", num1 | num2),
                            _ => println!("not a valid operation"),
                        }
                    } else {
                        println!("need a sub argument")
                    }
                }
                _ => println!("not a known sub command of 'bits' "),
            }
        }
    } else {
        let mut num = String::new();
        std::io::stdin().read_line(&mut num).unwrap();
        println!("");
        let num: u64 = num.trim().parse().unwrap();
        let bits = u64_to_bit_array(num);
        print_bits(bits);
    }
}