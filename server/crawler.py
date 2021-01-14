from bs4 import BeautifulSoup
from common import getCommonGameData, getTeam


def get_url(sport: str):
    return f"https://www.scorespro.com/{sport}"


def get_games_for_sport():
    url = get_url("ice-hockey")