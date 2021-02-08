import sys
import json
from common import Team


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("ERROR: must specify input json")
        sys.exit(1)

    def get_app_color_shit(color: str):
        red = int(color[0:2], 16)
        green = int(color[2:4], 16)
        blue = int(color[4:6], 16)
        return f"Color.fromRGBO({red}, {green}, {blue}, 1.0)"

    def get_teams(filename):
        print(f"Opening file {filename}")
        with open(filename) as f:
            j = json.load(f)
            json_teams = j["sports"][0]["leagues"][0]["teams"]
            server_teams = []
            app_teams = []
            for json_team in json_teams:
                team = json_team["team"]
                team_id = team["id"]  # string
                location = team["location"]
                name = team["name"]
                display_name = team["shortDisplayName"]
                abbreviation = team["abbreviation"]
                color = team["color"]
                secondary_color = team.get("alternateColor", "000000")
                server_teams.append(
                    f'"{team_id}": Team.createTeam("{team_id}", "{location}", "{name}", "{display_name}", "{abbreviation}", "{color}", "{secondary_color}"),'
                )
                app_teams.append(
                    f'{team_id}: Team({team_id}, "{location}", "{name}", "{abbreviation}", {get_app_color_shit(color)}, {get_app_color_shit(secondary_color)}),'
                )

            print(f"SERVER TEAMS")
            for team in server_teams:
                print(team)
            print(f"\n\nAPP TEAMS")
            for team in app_teams:
                print(team)

    get_teams(sys.argv[1])