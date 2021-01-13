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

def getTeam(id, display_name, abbreviation, primary_color, secondary_color):
    return {
        "id": id,
        "display_name": display_name,
        "abbreviation": abbreviation,
        "primary_color": primary_color,
        "secondary_color": secondary_color,
    }

blues = getTeam("19", "Blues", "STL", "002f87", "ffb81c")
vegas = getTeam("54", "Vegas", "VGK", "002f87", "ffb81c")

    
def getCommonGameData(home_team, away_team, status, ordinal, start_time, id=0, home_score=0, away_score=0):
    return {"home_team": home_team, 
        "away_team": away_team,
        "home_score": home_score,
        "away_score": away_score,
        "status": status,
        "ordinal": ordinal,
        "start_time": start_time,
        "id": id}

def getHockey(common, away_powerplay=False, home_powerplay=False, away_players=5, home_players=5):
    return {
        "common":  common,
        "away_powerplay": away_powerplay,
        "home_powerplay": home_powerplay,
        "away_players": away_players,
        "home_players": home_players,
    }

def getBaseball(common, is_inning_top=False, balls=0, outs=0, strikes=0, inning=1):
    return{
        "common":common, 
        "is_inning_top": is_inning_top,
        "balls": balls,
        "outs": outs,
        "strikes": strikes,
        "inning": inning
    } 
    




@app.route('/nhl')
def nhl():
    return get_hockey_games() 

@app.route('/mlb')
def mlb():
    return get_baseball_games() 

def get_hockey_games():
    return {"data": {"games": [getHockey(getCommonGameData(blues, vegas, "PREGAME", "1st", "2020-08-09T19:00:00Z"), True)]}}

def get_baseball_games():
    return {"data": {"games": [getBaseball(getCommonGameData(blues, vegas, "ACTIVE", "1st", "2020-08-09T19:00:00Z"))]}}

if __name__ == "__main__":
    app.run(debug=True, host="0.0.0.0")