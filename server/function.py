from college_basketball import CollegeBasketball
from basketball import Basketball
from college_football import CollegeFootball
from football import Football
from hockey import Hockey
from baseball import Baseball
from all_sports import All
import time
import json
import asyncio

cache = {}

REFRESH_WINDOW = 60  # seconds


def success_response(resp):
    return {"statusCode": 200, "body": json.dumps({"data": resp})}


def wrap_games(games):
    return {"games": games}


def lambda_handler(event, context):
    # First, hit the cache
    loop = asyncio.get_event_loop()
    print(event)
    sport = event["path"][1:]
    if sport in cache:
        item, timestamp = cache[sport]
        if time.time() - timestamp < REFRESH_WINDOW:
            return success_response(item)

    if sport == "college-basketball":
        result = wrap_games(loop.run_until_complete(CollegeBasketball.get_games(False)))
    elif sport == "basketball":
        result = wrap_games(loop.run_until_complete(Basketball.get_games(False)))
    elif sport == "hockey":
        result = wrap_games(loop.run_until_complete(Hockey.get_games(False)))
    elif sport == "college_football":
        result = wrap_games(loop.run_until_complete(CollegeFootball.get_games(False)))
    elif sport == "football":
        result = wrap_games(loop.run_until_complete(Football.get_games(False)))
    elif sport == "baseball":
        result = wrap_games(loop.run_until_complete(Baseball.get_games(False)))
    elif sport == "all":
        result = wrap_games(loop.run_until_complete(All.get_games_v1(False)))
    elif sport == "all_v2":
        result = wrap_games(loop.run_until_complete(All.get_games_v2(False)))
    else:
        result = None

    if result is not None:
        cache[sport] = (result, time.time())
        return success_response(result)

    print(f"ERROR: unknown sport {sport}")
    return None
