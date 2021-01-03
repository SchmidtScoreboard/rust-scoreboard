import enum
from flask import Flask
import json
app = Flask(__name__)


class GameStatus(int, enum.Enum):
    INVALID = 0
    PREGAME = 1
    ACTIVE = 2
    INTERMISSION = 3
    END = 4

class Team:
    def __init__(self, id, display_name, abbreviation, primary_color, secondary_color):
        self.id = id
        self.display_name = display_name
        self.abbreviation = abbreviation
        self.primary_color = primary_color
        self.secondary_color = secondary_color

blues = Team(19, "Blues", "STL", "0x002f87", "0xffb81c")
vegas = Team(54, "Vegas", "VGK", "0x002f87", "0xffb81c")

    
class CommonGameData:
    def __init__(self, home_team, away_team, status, ordinal, start_time, id=0, home_score=0, away_score=0):
        self.home_team = home_team
        self.away_team = away_team
        self.home_score = home_score
        self.away_score = away_score
        self.status = status
        self.ordinal = ordinal
        self.start_time = start_time
        self.id = id

class Hockey:
    def __init__(self, common, away_powerplay=False, home_powerplay=False, away_players=5, home_players=5):
        self.common = common
        self.away_powerplay = away_powerplay
        self.home_powerplay = home_powerplay
        self.away_players = away_players
        self.home_players = home_players

class Baseball:
    def __init__(self, common, is_inning_top, balls, outs, strikes):
        self.common = common
        self.is_inning_top = is_inning_top
        self.balls = balls
        self.outs = outs
        self.strikes = strikes
    




@app.route('/nhl')
def nhl():
    return json.dumps(Hockey(CommonGameData(blues, vegas, GameStatus.ACTIVE, "1st", "")))

@app.route('/mlb')
def mlb():
    return get_baseball_games() 

def get_hockey_games():
    return "Hello World"

def get_baseball_games():
    return "Hello World"

if __name__ == "__main__":
    app.run(debug=True, host="0.0.0.0")