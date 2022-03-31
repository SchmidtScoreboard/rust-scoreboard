import inflect
from team_generator import get_app_color_shit, get_display_name
from color import process_team_colors
import os
import json
from dateutil.parser import parse
import datetime
import pytz
from enum import Enum


p = inflect.engine()


class SportId(Enum):
    HOCKEY = 0
    BASEBALL = 1
    COLLEGE_BASKETBALL = 2
    BASKETBALL = 3
    FOOTBALL = 4
    COLLEGE_FOOTBALL = 5
    GOLF = 6


class Team:
    def create_team(
        id, location, name, display_name, abbreviation, primary_color, secondary_color
    ):
        return {
            "id": id,
            "location": location,
            "name": name,
            "display_name": display_name,
            "abbreviation": abbreviation,
            "primary_color": primary_color,
            "secondary_color": secondary_color,
        }

    def get_espn_team(team_id, competitor, team_map):
        if team_id in team_map:
            return team_map[team_id]
        else:
            team = competitor["team"]

            team_id = team["id"]  # string
            location = team["location"]
            name = team["name"]
            abbreviation = team["abbreviation"]
            display_name = get_display_name(team)
            color = team["color"]
            secondary_color = team.get("alternateColor", "000000")
            color, secondary_color = process_team_colors(color, secondary_color)
            server_out = f'"{team_id}": Team.create_team("{team_id}", "{location}", "{name}", "{display_name}", "{abbreviation}", "{color}", "{secondary_color}"),'
            app_out = f'{team_id}: Team({team_id}, "{location}", "{name}", "{abbreviation}", {get_app_color_shit(color)}, {get_app_color_shit(secondary_color)}),'
            print(f"Unknown team!\n {server_out}\n{app_out}")
            team = Team.create_team(
                team_id,
                location,
                name,
                display_name if len(display_name) <= 11 else abbreviation,
                abbreviation,
                color,
                secondary_color,
            )
            return team


class Common:
    def create_common(
        sport_id,
        home_team,
        away_team,
        status,
        ordinal,
        start_time,
        id=0,
        home_score=0,
        away_score=0,
    ):
        return {
            "sport_id": sport_id,
            "home_team": home_team,
            "away_team": away_team,
            "home_score": home_score,
            "away_score": away_score,
            "status": status,
            "ordinal": ordinal,
            "start_time": start_time,
            "id": id,
        }

    def convert_status(status: str):
        status_map = {
            "STATUS_IN_PROGRESS": "Active",
            "STATUS_FINAL": "END",
            "STATUS_SCHEDULED": "Pregame",
            "STATUS_END_PERIOD": "Intermission",
            "STATUS_HALFTIME": "Intermission",
            "STATUS_POSTPONED": None,
            "STATUS_CANCELED": None,
            "STATUS_PLAY_COMPLETE": "END",
            "STATUS_DELAYED": "Intermission"
        }

        if status not in status_map:
            raise Exception(f"Status {status} not in map")
        else:
            return status_map[status]

    def to_ordinal(period: int):
        return p.ordinal(period)

    def from_espn_json(json, team_func, team_map, screen_id):
        try:
            competition = json["competitions"][0]
            home_team, away_team = competition["competitors"]
            espn_status = competition["status"]["type"]["name"]
            status = Common.convert_status(espn_status)
            time = parse(competition["date"]).astimezone(pytz.utc)
            now = datetime.datetime.now(tz=pytz.UTC)
            delta = abs(now - time)
            if delta > datetime.timedelta(hours=12):
                return None

            ordinal = Common.to_ordinal(competition["status"]["period"])
            if status == "Intermission":
                ordinal += " INT"

            if espn_status == "STATUS_HALFTIME":
                ordinal = "HALFTIME"
            if status is not None:
                return Common.create_common(
                    screen_id.value,
                    team_func(home_team["id"], home_team, team_map),
                    team_func(away_team["id"], away_team, team_map),
                    status,
                    ordinal,
                    competition["date"],
                    int(competition["id"]),
                    int(home_team["score"]),
                    int(away_team["score"]),
                )
            else:
                return None
        except Exception as e:
            print(e)
            return None

    def from_schedule_json(json, team_map, screen_id):
        if json["status"]["detailedState"] == "Postponed":
            return None
        try:
            away_team = json["teams"]["away"]["team"]
            home_team = json["teams"]["home"]["team"]
            return Common.create_common(
                screen_id.value,
                team_map[home_team["id"]],
                team_map[away_team["id"]],
                "",  # status
                "",  # ordinal
                json["gameDate"],
                json["gamePk"],
                0,
                0,
            )
        except Exception as e:
            print(e)
            return None

    def get_testing_games(game_type: str):

        path = os.path.join("saved-games", f"{game_type}.json")
        try:
            with open(path) as games_file:
                data = json.load(games_file)
                return data["games"]
        except Exception as e:
            print(e)
            return []
