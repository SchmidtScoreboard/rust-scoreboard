import sys
import json
from common import Team
from color import getRGBFromHex, processTeamColors


def get_app_color_shit(color: str):
    red, green, blue = getRGBFromHex(color)
    return f"Color.fromRGBO({red}, {green}, {blue}, 1.0)"


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("ERROR: must specify input json")
        sys.exit(1)

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
                raw_display_name = team["shortDisplayName"]
                abbreviation = team["abbreviation"]
                color = team["color"]
                secondary_color = team.get("alternateColor", "000000")
                color, secondary_color = processTeamColors(color, secondary_color)

                display_name = raw_display_name
                if len(raw_display_name) > 11:
                    splat = raw_display_name.split()
                    display_name = raw_display_name
                    if splat[-1] == "State":
                        splat[-1] = "St"
                        display_name = " ".join(splat)
                        print(
                            f"Shortened state '{raw_display_name}' name to '{display_name}'"
                        )
                    directions = {
                        "North": "N",
                        "East": "E",
                        "South": "S",
                        "West": "W",
                        "Central": "C",
                    }
                    if splat[0] in directions:
                        splat[0] = directions[splat[0]]
                        display_name = " ".join(splat)
                        print(
                            f"Shortened state '{raw_display_name}' name to '{display_name}'"
                        )

                    if len(display_name) > 11:
                        display_name = input(
                            f"Display name {display_name} too long: {team}, enter alternative\n\n>"
                        )
                        print(
                            f"Got alternative name '{display_name}' length {len(display_name)}"
                        )

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
