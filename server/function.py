from college_basketball import CollegeBasketball
import time

cache = {}

REFRESH_WINDOW = 60  # seconds


def lambda_handler(event, context):
    # First, hit the cache
    print(event)
    sport = event["path"][1:]
    if sport in cache:
        item, timestamp = cache[event]
        if time.time() - timestamp < REFRESH_WINDOW:
            return item

    if sport == "college-basketball":
        result = CollegeBasketball.getGames(False)
        cache[event] = (result, time.time())
        return result

    print(f"ERROR: unknown sport {sport}")
    return None
