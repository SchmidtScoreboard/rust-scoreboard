def getRGBFromHex(color: str):
    red = int(color[0:2], 16)
    green = int(color[2:4], 16)
    blue = int(color[4:6], 16)
    return red, green, blue


def getLuminance(red, green, blue):
    r = red / 255
    g = green / 255
    b = blue / 255

    def new_value(c):
        if c <= 0.03928:
            return c / 12.92
        else:
            return ((c + 0.055) / 1.055) ** 2.4

    r = new_value(r)
    g = new_value(g)
    b = new_value(b)

    return (0.2126 * r) + (0.7152 * g) + (0.0722 * b)


def getContrast(primary, secondary):
    primary_luminance = getLuminance(*primary)
    secondary_luminance = getLuminance(*secondary)
    if primary_luminance > secondary_luminance:
        return (primary_luminance + 0.05) / (secondary_luminance + 0.05)
    else:
        return (secondary_luminance + 0.05) / (primary_luminance + 0.05)


def processTeamColors(primaryHex: str, secondaryHex: str):
    primary = getRGBFromHex(primaryHex)
    secondary = getRGBFromHex(secondaryHex)

    contrast = getContrast(primary, secondary)
    white_contrast = getContrast(primary, (255, 255, 255))
    black_contrast = getContrast(primary, (0, 0, 0))
    print(
        f"For colors {primaryHex} and {secondaryHex}, contrast is {contrast}, white contrast with primary is {white_contrast}, constrast with black is {black_contrast}"
    )
    if contrast > 3.5:
        return primaryHex, secondaryHex
    elif white_contrast > black_contrast:
        return primaryHex, "ffffff"
    else:
        return primaryHex, "000000"


if __name__ == "__main__":
    print(processTeamColors("de3129", "666666"))
