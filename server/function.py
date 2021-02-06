from college_basketball import CollegeBasketball
import time

cache = {}

REFRESH_WINDOW = 60  # seconds


def lambda_handler(event, context):
    # First, hit the cache
    print(event)
    if event in cache:
        item, timestamp = cache[event]
        if time.time() - timestamp < REFRESH_WINDOW:
            return item

    if event == "college-basketball":
        result = CollegeBasketball.getGames(False)
        cache[event] = (result, time.time())
        return result

    print(f"ERROR: unknown event {event}")
    return None
