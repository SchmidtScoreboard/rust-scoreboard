from hockey import get_hockey_games
from baseball import get_baseball_games
from flask import Flask


app = Flask(__name__)


@app.route("/nhl")
def nhl():
    return get_hockey_games(True)


@app.route("/mlb")
def mlb():
    return get_baseball_games(True)


if __name__ == "__main__":
    app.run(debug=True, host="0.0.0.0")