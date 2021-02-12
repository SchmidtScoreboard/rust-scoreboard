from common import Common, Team
from fetcher import Fetcher
from ncaa import team_map


class CollegeBasketball:
    def createGame(common):
        if common is None:
            return None
        return {"common": common}

    def getGames(testing: bool):
        if testing:
            return {"games": []}
        else:
            raw_games = Fetcher.fetch("basketball", "mens-college-basketball")
            games = [
                CollegeBasketball.createGame(
                    Common.from_json(game, Team.getTeam, team_map)
                )
                for game in raw_games
            ]
            return {"games": [g for g in games if g]}


if __name__ == "__main__":
    print("Fetching games")
    print(CollegeBasketball.getGames(False))
