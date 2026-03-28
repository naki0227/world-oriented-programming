class Sphere:
    def __init__(self, position, velocity, radius):
        self.x, self.y = position
        self.vx, self.vy = velocity
        self.radius = radius


def plane_margin(plane, sphere):
    nx, ny = plane["normal"]
    return nx * sphere.x + ny * sphere.y - plane["offset"] - sphere.radius


def inside_gate_aperture(sphere, gate):
    return gate["min_y"] <= sphere.y <= gate["max_y"]


def clamp_inside_halfspace(plane, sphere, margin):
    nx, ny = plane["normal"]
    correction = -margin + 1e-6
    sphere.x += nx * correction
    sphere.y += ny * correction
    normal_speed = sphere.vx * nx + sphere.vy * ny
    if normal_speed < 0.0:
        sphere.vx -= nx * normal_speed
        sphere.vy -= ny * normal_speed


def advance_with_gate(sphere, wall, gate, dt, steps):
    step_dt = dt / steps
    for _ in range(steps):
        sphere.x += sphere.vx * step_dt
        sphere.y += sphere.vy * step_dt

        while True:
            margin = plane_margin(wall, sphere)
            if margin >= -1e-6 or inside_gate_aperture(sphere, gate):
                break
            clamp_inside_halfspace(wall, sphere, margin)


def run():
    sphere = Sphere(position=(2.0, 4.0), velocity=(-2.0, 0.0), radius=0.5)
    wall = {"normal": (1.0, 0.0), "offset": 0.0}
    gate = {"min_y": 1.0, "max_y": 3.0}
    timeline = {}
    elapsed = 0.0
    for target in [0.0, 1.0, 2.0]:
        if target > elapsed:
            advance_with_gate(sphere, wall, gate, target - elapsed, steps=40)
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
