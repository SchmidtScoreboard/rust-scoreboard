from common import Common, Team
from fetcher import Fetcher
import time

team_map = {
    108: Team.createTeam("108", 'Los Angeles', "Angels", "Angels", "LAA", "ba0021", "c4ced4"),
    109: Team.createTeam("109", 'Arizona', "D-backs", "D-backs", "ARI", "a71930", "e3d4ad"),
    110: Team.createTeam("110", 'Baltimore', "Orioles", "Orioles", "BAL", "df4601", "27251f"),
    111: Team.createTeam("111", 'Boston', "Red Sox", "Red Sox", "BOS", "c6011f", "ffffff"),
    112: Team.createTeam("112", 'Chicago', "Cubs", "Cubs", "CHC", "0e3386", "cc3433"),
    113: Team.createTeam("113", 'Cincinnati', "Reds", "Reds", "CIN", "c6011f", "000000"),
    114: Team.createTeam("114", 'Cleveland', "Indians", "Indians", "CLE", "e31937", "0c2340"),
    115: Team.createTeam("115", 'Colorado', "Rockies", "Rockies", "COL", "33006f", "c4ced4"),
    116: Team.createTeam("116", 'Detroit', "Tigers", "Tigers", "DET", "0c2340", "fa4616"),
    117: Team.createTeam("117", 'Houston', "Astros", "Astros", "HOU", "002d62", "f4911e"),
    118: Team.createTeam("118", 'Kansas City', "Royals", "Royals", "KC", "004687", "bd9b60"),
    119: Team.createTeam("119", 'Los Angeles', "Dodgers", "Dodgers", "LAD", "005a9c", "ef3e42"),
    120: Team.createTeam("120", 'Washington', "Nationals", "Nationals", "WSH", "ab0003", "14225a"),
    121: Team.createTeam("121", 'New York', "Mets", "Mets", "NYM", "002d72", "fc5910"),
    133: Team.createTeam("133", 'Oakland', "Athletics", "Athletics", "OAK", "003831", "efb21e"),
    134: Team.createTeam("134", 'Pittsburgh', "Pirates", "Pirates", "PIT", "fdb827", "27251f"),
    135: Team.createTeam("135", 'San Diego', "Padres", "Padres", "SD", "002d62", "a2aaad"),
    136: Team.createTeam("136", 'Seattle', "Mariners", "Mariners", "SEA", "005c5c", "c4ced4"),
    137: Team.createTeam("137", 'San Francisco', "Giants", "Giants", "SF", "27251f", "fd5a1e"),
    138: Team.createTeam("138", 'St. Louis', "Cardinals", "Cardinals", "STL", "c41e3a", "0c2340"),
    139: Team.createTeam("139", 'Tampa Bay', "Rays", "Rays", "TB", "d65a24", "ffffff"),
    140: Team.createTeam("140", 'Texas', "Rangers", "Rangers", "TEX", "003278", "c0111f"),
    141: Team.createTeam("141", 'Toronto', "Blue Jays", "Blue Jays", "TOR", "134a8e", "b1b3b3"),
    142: Team.createTeam("142", 'Minnesota', "Twins", "Twins", "MIN", "002b5c", "d31145"),
    143: Team.createTeam("143", 'Philadelphia', "Phillies", "Phillies", "PHI", "e81828", "002d72"),
    144: Team.createTeam("144", 'Atlanta', "Braves", "Braves", "ATL", "13274f", "ce1141"),
    145: Team.createTeam("145", 'Chicago', "White Sox", "White Sox", "CWS", "27251f", "c4ced4"),
    146: Team.createTeam("146", 'Miami', "Marlins", "Marlins", "MIA", "000000", "00a3e0"),
    147: Team.createTeam("147", 'New York', "Yankees", "Yankees", "NYY", "0c2340", "ffffff"),
    158: Team.createTeam("158", 'Milkwaukee', "Brewers", "Brewers", "MIL", "13294b", "b6922e"),
    159: Team.createTeam("159", 'NL', "NL All Stars", "NL All Stars", "NL", "ff0000", "ffffff"),
    160: Team.createTeam("160", 'AL', "AL All Stars", "AL All Stars", "AL", "0000ff", "ffffff"),
}


class Baseball:
    def createGame(common, balls, outs, strikes, inning, is_inning_top):
        if common is None:
            return None
        return {"common": common, "balls": balls, "outs": outs, "strikes": strikes, "inning": inning, "is_inning_top" : is_inning_top}

    def getGames(testing: bool):
        if testing:
            return Common.get_testing_games("baseball")
        else:
            raw_games = Fetcher.schedule_fetch("http://statsapi.mlb.com/api/v1/schedule?sportId=1")
            games = [
                Common.from_schedule_json(game, team_map)
                for game in raw_games
            ]
            
            # TODO run this asynchronously
            complete_games = []
            for game in games:
                if game is None:
                    continue
                complete_games.append(Baseball.refresh_game(game))

            return {"games": g for g in complete_games if g}

    def refresh_game(game):
        data = Fetcher.game_fetch("http://statsapi.mlb.com/api/v1.1/game/" + str(game["id"]) + "/feed/live")
        linescore = data["liveData"]["linescore"]
        teams = linescore["teams"]
        away = teams["away"]
        home = teams["home"]
        game["away_score"] = away.get("runs", 0)
        game["home_score"] = home.get("runs", 0)
        inning = linescore.get("currentInning", 0)
        is_inning_top = linescore.get("isTopInning", False)

        state = data["gameData"]["status"]["abstractGameState"]
        if state == "Final":
            game["ordinal"] = "Final"
            game["status"] = "END"
        elif state == "Live":
            game["ordinal"] = linescore.get("currentInningOrdinal", "")
            game["status"] = "ACTIVE"
        elif state == "Preview":
            game["ordinal"] = ""
            game["status"] = "PREGAME"
        else:
            game["ordinal"] = "Stats Error"
            game["status"] = "INVALID"

        if game["status"] == "ACTIVE":
            balls = linescore["balls"]
            outs= linescore["outs"]
            strikes= linescore["strikes"]
            if(outs == 3):
                if inning >= 9 and ((is_inning_top and game["home_score"] > game["away_score"]) or (not is_inning_top and game["home_score"] != game["away_score"])):
                    print("Detected game end")
                    game["ordinal"] = "Final"
                    game["status"] = "END"
                else:
                    game["ordinal"] = "Middle " + game["ordinal"]
                    game["status"] = "INTERMISSION"
        return Baseball.createGame(game, balls, outs, strikes, inning, is_inning_top)


if __name__ == "__main__":
    while True:
        print("Fetching games")
        print(Baseball.getGames(False))
        time.sleep(60)


