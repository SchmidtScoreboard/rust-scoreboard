from common import getCommonGameData, getTeam


def getHockey(
    common, away_powerplay=False, home_powerplay=False, away_players=5, home_players=5
):
    return {
        "common": common,
        "away_powerplay": away_powerplay,
        "home_powerplay": home_powerplay,
        "away_players": away_players,
        "home_players": home_players,
    }


def get_hockey_games(testing: bool):
    if testing:
        blues = getTeam("19", "Blues", "STL", "002f87", "ffb81c")
        vegas = getTeam("54", "Vegas", "VGK", "002f87", "ffb81c")
        return {
            "data": {
                "games": [
                    # getHockey(
                    #     getCommonGameData(
                    #         blues, vegas, "PREGAME", "1st", "2020-08-09T19:00:00Z"
                    #     ),
                    #     True,
                    # ),
                    # getHockey(
                    #     getCommonGameData(
                    #         blues, vegas, "END", "1st", "2020-08-09T19:00:00Z"
                    #     ),
                    #     True,
                    # ),
                    getHockey(
                        getCommonGameData(
                            blues, vegas, "ACTIVE", "1st", "2020-08-09T19:00:00Z"
                        ),
                        True,
                        True,
                        4,
                        4,
                    ),
                ]
            }
        }
    else:
        return {"data": {"games": []}}
