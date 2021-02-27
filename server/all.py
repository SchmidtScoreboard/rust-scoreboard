from college_basketball import CollegeBasketball
from basketball import Basketball
from hockey import Hockey
from baseball import Baseball
import time

class All:
    def get_games(testing: bool):
        game_sets = [CollegeBasketball.get_games(testing), Basketball.get_games(testing), Hockey.get_games(testing), Baseball.get_games(testing)]

        flatten_list = [game for game_set in game_sets for game in game_set] 

        return {"data" : { "games" : flatten_list}}

if __name__ == "__main__":
    while True:
        print("Fetching games")
        print(All.get_games(False))
        time.sleep(60)