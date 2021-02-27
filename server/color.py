def get_rgb_from_hex(color: str):
    red = int(color[0:2], 16)
    green = int(color[2:4], 16)
    blue = int(color[4:6], 16)
    return red, green, blue


def get_luminance(red, green, blue):
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


def get_contrast(primary, secondary):
    primary_luminance = get_luminance(*primary)
    secondary_luminance = get_luminance(*secondary)
    if primary_luminance > secondary_luminance:
        return (primary_luminance + 0.05) / (secondary_luminance + 0.05)
    else:
        return (secondary_luminance + 0.05) / (primary_luminance + 0.05)


def process_team_colors(primary_hex: str, secondary_hex: str):
    primary = get_rgb_from_hex(primary_hex)
    secondary = get_rgb_from_hex(secondary_hex)

    contrast = get_contrast(primary, secondary)
    white_contrast = get_contrast(primary, (255, 255, 255))
    black_contrast = get_contrast(primary, (0, 0, 0))
    print(
        f"For colors {primary_hex} and {secondary_hex}, contrast is {contrast}, white contrast with primary is {white_contrast}, constrast with black is {black_contrast}"
    )
    if contrast > 3.5:
        return primary_hex, secondary_hex
    elif white_contrast > black_contrast:
        return primary_hex, "ffffff"
    else:
        return primary_hex, "000000"


if __name__ == "__main__":
    print(process_team_colors("de3129", "666666"))
