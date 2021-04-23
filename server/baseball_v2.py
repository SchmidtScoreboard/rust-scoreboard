from common import Common, Team, SportId
from fetcher import Fetcher
import time
import asyncio

team_map = {
    108: Team.create_team("108", 'Los Angeles', "Angels", "Angels", "LAA", "ba0021", "c4ced4"),
    109: Team.create_team("109", 'Arizona', "D-backs", "D-backs", "ARI", "a71930", "e3d4ad"),
    110: Team.create_team("110", 'Baltimore', "Orioles", "Orioles", "BAL", "df4601", "27251f"),
    111: Team.create_team("111", 'Boston', "Red Sox", "Red Sox", "BOS", "c6011f", "ffffff"),
    112: Team.create_team("112", 'Chicago', "Cubs", "Cubs", "CHC", "0e3386", "cc3433"),
    113: Team.create_team("113", 'Cincinnati', "Reds", "Reds", "CIN", "c6011f", "000000"),
    114: Team.create_team("114", 'Cleveland', "Indians", "Indians", "CLE", "e31937", "0c2340"),
    115: Team.create_team("115", 'Colorado', "Rockies", "Rockies", "COL", "33006f", "c4ced4"),
    116: Team.create_team("116", 'Detroit', "Tigers", "Tigers", "DET", "0c2340", "fa4616"),
    117: Team.create_team("117", 'Houston', "Astros", "Astros", "HOU", "002d62", "f4911e"),
    118: Team.create_team("118", 'Kansas City', "Royals", "Royals", "KC", "004687", "bd9b60"),
    119: Team.create_team("119", 'Los Angeles', "Dodgers", "Dodgers", "LAD", "005a9c", "ef3e42"),
    120: Team.create_team("120", 'Washington', "Nationals", "Nationals", "WSH", "ab0003", "14225a"),
    121: Team.create_team("121", 'New York', "Mets", "Mets", "NYM", "002d72", "fc5910"),
    133: Team.create_team("133", 'Oakland', "Athletics", "Athletics", "OAK", "003831", "efb21e"),
    134: Team.create_team("134", 'Pittsburgh', "Pirates", "Pirates", "PIT", "fdb827", "27251f"),
    135: Team.create_team("135", 'San Diego', "Padres", "Padres", "SD", "002d62", "a2aaad"),
    136: Team.create_team("136", 'Seattle', "Mariners", "Mariners", "SEA", "005c5c", "c4ced4"),
    137: Team.create_team("137", 'San Francisco', "Giants", "Giants", "SF", "27251f", "fd5a1e"),
    138: Team.create_team("138", 'St. Louis', "Cardinals", "Cardinals", "STL", "c41e3a", "0c2340"),
    139: Team.create_team("139", 'Tampa Bay', "Rays", "Rays", "TB", "d65a24", "ffffff"),
    140: Team.create_team("140", 'Texas', "Rangers", "Rangers", "TEX", "003278", "c0111f"),
    141: Team.create_team("141", 'Toronto', "Blue Jays", "Blue Jays", "TOR", "134a8e", "b1b3b3"),
    142: Team.create_team("142", 'Minnesota', "Twins", "Twins", "MIN", "002b5c", "d31145"),
    143: Team.create_team("143", 'Philadelphia', "Phillies", "Phillies", "PHI", "e81828", "002d72"),
    144: Team.create_team("144", 'Atlanta', "Braves", "Braves", "ATL", "13274f", "ce1141"),
    145: Team.create_team("145", 'Chicago', "White Sox", "White Sox", "CWS", "27251f", "c4ced4"),
    146: Team.create_team("146", 'Miami', "Marlins", "Marlins", "MIA", "000000", "00a3e0"),
    147: Team.create_team("147", 'New York', "Yankees", "Yankees", "NYY", "0c2340", "ffffff"),
    158: Team.create_team("158", 'Milkwaukee', "Brewers", "Brewers", "MIL", "13294b", "b6922e"),
    159: Team.create_team("159", 'NL', "NL All Stars", "NL All Stars", "NL", "ff0000", "ffffff"),
    160: Team.create_team("160", 'AL', "AL All Stars", "AL All Stars", "AL", "0000ff", "ffffff"),
}


class Baseball:
    def create_game(common, game):
        competition = game["competitions"][0]
        situation = competition.get("situation")
        balls = situation["balls"] if situation is not None else 0
        strikes = situation["strikes"] if situation is not None else 0
        outs = situation["outs"] if situation is not None else 0

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
            "is_inning_top" : is_inning_top,
            "onFirst": situation["onFirst"] if situation is not None else False,
            "onSecond": situation["onSecond"] if situation is not None else False,
            "onThird": situation["onThird"] if situation is not None else False,
        }
    async def get_games(testing: bool):
        if testing:
            return Common.get_testing_games("baseball")
        else:
            raw_games = await Fetcher.espn_fetch("baseball", "mlb")
            games = [
                Baseball.create_game(
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
    print(await Baseball.get_games(False))


if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    while True:
        loop.run_until_complete(main())
        time.sleep(60)
    loop.close()
