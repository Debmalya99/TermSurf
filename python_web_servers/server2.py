from flask import Flask

app = Flask(__name__)



@app.route('/')
def index():
    with open("server_programs/prog2.rhai",'r') as rf:
        data = rf.read()
    return data

if __name__ == "__main__":
    app.run(host='0.0.0.0',port = 5000)