# TermSurf : Text Based Web Browser

## Building the browser
* **You need the full Rust toolchain along with cargo.** This can be obtained here: [Rust homepage](https://www.rust-lang.org/).
* Clone this repository somewhere in your machine.
* Enter the directory where you have cloned and enter the following commands:
* You will also need Python 3 and flask and python requests installed to run the servers. I used Python 3.7.6 anaconda.
```
cargo run
```
or if you just want to build:
```
cargo build
```
and running will build the browser. To test the servers go to the python_web_servers/ directory. You will find a requirements.txt file. Use pip to install necessary libraries using this requirements.txt file.
Say you want to run the server4:
* Go to python_web_servers/ directory:
Run:
```
python server4.py
```
and run the broswer as well using:
```
cargo run
```
and enter the following command:
```
connect>>http://127.0.0.1:4000
```

### This can run Rhai scripts fetched using http requests
## What is Rhai script?
Rhai is an embedded scripting language and evaluation engine for Rust that gives a safe and easy way to add scripting to any application.
[Link to Rhai Project on Github](https://github.com/jonathandturner/rhai)
## What this broswer does.
This browser can run http requests as of now, and can fetch .rhai programs from web servers. The browser internally has a script runtime engine that executes these scripts to do all sorts of good stuff?

### What good stuff?
Well for now the good feature (in my opinion) is that I can create menu driven programs in rhai that can be fetched via an http get request and loaded from memory into the browser. These .rhai programs in turn can run http get requests to access rest APIs as defined in the server itself.
**Example:** An example of this is the server4.py and the prog4.rhai in the python_web_servers/ and python_web_servers/server_programs/ folders. The backend for now uses flask for the servers, because I wanted the servers to be quick to set up and easy to test.
<br>
The server4 has two routes:
* **The index route:** When the browser requests this route, the prog4.rhai gets loaded, which you can think of as the index.html file in this case(except it contains logic as well). It is a menu driven program, that asks for your choice and behaves accordingly.
* **The rest api route:** This server's main functionality is to output a square of a number. This route is basically defined as  /<num> where num is a positive integer. The returned response is a string containing the square of the number. The prog4.rhai uses a get request to fetch the square of an integer(which will be entered by the user) and prints the square of that number as returned by the server.
