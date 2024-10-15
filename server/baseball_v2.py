from common import Common, Team, SportId, pretty_print
from fetcher import Fetcher
import time
import asyncio

team_map = {
    "29": Team.create_team("29", "Arizona", "Diamondbacks", "D-backs", "ARI", "a40013", "ffffff"),
    "15": Team.create_team("15", "Atlanta", "Braves", "Braves", "ATL", "002248", "f1f2f3"),
    "1": Team.create_team("1", "Baltimore", "Orioles", "Orioles", "BAL", "201b1b", "ffffff"),
    "2": Team.create_team("2", "Boston", "Red Sox", "Red Sox", "BOS", "00224b", "ffffff"),
    "16": Team.create_team("16", "Chicago", "Cubs", "Cubs", "CHC", "00417d", "ffffff"),
    "4": Team.create_team("4", "Chicago", "White Sox", "White Sox", "CHW", "1b1516", "c4ced4"),
    "17": Team.create_team("17", "Cincinnati", "Reds", "Reds", "CIN", "c41422", "ffffff"),
    "5": Team.create_team("5", "Cleveland", "Guardians", "Guardians", "CLE", "00264e", "ffffff"),
    "27": Team.create_team("27", "Colorado", "Rockies", "Rockies", "COL", "220d48", "ffffff"),
    "6": Team.create_team("6", "Detroit", "Tigers", "Tigers", "DET", "002d5c", "ff6600"),
    "18": Team.create_team("18", "Houston", "Astros", "Astros", "HOU", "000000", "eb6e1f"),
    "7": Team.create_team("7", "Kansas City", "Royals", "Royals", "KC", "003b72", "7ab2dd"),
    "3": Team.create_team("3", "Los Angeles", "Angels", "Angels", "LAA", "a50017", "ffffff"),
    "19": Team.create_team("19", "Los Angeles", "Dodgers", "Dodgers", "LAD", "00337f", "a2aaad"),
    "28": Team.create_team("28", "Miami", "Marlins", "Marlins", "MIA", "0081c7", "000000"),
    "8": Team.create_team("8", "Milwaukee", "Brewers", "Brewers", "MIL", "050C33", "f1f2f3"),
    "9": Team.create_team("9", "Minnesota", "Twins", "Twins", "MIN", "012756", "f1f2f3"),
    "21": Team.create_team("21", "New York", "Mets", "Mets", "NYM", "00407b", "ffffff"),
    "10": Team.create_team("10", "New York", "Yankees", "Yankees", "NYY", "011739", "c4ced4"),
    "11": Team.create_team("11", "Oakland", "Athletics", "Athletics", "OAK", "014326", "efb21e"),
    "22": Team.create_team("22", "Philadelphia", "Phillies", "Phillies", "PHI", "be0011", "ffffff"),
    "23": Team.create_team("23", "Pittsburgh", "Pirates", "Pirates", "PIT", "111111", "ffffff"),
    "25": Team.create_team("25", "San Diego", "Padres", "Padres", "SD", "2f241d", "ffc425"),
    "26": Team.create_team("26", "San Francisco", "Giants", "Giants", "SF", "161415", "ffffff"),
    "12": Team.create_team("12", "Seattle", "Mariners", "Mariners", "SEA", "012a5b", "ffffff"),
    "24": Team.create_team("24", "St. Louis", "Cardinals", "Cardinals", "STL", "b80220", "ffffff"),
    "30": Team.create_team("30", "Tampa Bay", "Rays", "Rays", "TB", "002454", "8fbce6"),
    "13": Team.create_team("13", "Texas", "Rangers", "Rangers", "TEX", "003879", "ffffff"),
    "14": Team.create_team("14", "Toronto", "Blue Jays", "Blue Jays", "TOR", "0069ac", "ffffff"),
    "20": Team.create_team("20", "Washington", "Nationals", "Nationals", "WSH", "0a295d", "f1f2f3"),
}


class Baseball_v2:
    def create_game(common, game):
        competition = game["competitions"][0]
        situation = competition.get("situation")
        balls = situation.get("balls", 0) if situation is not None else 0
        strikes = situation.get("strikes", 0) if situation is not None else 0
        outs = situation.get("outs", 0) if situation is not None else 0
        # Delayed game, clear ordinal
        if game["status"]["type"]["name"] == "STATUS_DELAYED":
            common["ordinal"] = ""

        inning = competition["status"]["period"]
        is_inning_top = "Top" in competition["status"]["type"]["shortDetail"]
        if common is None:
            return None
        return {"type": "Baseball",
                "common": common,
                "balls": balls,
                "outs": outs,
                "strikes": strikes,
                "inning": inning,
                "is_inning_top": is_inning_top,
                "on_first": situation["onFirst"] if situation is not None else False,
                "on_second": situation["onSecond"] if situation is not None else False,
                "on_third": situation["onThird"] if situation is not None else False,
                }

    async def get_games(testing: bool):
        if testing:
            return Common.get_testing_games("baseball")
        else:
            raw_games = await Fetcher.espn_fetch("baseball", "mlb")
            games = [
                Baseball_v2.create_game(
                    Common.from_espn_json(
                        game, Team.get_espn_team, team_map, SportId.BASEBALL
                    ),
                    game
                )
                for game in raw_games
            ]
            return [g for g in games if g]


async def main():
    print("Fetching games")
    pretty_print(await Baseball_v2.get_games(False))


if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    while True:
        loop.run_until_complete(main())
        time.sleep(60)
    loop.close()
