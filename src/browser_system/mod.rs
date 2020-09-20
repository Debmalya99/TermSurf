pub mod module_request;
use module_request::*;

pub mod module_script;
use module_script::*;

pub mod module_cli;
use module_cli::*;

use std::io;
use std::io::Write;

use rhai::RegisterFn;

pub fn browser_system(cli:&mut CCLI,req:&CRequest,script:&mut CScriptRuntime,running:&mut bool){
    // First we register the functions
    script.engine.register_fn("getline",get_string);

    script.engine.register_type::<CRequest>();
    script.engine.register_fn("new_client",CRequest::new);
    script.engine.register_fn("get",CRequest::get);
    script.engine.register_fn("prompt",prompt);
    script.engine.register_fn("get_int",get_int);

    // The event loop
    while *running{
        // Set running to true in main
        print!("connect>>");
        io::stdout().flush().unwrap();  // Flush the prompt
        io::stdin().read_line(&mut cli.cmd_string).expect("Failed to read line from terminal");
        if cli.cmd_string.eq("exit\n"){
            *running = false;
            continue;
        }
        script.source = req.client.get(&cli.cmd_string).send().expect("Failed to make request").text().expect("Failed to read response text");
        script.engine.eval::<()>(&script.source).expect("Failed to evaluate script");

        cli.cmd_string = String::new(); // Basically clear the string
    }
}
