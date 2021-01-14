
from common import getCommonGameData, getTeam

def getBaseball(common, is_inning_top=False, balls=0, outs=0, strikes=0, inning=1):
    return{
        "common":common, 
        "is_inning_top": is_inning_top,
        "balls": balls,
        "outs": outs,
        "strikes": strikes,
        "inning": inning
    } 

def get_baseball_games(testing: bool):
    if testing:
        return {"data": {"games": []}}
    else:
        # TODO fetch data
        return {"data": {"games": []}}
