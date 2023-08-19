from common import Common, Team, SportId, pretty_print
from fetcher import Fetcher
from ncaa import team_map
import asyncio


class CollegeFootball:
    def create_game(common):
        if common is None:
            return None
        return {"type": "CollegeFootball", "common": common}

    async def get_games(testing: bool):
        if testing:
            return Common.get_testing_games("college-football")
        else:
            raw_games = await Fetcher.espn_fetch("football", "college-football")
            games = [
                CollegeFootball.create_game(
                    Common.from_espn_json(
                        game, Team.get_espn_team, team_map, SportId.COLLEGE_FOOTBALL
                    )
                )
                for game in raw_games
            ]
            return [g for g in games if g]


async def main():
    print("Fetching games")
    games = await CollegeFootball.get_games(False)
    pretty_print(games)


if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
    loop.close()
