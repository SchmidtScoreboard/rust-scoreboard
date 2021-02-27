from common import Common, Team
from fetcher import Fetcher
import time

team_map = {
    "1": Team.createTeam("1", "Atlanta", "Hawks", "Hawks", "ATL", "002B5C", "ffffff"),
    "2": Team.createTeam(
        "2", "Boston", "Celtics", "Celtics", "BOS", "006532", "f1f2f3"
    ),
    "17": Team.createTeam("17", "Brooklyn", "Nets", "Nets", "BKN", "000000", "ffffff"),
    "30": Team.createTeam(
        "30", "Charlotte", "Hornets", "Hornets", "CHA", "1D1060", "008ca8"
    ),
    "4": Team.createTeam("4", "Chicago", "Bulls", "Bulls", "CHI", "000000", "ffffff"),
    "5": Team.createTeam(
        "5", "Cleveland", "Cavaliers", "Cavaliers", "CLE", "061642", "fdbb30"
    ),
    "6": Team.createTeam(
        "6", "Dallas", "Mavericks", "Mavericks", "DAL", "0C479D", "c4ced3"
    ),
    "7": Team.createTeam(
        "7", "Denver", "Nuggets", "Nuggets", "DEN", "0860A8", "fdb927"
    ),
    "8": Team.createTeam(
        "8", "Detroit", "Pistons", "Pistons", "DET", "FA002C", "000000"
    ),
    "9": Team.createTeam(
        "9", "Golden State", "Warriors", "Warriors", "GS", "003da5", "fdb927"
    ),
    "10": Team.createTeam(
        "10", "Houston", "Rockets", "Rockets", "HOU", "D40026", "ffffff"
    ),
    "11": Team.createTeam(
        "11", "Indiana", "Pacers", "Pacers", "IND", "061642", "ffffff"
    ),
    "12": Team.createTeam(
        "12", "LA", "Clippers", "Clippers", "LAC", "FA0028", "f1f2f3"
    ),
    "13": Team.createTeam(
        "13", "Los Angeles", "Lakers", "Lakers", "LAL", "542582", "ffffff"
    ),
    "29": Team.createTeam(
        "29", "Memphis", "Grizzlies", "Grizzlies", "MEM", "5D76A8", "000000"
    ),
    "14": Team.createTeam("14", "Miami", "Heat", "Heat", "MIA", "000000", "ffffff"),
    "15": Team.createTeam(
        "15", "Milwaukee", "Bucks", "Bucks", "MIL", "003813", "f0ebd2"
    ),
    "16": Team.createTeam(
        "16", "Minnesota", "Timberwolves", "T Wolves", "MIN", "0E3764", "c4ced3"
    ),
    "3": Team.createTeam(
        "3", "New Orleans", "Pelicans", "Pelicans", "NO", "002a5c", "b4975a"
    ),
    "18": Team.createTeam(
        "18", "New York", "Knicks", "Knicks", "NY", "225EA8", "ffffff"
    ),
    "25": Team.createTeam(
        "25", "Oklahoma City", "Thunder", "Thunder", "OKC", "C67C03", "000000"
    ),
    "19": Team.createTeam("19", "Orlando", "Magic", "Magic", "ORL", "0860A8", "c4ced3"),
    "20": Team.createTeam(
        "20", "Philadelphia", "76ers", "76ers", "PHI", "000000", "f1f2f3"
    ),
    "21": Team.createTeam("21", "Phoenix", "Suns", "Suns", "PHX", "23006a", "f1f2f3"),
    "22": Team.createTeam(
        "22", "Portland", "Trail Blazers", "T Blazers", "POR", "000000", "bac3c9"
    ),
    "23": Team.createTeam(
        "23", "Sacramento", "Kings", "Kings", "SAC", "393996", "ffffff"
    ),
    "24": Team.createTeam(
        "24", "San Antonio", "Spurs", "Spurs", "SA", "000000", "ffffff"
    ),
    "28": Team.createTeam(
        "28", "Toronto", "Raptors", "Raptors", "TOR", "CE0F41", "ffffff"
    ),
    "26": Team.createTeam("26", "Utah", "Jazz", "Jazz", "UTAH", "06143F", "f9a01b"),
    "27": Team.createTeam(
        "27", "Washington", "Wizards", "Wizards", "WSH", "0E3764", "ffffff"
    ),
}


class Basketball:
    def createGame(common):
        if common is None:
            return None
        return {"common": common}

    def getGames(testing: bool):
        if testing:
            return Common.get_testing_games("basketball")
        else:
            raw_games = Fetcher.espn_fetch("basketball", "nba")
            games = [
                Basketball.createGame(Common.from_espn_json(game, Team.getESPNTeam, team_map))
                for game in raw_games
            ]
            return {"games": [g for g in games if g]}


if __name__ == "__main__":
    while True:
        print("Fetching games")
        print(Basketball.getGames(False))
        time.sleep(60)


