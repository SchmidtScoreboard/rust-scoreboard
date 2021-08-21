from common import Common, Team, SportId
from fetcher import Fetcher
import time
import asyncio
from dateutil.parser import parse
import datetime
import pytz
import re
import string


TEAMSTROKE_REGEX = re.compile(".*\s([a-zA-z ]+)\/([a-zA-z ]+)\s*([^\s]+)+")
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
            "display_name": last_name.translate(str.maketrans('', '', string.punctuation)),
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

        try:
            competition = game["competitions"]
            while isinstance(competition, list):
                competition = competition[0]
            top_5 = []
            if competition["scoringSystem"]["name"] == "Teamstroke":
                print("[GOLF] Looking at teamstroke")
                data = competition["rawData"]
                position = 1
                for line in data.splitlines():
                    match = TEAMSTROKE_REGEX.match(line)
                    if match:
                        groups = match.groups()
                        top_5.append({
                            "display_name": f"{groups[0][:5]}/{groups[1][:5]}",
                            "position": position,
                            "score": groups[2]
                        })
                        position += 1
                        if position > 5:
                            break
            else:
                players = competition["competitors"]

                top_5 = [
                    Golf.create_player(player)
                    for player in players
                    if 0 < int(player["status"]["position"]["id"]) < 5 
                ]
                top_5.sort(key=lambda player: player["position"])
                top_5 = top_5[:5]
        except Exception as e:
            print(f"Failed to parse data with {e}")
            return None

        name = game["shortName"].upper()
        words = name.split("PRES", 1)[0]
        words = words.split()

        for dumb_word in ["TOURNAMENT", "CHAMPIONSHIP", "CHALLENGE", "CLASSIC", "INVITATIONAL"]:
            if dumb_word in words:
                idx = words.index(dumb_word)
                words = words[:idx]

        if words[0] in ["THE"]:
            words = words[1:]

        if words[0].isdigit() or words[0] == "AT&T" or words[0] == "WGC-FEDEX":
            words = words[1:]
        
        try: 
            idx = words.index("OF")
            words = words[:idx] 
        except Exception as e:
            pass

        name = " ".join(words)
        if words[0] == "MASTERS":
            name = "THE " + " ".join(words)


        return {"type": "Golf", "common": common, "players": top_5, "name": name}

    def create_golf_common(game):
        competition = game["competitions"]
        while isinstance(competition, list):
            competition = competition[0]

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
            time, tee_time_display = (parse(competition["date"]).astimezone(pytz.utc), competition["date"])

        delta = abs(now - time)
        if delta > datetime.timedelta(hours=24) and status not in ["ACTIVE", "END"]:
            return None

            
        if status == "ACTIVE":
            if time > now: # if tee time in the future, this happens after Day X of play ends
                status = "END"
            if competition["scoringSystem"]["name"] == "Teamstroke":
                if "COMPLETE" in competition["rawData"]:
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
