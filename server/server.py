from flask import Flask
from college_basketball import CollegeBasketball


app = Flask(__name__)


@app.route("/college_basketball")
def college_basketball():
    return CollegeBasketball.(True)


@app.route("/mlb")
def mlb():
    return get_baseball_games(True)


if __name__ == "__main__":
    app.run(debug=True, host="0.0.0.0")
