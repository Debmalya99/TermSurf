// use rhai::{ImmutableString};

//Request component
#[derive(Clone)]
pub struct CRequest{
    pub client:reqwest::blocking::Client,
}

impl CRequest{
    pub fn new()->Self{
        CRequest{
            client: reqwest::blocking::Client::new(),
        }
    }
    // The following are the methods for the scripting API
    pub fn get(&mut self,url:&str)->String{
        let resp_str = self.client.get(url).send().expect("Failed to make GET Request").text().expect("Failed to read response text");
        return resp_str;
    }
}
//I will not add any get or post or other http methods as they are provided by the reqwest api
