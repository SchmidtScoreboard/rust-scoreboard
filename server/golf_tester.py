import re
import golf


data= """FIRST  ROUND - IN PROGRESS
-------------------------------------

      TEAM                              TO PAR        THRU
      ----                               -----        ----
  1.  Dufner/Bozzelli                       -4           4
  2.  Tway/Kraft                            -2           4
  2.  Vegas/Romero                          -2           3
  4.  Putnam/Harrington                     -1           4
  4.  Kim/Hagy                              -1           2
  4.  Maham/Haas                            -1           3
  4.  Todd/Kirk                             -1           1
  4.  Reavie/Glover                         -1           1
  4.  Horschel/Burns                        -1           1
  4.  Pan/Zhang                             -1           2
  4.  Thompson/Gordon                       -1           2
 12.  D. Lee/Bae                            E            3
 12.  Stadler/Wagner                        E            1
 12.  Trahan/Barnes                         E            3
 12.  Watson/Scheffler                      E            1
 12.  Schauffele/Cantlay                    E            1
 12.  Watson/Scheffler                      E            1
 12.  Champ/Finau                           E            1
 18.  Blaum/Byrd                            1            4
 18.  Austin/Mediate                        1            2
 18.  Laird/N. Taylor                       1            1
 """

TEAMSTROKE_REGEX = re.compile(".*\s([a-zA-z]+)\/([a-zA-z]+)\s*([^\s])+")
# TEAMSTROKE_REGEX = re.compile(".*\s([a-zA-z]+)")
for line in data.splitlines():
    match = TEAMSTROKE_REGEX.match(line)
    if match:
        # print(f"LINE: {line} match: {str(match)} groups: {match.groups()}")
        print(f"LINE: {line} groups: {match.groups()}")

