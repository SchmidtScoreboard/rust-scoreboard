import requests

class Fetcher:
    def get_url(sport: str, selection: str):
        return f"http://site.api.espn.com/apis/site/v2/sports/{sport}/{selection}/scoreboard"
    def fetch(sport: str, selection: str):
        r = requests.get(url=Fetcher.get_url(sport, selection))
        json = r.json()
        events = json["events"]
        return events


if __name__ == '__main__':
    json = Fetcher.fetch("basketball", "mens-college-basketball") 
