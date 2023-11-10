import requests

import aiohttp


class Fetcher:
    async def schedule_fetch(schedule_url: str):
        print(f"[STATSAPI] Fetching schedule at url {schedule_url}")
        try:
            async with aiohttp.ClientSession() as session:
                async with session.get(schedule_url) as r:
                    json = await r.json()
                    print(
                        f"[STATSAPI] Done fetching schedule at url {schedule_url}")
                    dates = json["gameWeek"]
                    if len(dates) > 0:
                        return dates[0]["games"]
                    else:
                        return []
        except Exception as e:
            print(f"[STATSAPI] FAILED REQUEST RESPONSE {e}")
            return []

    async def game_fetch(game_url: str):
        print(f"[STATSAPI] Fetching game at url {game_url}")
        async with aiohttp.ClientSession() as session:
            async with session.get(game_url) as r:
                result = await r.json()
                print(f"[STATSAPI] Done Fetching game at url {game_url}")
                return result

    def get_espn_url(sport: str, selection: str, suffix: str):
        if selection is None:
            return f"http://site.api.espn.com/apis/site/v2/sports/{sport}/{suffix}"
        else:
            return f"http://site.api.espn.com/apis/site/v2/sports/{sport}/{selection}/{suffix}"

    async def espn_fetch(sport: str, selection: str, suffix: str = "scoreboard"):
        url = Fetcher.get_espn_url(sport, selection, suffix)
        print(f"[ESPN] Fetching for sport {sport} {selection} from {url}")
        async with aiohttp.ClientSession() as session:
            async with session.get(url) as r:
                try:
                    json = await r.json()
                    events = json["events"]
                    print(f"[ESPN] Done fetching for {sport} {selection}")
                    return events
                except Exception as e:
                    print(f"[ESPN] FAILED REQUEST RESPONSE {e}")
                    return []


if __name__ == "__main__":
    json = Fetcher.espn_fetch("basketball", "mens-college-basketball")
