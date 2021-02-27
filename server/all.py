from college_basketball import CollegeBasketball
from basketball import Basketball
from hockey import Hockey
from baseball import Baseball
import time

class All:
    def getGames(testing: bool):
        game_sets = [CollegeBasketball.getGames(testing), Basketball.getGames(testing), Hockey.getGames(testing), Baseball.getGames(testing)]

        flatten_list = [game for game_set in game_sets for game in game_set] 

        return {"data" : { "games" : flatten_list}}

if __name__ == "__main__":
    while True:
        print("Fetching games")
        print(All.getGames(False))
        time.sleep(60)