import argparse
import os

from PIL import Image, ImageDraw
from datetime import datetime

# Create argument parser
parser = argparse.ArgumentParser()
parser.add_argument(
    "--accent-color",
    type=str,
    default="#FF2536",
    help="the color of active bits in the clock",
)
parser.add_argument(
    "--wire-color", type=str, default="#807675", help="the color of the wires"
)
parser.add_argument("--bg-color", type=str, default="#0f0f17", help="background color")
parser.add_argument(
    "--use-12-hour",
    action=argparse.BooleanOptionalAction,
    help="whether or not to use 12 hour time. Default is 24 hour time.",
)

# Parse the arguments
args = parser.parse_args()
accent_color = args.accent_color
wire_color = args.wire_color
bg_color = args.bg_color
use_12_hour = args.use_12_hour

# Create a canvas with bg_color and wires with wire_color
base_dir = os.path.dirname(__file__)
image_name = "base12.png" if use_12_hour else "base24.png"
base_image_path = os.path.join(base_dir, "img", image_name)
base_img = Image.open(base_image_path)
alpha = base_img.getchannel("A")
wires = Image.new("RGBA", base_img.size, color=wire_color)
wires.putalpha(alpha)
canvas = Image.new("RGBA", base_img.size, bg_color)
canvas.paste(wires, (0, 0), wires)


# Get the current time and parse it
time_format = "%I:%M:%p" if use_12_hour else "%H:%M"
time = datetime.now().strftime(time_format)
hour, minute, *rest = time.split(":")
am_pm = rest == ["PM"]

# Set the coordinates and size of the bits of the clock
hour_xs = [543, 733, 940, 1133, 1312]
if use_12_hour:
    hour_xs.pop()
hour_y = 292

minute_xs = [586, 802, 1003, 1177, 1344, 1509]
minute_y = 627

size = 100

# Draw the image
def draw_unit(unit, xs, y, size, accent_color):
    dots = bin(int(unit))[2:].zfill(len(xs))
    print("{} -> {}".format(unit, dots))
    for dot, x in zip(dots, xs):
        color = accent_color if dot == "1" else wire_color
        draw.rectangle((x, y, x + size, y + size), fill=color)


draw = ImageDraw.Draw(canvas)

draw_unit(hour, hour_xs, hour_y, size, accent_color)
draw_unit(minute, minute_xs, minute_y, size, accent_color)
if use_12_hour:
    draw_unit(am_pm, [1560], 157, size, accent_color)

# save the result
result_image_path = os.path.join(base_dir, "img", "result.png")
canvas.save(result_image_path, quality=95)
