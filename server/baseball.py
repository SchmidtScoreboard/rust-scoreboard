from common import getCommonGameData, getTeam


def getBaseball(common, is_inning_top=False, balls=0, outs=0, strikes=0, inning=1):
    return {
        "common": common,
        "is_inning_top": is_inning_top,
        "balls": balls,
        "outs": outs,
        "strikes": strikes,
        "inning": inning,
    }


def get_baseball_games(testing: bool):
    if testing:
        cards = getTeam("19", "Cardinals", "STL", "002f87", "ffb81c")
        cubs = getTeam("54", "Cubs", "CHI", "002f87", "ffb81c")
        return {
            "data": {
                "games": [
                    # getBaseball(
                    #     getCommonGameData(
                    #         cards, cubs, "ACTIVE", "1st", "2020-08-09T19:00:00Z"
                    #     ),
                    #     False,
                    #     2,
                    #     2,
                    #     2,
                    #     1,
                    # ),
                    getBaseball(
                        getCommonGameData(
                            cards, cubs, "END", "1st", "2020-08-09T19:00:00Z"
                        ),
                        True,
                        2,
                        2,
                        2,
                        1,
                    ),
                ]
            }
        }
    else:
        # TODO fetch data
        return {"data": {"games": []}}
