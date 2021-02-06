from flask import Flask
from function import lambda_handler

app = Flask(__name__)


@app.route("/college-basketball")
def college_basketball():
    return lambda_handler("college-basketball")


if __name__ == "__main__":
    app.run(debug=True, host="0.0.0.0")
