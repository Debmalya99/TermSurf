from flask import Flask

app = Flask(__name__)

with open("server_programs/prog1.rhai",'r') as rf:
    data = rf.read()

@app.route('/')
def index():
    return data

if __name__ == "__main__":
    app.run(host='0.0.0.0',port = 3000)