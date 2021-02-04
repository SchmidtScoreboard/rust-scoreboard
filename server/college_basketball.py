from common import Common, Team
from fetcher import Fetcher

class CollegeBasketball:
    def createGame(common):
        return { "common" : common }

    def getTeam(team_id: int, game):
        # TODO assemble a team map of all the D1 teams. Have an else statement
        # that will parse from the list
        return Team.createTeam(team_id, "Illini", "ILL", "ORANGE", "BLUE")

    def getGames(testing: bool):
        if testing:
            return { "games": []}
        else:
            raw_games = Fetcher.fetch("college_basketball")
            games = [createGame(Common.from_json(game, getTeam)) for game in
                    raw_games]
            return { "games": games}
