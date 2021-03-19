import requests

import aiohttp


class Fetcher:
    async def schedule_fetch(schedule_url: str):
        async with aiohttp.ClientSession() as session:
            async with session.get(schedule_url) as r:
                json = await r.json()
                dates = json["dates"]
                if len(dates) > 0:
                    return dates[0]["games"]
                else:
                    return []

    async def game_fetch(game_url: str):
        async with aiohttp.ClientSession() as session:
            async with session.get(game_url) as r:
                return await r.json()

    def get_espn_url(sport: str, selection: str):
        return f"http://site.api.espn.com/apis/site/v2/sports/{sport}/{selection}/scoreboard"

    async def espn_fetch(sport: str, selection: str):
        async with aiohttp.ClientSession() as session:
            async with session.get(Fetcher.get_espn_url(sport, selection)) as r:
                json = await r.json()
                events = json["events"]
                return events


if __name__ == "__main__":
    json = Fetcher.espn_fetch("basketball", "mens-college-basketball")
