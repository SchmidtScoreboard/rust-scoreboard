import inflect

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
            "STATUS_HALFTIME": "INTERMISSION"
        }
        if status not in status_map:
            raise Exception(f"Status {status} not in map")
        else:
            return status_map[status]

    def toOrdinal(period: int):
        return p.ordinal(period)

    def from_json(json, team_func):
        try:
            competition = json["competitions"][0]
            home_team, away_team = competition["competitors"]
            return Common.createCommon(
                team_func(home_team["id"], home_team),
                team_func(away_team["id"], away_team),
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
