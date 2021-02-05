from common import Common, Team
from fetcher import Fetcher

class CollegeBasketball:
    def createGame(common):
        return { "common" : common }

    def getTeam(team_id, competitor):
        # TODO assemble a team map of all the D1 teams. Have an else statement
        # that will parse from the list
        team_map = {}
        if team_id in team_map:
            pass
        else:
            return Team.createTeam(team_id, competitor["team"]["name"],
                    competitor["team"]["abbreviation"], "000000", "ffffff")

    def getGames(testing: bool):
        if testing:
            return { "games": []}
        else:
            raw_games = Fetcher.fetch("basketball", "mens-college-basketball")
            games = [CollegeBasketball.createGame(Common.from_json(game,
                CollegeBasketball.getTeam)) for game in
                    raw_games]
            return { "games": games}

if __name__ == '__main__':
    print(CollegeBasketball.getGames(False))
