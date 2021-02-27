from college_basketball import CollegeBasketball
from basketball import Basketball
from hockey import Hockey
from baseball import Baseball
import asyncio
import time


class All:
    async def get_games(testing: bool):
        game_sets = await asyncio.gather(*[CollegeBasketball.get_games(testing), Basketball.get_games(testing), Hockey.get_games(testing), Baseball.get_games(testing)])
        print(game_sets)
        flatten_list = [game for game_set in game_sets for game in game_set] 
        print(flatten_list)

        return flatten_list

async def main():
    print("Fetching games")
    print(await All.get_games(False))

if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    while True:
        loop.run_until_complete(main())
        time.sleep(60)
    loop.close()