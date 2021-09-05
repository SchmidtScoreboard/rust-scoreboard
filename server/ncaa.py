from common import Team

team_map = {
    "2000": Team.create_team(
        "2000",
        "Abilene Christian",
        "Wildcats",
        "Abil Christ",
        "ACU",
        "4e2683",
        "ebebeb",
    ),
    "2005": Team.create_team(
        "2005", "Air Force", "Falcons", "Air Force", "AFA", "004a7b", "ffffff"
    ),
    "2006": Team.create_team(
        "2006", "Akron", "Zips", "Akron", "AKR", "00285e", "ffffff"
    ),
    "2010": Team.create_team(
        "2010", "Alabama A&M", "Bulldogs", "Alabama A&M", "AAMU", "790000", "ffffff"
    ),
    "333": Team.create_team(
        "333", "Alabama", "Crimson Tide", "Alabama", "ALA", "690014", "f1f2f3"
    ),
    "2011": Team.create_team(
        "2011", "Alabama State", "Hornets", "Alabama St", "ALST", "e9a900", "0a0a0a"
    ),
    "399": Team.create_team(
        "399", "Albany", "Great Danes", "Albany", "ALB", "3D2777", "ffffff"
    ),
    "2016": Team.create_team(
        "2016", "Alcorn State", "Braves", "Alcorn St", "ALCN", "4b0058", "ffffff"
    ),
    "44": Team.create_team(
        "44", "American", "Eagles", "American", "AMER", "c41130", "ffffff"
    ),
    "2026": Team.create_team(
        "2026", "Appalachian State", "Mountaineers", "App St", "APP", "000000", "ffcd00"
    ),
    "9": Team.create_team(
        "9", "Arizona State", "Sun Devils", "Arizona St", "ASU", "942139", "f1f2f3"
    ),
    "12": Team.create_team(
        "12", "Arizona", "Wildcats", "Arizona", "ARIZ", "002449", "ffffff"
    ),
    "8": Team.create_team(
        "8", "Arkansas", "Razorbacks", "Arkansas", "ARK", "9c1831", "ffffff"
    ),
    "2032": Team.create_team(
        "2032",
        "Arkansas State",
        "Red Wolves",
        "Arkansas St",
        "ARST",
        "e81018",
        "000000",
    ),
    "2029": Team.create_team(
        "2029",
        "Arkansas-Pine Bluff",
        "Golden Lions",
        "Ark-Pine",
        "UAPB",
        "e0aa0f",
        "000000",
    ),
    "349": Team.create_team(
        "349", "Army", "Black Knights", "Army", "ARMY", "ce9c00", "231f20"
    ),
    "2": Team.create_team("2", "Auburn", "Tigers", "Auburn", "AUB", "03244d", "f1f2f3"),
    "2046": Team.create_team(
        "2046", "Austin Peay", "Governors", "Austin Peay", "APSU", "8e0b0b", "ffffff"
    ),
    "252": Team.create_team("252", "BYU", "Cougars", "BYU", "BYU", "001E4C", "ffffff"),
    "2050": Team.create_team(
        "2050", "Ball State", "Cardinals", "Ball State", "BALL", "DA0000", "ffffff"
    ),
    "239": Team.create_team(
        "239", "Baylor", "Bears", "Baylor", "BAY", "004834", "ffb81c"
    ),
    "91": Team.create_team(
        "91", "Bellarmine", "Knights", "Bellarmine", "BELL", "000000", "ffffff"
    ),
    "2057": Team.create_team(
        "2057", "Belmont", "Bruins", "Belmont", "BEL", "182142", "ffffff"
    ),
    "2066": Team.create_team(
        "2066", "Binghamton", "Bearcats", "Binghamton", "BING", "00614A", "f0f0f0"
    ),
    "68": Team.create_team(
        "68", "Boise State", "Broncos", "Boise State", "BSU", "09347A", "d8d9da"
    ),
    "103": Team.create_team(
        "103", "Boston College", "Eagles", "Boston C", "BC", "88001a", "ffffff"
    ),
    "104": Team.create_team(
        "104", "Boston Univ.", "Terriers", "Boston U", "BU", "cc0000", "ffffff"
    ),
    "189": Team.create_team(
        "189", "Bowling Green", "Falcons", "Bowling G", "BGSU", "2b1000", "ffffff"
    ),
    "71": Team.create_team(
        "71", "Bradley", "Braves", "Bradley", "BRAD", "b70002", "c0c0c0"
    ),
    "225": Team.create_team(
        "225", "Brown", "Bears", "Brown", "BRWN", "411e09", "949300"
    ),
    "2083": Team.create_team(
        "2083", "Bucknell", "Bison", "Bucknell", "BUCK", "000060", "ffffff"
    ),
    "2084": Team.create_team(
        "2084", "Buffalo", "Bulls", "Buffalo", "BUFF", "041A9B", "ebebeb"
    ),
    "2086": Team.create_team(
        "2086", "Butler", "Bulldogs", "Butler", "BUT", "0d1361", "00a3e0"
    ),
    "2239": Team.create_team(
        "2239", "CSU Fullerton", "Titans", "Fullerton", "CSUF", "10219c", "ffffff"
    ),
    "13": Team.create_team(
        "13", "Cal Poly", "Mustangs", "Cal Poly", "CP", "1E4D2B", "eed897"
    ),
    "25": Team.create_team(
        "25", "California", "Golden Bears", "California", "CAL", "031522", "ffc423"
    ),
    "2097": Team.create_team(
        "2097", "Campbell", "Fighting Camels", "Campbell", "CAM", "000000", "ffffff"
    ),
    "2099": Team.create_team(
        "2099", "Canisius", "Golden Griffins", "Canisius", "CAN", "004a81", "dda50f"
    ),
    "2110": Team.create_team(
        "2110", "Central Arkansas", "Bears", "Cent Ark", "UCA", "a7a9ac", "000000"
    ),
    "2115": Team.create_team(
        "2115",
        "Central Connecticut",
        "Blue Devils",
        "Cent Conn",
        "CCSU",
        "1B49A2",
        "d1d5d8",
    ),
    "2117": Team.create_team(
        "2117", "Central Michigan", "Chippewas", "Cent Mich", "CMU", "6a0032", "ffffff"
    ),
    "232": Team.create_team(
        "232", "Charleston", "Cougars", "Charleston", "COFC", "9c8456", "000000"
    ),
    "2127": Team.create_team(
        "2127",
        "Charleston Southern",
        "Buccaneers",
        "Charleston S",
        "CHSO",
        "2e3192",
        "ded090",
    ),
    "236": Team.create_team(
        "236", "Chattanooga", "Mocs", "Chattanooga", "UTC", "00386b", "dca71d"
    ),
    "2130": Team.create_team(
        "2130", "Chicago State", "Cougars", "Chicago St", "CHIC", "006700", "ffffff"
    ),
    "2132": Team.create_team(
        "2132", "Cincinnati", "Bearcats", "Cincinnati", "CIN", "000000", "717073"
    ),
    "228": Team.create_team(
        "228", "Clemson", "Tigers", "Clemson", "CLEM", "F66733", "000000"
    ),
    "325": Team.create_team(
        "325", "Cleveland State", "Vikings", "C St", "CLEV", "006633", "ffffff"
    ),
    "324": Team.create_team(
        "324",
        "Coastal Carolina",
        "Chanticleers",
        "C. Carolina",
        "CCAR",
        "007073",
        "ffffff",
    ),
    "2142": Team.create_team(
        "2142", "Colgate", "Raiders", "Colgate", "COLG", "8B011D", "ffffff"
    ),
    "38": Team.create_team(
        "38", "Colorado", "Buffaloes", "Colorado", "COLO", "d1c57e", "000000"
    ),
    "36": Team.create_team(
        "36", "Colorado State", "Rams", "Colorado St", "CSU", "004537", "ffc425"
    ),
    "171": Team.create_team(
        "171", "Columbia", "Lions", "Columbia", "CLMB", "174785", "ffffff"
    ),
    "2154": Team.create_team(
        "2154", "Coppin St", "Eagles", "Coppin St", "COPP", "2e3192", "ffd204"
    ),
    "172": Team.create_team(
        "172", "Cornell", "Big Red", "Cornell", "COR", "d60027", "101010"
    ),
    "156": Team.create_team(
        "156", "Creighton", "Bluejays", "Creighton", "CREI", "13299e", "ffffff"
    ),
    "159": Team.create_team(
        "159", "Dartmouth", "Big Green", "Dartmouth", "DART", "005730", "ffffff"
    ),
    "2166": Team.create_team(
        "2166", "Davidson", "Wildcats", "Davidson", "DAV", "000000", "e51837"
    ),
    "2168": Team.create_team(
        "2168", "Dayton", "Flyers", "Dayton", "DAY", "004B8D", "ffffff"
    ),
    "305": Team.create_team(
        "305", "DePaul", "Blue Demons", "DePaul", "DEP", "2d649c", "ffffff"
    ),
    "48": Team.create_team(
        "48", "Delaware", "Blue Hens", "Delaware", "DEL", "033594", "e8ce31"
    ),
    "2169": Team.create_team(
        "2169", "Delaware St", "Hornets", "Delaware St", "DSU", "FF3630", "000000"
    ),
    "2172": Team.create_team(
        "2172", "Denver", "Pioneers", "Denver", "DEN", "9c143d", "d6ba74"
    ),
    "2174": Team.create_team(
        "2174", "Detroit Mercy", "Titans", "Detroit M", "DET", "165b9e", "ffffff"
    ),
    "2181": Team.create_team(
        "2181", "Drake", "Bulldogs", "Drake", "DRKE", "004477", "c0c0c0"
    ),
    "2182": Team.create_team(
        "2182", "Drexel", "Dragons", "Drexel", "DREX", "020260", "ffd65a"
    ),
    "150": Team.create_team(
        "150", "Duke", "Blue Devils", "Duke", "DUKE", "001A57", "f1f2f3"
    ),
    "2184": Team.create_team(
        "2184", "Duquesne", "Dukes", "Duquesne", "DUQ", "002D62", "ffffff"
    ),
    "151": Team.create_team(
        "151", "East Carolina", "Pirates", "E Carolina", "ECU", "4b1869", "f0907b"
    ),
    "2193": Team.create_team(
        "2193", "East Tennessee State", "Buccaneers", "ETSU", "ETSU", "002d61", "ffc423"
    ),
    "2197": Team.create_team(
        "2197", "Eastern Illinois", "Panthers", "E Illinois", "EIU", "000000", "bebab9"
    ),
    "2198": Team.create_team(
        "2198", "Eastern Kentucky", "Colonels", "E Kentucky", "EKU", "660819", "f0f0f0"
    ),
    "2199": Team.create_team(
        "2199", "Eastern Michigan", "Eagles", "E Michigan", "EMU", "00331b", "f0f0f0"
    ),
    "331": Team.create_team(
        "331", "Eastern Washington", "Eagles", "E Wash", "EWU", "a10022", "abb4bc"
    ),
    "2210": Team.create_team(
        "2210", "Elon", "Phoenix", "Elon", "ELON", "020303", "b59a57"
    ),
    "339": Team.create_team(
        "339", "Evansville", "Purple Aces", "Evansville", "EVAN", "663399", "ffffff"
    ),
    "2217": Team.create_team(
        "2217", "Fairfield", "Stags", "Fairfield", "FAIR", "000000", "ebebeb"
    ),
    "161": Team.create_team(
        "161", "Fairleigh Dickinson", "Knights", "Fair Dick", "FDU", "00449C", "ffffff"
    ),
    "50": Team.create_team(
        "50", "Florida A&M", "Rattlers", "Florida A&M", "FAMU", "F89728", "000000"
    ),
    "2226": Team.create_team(
        "2226", "Florida Atlantic", "Owls", "FAU", "FAU", "004B85", "ffffff"
    ),
    "57": Team.create_team(
        "57", "Florida", "Gators", "Florida", "FLA", "0021A5", "ffffff"
    ),
    "526": Team.create_team(
        "526", "Florida Gulf Coast", "Eagles", "FGCU", "FGCU", "00885a", "000000"
    ),
    "2229": Team.create_team(
        "2229", "Florida Int'l", "Panthers", "FIU", "FIU", "091731", "c5960c"
    ),
    "52": Team.create_team(
        "52", "Florida State", "Seminoles", "Florida St", "FSU", "782F40", "ceb888"
    ),
    "2230": Team.create_team(
        "2230", "Fordham", "Rams", "Fordham", "FOR", "830032", "ffffff"
    ),
    "278": Team.create_team(
        "278", "Fresno State", "Bulldogs", "Fresno St", "FRES", "c41230", "ffffff"
    ),
    "231": Team.create_team(
        "231", "Furman", "Paladins", "Furman", "FUR", "4A2184", "909090"
    ),
    "2241": Team.create_team(
        "2241", "Gardner-Webb", "Bulldogs", "Gardner W", "GWEB", "c12535", "ffffff"
    ),
    "2244": Team.create_team(
        "2244", "George Mason", "Patriots", "George M", "GMU", "016600", "ecb010"
    ),
    "45": Team.create_team(
        "45", "George Washington", "Colonials", "G Wash", "GW", "002843", "e8d2a1"
    ),
    "46": Team.create_team(
        "46", "Georgetown", "Hoyas", "Georgetown", "GTWN", "110E42", "ffffff"
    ),
    "61": Team.create_team(
        "61", "Georgia", "Bulldogs", "Georgia", "UGA", "CC0000", "000000"
    ),
    "290": Team.create_team(
        "290", "Georgia Southern", "Eagles", "GA Southern", "GASO", "003775", "f0f0f0"
    ),
    "2247": Team.create_team(
        "2247", "Georgia State", "Panthers", "Georgia St", "GAST", "1e539a", "ebebeb"
    ),
    "59": Team.create_team(
        "59", "Georgia Tech", "Yellow Jackets", "G Tech", "GT", "00223e", "ffffff"
    ),
    "2250": Team.create_team(
        "2250", "Gonzaga", "Bulldogs", "Gonzaga", "GONZ", "002967", "cfd4d8"
    ),
    "2253": Team.create_team(
        "2253", "Grand Canyon", "Antelopes", "G Canyon", "GCU", "522398", "f0f0f0"
    ),
    "2261": Team.create_team(
        "2261", "Hampton", "Pirates", "Hampton", "HAMP", "0067AC", "000000"
    ),
    "42": Team.create_team(
        "42", "Hartford", "Hawks", "Hartford", "HART", "d60008", "000000"
    ),
    "108": Team.create_team(
        "108", "Harvard", "Crimson", "Harvard", "HARV", "990000", "dbdbdb"
    ),
    "62": Team.create_team(
        "62", "Hawai'i", "Rainbow Warriors", "Hawai'i", "HAW", "003420", "ffffff"
    ),
    "2272": Team.create_team(
        "2272", "High Point", "Panthers", "High Point", "HP", "b0b7bc", "000000"
    ),
    "2275": Team.create_team(
        "2275", "Hofstra", "Pride", "Hofstra", "HOF", "00337c", "f6c934"
    ),
    "107": Team.create_team(
        "107", "Holy Cross", "Crusaders", "Holy Cross", "HC", "0a0a0a", "ffffff"
    ),
    "2277": Team.create_team(
        "2277", "Houston Baptist", "Huskies", "H Baptist", "HBU", "00539c", "ffffff"
    ),
    "248": Team.create_team(
        "248", "Houston", "Cougars", "Houston", "HOU", "C90822", "ffffff"
    ),
    "47": Team.create_team("47", "Howard", "Bison", "Howard", "HOW", "9e0b0e", "ffffff"),
    "85": Team.create_team(
        "85", "IUPUI", "Jaguars", "IUPUI", "IUPU", "A81F30", "ffffff"
    ),
    "304": Team.create_team(
        "304", "Idaho State", "Bengals", "Idaho State", "IDST", "ef8c00", "000000"
    ),
    "70": Team.create_team(
        "70", "Idaho", "Vandals", "Idaho", "IDHO", "000000", "8c6e4a"
    ),
    "356": Team.create_team(
        "356", "Illinois", "Fighting Illini", "Illinois", "ILL", "f77329", "000000"
    ),
    "2287": Team.create_team(
        "2287", "Illinois State", "Redbirds", "Illinois St", "ILST", "CE1126", "ffe716"
    ),
    "84": Team.create_team(
        "84", "Indiana", "Hoosiers", "Indiana", "IU", "7D110C", "eeedeb"
    ),
    "282": Team.create_team(
        "282", "Indiana State", "Sycamores", "Indiana St", "INST", "00669a", "f0f0f0"
    ),
    "314": Team.create_team("314", "Iona", "Gaels", "Iona", "IONA", "8c001a", "f6a704"),
    "2294": Team.create_team(
        "2294", "Iowa", "Hawkeyes", "Iowa", "IOWA", "000000", "ffe100"
    ),
    "66": Team.create_team(
        "66", "Iowa State", "Cyclones", "Iowa State", "ISU", "660015", "ffffff"
    ),
    "2296": Team.create_team(
        "2296", "Jackson State", "Tigers", "Jackson St", "JKST", "123297", "b5b7ba"
    ),
    "294": Team.create_team(
        "294", "Jacksonville", "Dolphins", "Jack", "JAX", "00523e", "ffffff"
    ),
    "55": Team.create_team(
        "55", "Jacksonville State", "Gamecocks", "Jack St", "JVST", "b50500", "ffffff"
    ),
    "256": Team.create_team(
        "256", "James Madison", "Dukes", "James M", "JMU", "450084", "cbb778"
    ),
    "2305": Team.create_team(
        "2305", "Kansas", "Jayhawks", "Kansas", "KU", "0022B4", "ffffff"
    ),
    "2306": Team.create_team(
        "2306", "Kansas State", "Wildcats", "Kansas St", "KSU", "633194", "e7d2ad"
    ),
    "338": Team.create_team(
        "338", "Kennesaw State", "Owls", "Kennesaw St", "KENN", "000000", "fdbb30"
    ),
    "2309": Team.create_team(
        "2309", "Kent State", "Golden Flashes", "Kent State", "KENT", "002445", "f0b510"
    ),
    "96": Team.create_team(
        "96", "Kentucky", "Wildcats", "Kentucky", "UK", "005DAA", "ffffff"
    ),
    "99": Team.create_team("99", "LSU", "Tigers", "LSU", "LSU", "2B0D57", "fdd023"),
    "2325": Team.create_team(
        "2325", "La Salle", "Explorers", "La Salle", "LAS", "000063", "feca26"
    ),
    "322": Team.create_team(
        "322", "Lafayette", "Leopards", "Lafayette", "LAF", "790000", "a59474"
    ),
    "2320": Team.create_team(
        "2320", "Lamar", "Cardinals", "Lamar", "LAM", "000000", "ebebeb"
    ),
    "2329": Team.create_team(
        "2329", "Lehigh", "Mountain Hawks", "Lehigh", "LEH", "6c2b2a", "b69e70"
    ),
    "2335": Team.create_team(
        "2335", "Liberty", "Flames", "Liberty", "LIB", "071740", "ffffff"
    ),
    "288": Team.create_team(
        "288", "Lipscomb", "Bisons", "Lipscomb", "LIP", "20366C", "f6b734"
    ),
    "2031": Team.create_team(
        "2031", "Little Rock", "Trojans", "Little Rock", "UALR", "AD0000", "ffffff"
    ),
    "299": Team.create_team(
        "299", "Long Beach State", "Beach", "LB State", "LBSU", "000000", "f1f2f3"
    ),
    "2344": Team.create_team(
        "2344", "Longwood", "Lancers", "Longwood", "LONG", "003273", "9ea2a3"
    ),
    "309": Team.create_team(
        "309", "Louisiana", "Ragin' Cajuns", "Louisiana", "ULL", "ce2842", "000000"
    ),
    "2348": Team.create_team(
        "2348", "Louisiana Tech", "Bulldogs", "L Tech", "LT", "002d65", "ffffff"
    ),
    "97": Team.create_team(
        "97", "Louisville", "Cardinals", "Louisville", "LOU", "ad000a", "cccccc"
    ),
    "2352": Team.create_team(
        "2352", "Loyola (MD)", "Greyhounds", "Loyola (MD)", "L-MD", "76a7a0", "000000"
    ),
    "2350": Team.create_team(
        "2350", "Loyola Chicago", "Ramblers", "Loyola C", "LUC", "9d1244", "ffffff"
    ),
    "2351": Team.create_team(
        "2351", "Loyola Marymount", "Lions", "Loyola M", "LMU", "880029", "ffffff"
    ),
    "311": Team.create_team(
        "311", "Maine", "Black Bears", "Maine", "ME", "127dbe", "000000"
    ),
    "269": Team.create_team(
        "269", "Marquette", "Golden Eagles", "Marquette", "MARQ", "083963", "ffffff"
    ),
    "276": Team.create_team(
        "276", "Marshall", "Thundering Herd", "Marshall", "MRSH", "186329", "ffffff"
    ),
    "120": Team.create_team(
        "120", "Maryland", "Terrapins", "Maryland", "MD", "D5002B", "ffcd00"
    ),
    "235": Team.create_team(
        "235", "Memphis", "Tigers", "Memphis", "MEM", "002447", "ffffff"
    ),
    "193": Team.create_team(
        "193", "Miami (OH)", "Redhawks", "Miami (OH)", "M-OH", "a4000c", "f0f0f0"
    ),
    "127": Team.create_team(
        "127", "Michigan State", "Spartans", "Michigan St", "MSU", "18453B", "ffffff"
    ),
    "130": Team.create_team(
        "130", "Michigan", "Wolverines", "Michigan", "MICH", "00274c", "ffcb05"
    ),
    "270": Team.create_team(
        "270", "Milwaukee", "Panthers", "Milwaukee", "MILW", "000000", "ffc20e"
    ),
    "135": Team.create_team(
        "135", "Minnesota", "Golden Gophers", "Minnesota", "MINN", "981a31", "ffffff"
    ),
    "344": Team.create_team(
        "344", "Mississippi State", "Bulldogs", "Miss St", "MSST", "762123", "c8c8c8"
    ),
    "142": Team.create_team(
        "142", "Missouri", "Tigers", "Missouri", "MIZ", "000000", "ffffff"
    ),
    "149": Team.create_team(
        "149", "Montana", "Grizzlies", "Montana", "MONT", "751D4A", "ffffff"
    ),
    "147": Team.create_team(
        "147", "Montana State", "Bobcats", "Montana St", "MTST", "003875", "bf965c"
    ),
    "116": Team.create_team(
        "116",
        "Mount St. Mary's",
        "Mountaineers",
        "Mt St Mary",
        "MSM",
        "005596",
        "ebebeb",
    ),
    "93": Team.create_team(
        "93", "Murray State", "Racers", "Murray St", "MUR", "002148", "ffffff"
    ),
    "152": Team.create_team(
        "152", "NC State", "Wolfpack", "NC State", "NCST", "EF1216", "231f20"
    ),
    "158": Team.create_team(
        "158", "Nebraska", "Cornhuskers", "Nebraska", "NEB", "F20017", "f5f1e7"
    ),
    "160": Team.create_team(
        "160", "New Hampshire", "Wildcats", "New Hamp", "UNH", "004990", "c3c4c6"
    ),
    "167": Team.create_team(
        "167", "New Mexico", "Lobos", "New Mexico", "UNM", "000000", "ffffff"
    ),
    "166": Team.create_team(
        "166", "New Mexico State", "Aggies", "New Mex St", "NMSU", "891216", "ffffff"
    ),
    "315": Team.create_team(
        "315", "Niagara", "Purple Eagles", "Niagara", "NIAG", "69207E", "f0f0f0"
    ),
    "153": Team.create_team(
        "153", "North Carolina", "Tar Heels", "N Carolina", "UNC", "99bfe5", "13294b"
    ),
    "155": Team.create_team(
        "155", "North Dakota", "Fighting Hawks", "N Dakota", "UND", "00A26B", "000000"
    ),
    "249": Team.create_team(
        "249", "North Texas", "Mean Green", "North Texas", "UNT", "00853D", "000000"
    ),
    "111": Team.create_team(
        "111", "Northeastern", "Huskies", "NE", "NE", "CC0001", "ffffff"
    ),
    "94": Team.create_team(
        "94", "Northern Kentucky", "Norse", "N Kentucky", "NKU", "000000", "eab621"
    ),
    "77": Team.create_team(
        "77", "Northwestern", "Wildcats", "N Western", "NW", "372286", "d6cac1"
    ),
    "87": Team.create_team(
        "87", "Notre Dame", "Fighting Irish", "Notre Dame", "ND", "00122b", "ae9142"
    ),
    "195": Team.create_team(
        "195", "Ohio", "Bobcats", "Ohio", "OHIO", "295A29", "e4bb85"
    ),
    "194": Team.create_team(
        "194", "Ohio State", "Buckeyes", "Ohio State", "OSU", "bb0000", "ffffff"
    ),
    "201": Team.create_team(
        "201", "Oklahoma", "Sooners", "Oklahoma", "OU", "7b0000", "cccccc"
    ),
    "197": Team.create_team(
        "197", "Oklahoma State", "Cowboys", "Oklahoma St", "OKST", "FF6500", "000000"
    ),
    "295": Team.create_team(
        "295", "Old Dominion", "Monarchs", "Old D", "ODU", "00507d", "a1d2f1"
    ),
    "145": Team.create_team(
        "145", "Ole Miss", "Rebels", "Ole Miss", "MISS", "001148", "ffffff"
    ),
    "198": Team.create_team(
        "198", "Oral Roberts", "Golden Eagles", "Oral R", "ORU", "002955", "ccb48c"
    ),
    "204": Team.create_team(
        "204", "Oregon State", "Beavers", "Oregon St", "ORST", "c34500", "ffffff"
    ),
    "279": Team.create_team(
        "279", "Pacific", "Tigers", "Pacific", "PAC", "F47820", "000000"
    ),
    "213": Team.create_team(
        "213", "Penn State", "Nittany Lions", "Penn State", "PSU", "00265D", "ffffff"
    ),
    "219": Team.create_team(
        "219", "Pennsylvania", "Quakers", "Penn", "PENN", "082A74", "ffffff"
    ),
    "221": Team.create_team(
        "221", "Pittsburgh", "Panthers", "Pittsburgh", "PITT", "003263", "ffffff"
    ),
    "163": Team.create_team(
        "163", "Princeton", "Tigers", "Princeton", "PRIN", "ff9408", "080808"
    ),
    "227": Team.create_team(
        "227", "Rhode Island", "Rams", "Rhode Isl", "URI", "091f3f", "5ab3e8"
    ),
    "242": Team.create_team("242", "Rice", "Owls", "Rice", "RICE", "003D7D", "d1d5d8"),
    "257": Team.create_team(
        "257", "Richmond", "Spiders", "Richmond", "RICH", "9e0712", "ffffff"
    ),
    "164": Team.create_team(
        "164", "Rutgers", "Scarlet Knights", "Rutgers", "RUTG", "d21034", "ffffff"
    ),
    "16": Team.create_team(
        "16", "Sacramento State", "Hornets", "Sac St", "SAC", "00573C", "cdb97d"
    ),
    "139": Team.create_team(
        "139", "Saint Louis", "Billikens", "Saint Louis", "SLU", "00539C", "ebebeb"
    ),
    "21": Team.create_team(
        "21", "San Diego State", "Aztecs", "San D St", "SDSU", "BF2C37", "ffffff"
    ),
    "301": Team.create_team(
        "301", "San Diego", "Toreros", "San Diego", "USD", "2f99d4", "000000"
    ),
    "23": Team.create_team(
        "23", "San José St", "Spartans", "San José St", "SJSU", "005893", "fdba31"
    ),
    "6": Team.create_team(
        "6", "South Alabama", "Jaguars", "S Alabama", "USA", "003E7E", "ffffff"
    ),
    "233": Team.create_team(
        "233", "South Dakota", "Coyotes", "S Dakota", "SDAK", "CD1241", "f0f0f0"
    ),
    "58": Team.create_team(
        "58", "South Florida", "Bulls", "S Florida", "USF", "004A36", "ffffff"
    ),
    "79": Team.create_team(
        "79", "Southern Illinois", "Salukis", "So Illinois", "SIU", "85283D", "c2c3c0"
    ),
    "253": Team.create_team(
        "253", "Southern Utah", "Thunderbirds", "S Utah", "SUU", "d10000", "ffffff"
    ),
    "179": Team.create_team(
        "179", "St. Bonaventure", "Bonnies", "St. Bon", "SBU", "70261D", "ffffff"
    ),
    "24": Team.create_team(
        "24", "Stanford", "Cardinal", "Stanford", "STAN", "A80532", "ffffff"
    ),
    "56": Team.create_team(
        "56", "Stetson", "Hatters", "Stetson", "STET", "0a5640", "ffffff"
    ),
    "183": Team.create_team(
        "183", "Syracuse", "Orange", "Syracuse", "SYR", "F37321", "0d1d37"
    ),
    "218": Team.create_team(
        "218", "Temple", "Owls", "Temple", "TEM", "A80532", "ffffff"
    ),
    "245": Team.create_team(
        "245", "Texas A&M", "Aggies", "Texas A&M", "TA&M", "5C0025", "ffffff"
    ),
    "357": Team.create_team(
        "357", "Texas A&M-CC", "Islanders", "Tex A&M CC", "AMCC", "00639c", "ffffff"
    ),
    "251": Team.create_team(
        "251", "Texas", "Longhorns", "Texas", "TEX", "EE7524", "000000"
    ),
    "326": Team.create_team(
        "326", "Texas State", "Bobcats", "Texas State", "TXST", "4e1719", "b4975a"
    ),
    "119": Team.create_team(
        "119", "Towson", "Tigers", "Towson", "TOW", "FFC229", "000000"
    ),
    "202": Team.create_team(
        "202", "Tulsa", "Golden Hurricane", "Tulsa", "TLSA", "004371", "ffffff"
    ),
    "5": Team.create_team("5", "UAB", "Blazers", "UAB", "UAB", "054338", "ffc845"),
    "302": Team.create_team(
        "302", "UC Davis", "Aggies", "UC Davis", "UCD", "183563", "bc9305"
    ),
    "300": Team.create_team(
        "300", "UC Irvine", "Anteaters", "UC Irvine", "UCI", "002B5C", "fec52e"
    ),
    "27": Team.create_team(
        "27", "UC Riverside", "Highlanders", "UC River", "UCR", "14234F", "ffffff"
    ),
    "28": Team.create_team(
        "28", "UC San Diego", "Tritons", "UC SD", "UCSD", "000000", "ffffff"
    ),
    "2116": Team.create_team("2116", "UCF", "Knights", "UCF", "UCF", "000000", "ffffff"),
    "26": Team.create_team("26", "UCLA", "Bruins", "UCLA", "UCLA", "005C8E", "ffc72c"),
    "41": Team.create_team(
        "41", "UConn", "Huskies", "UConn", "CONN", "001d40", "f1f2f3"
    ),
    "82": Team.create_team("82", "UIC", "Flames", "UIC", "UIC", "234077", "ffffff"),
    "140": Team.create_team(
        "140", "UM Kansas City", "Roos", "UM KC", "UMKC", "004b87", "ffc72c"
    ),
    "2349": Team.create_team(
        "2349", "UMass Lowell", "River Hawks", "UMass L", "UML", "00529C", "ffffff"
    ),
    "113": Team.create_team(
        "113", "UMass", "Minutemen", "UMass", "MASS", "880007", "ffffff"
    ),
    "350": Team.create_team(
        "350", "UNC Wilmington", "Seahawks", "UNC Wilm", "UNCW", "1d2f68", "ffffff"
    ),
    "30": Team.create_team("30", "USC", "Trojans", "USC", "USC", "AE2531", "ffc72c"),
    "250": Team.create_team(
        "250", "UT Arlington", "Mavericks", "UT A", "UTA", "004b7c", "ffffff"
    ),
    "292": Team.create_team(
        "292", "UT Rio Grande Valley", "Vaqueros", "UT RG", "UTRGV", "dc6000", "000000"
    ),
    "328": Team.create_team(
        "328", "Utah State", "Aggies", "Utah State", "USU", "003263", "949ca1"
    ),
    "254": Team.create_team("254", "Utah", "Utes", "Utah", "UTAH", "890012", "ffffff"),
    "238": Team.create_team(
        "238", "Vanderbilt", "Commodores", "Vanderbilt", "VAN", "000000", "ffffff"
    ),
    "261": Team.create_team(
        "261", "Vermont", "Catamounts", "Vermont", "UVM", "013C24", "ffc72c"
    ),
    "222": Team.create_team(
        "222", "Villanova", "Wildcats", "Villanova", "VILL", "123d7C", "f0f0f0"
    ),
    "258": Team.create_team(
        "258", "Virginia", "Cavaliers", "Virginia", "UVA", "f84c1e", "242e4a"
    ),
    "259": Team.create_team(
        "259", "Virginia Tech", "Hokies", "V Tech", "VT", "74232D", "c2c1ba"
    ),
    "154": Team.create_team(
        "154", "Wake Forest", "Demon Deacons", "Wake Forest", "WAKE", "9E7E38", "000000"
    ),
    "264": Team.create_team(
        "264", "Washington", "Huskies", "Washington", "WASH", "2B2F64", "e8e3d3"
    ),
    "265": Team.create_team(
        "265", "Washington State", "Cougars", "Wash St", "WSU", "94022a", "ffffff"
    ),
    "277": Team.create_team(
        "277", "West Virginia", "Mountaineers", "W Virginia", "WVU", "FFC600", "000000"
    ),
    "98": Team.create_team(
        "98", "Western Kentucky", "Hilltoppers", "Western KY", "WKU", "F32026", "000000"
    ),
    "275": Team.create_team(
        "275", "Wisconsin", "Badgers", "Wisconsin", "WISC", "A00002", "f7f7f7"
    ),
    "43": Team.create_team("43", "Yale", "Bulldogs", "Yale", "YALE", "004a81", "ffffff"),
    "2674": Team.create_team(
        "2674", "Valparaiso", "Crusaders", "Valparaiso", "VAL", "794500", "000000"
    ),
    "2509": Team.create_team(
        "2509", "Purdue", "Boilermakers", "Purdue", "PUR", "CEB888", "000000"
    ),
    "2636": Team.create_team("2636", "UTSA", "Roadrunners", "R Runners", "UTSA", "002A5C", "f47321"),
    "2572": Team.create_team("2572", "Southern Miss", "Golden Eagles", "G Eagles", "USM", "FFAA3C", "000000"),
    "2638": Team.create_team("2638", "UTEP", "Miners", "Miners", "UTEP", "ff8200", "041e42"),
    "2390": Team.create_team("2390", "Miami", "Hurricanes", "Miami", "MIA", "004325", "f0f0f0"),
      
}
