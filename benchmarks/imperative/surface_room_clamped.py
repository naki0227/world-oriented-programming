import math


class Sphere:
    def __init__(self, position, velocity, radius):
        self.x, self.y = position
        self.vx, self.vy = velocity
        self.radius = radius


def normalize(vector):
    x, y = vector
    magnitude = math.sqrt(x * x + y * y)
    return (x / magnitude, y / magnitude)


def plane_margin(plane, sphere):
    nx, ny = plane["normal"]
    return nx * sphere.x + ny * sphere.y - plane["offset"] - sphere.radius


def clamp_inside_plane(plane, sphere, margin):
    nx, ny = plane["normal"]
    correction = -margin + 1e-6
    sphere.x += nx * correction
    sphere.y += ny * correction
    normal_speed = sphere.vx * nx + sphere.vy * ny
    if normal_speed < 0.0:
        sphere.vx -= nx * normal_speed
        sphere.vy -= ny * normal_speed


def advance_inside_surface_room(sphere, planes, dt, steps):
    step_dt = dt / steps
    for _ in range(steps):
        sphere.x += sphere.vx * step_dt
        sphere.y += sphere.vy * step_dt

        while True:
            violating = []
            for plane in planes:
                margin = plane_margin(plane, sphere)
                if margin < -1e-6:
                    violating.append((margin, plane))
            if not violating:
                break
            margin, plane = min(violating, key=lambda item: item[0])
            clamp_inside_plane(plane, sphere, margin)


def run():
    sphere = Sphere(position=(1.2, 2.6), velocity=(-1.2, 1.0), radius=0.8)
    planes = [
        {"normal": normalize((0.0, 1.0)), "offset": 0.0},
        {"normal": normalize((-0.5, -1.0)), "offset": -4.0},
        {"normal": normalize((1.0, 0.0)), "offset": 0.0},
        {"normal": normalize((-1.0, 0.0)), "offset": -5.0},
    ]
    timeline = {}
    elapsed = 0.0
    for target in [0.0, 1.0, 2.0]:
        if target > elapsed:
            advance_inside_surface_room(sphere, planes, target - elapsed, steps=40)
        elapsed = target
        timeline[target] = (
            round(sphere.x, 3),
            round(sphere.y, 3),
            round(sphere.vx, 3),
            round(sphere.vy, 3),
        )
    return timeline


if __name__ == "__main__":
    for time, state in run().items():
        print(time, state)
