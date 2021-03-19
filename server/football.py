from common import Common, Team, SportId
from fetcher import Fetcher
import time
import asyncio

team_map = {
    "22": Team.create_team(
        "22", "Arizona", "Cardinals", "Cardinals", "ARI", "A40227", "ffffff"
    ),
    "1": Team.create_team(
        "1", "Atlanta", "Falcons", "Falcons", "ATL", "000000", "ffffff"
    ),
    "2": Team.create_team("2", "Buffalo", "Bills", "Bills", "BUF", "04407F", "ffffff"),
    "3": Team.create_team("3", "Chicago", "Bears", "Bears", "CHI", "152644", "ffffff"),
    "4": Team.create_team(
        "4", "Cincinnati", "Bengals", "Bengals", "CIN", "FF2700", "000000"
    ),
    "5": Team.create_team(
        "5", "Cleveland", "Browns", "Browns", "CLE", "4C230E", "ffffff"
    ),
    "6": Team.create_team(
        "6", "Dallas", "Cowboys", "Cowboys", "DAL", "002E4D", "b0b7bc"
    ),
    "7": Team.create_team(
        "7", "Denver", "Broncos", "Broncos", "DEN", "002E4D", "fb4f14"
    ),
    "8": Team.create_team("8", "Detroit", "Lions", "Lions", "DET", "035C98", "ffffff"),
    "9": Team.create_team(
        "9", "Green Bay", "Packers", "Packers", "GB", "204E32", "ffb612"
    ),
    "11": Team.create_team(
        "11", "Indianapolis", "Colts", "Colts", "IND", "00417E", "ffffff"
    ),
    "12": Team.create_team(
        "12", "Kansas City", "Chiefs", "Chiefs", "KC", "BE1415", "ffffff"
    ),
    "13": Team.create_team(
        "13", "Las Vegas", "Raiders", "Raiders", "LV", "000000", "a5acaf"
    ),
    "24": Team.create_team(
        "24", "Los Angeles", "Chargers", "Chargers", "LAC", "042453", "ffffff"
    ),
    "14": Team.create_team(
        "14", "Los Angeles", "Rams", "Rams", "LAR", "00295B", "b3995d"
    ),
    "15": Team.create_team(
        "15", "Miami", "Dolphins", "Dolphins", "MIA", "006B79", "ffffff"
    ),
    "16": Team.create_team(
        "16", "Minnesota", "Vikings", "Vikings", "MIN", "240A67", "ffc62f"
    ),
    "17": Team.create_team(
        "17", "New England", "Patriots", "Patriots", "NE", "02244A", "b0b7bc"
    ),
    "18": Team.create_team(
        "18", "New Orleans", "Saints", "Saints", "NO", "020202", "ffffff"
    ),
    "19": Team.create_team(
        "19", "New York", "Giants", "Giants", "NYG", "052570", "ffffff"
    ),
    "20": Team.create_team("20", "New York", "Jets", "Jets", "NYJ", "174032", "ffffff"),
    "21": Team.create_team(
        "21", "Philadelphia", "Eagles", "Eagles", "PHI", "06424D", "a5acaf"
    ),
    "23": Team.create_team(
        "23", "Pittsburgh", "Steelers", "Steelers", "PIT", "000000", "ffb612"
    ),
    "25": Team.create_team(
        "25", "San Francisco", "49ers", "49ers", "SF", "981324", "ffffff"
    ),
    "10": Team.create_team(
        "10", "Tennessee", "Titans", "Titans", "TEN", "2F95DD", "000000"
    ),
    "27": Team.create_team(
        "27", "Tampa Bay", "Buccaneers", "Buccaneers", "TB", "A80D08", "ffffff"
    ),
}


class Football:
    def create_game(common):
        if common is None:
            return None
        return {"type": "Football", "common": common}

    async def get_games(testing: bool):
        if testing:
            return Common.get_testing_games("football")
        else:
            raw_games = await Fetcher.espn_fetch("football", "nfl")
            games = [
                Football.create_game(
                    Common.from_espn_json(
                        game, Team.get_espn_team, team_map, SportId.FOOTBALL
                    )
                )
                for game in raw_games
            ]
            return [g for g in games if g]


async def main():
    print("Fetching games")
    print(await Football.get_games(False))


if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    while True:
        loop.run_until_complete(main())
        time.sleep(60)
    loop.close()
