from common import Common, Team, SportId, pretty_print
from fetcher import Fetcher
from ncaa import team_map
import asyncio


class CollegeFootball_v2:
    def create_game(common, game):
        if common is None:
            return None
        competition = game["competitions"][0]
        situation = competition.get("situation")
        status = competition.get("status")
        time_remaining = status.get(
            "displayClock", "") if status is not None else ""
        if time_remaining == "0:00":
            time_remaining = ""
        ball_position = situation.get(
            "possessionText", "") if situation is not None else ""
        down_string = situation.get(
            "shortDownDistanceText", "") if situation is not None else ""
        down_string = down_string.replace("&", "+")

        home_team, away_team = competition["competitors"]
        possessing_team_id = situation.get(
            "possession", None) if situation is not None else None
        if possessing_team_id is None:
            home_possession = None
        else:
            home_possession = possessing_team_id == home_team["id"]
        return {"type": "CollegeFootball", "common": common, "extra_data": {
            "time_remaining": time_remaining,
            "ball_position": ball_position,
            "down_string": down_string,
            "home_possession": home_possession
        }}

    async def get_games(testing: bool):
        if testing:
            return Common.get_testing_games("college-football")
        else:
            raw_games = await Fetcher.espn_fetch("football", "college-football", "scoreboard?groups=80")
            games = [
                CollegeFootball_v2.create_game(
                    Common.from_espn_json(
                        game, Team.get_espn_team, team_map, SportId.COLLEGE_FOOTBALL
                    ),
                    game
                )
                for game in raw_games
            ]
            return [g for g in games if g]


async def main():
    print("Fetching games")
    games = await CollegeFootball_v2.get_games(False)
    pretty_print(games)


if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
    loop.close()
