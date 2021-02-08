from college_basketball import CollegeBasketball
import time
import json

cache = {}

REFRESH_WINDOW = 60  # seconds


def success_response(resp):
    return {"statusCode": 200, "body": {"data": json.dumps(resp)}}


def lambda_handler(event, context):
    # First, hit the cache
    print(event)
    sport = event["path"][1:]
    if sport in cache:
        item, timestamp = cache[sport]
        if time.time() - timestamp < REFRESH_WINDOW:
            return success_response(item)

    if sport == "college-basketball":
        result = CollegeBasketball.getGames(False)
        cache[sport] = (result, time.time())
        return success_response(result)

    print(f"ERROR: unknown sport {sport}")
    return None
