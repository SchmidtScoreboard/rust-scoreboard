import inflect
from team_generator import get_app_color_shit, getDisplayName
from color import processTeamColors


p = inflect.engine()


class Team:
    def createTeam(
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

    def getTeam(team_id, competitor, team_map):
        if team_id in team_map:
            return team_map[team_id]
        else:
            team = competitor["team"]

            team_id = team["id"]  # string
            location = team["location"]
            name = team["name"]
            abbreviation = team["abbreviation"]
            display_name = getDisplayName(team)
            color = team["color"]
            secondary_color = team.get("alternateColor", "000000")
            color, secondary_color = processTeamColors(color, secondary_color)
            server_out = f'"{team_id}": Team.createTeam("{team_id}", "{location}", "{name}", "{display_name}", "{abbreviation}", "{color}", "{secondary_color}"),'
            app_out = f'{team_id}: Team({team_id}, "{location}", "{name}", "{abbreviation}", {get_app_color_shit(color)}, {get_app_color_shit(secondary_color)}),'
            print(f"Unknown team!\n {server_out}\n{app_out}")
            team = Team.createTeam(
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
    def createCommon(
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
            "home_team": home_team,
            "away_team": away_team,
            "home_score": home_score,
            "away_score": away_score,
            "status": status,
            "ordinal": ordinal,
            "start_time": start_time,
            "id": id,
        }

    def convertStatus(status: str):
        status_map = {
            "STATUS_IN_PROGRESS": "ACTIVE",
            "STATUS_FINAL": "END",
            "STATUS_SCHEDULED": "PREGAME",
            "STATUS_END_PERIOD": "INTERMISSION",
            "STATUS_HALFTIME": "INTERMISSION",
            "STATUS_POSTPONED": None,
        }

        if status not in status_map:
            raise Exception(f"Status {status} not in map")
        else:
            return status_map[status]

    def toOrdinal(period: int):
        return p.ordinal(period)

    def from_json(json, team_func, team_map):
        try:
            competition = json["competitions"][0]
            home_team, away_team = competition["competitors"]
            return Common.createCommon(
                team_func(home_team["id"], home_team, team_map),
                team_func(away_team["id"], away_team, team_map),
                Common.convertStatus(competition["status"]["type"]["name"]),
                Common.toOrdinal(competition["status"]["period"]),
                competition["date"],
                competition["id"],
                int(home_team["score"]),
                int(away_team["score"]),
            )
        except Exception as e:
            print(e)
            return None
