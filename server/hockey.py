from common import Common, Team, SportId, pretty_print
from fetcher import Fetcher
import time
import asyncio

# .*: \{ id: (.*), city: (.*), name: (.*), display_name: (.*), abbreviation: (.*), primary_color: (.*), secondary_color: (.*) \},
team_map = {
    1: Team.create_team("1", "New Jersey", "Devils", "Devils", "NJD", "c8102e", "000000"),
    2: Team.create_team("2", "New York", "Islanders", "Islanders", "NYI", "003087", "fc4c02"),
    3: Team.create_team("3", "New York", "Rangers", "Rangers", "NYR", "0033a0", "c8102e"),
    4: Team.create_team("4", "Philadelphia", "Flyers", "Flyers", "PHI", "fa4616", "000000"),
    5: Team.create_team("5", "Pittsburgh", "Penguins", "Penguins", "PIT", "ffb81c", "000000"),
    6: Team.create_team("6", "Boston", "Bruins", "Bruins", "BOS", "fcb514", "000000"),
    7: Team.create_team("7", "Buffalo", "Sabres", "Sabres", "BUF", "002654", "fcb514"),
    8: Team.create_team("8", "Montréal", "Canadiens", "Canadiens", "MTL", "a6192e", "001e62"),
    9: Team.create_team("9", "Ottawa", "Senators", "Senators", "OTT", "c8102e", "c69214"),
    10: Team.create_team("10", "Toronto", "Maple Leafs", "Leafs", "TOR", "00205b", "ffffff"),
    12: Team.create_team("12", "Carolina", "Hurricanes", "Canes", "CAR", "cc0000", "ffffff"),
    13: Team.create_team("13", "Florida", "Panthers", "Panthers", "FLA", "041e42", "b9975b"),
    14: Team.create_team("14", "Tampa Bay", "Lightning", "Lightning", "TBL", "00205b", "ffffff"),
    15: Team.create_team("15", "Washington", "Capitals", "Capitals", "WSH", "041e42", "c8102e"),
    16: Team.create_team("16", "Chicago", "Blackhawks", "B Hawks", "CHI", "ce1126", "000000"),
    17: Team.create_team("17", "Detroit", "Red Wings", "Red Wings", "DET", "c8102e", "ffffff"),
    18: Team.create_team("18", "Nashville", "Predators", "Predators", "NSH", "ffb81c", "041e42"),
    19: Team.create_team("19", "St. Louis", "Blues", "Blues", "STL", "002f87", "ffb81c"),
    20: Team.create_team("20", "Calgary", "Flames", "Flames", "CGY", "ce1126", "f3bc52"),
    21: Team.create_team("21", "Colorado", "Avalanche", "Avalanche", "COL", "236192", "d94574"),
    22: Team.create_team("22", "Edmonton", "Oilers", "Oilers", "EDM", "fc4c02", "041e42"),
    23: Team.create_team("23", "Vancouver", "Canucks", "Canucks", "VAN", "00843D", "ffffff"),
    24: Team.create_team("24", "Anaheim", "Ducks", "Ducks", "ANA", "b5985a", "ffffff"),
    25: Team.create_team("25", "Dallas", "Stars", "Stars", "DAL", "006341", "a2aaad"),
    26: Team.create_team("26", "Los Angeles", "Kings", "Kings", "LAK", "a2aaad", "000000"),
    28: Team.create_team("28", "San Jose", "Sharks", "Sharks", "SJS", "006272", "e57200"),
    29: Team.create_team("29", "Columbus", "Blue Jackets", "B Jackets", "CBJ", "041e42", "c8102e"),
    30: Team.create_team("30", "Minnesota", "Wild", "Wild", "MIN", "154734", "a6192e"),
    52: Team.create_team("52", "Winnipeg", "Jets", "Jets", "WPG", "041e42", "a2aaad"),
    53: Team.create_team("53", "Arizona", "Coyotes", "Coyotes", "ARI", "8c2633", "e2d6b5"),
    54: Team.create_team("54", "Las Vegas", "Golden Knights", "Knights", "VGK", "B4975A", "000000"),
    55: Team.create_team("55", "Seattle", "Kraken", "Kraken", "SEA", "001628", "99D9D9"),
    87: Team.create_team("87", "Atlantic", "Atlantic All Stars", "Atlantic", "ATL", "fa1b1b", "000000"),
    88: Team.create_team("88", "Metropolitan", "Metropolitan All Stars", "Metro", "MET", "fae71b", "000000"),
    89: Team.create_team("89", "Central", "Central All Stars", "Central", "CEN", "1411bd", "000000"),
    90: Team.create_team("90", "Pacific", "Pacific All Stars", "Pacific", "PAC", "11bd36", "000000"),
    7460: Team.create_team("7460", "Canada", "Canadian All Stars", "Canada", "CA", "d11717", "ffffff"),
    7461: Team.create_team("7461", "America", "American All Stars", "America", "USA", "3271a8", "ffffff"),
}


class Hockey:
    def create_game(common, away_powerplay, home_powerplay, away_players, home_players):
        if common is None:
            return None
        return {"type": "Hockey", "common": common, "away_powerplay": away_powerplay, "home_powerplay": home_powerplay, "away_players": away_players, "home_players": home_players}

    async def get_games(testing: bool):
        if testing:
            return Common.get_testing_games("hockey")
        else:
            raw_games = await Fetcher.schedule_fetch("http://statsapi.web.nhl.com/api/v1/schedule")
            games = [
                Common.from_schedule_json(game, team_map, SportId.HOCKEY)
                for game in raw_games
            ]

            group = asyncio.gather(*[Hockey.refresh_game(game)
                                   for game in games if game])
            return await group

    async def refresh_game(game):
        data = await Fetcher.game_fetch("http://statsapi.web.nhl.com/api/v1/game/" + str(game["id"]) + "/linescore")
        teams = data["teams"]
        away = teams["away"]
        home = teams["home"]
        game["away_score"] = away.get("goals", 0)
        game["home_score"] = home.get("goals", 0)
        away_powerplay = away["powerPlay"]
        home_powerplay = home["powerPlay"]
        away_players = max(away["numSkaters"], 0)
        home_players = max(home["numSkaters"], 0)
        period = data["currentPeriod"]

        period_time = data.get("currentPeriodTimeRemaining", "20:00")
        if period >= 1:
            game["ordinal"] = data.get("currentPeriodOrdinal", "1st")

        status = "INVALID"
        if period_time == "Final":
            status = "END"
        elif period_time == "END":
            if period >= 3 and game["away_score"] != game["home_score"]:
                status = "END"
            else:
                status = "INTERMISSION"
                game["ordinal"] += " INT"
        elif period_time == "20:00" and period > 1:
            status = "INTERMISSION"
            game["ordinal"] += " INT"
        elif period_time != "20:00" and period >= 1:
            status = "ACTIVE"
        else:
            status = "PREGAME"

        game["status"] = status
        game = Hockey.create_game(
            game, away_powerplay, home_powerplay, away_players, home_players)
        return game


async def main():
    print("Fetching games")
    pretty_print(await Hockey.get_games(False))

if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    while True:
        loop.run_until_complete(main())
        time.sleep(60)
    loop.close()
