from common import Common, Team, SportId
from fetcher import Fetcher
from ncaa import team_map
import asyncio


class CollegeBasketball:
    def create_game(common):
        if common is None:
            return None
        return {"type": "CollegeBasketball", "common": common}

    async def get_games(testing: bool):
        if testing:
            return Common.get_testing_games("college-basketball")
        else:
            raw_games = await Fetcher.espn_fetch("basketball", "mens-college-basketball")
            games = [
                CollegeBasketball.create_game(
                    Common.from_espn_json(game, Team.get_espn_team, team_map, SportId.COLLEGE_BASKETBALL)
                )
                for game in raw_games
            ]
            return [g for g in games if g]


async def main():
    print("Fetching games")
    games = await CollegeBasketball.get_games(False)
    print(games)


if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
    loop.close()
