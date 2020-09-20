//Component CLI(Command Line Interface)
pub struct CCLI{
    pub prompt:String,
    pub cmd_string:String,
}

impl CCLI{
    pub fn new()->CCLI{
        CCLI{
            prompt:String::from("connect>>"),
            cmd_string:String::from(""),   // A blank string to start with
        }
    }
}