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
            return Common.get_testing_games("college-basketball")
        else:
            raw_games = Fetcher.espn_fetch("basketball", "mens-college-basketball")
            games = [
                CollegeBasketball.createGame(
                    Common.from_espn_json(game, Team.getESPNTeam, team_map)
                )
                for game in raw_games
            ]
            return {"games": [g for g in games if g]}


if __name__ == "__main__":
    print("Fetching games")
    print(CollegeBasketball.getGames(False))
