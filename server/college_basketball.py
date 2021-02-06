from common import Common, Team
from fetcher import Fetcher

team_map = {
    66: {
        "id": "66",
        "display_name": "Iowa State",
        "abbreviation": "ISU",
        "primary_color": "660015",
        "secondary_color": "830b2c",
    },
    201: {
        "id": "201",
        "display_name": "Oklahoma",
        "abbreviation": "OU",
        "primary_color": "7b0000",
        "secondary_color": "cccccc",
    },
    333: {
        "id": "333",
        "display_name": "Alabama",
        "abbreviation": "ALA",
        "primary_color": "690014",
        "secondary_color": "f1f2f3",
    },
    142: {
        "id": "142",
        "display_name": "Missouri",
        "abbreviation": "MIZ",
        "primary_color": "000000",
        "secondary_color": "000000",
    },
    259: {
        "id": "259",
        "display_name": "Virginia Tech",
        "abbreviation": "VT",
        "primary_color": "74232D",
        "secondary_color": "c2c1ba",
    },
    2390: {
        "id": "2390",
        "display_name": "Miami",
        "abbreviation": "MIA",
        "primary_color": "004325",
        "secondary_color": "f0f0f0",
    },
    2305: {
        "id": "2305",
        "display_name": "Kansas",
        "abbreviation": "KU",
        "primary_color": "0022B4",
        "secondary_color": "e8000d",
    },
    277: {
        "id": "277",
        "display_name": "West Virginia",
        "abbreviation": "WVU",
        "primary_color": "FFC600",
        "secondary_color": "eaaa00",
    },
    2181: {
        "id": "2181",
        "display_name": "Drake",
        "abbreviation": "DRKE",
        "primary_color": "004477",
        "secondary_color": "c0c0c0",
    },
    2674: {
        "id": "2674",
        "display_name": "Valparaiso",
        "abbreviation": "VAL",
        "primary_color": "794500",
        "secondary_color": "ffffff",
    },
    275: {
        "id": "275",
        "display_name": "Wisconsin",
        "abbreviation": "WISC",
        "primary_color": "A00002",
        "secondary_color": "f7f7f7",
    },
    356: {
        "id": "356",
        "display_name": "Illinois",
        "abbreviation": "ILL",
        "primary_color": "f77329",
        "secondary_color": "fa6300",
    },
    3158: {
        "id": "3158",
        "display_name": "Our Lady",
        "abbreviation": "LDY",
        "primary_color": "00000",
        "secondary_color": "ffffff",
    },
    248: {
        "id": "248",
        "display_name": "Houston",
        "abbreviation": "HOU",
        "primary_color": "C90822",
        "secondary_color": "231f20",
    },
    251: {
        "id": "251",
        "display_name": "Texas",
        "abbreviation": "TEX",
        "primary_color": "EE7524",
        "secondary_color": "f0f0f0",
    },
    197: {
        "id": "197",
        "display_name": "Oklahoma State",
        "abbreviation": "OKST",
        "primary_color": "FF6500",
        "secondary_color": "ff9900",
    },
    2641: {
        "id": "2641",
        "display_name": "Texas Tech",
        "abbreviation": "TTU",
        "primary_color": "C80025",
        "secondary_color": "231f20",
    },
    2306: {
        "id": "2306",
        "display_name": "Kansas State",
        "abbreviation": "KSU",
        "primary_color": "633194",
        "secondary_color": "e7d2ad",
    },
    221: {
        "id": "221",
        "display_name": "Pittsburgh",
        "abbreviation": "PITT",
        "primary_color": "003263",
        "secondary_color": "231f20",
    },
    258: {
        "id": "258",
        "display_name": "Virginia",
        "abbreviation": "UVA",
        "primary_color": "f84c1e",
        "secondary_color": "242e4a",
    },
    77: {
        "id": "77",
        "display_name": "Northwestern",
        "abbreviation": "NW",
        "primary_color": "372286",
        "secondary_color": "d6cac1",
    },
    2509: {
        "id": "2509",
        "display_name": "Purdue",
        "abbreviation": "PUR",
        "primary_color": "B89D29",
        "secondary_color": "a4a9ad",
    },
    156: {
        "id": "156",
        "display_name": "Creighton",
        "abbreviation": "CREI",
        "primary_color": "13299e",
        "secondary_color": "00235d",
    },
    269: {
        "id": "269",
        "display_name": "Marquette",
        "abbreviation": "MARQ",
        "primary_color": "083963",
        "secondary_color": "ffffff",
    },
    2633: {
        "id": "2633",
        "display_name": "Tennessee",
        "abbreviation": "TENN",
        "primary_color": "EE9627",
        "secondary_color": "ffffff",
    },
    96: {
        "id": "96",
        "display_name": "Kentucky",
        "abbreviation": "UK",
        "primary_color": "005DAA",
        "secondary_color": "ffffff",
    },
    26: {
        "id": "26",
        "display_name": "UCLA",
        "abbreviation": "UCLA",
        "primary_color": "005C8E",
        "secondary_color": "ffc72c",
    },
    30: {
        "id": "30",
        "display_name": "USC",
        "abbreviation": "USC",
        "primary_color": "AE2531",
        "secondary_color": "ffc72c",
    },
    2541: {
        "id": "2541",
        "display_name": "Santa Clara",
        "abbreviation": "SCU",
        "primary_color": "690b0b",
        "secondary_color": "101010",
    },
    2250: {
        "id": "2250",
        "display_name": "Gonzaga",
        "abbreviation": "GONZ",
        "primary_color": "002967",
        "secondary_color": "cfd4d8",
    },
    2628: {
        "id": "2628",
        "display_name": "TCU",
        "abbreviation": "TCU",
        "primary_color": "3C377D",
        "secondary_color": "f1f2f3",
    },
    239: {
        "id": "239",
        "display_name": "Baylor",
        "abbreviation": "BAY",
        "primary_color": "004834",
        "secondary_color": "ffb81c",
    },
    57: {
        "id": "57",
        "display_name": "Florida",
        "abbreviation": "FLA",
        "primary_color": "0021A5",
        "secondary_color": "0021a5",
    },
    99: {
        "id": "99",
        "display_name": "LSU",
        "abbreviation": "LSU",
        "primary_color": "2B0D57",
        "secondary_color": "fdd023",
    },
}


class CollegeBasketball:
    def createGame(common):
        return {"common": common}

    def getTeam(team_id, competitor):
        if int(team_id) in team_map:
            return team_map[int(team_id)]
        else:
            team = Team.createTeam(
                int(team_id),
                competitor["team"]["shortDisplayName"],
                competitor["team"]["abbreviation"],
                competitor["team"].get("color", "00000"),
                competitor["team"].get("alternateColor", "ffffff"),
            )
            print(f"{str(team_id)}: {team},")
            return team

    def getGames(testing: bool):
        if testing:
            return {"games": []}
        else:
            raw_games = Fetcher.fetch("basketball", "mens-college-basketball")
            games = [
                CollegeBasketball.createGame(
                    Common.from_json(game, CollegeBasketball.getTeam)
                )
                for game in raw_games
            ]
            return {"games": games}


if __name__ == "__main__":
    print(CollegeBasketball.getGames(False))
