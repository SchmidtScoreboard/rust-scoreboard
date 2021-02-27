from college_basketball import CollegeBasketball
from basketball import Basketball
from hockey import Hockey
from baseball import Baseball
import time
import json

cache = {}

REFRESH_WINDOW = 60  # seconds


def success_response(resp):
    return {"statusCode": 200, "body": json.dumps({"data": resp})}

def wrap_games(games):
    return {"games": games}


def lambda_handler(event, context):
    # First, hit the cache
    print(event)
    sport = event["path"][1:]
    if sport in cache:
        item, timestamp = cache[sport]
        if time.time() - timestamp < REFRESH_WINDOW:
            return success_response(item)

    if sport == "college-basketball":
        result = wrap_games(CollegeBasketball.get_games(False))
    elif sport == "basketball":
        result = wrap_games(Basketball.get_games(False))
    elif sport == "hockey":
        result = wrap_games(Hockey.get_games(False))
    elif sport == "baseball":
        result = wrap_games(Baseball.get_games(False))
    else:
        result = None

    if result is not None:
        cache[sport] = (result, time.time())
        return success_response(result)

    print(f"ERROR: unknown sport {sport}")
    return None
