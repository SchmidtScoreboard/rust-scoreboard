import requests


class Fetcher:
    def schedule_fetch(schedule_url: str):
        r = requests.get(url=schedule_url)
        json = r.json()
        dates = json["dates"]
        if len(dates) > 0:
            return dates[0]["games"]
        else:
            return []
            

    def game_fetch(game_url: str):
        r = requests.get(url=game_url)
        return r.json()


    def get_espn_url(sport: str, selection: str):
        return f"http://site.api.espn.com/apis/site/v2/sports/{sport}/{selection}/scoreboard"

    def espn_fetch(sport: str, selection: str):
        r = requests.get(url=Fetcher.get_espn_url(sport, selection))
        json = r.json()
        events = json["events"]
        return events


if __name__ == "__main__":
    json = Fetcher.espn_fetch("basketball", "mens-college-basketball")
