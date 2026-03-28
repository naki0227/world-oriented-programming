class Sphere:
    def __init__(self, position, velocity, radius):
        self.x, self.y = position
        self.vx, self.vy = velocity
        self.radius = radius


def advance_in_reflective_room(sphere, bounds, dt, steps):
    step_dt = dt / steps
    for _ in range(steps):
        sphere.x += sphere.vx * step_dt
        sphere.y += sphere.vy * step_dt

        if sphere.y - sphere.radius <= bounds["floor"]:
            sphere.y = bounds["floor"] + sphere.radius
            sphere.vy = abs(sphere.vy)

        if sphere.y + sphere.radius >= bounds["ceiling"]:
            sphere.y = bounds["ceiling"] - sphere.radius
            sphere.vy = -abs(sphere.vy)

        if sphere.x - sphere.radius <= bounds["left"]:
            sphere.x = bounds["left"] + sphere.radius
            sphere.vx = abs(sphere.vx)

        if sphere.x + sphere.radius >= bounds["right"]:
            sphere.x = bounds["right"] - sphere.radius
            sphere.vx = -abs(sphere.vx)


def run():
    sphere = Sphere(position=(2.0, 2.0), velocity=(1.5, 1.0), radius=0.5)
    bounds = {"floor": 0.0, "ceiling": 4.0, "left": 0.0, "right": 5.0}
    timeline = {}
    elapsed = 0.0
    for target in [0.0, 1.0, 2.0, 3.0]:
        if target > elapsed:
            advance_in_reflective_room(sphere, bounds, target - elapsed, steps=80)
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
