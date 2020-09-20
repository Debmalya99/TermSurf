mod browser_system;
use browser_system::*;
use crate::browser_system::module_request::*;
use crate::browser_system::module_cli::*;
use crate::browser_system::module_script::*;

fn main(){
    let request: CRequest = CRequest::new();
    let mut scriptrt:CScriptRuntime = CScriptRuntime::new();
    let mut cli:CCLI = CCLI::new();
    let mut running = true;
    browser_system(&mut cli,&request,&mut scriptrt,&mut running);
}