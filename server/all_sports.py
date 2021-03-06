from college_basketball import CollegeBasketball
from basketball import Basketball
from hockey import Hockey
from baseball import Baseball
from football import Football
from baseball_v2 import Baseball_v2
from golf import Golf
from college_football import CollegeFootball
import asyncio
import time


class All:
    async def get_games_v1(testing: bool):
        game_sets = await asyncio.gather(
            *[
                CollegeBasketball.get_games(testing),
                Basketball.get_games(testing),
                Hockey.get_games(testing),
                Baseball.get_games(testing),
                Football.get_games(testing),
                CollegeFootball.get_games(testing),
            ]
        )
        flatten_list = [game for game_set in game_sets for game in game_set]

        return flatten_list

    async def get_games_v2(testing: bool):
        print("[ALL] Beginning fetching all sports")
        game_sets = await asyncio.gather(
            *[
                Golf.get_games(testing),
                CollegeBasketball.get_games(testing),
                Basketball.get_games(testing),
                Hockey.get_games(testing),
                Baseball.get_games(testing),
                Football.get_games(testing),
                CollegeFootball.get_games(testing),
            ]
        )
        print("[ALL] Finished fetching all sports")
        flatten_list = [game for game_set in game_sets for game in game_set]

        return flatten_list

    async def get_games_v3(testing: bool):
        print("[ALL] Beginning fetching all sports")
        game_sets = await asyncio.gather(
            *[
                Golf.get_games(testing),
                CollegeBasketball.get_games(testing),
                Basketball.get_games(testing),
                Hockey.get_games(testing),
                Baseball_v2.get_games(testing),
                Football.get_games(testing),
                CollegeFootball.get_games(testing),
            ]
        )
        print("[ALL] Finished fetching all sports")
        flatten_list = [game for game_set in game_sets for game in game_set]

        return flatten_list


async def main():
    print("Fetching games")
    print(await All.get_games_v3(False))


if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    while True:
        loop.run_until_complete(main())
        time.sleep(60)
    loop.close()