#!/usr/bin/env python3
import argparse
import json
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent

SCENARIO_NAMES = [
    "bounce",
    "two_body_collision",
    "candidate_velocity",
    "clamped_region",
    "candidate_velocity_deferred",
    "visibility_occluded",
    "visibility_pursuit_occluded",
    "visibility_pursuit_world_occluded",
    "visibility_corridor_world_occluded",
    "visibility_coordination_flagship",
]

BASELINE_FAMILIES = [
    ("imperative", ROOT / "benchmarks" / "imperative"),
    ("event_driven", ROOT / "benchmarks" / "event_driven"),
    ("library_style", ROOT / "benchmarks" / "library_style"),
]


def load_lines(path: Path):
    return path.read_text().splitlines()


def logical_lines(lines):
    return [line for line in lines if line.strip() and not line.strip().startswith("#")]


def token_count(lines):
    return len(re.findall(r"[A-Za-z_][A-Za-z0-9_]*|[-+]?[0-9]*\.?[0-9]+|[^\s]", "\n".join(lines)))


DEFAULT_CATEGORIES = (
    "world_declaration",
    "law_declaration",
    "action_declaration",
    "observation_request",
    "update_mechanics",
    "event_detection",
    "repair_logic",
    "branching_selection",
    "reporting_boilerplate",
    "language_boilerplate",
    "other",
)


def empty_categories():
    return {category: 0 for category in DEFAULT_CATEGORIES}


def density(part: int, total: int):
    return round(part / total, 3) if total else 0.0


def category_total(categories, names):
    return sum(categories.get(name, 0) for name in names)


def metrics_for_sekai(path: Path):
    lines = logical_lines(load_lines(path))
    categories = empty_categories()
    mode = "top"

    for line in lines:
        stripped = line.strip()
        if stripped == "constraint:":
            mode = "constraint"
            categories["law_declaration"] += 1
            continue
        if stripped == "observe:":
            mode = "observe"
            categories["observation_request"] += 1
            continue
        if stripped == "action:":
            mode = "action"
            categories["action_declaration"] += 1
            continue
        if not line.startswith((" ", "\t")):
            mode = "top"

        if mode == "constraint":
            categories["law_declaration"] += 1
        elif mode == "observe" or stripped.startswith("snapshot at "):
            categories["observation_request"] += 1
        elif mode == "action":
            categories["action_declaration"] += 1
        elif re.match(r"^(sphere|plane|region)\s+", stripped) or re.match(
            r"^(position|velocity|radius|normal|offset|min|max)\(", stripped
        ):
            categories["world_declaration"] += 1
        else:
            categories["other"] += 1

    world_content_lines = category_total(
        categories,
        ("world_declaration", "law_declaration", "action_declaration", "observation_request"),
    )
    mechanics_lines = category_total(
        categories,
        ("update_mechanics", "event_detection", "repair_logic", "branching_selection"),
    )
    metrics = {
        "logical_loc": len(lines),
        "token_count": token_count(lines),
        "branch_keywords": 0,
        "loop_keywords": 0,
        "state_assignment_lines": sum(1 for line in lines if "=" in line and not line.strip().endswith(":")),
        "category_lines": categories,
        "world_content_lines": world_content_lines,
        "world_content_density": density(world_content_lines, len(lines)),
        "mechanics_lines": mechanics_lines,
        "mechanics_density": density(mechanics_lines, len(lines)),
    }
    # Backward-compatible aliases for older notes. New evaluation text should prefer
    # world_content_density because it is computed for both language families.
    metrics["declarative_lines"] = world_content_lines
    metrics["declarative_density"] = metrics["world_content_density"]
    return metrics


ASSIGNMENT_RE = re.compile(r"^\s*[A-Za-z_][A-Za-z0-9_\.\[\], ]*\s*=")


def classify_imperative_line(line: str):
    stripped = line.strip()

    if stripped.startswith("from worldlib import"):
        return "language_boilerplate"

    if re.search(r"\bworld\.(sphere|plane|region)\(", stripped):
        return "world_declaration"

    if re.search(r"\bworld\.(law|prefer_if_visible|prefer_if_occluded)\(", stripped):
        return "law_declaration"

    if re.search(r"\bworld\.candidate_velocity\(", stripped):
        return "action_declaration"

    if re.search(r"\bworld\.observe\(", stripped):
        return "observation_request"

    if re.search(r"\bworld\.simulate\(", stripped):
        return "reporting_boilerplate"

    if stripped.startswith(("class ", "def ", "if __name__")):
        return "language_boilerplate"

    if re.search(r"\breturn\b", stripped):
        return "reporting_boilerplate"

    if re.search(r"\b(for|while|if|elif|else)\b", stripped) or ".sort(" in stripped:
        if any(term in stripped for term in ("collision", "intersect", "inside", "see", "visible", "ZONE", "box")):
            return "event_detection"
        return "branching_selection"

    if any(
        term in stripped
        for term in (
            "position[",
            "velocity[",
            ".position",
            ".velocity",
            "current_time",
            "dt",
            "step(",
            "simulate(",
        )
    ):
        if any(term in stripped for term in ("=", "+=", "-=", "*=", "/=")):
            return "update_mechanics"

    if any(
        term in stripped
        for term in (
            "clamp",
            "reflect",
            "repair",
            "radius",
            "collision",
            "intersect",
            "inside",
            "can_see",
            "line_segment",
        )
    ):
        return "event_detection"

    if any(term in stripped for term in ("candidate", "score", "prefer", "selected", "resolve_velocity")):
        return "action_declaration" if any(term in stripped for term in ("candidates", "label", "score")) else "branching_selection"

    if re.search(r"\b(Sphere|wall_min|wall_max|zone|ZONE_|target_times|snapshots)\b", stripped):
        return "world_declaration"

    if any(term in stripped for term in ("False", "True", "continue")):
        return "branching_selection"

    if ASSIGNMENT_RE.search(line):
        return "world_declaration"

    return "other"


def metrics_for_imperative(path: Path):
    lines = logical_lines(load_lines(path))
    categories = empty_categories()
    for line in lines:
        categories[classify_imperative_line(line)] += 1

    branch_keywords = sum(
        1
        for line in lines
        if re.search(r"\b(if|elif|else)\b", line) and not line.strip().startswith("if __name__")
    )
    loop_keywords = sum(1 for line in lines if re.search(r"\b(for|while)\b", line))
    state_assignment_lines = sum(
        1 for line in lines if ASSIGNMENT_RE.search(line) and not line.lstrip().startswith("def ")
    )

    world_content_lines = category_total(
        categories,
        ("world_declaration", "law_declaration", "action_declaration", "observation_request"),
    )
    mechanics_lines = category_total(
        categories,
        ("update_mechanics", "event_detection", "repair_logic", "branching_selection"),
    )
    return {
        "logical_loc": len(lines),
        "token_count": token_count(lines),
        "branch_keywords": branch_keywords,
        "loop_keywords": loop_keywords,
        "state_assignment_lines": state_assignment_lines,
        "category_lines": categories,
        "world_content_lines": world_content_lines,
        "world_content_density": density(world_content_lines, len(lines)),
        "mechanics_lines": mechanics_lines,
        "mechanics_density": density(mechanics_lines, len(lines)),
        "declarative_lines": world_content_lines,
        "declarative_density": density(world_content_lines, len(lines)),
    }


def collect_metrics():
    rows = []
    for name in SCENARIO_NAMES:
        baseline_metrics = {}
        for family_name, family_dir in BASELINE_FAMILIES:
            baseline_path = family_dir / f"{name}.py"
            if baseline_path.exists():
                baseline_metrics[family_name] = metrics_for_imperative(baseline_path)
        rows.append(
            {
                "scenario": name,
                "sekai": metrics_for_sekai(ROOT / "examples" / f"{name}.sk"),
                "baselines": baseline_metrics,
            }
        )
    return {"scenarios": rows}


def titleize_scenario(name):
    return name.replace("_", " ").title()


def render_markdown(report):
    baseline_names = []
    for row in report["scenarios"]:
        for name in row["baselines"]:
            if name not in baseline_names:
                baseline_names.append(name)

    lines = [
        "# Phase K Baseline Metrics",
        "",
        "Measurements from `python3 scripts/spec_metrics.py --markdown`.",
        "",
        "These numbers are structural coding results, not final evidence. The current pass uses common categories for `sekai` and every baseline family so that setup and world-description lines in baselines are not forced to zero.",
        "",
    ]

    for row in report["scenarios"]:
        sekai = row["sekai"]
        columns = [("sekai", sekai)] + [(name.replace("_", " "), row["baselines"][name]) for name in baseline_names if name in row["baselines"]]
        header = "| metric | " + " | ".join(name for name, _ in columns) + " |"
        align = "| --- | " + " | ".join("---:" for _ in columns) + " |"

        def metric_row(label, key, numeric=False):
            values = []
            for _, metrics in columns:
                value = metrics[key]
                values.append(f"{value:.3f}" if numeric else str(value))
            return f"| {label} | " + " | ".join(values) + " |"

        lines.extend(
            [
                f"## {titleize_scenario(row['scenario'])}",
                "",
                header,
                align,
                metric_row("logical LOC", "logical_loc"),
                metric_row("token count", "token_count"),
                metric_row("branch keywords", "branch_keywords"),
                metric_row("loop keywords", "loop_keywords"),
                metric_row("state-assignment lines", "state_assignment_lines"),
                metric_row("world-content lines", "world_content_lines"),
                metric_row("world-content density", "world_content_density", numeric=True),
                metric_row("mechanics lines", "mechanics_lines"),
                metric_row("mechanics density", "mechanics_density", numeric=True),
                "",
            ]
        )

    lines.extend(
        [
            "## Reading",
            "",
            "- The current corpus still shows `sekai` specifications spending more of their surface form on world content.",
            "- The baselines now receive credit for setup and domain declarations, so the comparison no longer depends on assigning them zero declarative content.",
            "- The event-driven baselines are a stronger comparison point than the compact imperative baselines because they remove some frame-loop machinery from user code.",
            "- The library-style baselines show that a good host-language API can hide mechanics; the remaining `sekai` claim must therefore focus on language-level laws, contradiction identity, source spans, reports, and viewer continuity.",
            "- The stronger signal is the difference between world-content density and mechanics density, not raw LOC alone.",
            "- These measurements remain heuristic. The next step is to manually review the flagship scenario coding and compare report/source integration, not only line counts.",
        ]
    )
    return "\n".join(lines) + "\n"


def main():
    parser = argparse.ArgumentParser(description="Compute structural metrics for sekai examples and baselines.")
    parser.add_argument("--markdown", action="store_true", help="render the report as Markdown")
    args = parser.parse_args()

    report = collect_metrics()
    if args.markdown:
        print(render_markdown(report), end="")
    else:
        print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
