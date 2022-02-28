use connect_four::{run};


fn main(){
    println!("{:?}",std::env::args().nth(1));
    let pretty = if let Some(arg) = std::env::args().nth(1){
        if arg.eq("debug"){
            false
        } else if arg.eq("pretty"){
            true
        } else{
            true
        }
    }else if !std::env::var("PRETTY_OFF").is_err(){
        false
    }else{
        true
    };
    println!("sequence: {}",run(pretty));
}