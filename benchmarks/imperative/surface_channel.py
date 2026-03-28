class Sphere:
    def __init__(self, position, velocity, radius):
        self.x, self.y = position
        self.vx, self.vy = velocity
        self.radius = radius


def advance_with_surface_channel(sphere, floor_y, ceiling_y, dt, steps):
    snapshots = []
    step_dt = dt / steps
    for _ in range(steps):
        sphere.x += sphere.vx * step_dt
        sphere.y += sphere.vy * step_dt

        if sphere.y - sphere.radius <= floor_y:
            sphere.y = floor_y + sphere.radius
            sphere.vy = abs(sphere.vy)

        if sphere.y + sphere.radius >= ceiling_y:
            sphere.y = ceiling_y - sphere.radius
            sphere.vy = -abs(sphere.vy)

        snapshots.append((round(sphere.x, 3), round(sphere.y, 3), round(sphere.vx, 3), round(sphere.vy, 3)))
    return snapshots


def run():
    sphere = Sphere(position=(0.0, 2.0), velocity=(1.0, 2.0), radius=1.0)
    floor_y = 0.0
    ceiling_y = 4.0
    timeline = {}
    elapsed = 0.0
    for target in [0.0, 1.0, 2.0, 3.0]:
        if target > elapsed:
            advance_with_surface_channel(sphere, floor_y, ceiling_y, target - elapsed, steps=40)
        elapsed = target
        timeline[target] = (round(sphere.x, 3), round(sphere.y, 3), round(sphere.vx, 3), round(sphere.vy, 3))
    return timeline


if __name__ == "__main__":
    for time, state in run().items():
        print(time, state)
