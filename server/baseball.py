from common import Common, Team
from fetcher import Fetcher
import time

team_map = {
}


class Baseball:
    def createGame(common):
        if common is None:
            return None
        return {"common": common}

    def getGames(testing: bool):
        if testing:
            return Common.get_testing_games("baseball")
        else:
            raw_games = Fetcher.fetch("baseball", "mlb")
            games = [
                Baseball.createGame(Common.from_json(game, Team.getTeam, team_map))
                for game in raw_games
            ]
            return {"games": [g for g in games if g]}


if __name__ == "__main__":
    while True:
        print("Fetching games")
        print(Baseball.getGames(False))
        time.sleep(60)

