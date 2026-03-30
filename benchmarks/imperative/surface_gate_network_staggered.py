from dataclasses import dataclass


@dataclass
class Sphere:
    x: float
    y: float
    vx: float
    vy: float
    radius: float


LEFT_WALL_X = -4.0
RIGHT_WALL_X = 4.0
DOORS = {
    "left_a": (1.0, 3.0),
    "right_a": (1.0, 3.0),
    "left_b": (-3.0, -1.0),
    "right_b": (-3.0, -1.0),
}


def gate_open(name: str, current_time: float) -> bool:
    if name == "left_a":
        return current_time >= 1.0
    if name == "right_a":
        return current_time >= 4.0
    if name == "left_b":
        return current_time >= 4.0
    if name == "right_b":
        return current_time >= 2.0
    return False


def point_in_gate(y: float, gate_bounds):
    return gate_bounds[0] <= y <= gate_bounds[1]


def choose_velocity(name: str, current_time: float):
    if name == "A":
        if gate_open("left_a", current_time):
            return -3.0, 0.0
        if gate_open("right_a", current_time):
            return 3.0, 0.0
        return 0.0, 0.0
    if gate_open("left_b", current_time):
        return -3.0, 0.0
    if gate_open("right_b", current_time):
        return 3.0, 0.0
    return 0.0, 0.0


def enforce_gate(sphere: Sphere, previous_x: float, current_time: float, left_gate: str, right_gate: str):
    crossed_left = previous_x - sphere.radius >= LEFT_WALL_X and sphere.x - sphere.radius < LEFT_WALL_X
    crossed_right = previous_x + sphere.radius <= RIGHT_WALL_X and sphere.x + sphere.radius > RIGHT_WALL_X

    if crossed_left:
        if not gate_open(left_gate, current_time) or not point_in_gate(sphere.y, DOORS[left_gate]):
            sphere.x = LEFT_WALL_X + sphere.radius
            if sphere.vx < 0.0:
                sphere.vx = 0.0

    if crossed_right:
        if not gate_open(right_gate, current_time) or not point_in_gate(sphere.y, DOORS[right_gate]):
            sphere.x = RIGHT_WALL_X - sphere.radius
            if sphere.vx > 0.0:
                sphere.vx = 0.0


def simulate():
    spheres = {
        "A": Sphere(x=0.0, y=2.0, vx=0.0, vy=0.0, radius=0.5),
        "B": Sphere(x=0.0, y=-2.0, vx=0.0, vy=0.0, radius=0.5),
    }
    dt = 0.25
    current_time = 0.0
    while current_time < 3.0:
        if abs(current_time - 1.0) < 1e-9:
            spheres["A"].vx, spheres["A"].vy = choose_velocity("A", current_time)
        if abs(current_time - 2.0) < 1e-9:
            spheres["B"].vx, spheres["B"].vy = choose_velocity("B", current_time)

        for name, sphere in spheres.items():
            previous_x = sphere.x
            sphere.x += sphere.vx * dt
            sphere.y += sphere.vy * dt
            if name == "A":
                enforce_gate(sphere, previous_x, round(current_time + dt, 10), "left_a", "right_a")
            else:
                enforce_gate(sphere, previous_x, round(current_time + dt, 10), "left_b", "right_b")

        current_time = round(current_time + dt, 10)
    return spheres


if __name__ == "__main__":
    final = simulate()
    print(
        {
            name: {"position": (sphere.x, sphere.y), "velocity": (sphere.vx, sphere.vy)}
            for name, sphere in final.items()
        }
    )
