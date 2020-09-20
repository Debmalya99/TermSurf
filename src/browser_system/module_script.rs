use std::io;
use std::io::Write;

use rhai::{Engine};

//Component script
pub struct CScriptRuntime{
    pub engine:Engine,
    pub source:String,
}

impl CScriptRuntime{
    pub fn new()->CScriptRuntime{
        CScriptRuntime{
            engine:Engine::new(),
            source:String::new()
        }
    }
}

// Functions to integrate in the script
pub fn get_string()->String{
    let mut in_string = String::new();
    io::stdin().read_line(&mut in_string).expect("Could not read line");
    in_string = in_string.trim().parse::<String>().expect("Failed to parse string");
    in_string
}

pub fn prompt(s:&str){
    print!("{}",s);
    io::stdout().flush().unwrap();
}

pub fn get_int()->i64{
    let mut in_str = String::new();
    io::stdin().read_line(&mut in_str).expect("Could to read line");
    let num:i64 = in_str.trim().parse::<i64>().expect("Failed to parse integer");
    num
}
