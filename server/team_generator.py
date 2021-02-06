import sys
import json
from common import Team


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("ERROR: must specify input json and output json")
        sys.exit(1)

    def get_teams(filename):
        print(f"Opening file {filename}")
        with open(filename) as f:
            j = json.load(f)
            json_teams = j["sports"][0]["leagues"][0]["teams"]
            return {
                team["id"]: Team.createTeam(
                    int(team["id"]),
                    team["shortDisplayName"],
                    team["abbreviation"],
                    team["color"],
                    team.get("alternateColor", "000000"),
                )
                for team in [team["team"] for team in json_teams]
            }

    teams = get_teams(sys.argv[1])
    print("Parsed file")

    with open(sys.argv[2], "w+") as out:
        print(f"Outputting to file {sys.argv[2]}")
        json.dump(teams, out)
