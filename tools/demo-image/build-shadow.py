from PIL import Image
import math

img = Image.open("shadow.png").convert("RGBA")
pixels = img.load()

width, height = img.size
cx = (width - 1) / 2
cy = (height - 1) / 2

# Adjust these values to taste.
max_opacity = 0.35
fade_start = 0.15
fade_end = 0.50

max_radius = math.hypot(
    max(cx, width - 1 - cx),
    max(cy, height - 1 - cy),
)

start_radius = fade_start * max_radius
end_radius = fade_end * max_radius

for y in range(height):
    for x in range(width):
        r, g, b, a = pixels[x, y] # type: ignore

        # Preserve pixels that are already fully transparent.
        if a == 0:
            continue

        # distance = math.hypot(x - cx, y - cy)
        distance = min(
            x,
            y,
            width - 1 - x,
            height - 1 - y,
        )

        # if distance <= start_radius:
        #     fade = 1.0
        # elif distance >= end_radius:
        #     fade = 0.0
        # else:
        t = (distance - start_radius) / (
            end_radius - start_radius
        )
        # Smoothstep interpolation.
        fade = 1.0 - t * t * (3.0 - 2.0 * t)

        new_alpha = round(a * max_opacity * (2 * fade)**2)
        pixels[x, y] = (0, 0, 0, new_alpha) # type: ignore

img.save("shadow-faded.png")
