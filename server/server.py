from flask import Flask
from college_basketball import CollegeBasketball
from basketball import Basketball
from hockey import Hockey
from baseball import Baseball
from college_football import CollegeFootball
from football import Football
from all_sports import All
from golf import Golf
import asyncio
import argparse

parser = argparse.ArgumentParser(description="Run a testing server")
parser.add_argument("--fake", help="Run with fake data")
args = parser.parse_args()

testing = args.fake

loop = asyncio.get_event_loop()
app = Flask(__name__)


@app.route("/college-basketball")
def college_basketball():
    return {
        "data": {"games": loop.run_until_complete(CollegeBasketball.get_games(testing))}
    }


@app.route("/college-football")
def college_football():
    return {
        "data": {"games": loop.run_until_complete(CollegeFootball.get_games(testing))}
    }


@app.route("/all")
def all():
    return {"data": {"games": loop.run_until_complete(All.get_games_v1(testing))}}

@app.route("/all_v2")
def all_v2():
    return {"data": {"games": loop.run_until_complete(All.get_games_v2(testing))}}


@app.route("/basketball")
def basketball():
    return {"data": {"games": loop.run_until_complete(Basketball.get_games(testing))}}


@app.route("/football")
def football():
    return {"data": {"games": loop.run_until_complete(Football.get_games(testing))}}


@app.route("/nhl")
def hockey():
    return {"data": {"games": loop.run_until_complete(Hockey.get_games(testing))}}


@app.route("/mlb")
def baseball():
    return {"data": {"games": loop.run_until_complete(Baseball.get_games(testing))}}
@app.route("/golf")
def golf():
    return {"data": {"games": loop.run_until_complete(Golf.get_games(testing))}}


if __name__ == "__main__":
    app.run(debug=True, host="0.0.0.0")
