from common import Common, Team, SportId
from fetcher import Fetcher
import time
import asyncio

team_map = {
    "1": Team.create_team("1", "Atlanta", "Hawks", "Hawks", "ATL", "002B5C", "ffffff"),
    "2": Team.create_team(
        "2", "Boston", "Celtics", "Celtics", "BOS", "006532", "f1f2f3"
    ),
    "17": Team.create_team("17", "Brooklyn", "Nets", "Nets", "BKN", "000000", "ffffff"),
    "30": Team.create_team(
        "30", "Charlotte", "Hornets", "Hornets", "CHA", "1D1060", "008ca8"
    ),
    "4": Team.create_team("4", "Chicago", "Bulls", "Bulls", "CHI", "ce1141", "000000"),
    "5": Team.create_team(
        "5", "Cleveland", "Cavaliers", "Cavaliers", "CLE", "061642", "fdbb30"
    ),
    "6": Team.create_team(
        "6", "Dallas", "Mavericks", "Mavericks", "DAL", "0C479D", "c4ced3"
    ),
    "7": Team.create_team(
        "7", "Denver", "Nuggets", "Nuggets", "DEN", "0860A8", "fdb927"
    ),
    "8": Team.create_team(
        "8", "Detroit", "Pistons", "Pistons", "DET", "FA002C", "000000"
    ),
    "9": Team.create_team(
        "9", "Golden State", "Warriors", "Warriors", "GS", "003da5", "fdb927"
    ),
    "10": Team.create_team(
        "10", "Houston", "Rockets", "Rockets", "HOU", "D40026", "ffffff"
    ),
    "11": Team.create_team(
        "11", "Indiana", "Pacers", "Pacers", "IND", "061642", "ffffff"
    ),
    "12": Team.create_team(
        "12", "LA", "Clippers", "Clippers", "LAC", "FA0028", "f1f2f3"
    ),
    "13": Team.create_team(
        "13", "Los Angeles", "Lakers", "Lakers", "LAL", "542582", "ffffff"
    ),
    "29": Team.create_team(
        "29", "Memphis", "Grizzlies", "Grizzlies", "MEM", "5D76A8", "000000"
    ),
    "14": Team.create_team("14", "Miami", "Heat", "Heat", "MIA", "000000", "ffffff"),
    "15": Team.create_team(
        "15", "Milwaukee", "Bucks", "Bucks", "MIL", "003813", "f0ebd2"
    ),
    "16": Team.create_team(
        "16", "Minnesota", "Timberwolves", "T Wolves", "MIN", "0E3764", "c4ced3"
    ),
    "3": Team.create_team(
        "3", "New Orleans", "Pelicans", "Pelicans", "NO", "002a5c", "b4975a"
    ),
    "18": Team.create_team(
        "18", "New York", "Knicks", "Knicks", "NY", "225EA8", "ffffff"
    ),
    "25": Team.create_team(
        "25", "Oklahoma City", "Thunder", "Thunder", "OKC", "C67C03", "000000"
    ),
    "19": Team.create_team("19", "Orlando", "Magic", "Magic", "ORL", "0860A8", "c4ced3"),
    "20": Team.create_team(
        "20", "Philadelphia", "76ers", "76ers", "PHI", "006BB6", "ffffff"
    ),
    "21": Team.create_team("21", "Phoenix", "Suns", "Suns", "PHX", "23006a", "f1f2f3"),
    "22": Team.create_team(
        "22", "Portland", "Trail Blazers", "T Blazers", "POR", "000000", "bac3c9"
    ),
    "23": Team.create_team(
        "23", "Sacramento", "Kings", "Kings", "SAC", "393996", "ffffff"
    ),
    "24": Team.create_team(
        "24", "San Antonio", "Spurs", "Spurs", "SA", "000000", "ffffff"
    ),
    "28": Team.create_team(
        "28", "Toronto", "Raptors", "Raptors", "TOR", "CE0F41", "ffffff"
    ),
    "26": Team.create_team("26", "Utah", "Jazz", "Jazz", "UTAH", "06143F", "f9a01b"),
    "27": Team.create_team(
        "27", "Washington", "Wizards", "Wizards", "WSH", "0E3764", "ffffff"
    ),
}


class Basketball:
    def create_game(common):
        if common is None:
            return None
        return {"type": "Basketball", "common": common}

    async def get_games(testing: bool):
        if testing:
            return Common.get_testing_games("basketball")
        else:
            raw_games = await Fetcher.espn_fetch("basketball", "nba")
            games = [
                Basketball.create_game(Common.from_espn_json(game, Team.get_espn_team, team_map, SportId.BASKETBALL))
                for game in raw_games
            ]
            return [g for g in games if g]

async def main():
    print("Fetching games")
    print(await Basketball.get_games(False))

if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    while True:
        loop.run_until_complete(main())
        time.sleep(60)
    loop.close()


