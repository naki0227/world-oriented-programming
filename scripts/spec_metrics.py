#!/usr/bin/env python3
import json
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent

SCENARIOS = [
    ("bounce", ROOT / "examples" / "bounce.sk", ROOT / "benchmarks" / "imperative" / "bounce.py"),
    (
        "two_body_collision",
        ROOT / "examples" / "two_body_collision.sk",
        ROOT / "benchmarks" / "imperative" / "two_body_collision.py",
    ),
    (
        "candidate_velocity",
        ROOT / "examples" / "candidate_velocity.sk",
        ROOT / "benchmarks" / "imperative" / "candidate_velocity.py",
    ),
    (
        "clamped_region",
        ROOT / "examples" / "clamped_region.sk",
        ROOT / "benchmarks" / "imperative" / "clamped_region.py",
    ),
    (
        "candidate_velocity_deferred",
        ROOT / "examples" / "candidate_velocity_deferred.sk",
        ROOT / "benchmarks" / "imperative" / "candidate_velocity_deferred.py",
    ),
    (
        "visibility_occluded",
        ROOT / "examples" / "visibility_occluded.sk",
        ROOT / "benchmarks" / "imperative" / "visibility_occluded.py",
    ),
    (
        "visibility_pursuit_occluded",
        ROOT / "examples" / "visibility_pursuit_occluded.sk",
        ROOT / "benchmarks" / "imperative" / "visibility_pursuit_occluded.py",
    ),
]


def load_lines(path: Path):
    return path.read_text().splitlines()


def logical_lines(lines):
    return [line for line in lines if line.strip() and not line.strip().startswith("#")]


def token_count(lines):
    return len(re.findall(r"[A-Za-z_][A-Za-z0-9_]*|[-+]?[0-9]*\.?[0-9]+|[^\s]", "\n".join(lines)))


def metrics_for_sekai(path: Path):
    lines = logical_lines(load_lines(path))
    declarative_keywords = (
        "sphere ",
        "plane ",
        "region ",
        "position(",
        "velocity(",
        "radius(",
        "normal(",
        "offset(",
        "min(",
        "max(",
        "snapshot at ",
        "candidate_velocity(",
        "constraint:",
        "observe:",
        "action:",
    )
    declarative_lines = sum(1 for line in lines if line.strip().startswith(declarative_keywords))
    return {
        "logical_loc": len(lines),
        "token_count": token_count(lines),
        "branch_keywords": 0,
        "loop_keywords": 0,
        "state_assignment_lines": sum(1 for line in lines if "=" in line and not line.strip().endswith(":")),
        "declarative_lines": declarative_lines,
        "declarative_density": round(declarative_lines / len(lines), 3) if lines else 0.0,
    }


ASSIGNMENT_RE = re.compile(r"^\s*[A-Za-z_][A-Za-z0-9_\.\[\], ]*\s*=")


def metrics_for_imperative(path: Path):
    lines = logical_lines(load_lines(path))
    branch_keywords = sum(1 for line in lines if re.search(r"\b(if|elif|else)\b", line))
    loop_keywords = sum(1 for line in lines if re.search(r"\b(for|while)\b", line))
    state_assignment_lines = sum(
        1 for line in lines if ASSIGNMENT_RE.search(line) and not line.lstrip().startswith("def ")
    )
    return {
        "logical_loc": len(lines),
        "token_count": token_count(lines),
        "branch_keywords": branch_keywords,
        "loop_keywords": loop_keywords,
        "state_assignment_lines": state_assignment_lines,
        "declarative_lines": 0,
        "declarative_density": 0.0,
    }


def main():
    rows = []
    for name, sekai_path, imperative_path in SCENARIOS:
        rows.append(
            {
                "scenario": name,
                "sekai": metrics_for_sekai(sekai_path),
                "imperative": metrics_for_imperative(imperative_path),
            }
        )
    print(json.dumps({"scenarios": rows}, indent=2))


if __name__ == "__main__":
    main()
