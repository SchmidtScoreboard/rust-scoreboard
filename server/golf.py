from common import Common, Team, SportId
from fetcher import Fetcher
import time
import asyncio
from dateutil.parser import parse
import datetime
import pytz

class Golf:

    def create_player(player):
        return {
            "display_name":  player["athlete"]["displayName"].split()[-1], # Get last name
            "position": int(player["status"]["position"]["id"]),
            "score": player["score"]["displayValue"], # +7, -8 etc
        }

    def create_game(common, game):
        if common is None:
            return None

        
        competition = game["competitions"][0]
        players = competition["competitors"]

        top_20 = [Golf.create_player(player) for player in players if int(player["status"]["position"]["id"]) < 20 ]

        
        return {
            "type": "Golf", 
            "common": common,
            "players": top_20,
            "name":  game["shortName"]
            }
    
    def create_golf_common(game):
        competition = game["competitions"][0]
        espn_status = competition["status"]["type"]["name"]
        status = Common.convert_status(espn_status)
        ordinal = Common.to_ordinal(competition["status"]["period"])
        game_id = game["id"]

        earliest_tee_time = None

        # Find start time by looking at players
        for player in competition["competitors"]:
            tee_time = player["status"].get("teeTime")
            if tee_time is None:
                continue
            tee_time = parse(tee_time).astimezone(pytz.utc)
            if earliest_tee_time is None or tee_time < earliest_tee_time:
                earliest_tee_time = tee_time

        empty_team = Team.create_team("0", "", "", "", "", "000000", "000000")

        if status is not None:
            return Common.create_common(
                SportId.GOLF.value,
                empty_team,
                empty_team,
                status,
                ordinal,
                str(earliest_tee_time),
                game_id,
                0,
                0 
            )
        else:
            return None

    async def get_games(testing: bool):
        if testing:
            return Common.get_testing_games("golf")
        else:
            raw_games = await Fetcher.espn_fetch("golf", None, "leaderboard?league=pga")
            games = [
                Golf.create_game(Golf.create_golf_common(game), game)
                for game in raw_games
            ]
            return [g for g in games if g]

async def main():
    print("Fetching games")
    print(await Golf.get_games(False))

if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    while True:
        loop.run_until_complete(main())
        time.sleep(60)
    loop.close()


