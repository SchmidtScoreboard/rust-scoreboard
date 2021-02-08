from common import Common, Team
from fetcher import Fetcher

team_map = {
    "2000": Team.createTeam(
        "2000",
        "Abilene Christian",
        "Wildcats",
        "Abil Christian",
        "ACU",
        "4e2683",
        "ebebeb",
    ),
    "2005": Team.createTeam(
        "2005", "Air Force", "Falcons", "Air Force", "AFA", "004a7b", "ffffff"
    ),
    "2006": Team.createTeam(
        "2006", "Akron", "Zips", "Akron", "AKR", "00285e", "84754e"
    ),
    "2010": Team.createTeam(
        "2010", "Alabama A&M", "Bulldogs", "Alabama A&M", "AAMU", "790000", "ffffff"
    ),
    "333": Team.createTeam(
        "333", "Alabama", "Crimson Tide", "Alabama", "ALA", "690014", "f1f2f3"
    ),
    "2011": Team.createTeam(
        "2011", "Alabama State", "Hornets", "Alabama State", "ALST", "e9a900", "0a0a0a"
    ),
    "399": Team.createTeam(
        "399", "Albany", "Great Danes", "Albany", "ALB", "3D2777", "ffffff"
    ),
    "2016": Team.createTeam(
        "2016", "Alcorn State", "Braves", "Alcorn State", "ALCN", "4b0058", "46166a"
    ),
    "44": Team.createTeam(
        "44", "American", "Eagles", "American", "AMER", "c41130", "c8102e"
    ),
    "2026": Team.createTeam(
        "2026",
        "Appalachian State",
        "Mountaineers",
        "Appalachian St",
        "APP",
        "000000",
        "ffcd00",
    ),
    "9": Team.createTeam(
        "9", "Arizona State", "Sun Devils", "Arizona State", "ASU", "942139", "f1f2f3"
    ),
    "12": Team.createTeam(
        "12", "Arizona", "Wildcats", "Arizona", "ARIZ", "002449", "00205b"
    ),
    "8": Team.createTeam(
        "8", "Arkansas", "Razorbacks", "Arkansas", "ARK", "9c1831", "000000"
    ),
    "2032": Team.createTeam(
        "2032",
        "Arkansas State",
        "Red Wolves",
        "Arkansas State",
        "ARST",
        "e81018",
        "000000",
    ),
    "2029": Team.createTeam(
        "2029",
        "Arkansas-Pine Bluff",
        "Golden Lions",
        "Ark-Pine Bluff",
        "UAPB",
        "e0aa0f",
        "eaaa00",
    ),
    "349": Team.createTeam(
        "349", "Army", "Black Knights", "Army", "ARMY", "ce9c00", "231f20"
    ),
    "2": Team.createTeam("2", "Auburn", "Tigers", "Auburn", "AUB", "03244d", "f1f2f3"),
    "2046": Team.createTeam(
        "2046", "Austin Peay", "Governors", "Austin Peay", "APSU", "8e0b0b", "000000"
    ),
    "252": Team.createTeam("252", "BYU", "Cougars", "BYU", "BYU", "001E4C", "ffffff"),
    "2050": Team.createTeam(
        "2050", "Ball State", "Cardinals", "Ball State", "BALL", "DA0000", "ffffff"
    ),
    "239": Team.createTeam(
        "239", "Baylor", "Bears", "Baylor", "BAY", "004834", "ffb81c"
    ),
    "91": Team.createTeam(
        "91", "Bellarmine", "Knights", "Bellarmine", "BELL", "000000", "000000"
    ),
    "2057": Team.createTeam(
        "2057", "Belmont", "Bruins", "Belmont", "BEL", "182142", "c9262d"
    ),
    "2066": Team.createTeam(
        "2066", "Binghamton", "Bearcats", "Binghamton", "BING", "00614A", "f0f0f0"
    ),
    "68": Team.createTeam(
        "68", "Boise State", "Broncos", "Boise State", "BSU", "09347A", "d8d9da"
    ),
    "103": Team.createTeam(
        "103", "Boston College", "Eagles", "Boston College", "BC", "88001a", "a39161"
    ),
    "104": Team.createTeam(
        "104", "Boston Univ.", "Terriers", "Boston Univ.", "BU", "cc0000", "ffffff"
    ),
    "189": Team.createTeam(
        "189", "Bowling Green", "Falcons", "Bowling Green", "BGSU", "2b1000", "492000"
    ),
    "71": Team.createTeam(
        "71", "Bradley", "Braves", "Bradley", "BRAD", "b70002", "c0c0c0"
    ),
    "225": Team.createTeam(
        "225", "Brown", "Bears", "Brown", "BRWN", "411e09", "949300"
    ),
    "2083": Team.createTeam(
        "2083", "Bucknell", "Bison", "Bucknell", "BUCK", "000060", "00316e"
    ),
    "2084": Team.createTeam(
        "2084", "Buffalo", "Bulls", "Buffalo", "BUFF", "041A9B", "ebebeb"
    ),
    "2086": Team.createTeam(
        "2086", "Butler", "Bulldogs", "Butler", "BUT", "0d1361", "00a3e0"
    ),
    "2239": Team.createTeam(
        "2239", "CSU Fullerton", "Titans", "CSU Fullerton", "CSUF", "10219c", "003057"
    ),
    "13": Team.createTeam(
        "13", "Cal Poly", "Mustangs", "Cal Poly", "CP", "1E4D2B", "eed897"
    ),
    "25": Team.createTeam(
        "25", "California", "Golden Bears", "California", "CAL", "031522", "ffc423"
    ),
    "2097": Team.createTeam(
        "2097", "Campbell", "Fighting Camels", "Campbell", "CAM", "000000", "000000"
    ),
    "2099": Team.createTeam(
        "2099", "Canisius", "Golden Griffins", "Canisius", "CAN", "004a81", "dda50f"
    ),
    "2110": Team.createTeam(
        "2110", "Central Arkansas", "Bears", "Cent Arkansas", "UCA", "a7a9ac", "8e959a"
    ),
    "2115": Team.createTeam(
        "2115",
        "Central Connecticut",
        "Blue Devils",
        "Cent Conn St",
        "CCSU",
        "1B49A2",
        "d1d5d8",
    ),
    "2117": Team.createTeam(
        "2117",
        "Central Michigan",
        "Chippewas",
        "Cent Michigan",
        "CMU",
        "6a0032",
        "ffffff",
    ),
    "232": Team.createTeam(
        "232", "Charleston", "Cougars", "Charleston", "COFC", "9c8456", "e4e2dd"
    ),
    "2127": Team.createTeam(
        "2127",
        "Charleston Southern",
        "Buccaneers",
        "Charleston So",
        "CHSO",
        "2e3192",
        "ded090",
    ),
    "236": Team.createTeam(
        "236", "Chattanooga", "Mocs", "Chattanooga", "UTC", "00386b", "dca71d"
    ),
    "2130": Team.createTeam(
        "2130", "Chicago State", "Cougars", "Chicago State", "CHIC", "006700", "000000"
    ),
    "2132": Team.createTeam(
        "2132", "Cincinnati", "Bearcats", "Cincinnati", "CIN", "000000", "717073"
    ),
    "228": Team.createTeam(
        "228", "Clemson", "Tigers", "Clemson", "CLEM", "F66733", "522d80"
    ),
    "325": Team.createTeam(
        "325",
        "Cleveland State",
        "Vikings",
        "Cleveland State",
        "CLEV",
        "006633",
        "231f20",
    ),
    "324": Team.createTeam(
        "324",
        "Coastal Carolina",
        "Chanticleers",
        "C. Carolina",
        "CCAR",
        "007073",
        "876447",
    ),
    "2142": Team.createTeam(
        "2142", "Colgate", "Raiders", "Colgate", "COLG", "8B011D", "231f20"
    ),
    "38": Team.createTeam(
        "38", "Colorado", "Buffaloes", "Colorado", "COLO", "d1c57e", "ffd200"
    ),
    "36": Team.createTeam(
        "36", "Colorado State", "Rams", "Colorado State", "CSU", "004537", "ffc425"
    ),
    "171": Team.createTeam(
        "171", "Columbia", "Lions", "Columbia", "CLMB", "174785", "004b85"
    ),
    "2154": Team.createTeam(
        "2154", "Coppin St", "Eagles", "Coppin State", "COPP", "2e3192", "ffd204"
    ),
    "172": Team.createTeam(
        "172", "Cornell", "Big Red", "Cornell", "COR", "d60027", "101010"
    ),
    "156": Team.createTeam(
        "156", "Creighton", "Bluejays", "Creighton", "CREI", "13299e", "00235d"
    ),
    "159": Team.createTeam(
        "159", "Dartmouth", "Big Green", "Dartmouth", "DART", "005730", "000000"
    ),
    "2166": Team.createTeam(
        "2166", "Davidson", "Wildcats", "Davidson", "DAV", "000000", "e51837"
    ),
    "2168": Team.createTeam(
        "2168", "Dayton", "Flyers", "Dayton", "DAY", "004B8D", "ffffff"
    ),
    "305": Team.createTeam(
        "305", "DePaul", "Blue Demons", "DePaul", "DEP", "2d649c", "ce1125"
    ),
    "48": Team.createTeam(
        "48", "Delaware", "Blue Hens", "Delaware", "DEL", "033594", "e8ce31"
    ),
    "2169": Team.createTeam(
        "2169", "Delaware St", "Hornets", "Delaware State", "DSU", "FF3630", "009cdd"
    ),
    "2172": Team.createTeam(
        "2172", "Denver", "Pioneers", "Denver", "DEN", "9c143d", "d6ba74"
    ),
    "2174": Team.createTeam(
        "2174", "Detroit Mercy", "Titans", "Detroit Mercy", "DET", "165b9e", "d31733"
    ),
    "2181": Team.createTeam(
        "2181", "Drake", "Bulldogs", "Drake", "DRKE", "004477", "c0c0c0"
    ),
    "2182": Team.createTeam(
        "2182", "Drexel", "Dragons", "Drexel", "DREX", "020260", "ffd65a"
    ),
    "150": Team.createTeam(
        "150", "Duke", "Blue Devils", "Duke", "DUKE", "001A57", "f1f2f3"
    ),
    "2184": Team.createTeam(
        "2184", "Duquesne", "Dukes", "Duquesne", "DUQ", "002D62", "b90b2e"
    ),
    "151": Team.createTeam(
        "151", "East Carolina", "Pirates", "East Carolina", "ECU", "4b1869", "f0907b"
    ),
    "2193": Team.createTeam(
        "2193", "East Tennessee State", "Buccaneers", "ETSU", "ETSU", "002d61", "ffc423"
    ),
    "2197": Team.createTeam(
        "2197", "Eastern Illinois", "Panthers", "E Illinois", "EIU", "000000", "bebab9"
    ),
    "2198": Team.createTeam(
        "2198", "Eastern Kentucky", "Colonels", "E Kentucky", "EKU", "660819", "f0f0f0"
    ),
    "2199": Team.createTeam(
        "2199", "Eastern Michigan", "Eagles", "E Michigan", "EMU", "00331b", "f0f0f0"
    ),
    "331": Team.createTeam(
        "331", "Eastern Washington", "Eagles", "E Washington", "EWU", "a10022", "abb4bc"
    ),
    "2210": Team.createTeam(
        "2210", "Elon", "Phoenix", "Elon", "ELON", "020303", "b59a57"
    ),
    "339": Team.createTeam(
        "339", "Evansville", "Purple Aces", "Evansville", "EVAN", "663399", "ef6f00"
    ),
    "2217": Team.createTeam(
        "2217", "Fairfield", "Stags", "Fairfield", "FAIR", "000000", "ebebeb"
    ),
    "161": Team.createTeam(
        "161",
        "Fairleigh Dickinson",
        "Knights",
        "Fair. Dickinson",
        "FDU",
        "00449C",
        "a80532",
    ),
    "50": Team.createTeam(
        "50", "Florida A&M", "Rattlers", "Florida A&M", "FAMU", "F89728", "00843d"
    ),
    "2226": Team.createTeam(
        "2226", "Florida Atlantic", "Owls", "FAU", "FAU", "004B85", "bb2f4c"
    ),
    "57": Team.createTeam(
        "57", "Florida", "Gators", "Florida", "FLA", "0021A5", "0021a5"
    ),
    "526": Team.createTeam(
        "526", "Florida Gulf Coast", "Eagles", "FGCU", "FGCU", "00885a", "076c3b"
    ),
    "2229": Team.createTeam(
        "2229", "Florida Int'l", "Panthers", "FIU", "FIU", "091731", "c5960c"
    ),
    "52": Team.createTeam(
        "52", "Florida State", "Seminoles", "Florida State", "FSU", "782F40", "ceb888"
    ),
    "2230": Team.createTeam(
        "2230", "Fordham", "Rams", "Fordham", "FOR", "830032", "909090"
    ),
    "278": Team.createTeam(
        "278", "Fresno State", "Bulldogs", "Fresno State", "FRES", "c41230", "231f20"
    ),
    "231": Team.createTeam(
        "231", "Furman", "Paladins", "Furman", "FUR", "4A2184", "909090"
    ),
    "2241": Team.createTeam(
        "2241", "Gardner-Webb", "Bulldogs", "Gardner-Webb", "GWEB", "c12535", "909090"
    ),
    "2244": Team.createTeam(
        "2244", "George Mason", "Patriots", "George Mason", "GMU", "016600", "ecb010"
    ),
    "45": Team.createTeam(
        "45", "George Washington", "Colonials", "G Washington", "GW", "002843", "e8d2a1"
    ),
    "46": Team.createTeam(
        "46", "Georgetown", "Hoyas", "Georgetown", "GTWN", "110E42", "001c58"
    ),
    "61": Team.createTeam(
        "61", "Georgia", "Bulldogs", "Georgia", "UGA", "CC0000", "000000"
    ),
    "290": Team.createTeam(
        "290", "Georgia Southern", "Eagles", "GA Southern", "GASO", "003775", "f0f0f0"
    ),
    "2247": Team.createTeam(
        "2247", "Georgia State", "Panthers", "Georgia State", "GAST", "1e539a", "ebebeb"
    ),
    "59": Team.createTeam(
        "59", "Georgia Tech", "Yellow Jackets", "Georgia Tech", "GT", "00223e", "002c56"
    ),
    "2250": Team.createTeam(
        "2250", "Gonzaga", "Bulldogs", "Gonzaga", "GONZ", "002967", "cfd4d8"
    ),
    "2253": Team.createTeam(
        "2253", "Grand Canyon", "Antelopes", "Grand Canyon", "GCU", "522398", "f0f0f0"
    ),
    "2261": Team.createTeam(
        "2261", "Hampton", "Pirates", "Hampton", "HAMP", "0067AC", "000000"
    ),
    "42": Team.createTeam(
        "42", "Hartford", "Hawks", "Hartford", "HART", "d60008", "000000"
    ),
    "108": Team.createTeam(
        "108", "Harvard", "Crimson", "Harvard", "HARV", "990000", "dbdbdb"
    ),
    "62": Team.createTeam(
        "62", "Hawai'i", "Rainbow Warriors", "Hawai'i", "HAW", "003420", "ffffff"
    ),
    "2272": Team.createTeam(
        "2272", "High Point", "Panthers", "High Point", "HP", "b0b7bc", "ebebeb"
    ),
    "2275": Team.createTeam(
        "2275", "Hofstra", "Pride", "Hofstra", "HOF", "00337c", "f6c934"
    ),
    "107": Team.createTeam(
        "107", "Holy Cross", "Crusaders", "Holy Cross", "HC", "0a0a0a", "080808"
    ),
    "2277": Team.createTeam(
        "2277",
        "Houston Baptist",
        "Huskies",
        "Houston Baptist",
        "HBU",
        "00539c",
        "000000",
    ),
    "248": Team.createTeam(
        "248", "Houston", "Cougars", "Houston", "HOU", "C90822", "231f20"
    ),
    "47": Team.createTeam("47", "Howard", "Bison", "Howard", "HOW", "9e0b0e", "6b818d"),
    "85": Team.createTeam(
        "85", "IUPUI", "Jaguars", "IUPUI", "IUPU", "A81F30", "d59f0f"
    ),
    "304": Team.createTeam(
        "304", "Idaho State", "Bengals", "Idaho State", "IDST", "ef8c00", "e9a126"
    ),
    "70": Team.createTeam(
        "70", "Idaho", "Vandals", "Idaho", "IDHO", "000000", "8c6e4a"
    ),
    "356": Team.createTeam(
        "356", "Illinois", "Fighting Illini", "Illinois", "ILL", "f77329", "fa6300"
    ),
    "2287": Team.createTeam(
        "2287",
        "Illinois State",
        "Redbirds",
        "Illinois State",
        "ILST",
        "CE1126",
        "ffe716",
    ),
    "84": Team.createTeam(
        "84", "Indiana", "Hoosiers", "Indiana", "IU", "7D110C", "eeedeb"
    ),
    "282": Team.createTeam(
        "282", "Indiana State", "Sycamores", "Indiana State", "INST", "00669a", "f0f0f0"
    ),
    "314": Team.createTeam("314", "Iona", "Gaels", "Iona", "IONA", "8c001a", "f6a704"),
    "2294": Team.createTeam(
        "2294", "Iowa", "Hawkeyes", "Iowa", "IOWA", "000000", "ffe100"
    ),
    "66": Team.createTeam(
        "66", "Iowa State", "Cyclones", "Iowa State", "ISU", "660015", "830b2c"
    ),
    "2296": Team.createTeam(
        "2296", "Jackson State", "Tigers", "Jackson State", "JKST", "123297", "b5b7ba"
    ),
    "294": Team.createTeam(
        "294", "Jacksonville", "Dolphins", "Jacksonville", "JAX", "00523e", "000000"
    ),
    "55": Team.createTeam(
        "55",
        "Jacksonville State",
        "Gamecocks",
        "Jacksonville St",
        "JVST",
        "b50500",
        "b5b7ba",
    ),
    "256": Team.createTeam(
        "256", "James Madison", "Dukes", "James Madison", "JMU", "450084", "cbb778"
    ),
    "2305": Team.createTeam(
        "2305", "Kansas", "Jayhawks", "Kansas", "KU", "0022B4", "e8000d"
    ),
    "2306": Team.createTeam(
        "2306", "Kansas State", "Wildcats", "Kansas State", "KSU", "633194", "e7d2ad"
    ),
    "338": Team.createTeam(
        "338", "Kennesaw State", "Owls", "Kennesaw State", "KENN", "000000", "fdbb30"
    ),
    "2309": Team.createTeam(
        "2309", "Kent State", "Golden Flashes", "Kent State", "KENT", "002445", "f0b510"
    ),
    "96": Team.createTeam(
        "96", "Kentucky", "Wildcats", "Kentucky", "UK", "005DAA", "ffffff"
    ),
    "99": Team.createTeam("99", "LSU", "Tigers", "LSU", "LSU", "2B0D57", "fdd023"),
    "2325": Team.createTeam(
        "2325", "La Salle", "Explorers", "La Salle", "LAS", "000063", "feca26"
    ),
    "322": Team.createTeam(
        "322", "Lafayette", "Leopards", "Lafayette", "LAF", "790000", "a59474"
    ),
    "2320": Team.createTeam(
        "2320", "Lamar", "Cardinals", "Lamar", "LAM", "000000", "ebebeb"
    ),
    "2329": Team.createTeam(
        "2329", "Lehigh", "Mountain Hawks", "Lehigh", "LEH", "6c2b2a", "b69e70"
    ),
    "2335": Team.createTeam(
        "2335", "Liberty", "Flames", "Liberty", "LIB", "071740", "a61f21"
    ),
    "288": Team.createTeam(
        "288", "Lipscomb", "Bisons", "Lipscomb", "LIP", "20366C", "f6b734"
    ),
    "2031": Team.createTeam(
        "2031", "Little Rock", "Trojans", "Little Rock", "UALR", "AD0000", "898d8f"
    ),
    "299": Team.createTeam(
        "299", "Long Beach State", "Beach", "Long Beach St", "LBSU", "000000", "f1f2f3"
    ),
    "2344": Team.createTeam(
        "2344", "Longwood", "Lancers", "Longwood", "LONG", "003273", "9ea2a3"
    ),
    "309": Team.createTeam(
        "309", "Louisiana", "Ragin' Cajuns", "Louisiana", "ULL", "ce2842", "000000"
    ),
    "2348": Team.createTeam(
        "2348", "Louisiana Tech", "Bulldogs", "Louisiana Tech", "LT", "002d65", "d3313a"
    ),
    "97": Team.createTeam(
        "97", "Louisville", "Cardinals", "Louisville", "LOU", "ad000a", "cccccc"
    ),
    "2352": Team.createTeam(
        "2352", "Loyola (MD)", "Greyhounds", "Loyola (MD)", "L-MD", "76a7a0", "c9cbca"
    ),
    "2350": Team.createTeam(
        "2350",
        "Loyola Chicago",
        "Ramblers",
        "Loyola Chicago",
        "LUC",
        "9d1244",
        "000000",
    ),
    "2351": Team.createTeam(
        "2351", "Loyola Marymount", "Lions", "Loyola Marymnt", "LMU", "880029", "00345b"
    ),
    "311": Team.createTeam(
        "311", "Maine", "Black Bears", "Maine", "ME", "127dbe", "000000"
    ),
    "269": Team.createTeam(
        "269", "Marquette", "Golden Eagles", "Marquette", "MARQ", "083963", "ffffff"
    ),
    "276": Team.createTeam(
        "276", "Marshall", "Thundering Herd", "Marshall", "MRSH", "186329", "be854c"
    ),
    "120": Team.createTeam(
        "120", "Maryland", "Terrapins", "Maryland", "MD", "D5002B", "ffcd00"
    ),
    "235": Team.createTeam(
        "235", "Memphis", "Tigers", "Memphis", "MEM", "002447", "231f20"
    ),
    "193": Team.createTeam(
        "193", "Miami (OH)", "Redhawks", "Miami (OH)", "M-OH", "a4000c", "f0f0f0"
    ),
    "127": Team.createTeam(
        "127", "Michigan State", "Spartans", "Michigan State", "MSU", "18453B", "ffffff"
    ),
    "130": Team.createTeam(
        "130", "Michigan", "Wolverines", "Michigan", "MICH", "00274c", "00274c"
    ),
    "270": Team.createTeam(
        "270", "Milwaukee", "Panthers", "Milwaukee", "MILW", "000000", "ffc20e"
    ),
    "135": Team.createTeam(
        "135", "Minnesota", "Golden Gophers", "Minnesota", "MINN", "981a31", "981a31"
    ),
    "344": Team.createTeam(
        "344",
        "Mississippi State",
        "Bulldogs",
        "Mississippi St",
        "MSST",
        "762123",
        "c8c8c8",
    ),
    "142": Team.createTeam(
        "142", "Missouri", "Tigers", "Missouri", "MIZ", "000000", "000000"
    ),
    "149": Team.createTeam(
        "149", "Montana", "Grizzlies", "Montana", "MONT", "751D4A", "666666"
    ),
    "147": Team.createTeam(
        "147", "Montana State", "Bobcats", "Montana State", "MTST", "003875", "bf965c"
    ),
    "116": Team.createTeam(
        "116",
        "Mount St. Mary's",
        "Mountaineers",
        "Mt. St. Mary's",
        "MSM",
        "005596",
        "ebebeb",
    ),
    "93": Team.createTeam(
        "93", "Murray State", "Racers", "Murray State", "MUR", "002148", "000e00"
    ),
    "152": Team.createTeam(
        "152", "NC State", "Wolfpack", "NC State", "NCST", "EF1216", "231f20"
    ),
    "158": Team.createTeam(
        "158", "Nebraska", "Cornhuskers", "Nebraska", "NEB", "F20017", "f5f1e7"
    ),
    "160": Team.createTeam(
        "160", "New Hampshire", "Wildcats", "New Hampshire", "UNH", "004990", "c3c4c6"
    ),
    "167": Team.createTeam(
        "167", "New Mexico", "Lobos", "New Mexico", "UNM", "000000", "231f20"
    ),
    "166": Team.createTeam(
        "166", "New Mexico State", "Aggies", "New Mexico St", "NMSU", "891216", "000000"
    ),
    "315": Team.createTeam(
        "315", "Niagara", "Purple Eagles", "Niagara", "NIAG", "69207E", "f0f0f0"
    ),
    "153": Team.createTeam(
        "153",
        "North Carolina",
        "Tar Heels",
        "North Carolina",
        "UNC",
        "99bfe5",
        "13294b",
    ),
    "155": Team.createTeam(
        "155",
        "North Dakota",
        "Fighting Hawks",
        "North Dakota",
        "UND",
        "00A26B",
        "c2c3c0",
    ),
    "249": Team.createTeam(
        "249", "North Texas", "Mean Green", "North Texas", "UNT", "00853D", "000000"
    ),
    "111": Team.createTeam(
        "111", "Northeastern", "Huskies", "Northeastern", "NE", "CC0001", "c2c3c0"
    ),
    "94": Team.createTeam(
        "94", "Northern Kentucky", "Norse", "N Kentucky", "NKU", "000000", "eab621"
    ),
    "77": Team.createTeam(
        "77", "Northwestern", "Wildcats", "Northwestern", "NW", "372286", "d6cac1"
    ),
    "87": Team.createTeam(
        "87", "Notre Dame", "Fighting Irish", "Notre Dame", "ND", "00122b", "ae9142"
    ),
    "195": Team.createTeam(
        "195", "Ohio", "Bobcats", "Ohio", "OHIO", "295A29", "e4bb85"
    ),
    "194": Team.createTeam(
        "194", "Ohio State", "Buckeyes", "Ohio State", "OSU", "de3129", "666666"
    ),
    "201": Team.createTeam(
        "201", "Oklahoma", "Sooners", "Oklahoma", "OU", "7b0000", "cccccc"
    ),
    "197": Team.createTeam(
        "197", "Oklahoma State", "Cowboys", "Oklahoma State", "OKST", "FF6500", "ff9900"
    ),
    "295": Team.createTeam(
        "295", "Old Dominion", "Monarchs", "Old Dominion", "ODU", "00507d", "a1d2f1"
    ),
    "145": Team.createTeam(
        "145", "Ole Miss", "Rebels", "Ole Miss", "MISS", "001148", "00205b"
    ),
    "198": Team.createTeam(
        "198",
        "Oral Roberts",
        "Golden Eagles",
        "Oral Roberts",
        "ORU",
        "002955",
        "ccb48c",
    ),
    "204": Team.createTeam(
        "204", "Oregon State", "Beavers", "Oregon State", "ORST", "c34500", "dea076"
    ),
    "279": Team.createTeam(
        "279", "Pacific", "Tigers", "Pacific", "PAC", "F47820", "c2c3c0"
    ),
    "213": Team.createTeam(
        "213", "Penn State", "Nittany Lions", "Penn State", "PSU", "00265D", "002e5c"
    ),
    "219": Team.createTeam(
        "219", "Pennsylvania", "Quakers", "Penn", "PENN", "082A74", "a6163d"
    ),
    "221": Team.createTeam(
        "221", "Pittsburgh", "Panthers", "Pittsburgh", "PITT", "003263", "231f20"
    ),
    "163": Team.createTeam(
        "163", "Princeton", "Tigers", "Princeton", "PRIN", "ff9408", "080808"
    ),
    "227": Team.createTeam(
        "227", "Rhode Island", "Rams", "Rhode Island", "URI", "091f3f", "5ab3e8"
    ),
    "242": Team.createTeam("242", "Rice", "Owls", "Rice", "RICE", "003D7D", "d1d5d8"),
    "257": Team.createTeam(
        "257", "Richmond", "Spiders", "Richmond", "RICH", "9e0712", "b90b2e"
    ),
    "164": Team.createTeam(
        "164", "Rutgers", "Scarlet Knights", "Rutgers", "RUTG", "d21034", "ffffff"
    ),
    "16": Team.createTeam(
        "16", "Sacramento State", "Hornets", "Sacramento St", "SAC", "00573C", "cdb97d"
    ),
    "139": Team.createTeam(
        "139", "Saint Louis", "Billikens", "Saint Louis", "SLU", "00539C", "ebebeb"
    ),
    "21": Team.createTeam(
        "21", "San Diego State", "Aztecs", "San Diego State", "SDSU", "BF2C37", "a8996e"
    ),
    "301": Team.createTeam(
        "301", "San Diego", "Toreros", "San Diego", "USD", "2f99d4", "2f99d4"
    ),
    "23": Team.createTeam(
        "23", "San José St", "Spartans", "San José State", "SJSU", "005893", "fdba31"
    ),
    "6": Team.createTeam(
        "6", "South Alabama", "Jaguars", "South Alabama", "USA", "003E7E", "000000"
    ),
    "233": Team.createTeam(
        "233", "South Dakota", "Coyotes", "South Dakota", "SDAK", "CD1241", "f0f0f0"
    ),
    "58": Team.createTeam(
        "58", "South Florida", "Bulls", "South Florida", "USF", "004A36", "231f20"
    ),
    "79": Team.createTeam(
        "79", "Southern Illinois", "Salukis", "So Illinois", "SIU", "85283D", "c2c3c0"
    ),
    "253": Team.createTeam(
        "253",
        "Southern Utah",
        "Thunderbirds",
        "Southern Utah",
        "SUU",
        "d10000",
        "c2c3c0",
    ),
    "179": Team.createTeam(
        "179",
        "St. Bonaventure",
        "Bonnies",
        "St. Bonaventure",
        "SBU",
        "70261D",
        "000000",
    ),
    "24": Team.createTeam(
        "24", "Stanford", "Cardinal", "Stanford", "STAN", "A80532", "ffffff"
    ),
    "56": Team.createTeam(
        "56", "Stetson", "Hatters", "Stetson", "STET", "0a5640", "56854e"
    ),
    "183": Team.createTeam(
        "183", "Syracuse", "Orange", "Syracuse", "SYR", "F37321", "0d1d37"
    ),
    "218": Team.createTeam(
        "218", "Temple", "Owls", "Temple", "TEM", "A80532", "a7a9ac"
    ),
    "245": Team.createTeam(
        "245", "Texas A&M", "Aggies", "Texas A&M", "TA&M", "5C0025", "ffffff"
    ),
    "357": Team.createTeam(
        "357", "Texas A&M-CC", "Islanders", "Texas A&M-CC", "AMCC", "00639c", "008752"
    ),
    "251": Team.createTeam(
        "251", "Texas", "Longhorns", "Texas", "TEX", "EE7524", "f0f0f0"
    ),
    "326": Team.createTeam(
        "326", "Texas State", "Bobcats", "Texas State", "TXST", "4e1719", "b4975a"
    ),
    "119": Team.createTeam(
        "119", "Towson", "Tigers", "Towson", "TOW", "FFC229", "000000"
    ),
    "202": Team.createTeam(
        "202", "Tulsa", "Golden Hurricane", "Tulsa", "TLSA", "004371", "ee3b33"
    ),
    "5": Team.createTeam("5", "UAB", "Blazers", "UAB", "UAB", "054338", "ffc845"),
    "302": Team.createTeam(
        "302", "UC Davis", "Aggies", "UC Davis", "UCD", "183563", "bc9305"
    ),
    "300": Team.createTeam(
        "300", "UC Irvine", "Anteaters", "UC Irvine", "UCI", "002B5C", "fec52e"
    ),
    "27": Team.createTeam(
        "27", "UC Riverside", "Highlanders", "UC Riverside", "UCR", "14234F", "000000"
    ),
    "28": Team.createTeam(
        "28", "UC San Diego", "Tritons", "UC San Diego", "UCSD", "000000", "000000"
    ),
    "2116": Team.createTeam("2116", "UCF", "Knights", "UCF", "UCF", "000000", "231f20"),
    "26": Team.createTeam("26", "UCLA", "Bruins", "UCLA", "UCLA", "005C8E", "ffc72c"),
    "41": Team.createTeam(
        "41", "UConn", "Huskies", "UConn", "CONN", "001d40", "f1f2f3"
    ),
    "82": Team.createTeam("82", "UIC", "Flames", "UIC", "UIC", "234077", "c30031"),
    "140": Team.createTeam(
        "140", "UM Kansas City", "Roos", "UM Kansas City", "UMKC", "004b87", "ffc72c"
    ),
    "2349": Team.createTeam(
        "2349", "UMass Lowell", "River Hawks", "UMass Lowell", "UML", "00529C", "cf1f2f"
    ),
    "113": Team.createTeam(
        "113", "UMass", "Minutemen", "UMass", "MASS", "880007", "000000"
    ),
    "350": Team.createTeam(
        "350",
        "UNC Wilmington",
        "Seahawks",
        "UNC Wilmington",
        "UNCW",
        "1d2f68",
        "00665e",
    ),
    "30": Team.createTeam("30", "USC", "Trojans", "USC", "USC", "AE2531", "ffc72c"),
    "250": Team.createTeam(
        "250", "UT Arlington", "Mavericks", "UT Arlington", "UTA", "004b7c", "f58024"
    ),
    "292": Team.createTeam(
        "292",
        "UT Rio Grande Valley",
        "Vaqueros",
        "UT Rio Grande",
        "UTRGV",
        "dc6000",
        "e1732d",
    ),
    "328": Team.createTeam(
        "328", "Utah State", "Aggies", "Utah State", "USU", "003263", "949ca1"
    ),
    "254": Team.createTeam("254", "Utah", "Utes", "Utah", "UTAH", "890012", "7e8083"),
    "238": Team.createTeam(
        "238", "Vanderbilt", "Commodores", "Vanderbilt", "VAN", "000000", "231f20"
    ),
    "261": Team.createTeam(
        "261", "Vermont", "Catamounts", "Vermont", "UVM", "013C24", "ffc72c"
    ),
    "222": Team.createTeam(
        "222", "Villanova", "Wildcats", "Villanova", "VILL", "123d7C", "f0f0f0"
    ),
    "258": Team.createTeam(
        "258", "Virginia", "Cavaliers", "Virginia", "UVA", "f84c1e", "242e4a"
    ),
    "259": Team.createTeam(
        "259", "Virginia Tech", "Hokies", "Virginia Tech", "VT", "74232D", "c2c1ba"
    ),
    "154": Team.createTeam(
        "154", "Wake Forest", "Demon Deacons", "Wake Forest", "WAKE", "9E7E38", "000000"
    ),
    "264": Team.createTeam(
        "264", "Washington", "Huskies", "Washington", "WASH", "2B2F64", "e8e3d3"
    ),
    "265": Team.createTeam(
        "265", "Washington State", "Cougars", "Washington St", "WSU", "94022a", "6a747c"
    ),
    "277": Team.createTeam(
        "277",
        "West Virginia",
        "Mountaineers",
        "West Virginia",
        "WVU",
        "FFC600",
        "eaaa00",
    ),
    "98": Team.createTeam(
        "98", "Western Kentucky", "Hilltoppers", "Western KY", "WKU", "F32026", "b3b5b8"
    ),
    "275": Team.createTeam(
        "275", "Wisconsin", "Badgers", "Wisconsin", "WISC", "A00002", "f7f7f7"
    ),
    "43": Team.createTeam("43", "Yale", "Bulldogs", "Yale", "YALE", "004a81", "286dc0"),
}


class CollegeBasketball:
    def createGame(common):
        return {"common": common}

    def getTeam(team_id, competitor):
        if team_id in team_map:
            return team_map[team_id]
        else:
            team = Team.createTeam(
                int(team_id),
                competitor["team"]["location"],
                competitor["team"]["name"],
                competitor["team"]["shortDisplayName"],
                competitor["team"]["abbreviation"],
                competitor["team"].get("color", "00000"),
                competitor["team"].get("alternateColor", "ffffff"),
            )
            print(f'Unknown team! "{str(team_id)}": {team},')
            return team

    def getGames(testing: bool):
        if testing:
            return {"games": []}
        else:
            raw_games = Fetcher.fetch("basketball", "mens-college-basketball")
            games = [
                CollegeBasketball.createGame(
                    Common.from_json(game, CollegeBasketball.getTeam)
                )
                for game in raw_games
            ]
            return {"games": games}


if __name__ == "__main__":
    print(CollegeBasketball.getGames(False))
