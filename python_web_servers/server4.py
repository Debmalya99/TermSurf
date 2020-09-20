from flask import Flask

app = Flask(__name__)


@app.route('/')
def index():
    with open("server_programs/prog4.rhai",'r') as rf:
        data = rf.read()
    return data

@app.route('/<int:num>',methods=['GET'])
def process(num):
    return str(num**2)

if __name__ == "__main__":
    app.run(host='0.0.0.0',port = 4000)
