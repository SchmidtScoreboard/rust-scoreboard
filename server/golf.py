from common import Common, Team, SportId
from fetcher import Fetcher
import time
import asyncio
from dateutil.parser import parse
import datetime
import pytz


class Golf:
    def create_player(player):
        stats = player["statistics"]
        full_name = player["athlete"]["displayName"].upper().split()
        last_name = full_name[-1]
        full_name = full_name[:-1]
        while last_name in ["JR.", "JR", "SR", "SR.", "II", "III", "IV", "V", "VI"]:
            last_name = full_name[-1]
            full_name = full_name[:-1]
        if len(stats) == 0:
            score = "E"
        else:
            score = stats[0]["displayValue"]
        return {
            "display_name": last_name,
            "position": int(player["status"]["position"]["id"]),
            "score": score,
        }

    def is_better_score(a, b):
        def normalize(string):
            if string == "E":
                return "0"

        return int(normalize(a)) < int(normalize(b))

    def create_game(common, game):
        if common is None:
            return None

        competition = game["competitions"][0]
        players = competition["competitors"]

        top_5 = [
            Golf.create_player(player)
            for player in players
            if 0 < int(player["status"]["position"]["id"]) < 5 
        ]
        top_5.sort(key=lambda player: player["position"])
        top_5 = top_5[:5]

        name = game["shortName"].upper()
        words = name.split()
        if words[-1] in ["TOURNAMENT", "CHAMPIONSHIP"]:
            words = words[:-1]

        if words[0].isdigit():
            words = words[1:]

        if words[0] == "MASTERS":
            name = "THE " + " ".join(words)

        return {"type": "Golf", "common": common, "players": top_5, "name": name}

    def create_golf_common(game):
        competition = game["competitions"][0]
        espn_status = competition["status"]["type"]["name"]
        status = Common.convert_status(espn_status)
        ordinal = Common.to_ordinal(competition["status"]["period"])
        game_id = game["id"]

        earliest_tee_time = (None, None)

        # Find start time by looking at players
        for player in competition["competitors"]:
            tee_time = player["status"].get("teeTime")
            if tee_time is None:
                continue
            tee_time_value = parse(tee_time).astimezone(pytz.utc)
            earliest_tee_time_value, _ = earliest_tee_time
            if (
                earliest_tee_time_value is None
                or tee_time_value < earliest_tee_time_value
            ):
                earliest_tee_time = (tee_time_value, tee_time)

        empty_team = Team.create_team("0", "", "", "", "", "000000", "000000")

        time, tee_time_display = earliest_tee_time
        now = datetime.datetime.now(tz=pytz.UTC)
        if time is None:
            return None

        delta = abs(now - time)
        if delta > datetime.timedelta(hours=24):
            return None

            
        if status == "ACTIVE":
            if time > now: # if tee time in the future, this happens after Day X of play ends
                status = "END"
                

        if status is not None:
            return Common.create_common(
                SportId.GOLF.value,
                empty_team,
                empty_team,
                status,
                ordinal,
                str(tee_time_display),
                int(game_id),
                0,
                0,
            )
        else:
            return None

    async def get_games(testing: bool):
        if testing:
            return Common.get_testing_games("golf")
        else:
            raw_games = await Fetcher.espn_fetch("golf", None, "leaderboard?league=pga")
            games = [
                Golf.create_game(Golf.create_golf_common(game), game)
                for game in raw_games
            ]
            return [g for g in games if g]


async def main():
    print("Fetching games")
    print(await Golf.get_games(False))


if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    while True:
        loop.run_until_complete(main())
        time.sleep(60)
    loop.close()
