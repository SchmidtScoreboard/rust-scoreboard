from common import Common, Team, SportId
from fetcher import Fetcher
import time
import asyncio

team_map = {
    "22": Team.create_team("22", "Arizona", "Cardinals", "Cardinals", "ARI", "A40227", "ffffff"),
    "1": Team.create_team("1", "Atlanta", "Falcons", "Falcons", "ATL", "000000", "ffffff"),
    "33": Team.create_team("33", "Baltimore", "Ravens", "Ravens", "BAL", "2B025B", "9e7c0c"),
    "2": Team.create_team("2", "Buffalo", "Bills", "Bills", "BUF", "04407F", "ffffff"),
    "29": Team.create_team("29", "Carolina", "Panthers", "Panthers", "CAR", "2177B0", "ffffff"),
    "3": Team.create_team("3", "Chicago", "Bears", "Bears", "CHI", "0B162A", "C83803"),
    "4": Team.create_team("4", "Cincinnati", "Bengals", "Bengals", "CIN", "FF2700", "000000"),
    "5": Team.create_team("5", "Cleveland", "Browns", "Browns", "CLE", "4C230E", "ffffff"),
    "6": Team.create_team("6", "Dallas", "Cowboys", "Cowboys", "DAL", "002E4D", "b0b7bc"),
    "7": Team.create_team("7", "Denver", "Broncos", "Broncos", "DEN", "002E4D", "fb4f14"),
    "8": Team.create_team("8", "Detroit", "Lions", "Lions", "DET", "035C98", "ffffff"),
    "9": Team.create_team("9", "Green Bay", "Packers", "Packers", "GB", "204E32", "ffb612"),
    "34": Team.create_team("34", "Houston", "Texans", "Texans", "HOU", "00133F", "ffffff"),
    "11": Team.create_team("11", "Indianapolis", "Colts", "Colts", "IND", "00417E", "ffffff"),
    "30": Team.create_team("30", "Jacksonville", "Jaguars", "Jaguars", "JAX", "00839C", "000000"),
    "12": Team.create_team("12", "Kansas City", "Chiefs", "Chiefs", "KC", "BE1415", "ffffff"),
    "13": Team.create_team("13", "Las Vegas", "Raiders", "Raiders", "LV", "000000", "a5acaf"),
    "24": Team.create_team("24", "Los Angeles", "Chargers", "Chargers", "LAC", "042453", "ffffff"),
    "14": Team.create_team("14", "Los Angeles", "Rams", "Rams", "LAR", "00295B", "b3995d"),
    "15": Team.create_team("15", "Miami", "Dolphins", "Dolphins", "MIA", "006B79", "ffffff"),
    "16": Team.create_team("16", "Minnesota", "Vikings", "Vikings", "MIN", "240A67", "ffc62f"),
    "17": Team.create_team("17", "New England", "Patriots", "Patriots", "NE", "02244A", "b0b7bc"),
    "18": Team.create_team("18", "New Orleans", "Saints", "Saints", "NO", "020202", "ffffff"),
    "19": Team.create_team("19", "New York", "Giants", "Giants", "NYG", "052570", "ffffff"),
    "20": Team.create_team("20", "New York", "Jets", "Jets", "NYJ", "174032", "ffffff"),
    "21": Team.create_team("21", "Philadelphia", "Eagles", "Eagles", "PHI", "06424D", "a5acaf"),
    "23": Team.create_team("23", "Pittsburgh", "Steelers", "Steelers", "PIT", "000000", "ffb612"),
    "25": Team.create_team("25", "San Francisco", "49ers", "49ers", "SF", "981324", "ffffff"),
    "26": Team.create_team("26", "Seattle", "Seahawks", "Seahawks", "SEA", "224970", "69be28"),
    "27": Team.create_team("27", "Tampa Bay", "Buccaneers", "Buccaneers", "TB", "A80D08", "ffffff"),
    "10": Team.create_team("10", "Tennessee", "Titans", "Titans", "TEN", "2F95DD", "000000"),
    "28": Team.create_team("28", "Washington", "Washington", "Washington", "WSH", "650415", "ffffff"),
}


class Football_v2:
    def create_game(common, game):
        if common is None:
            return None
        competition = game["competitions"][0]
        situation = competition.get("situation")
        status = competition.get("status")

        time_remaining = status.get("displayClock", "") if status is not None else ""
        espn_status = competition["status"]["type"]["name"]
        status = Common.convert_status(espn_status)
        if status not in ["Active"]:
            time_remaining = ""
    
        ball_position = situation.get("possessionText", "") if situation is not None else ""

        down_string = situation.get("shortDownDistanceText", "") if situation is not None else ""
        down_string = down_string.replace("&", "+")
        
        home_team, away_team = competition["competitors"]
        possessing_team_id = situation.get("possession", None) if situation is not None else None
        if possessing_team_id is None:
            home_possession = None
        else:
            home_possession = possessing_team_id == home_team["id"]
        return {"type": "Football", "common": common, "extra_data": {
            "time_remaining": time_remaining,
            "ball_position": ball_position,
            "down_string": down_string,
            "home_possession": home_possession 
        }}

    async def get_games(testing: bool):
        if testing:
            return Common.get_testing_games("football")
        else:
            raw_games = await Fetcher.espn_fetch("football", "nfl")
            games = [
                Football_v2.create_game(
                    Common.from_espn_json(
                        game, Team.get_espn_team, team_map, SportId.FOOTBALL
                    ),
                    game
                )
                for game in raw_games
            ]
            return [g for g in games if g]


async def main():
    print("Fetching games")
    print(await Football_v2.get_games(False))


if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    while True:
        loop.run_until_complete(main())
        time.sleep(60)
    loop.close()
