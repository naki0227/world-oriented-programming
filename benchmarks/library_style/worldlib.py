class World:
    def __init__(self):
        self.entities = {}
        self.regions = {}
        self.laws = []
        self.candidates = {}
        self.observations = []

    def sphere(self, name, position, velocity=(0, 0, 0), radius=1):
        self.entities[name] = {
            "kind": "sphere",
            "position": tuple(position),
            "velocity": tuple(velocity),
            "radius": radius,
        }

    def plane(self, name, normal=(0, 1, 0), offset=0):
        self.entities[name] = {
            "kind": "plane",
            "normal": tuple(normal),
            "offset": offset,
        }

    def region(self, name, minimum, maximum):
        self.regions[name] = {
            "min": tuple(minimum),
            "max": tuple(maximum),
        }

    def law(self, kind, *targets, policy=None):
        self.laws.append({
            "kind": kind,
            "targets": targets,
            "policy": policy,
        })

    def candidate_velocity(self, entity, label, velocity, score):
        self.candidates.setdefault(entity, []).append({
            "label": label,
            "velocity": tuple(velocity),
            "score": score,
        })

    def prefer_if_visible(self, entity, label, target):
        self.law("prefer_if_visible", entity, label, target)

    def prefer_if_occluded(self, entity, label, target):
        self.law("prefer_if_occluded", entity, label, target)

    def observe(self, time):
        self.observations.append(time)

    def simulate(self):
        return {
            "entities": self.entities,
            "regions": self.regions,
            "laws": self.laws,
            "candidates": self.candidates,
            "observations": self.observations,
        }

