import inflect

class Team:
    def createTeam(id, display_name, abbreviation, primary_color, secondary_color):
        return {
            "id": id,
            "display_name": display_name,
            "abbreviation": abbreviation,
            "primary_color": primary_color,
            "secondary_color": secondary_color,
        }

    
class Common:
    def createCommon(home_team, away_team, status, ordinal, start_time, id=0, home_score=0, away_score=0):
        return {"home_team": home_team, 
            "away_team": away_team,
            "home_score": home_score,
            "away_score": away_score,
            "status": status,
            "ordinal": ordinal,
            "start_time": start_time,
            "id": id}

    def convertStatus(status: str):
        status_map = {"STATUS_IN_PROGRESS": "ACTIVE", "STATUS_FINAL": "END"}
        if status not in status_map:
            print f"Status {status} not in map"
            return status
        else:
            return status_map[status]

    def toOrdinal(period: int):
        p = inflect.engine
        return p.ordinal(period)


    def from_json(json, team_func):
        competition = json["competitions"][0]
        away_team, home_team = competition["competitors"]
        print(competition)
         
        
        return Common.createCommon(
                team_func(home_team["id"], home_team), 
                team_func(away_team["id"], away_team),
                convertStatus(competition["status"]["type"]["name"]),
                toOrdinal(competition["status"]["period"]),
                competition["date"], 
                competition["id"], 
                home_team["score"], 
                away_team["score"])
    




