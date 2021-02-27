from flask import Flask
from college_basketball import CollegeBasketball
from basketball import Basketball
from hockey import Hockey
from baseball import Baseball

app = Flask(__name__)


@app.route("/college-basketball")
def college_basketball():
    return {"data": { "games" : CollegeBasketball.getGames(True)}}


@app.route("/basketball")
def basketball():
    return {"data": { "games" : Basketball.getGames(True)}}

@app.route("/nhl")
def hockey():
    return {"data": { "games" : Hockey.getGames(True)}}

@app.route("/mlb")
def baseball():
    return {"data": { "games" : Baseball.getGames(True)}}


if __name__ == "__main__":
    app.run(debug=True, host="0.0.0.0")
