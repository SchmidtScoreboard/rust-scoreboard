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
    def from_json(json, team_func):
        return createCommon(team_func(1, json), team_func(0, json), "PREGAME", "1st",
        "INSERT_DATE", 0, 0, 0)
    




