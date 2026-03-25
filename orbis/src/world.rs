use std::{collections::BTreeMap, fmt};

use crate::parser::{ActionCandidateDecl, ActionDirectiveDecl, Program};

const EPSILON: f64 = 1e-9;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn magnitude(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalized(self) -> Result<Self, SimulationError> {
        let length = self.magnitude();
        if length <= EPSILON {
            return Err(SimulationError::InvalidPlaneNormal);
        }
        Ok(Self::new(self.x / length, self.y / length, self.z / length))
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.3}, {:.3}, {:.3})", self.x, self.y, self.z)
    }
}

#[derive(Clone, Debug)]
pub struct Sphere {
    pub name: String,
    pub position: Vec3,
    pub velocity: Vec3,
    pub radius: f64,
    pub last_update_time: f64,
}

#[derive(Clone, Debug)]
pub struct Plane {
    pub name: String,
    pub normal: Vec3,
    pub offset: f64,
}

#[derive(Clone, Debug)]
pub struct Region {
    pub name: String,
    pub min: Vec3,
    pub max: Vec3,
}

#[derive(Clone, Debug)]
pub enum Constraint {
    ReflectOnCollision { sphere_index: usize },
    VelocityLimit {
        sphere_index: usize,
        max_speed: f64,
        policy: RepairPolicy,
    },
    NotInside {
        sphere_index: usize,
        policy: RepairPolicy,
    },
    ElasticCollision { left_index: usize, right_index: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RepairPolicy {
    Reject,
    Clamp,
    Reflect,
}

#[derive(Clone, Debug)]
pub struct World {
    pub spheres: Vec<Sphere>,
    pub plane: Plane,
    pub region: Option<Region>,
    pub constraints: Vec<Constraint>,
    pub constraint_traces: Vec<ConstraintTrace>,
    pub candidate_resolutions: Vec<CandidateResolution>,
    pub activity_log: Vec<ActivityEntry>,
}

#[derive(Clone, Debug)]
pub struct SphereSnapshot {
    pub name: String,
    pub position: Vec3,
    pub velocity: Vec3,
}

#[derive(Clone, Debug)]
pub struct Snapshot {
    pub time: f64,
    pub spheres: Vec<SphereSnapshot>,
}

#[derive(Clone, Debug)]
pub struct SimulationReport {
    pub analytics: LawAnalytics,
    pub constraints: Vec<ConstraintSummary>,
    pub convergence_analytics: ConvergenceAnalytics,
    pub observation_summary: ObservationSummary,
    pub candidate_resolutions: Vec<CandidateResolution>,
    pub activities: Vec<ActivityEntry>,
    pub snapshots: Vec<Snapshot>,
}

#[derive(Clone, Debug)]
pub struct LawInventory {
    pub analytics: LawAnalytics,
    pub constraints: Vec<ConstraintSummary>,
    pub candidate_inventory: Vec<CandidateInventorySummary>,
    pub action_directive_inventory: Vec<ActionDirectiveSummary>,
}

#[derive(Clone, Debug)]
pub struct CandidateInventorySummary {
    pub entity: String,
    pub total_candidates: usize,
    pub labels: Vec<String>,
    pub top_score: String,
    pub top_labels: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct ActionDirectiveSummary {
    pub entity: String,
    pub kind: String,
}

#[derive(Clone, Debug)]
pub struct CandidateResolution {
    pub entity: String,
    pub total_candidates: usize,
    pub rejected_candidates: usize,
    pub skipped_candidates: usize,
    pub convergence_mode: String,
    pub observation_mode: String,
    pub observation_labels: Vec<String>,
    pub symbolically_underdetermined: bool,
    pub observationally_underdetermined: bool,
    pub selected_candidate: Option<String>,
    pub selected_score: Option<String>,
    pub top_score: String,
    pub top_labels: Vec<String>,
    pub tie_broken: bool,
    pub equivalent_top_labels: Vec<String>,
    pub observationally_equivalent_tie: bool,
    pub repaired_after_selection: bool,
}

#[derive(Clone, Debug)]
pub struct ConvergenceAnalytics {
    pub candidate_entities: usize,
    pub direct_entities: usize,
    pub fallback_entities: usize,
    pub repaired_entities: usize,
    pub deferred_entities: usize,
    pub tie_broken_entities: usize,
    pub equivalent_tie_entities: usize,
    pub determinate_entities: usize,
    pub representative_entities: usize,
    pub ambiguous_entities: usize,
    pub symbolically_underdetermined_entities: usize,
    pub observationally_underdetermined_entities: usize,
    pub rejected_candidates_total: usize,
    pub skipped_candidates_total: usize,
}

#[derive(Clone, Debug)]
pub struct ObservationSummary {
    pub status: String,
    pub representative_entities: usize,
    pub ambiguous_entities: usize,
}

#[derive(Clone, Debug)]
pub struct LawAnalytics {
    pub total_constraints: usize,
    pub invariant_constraints: usize,
    pub boundary_constraints: usize,
    pub interaction_constraints: usize,
    pub idle_constraints: usize,
    pub fired_constraints: usize,
    pub repaired_constraints: usize,
    pub contradicted_constraints: usize,
}

#[derive(Clone, Debug)]
pub struct ConstraintSummary {
    pub kind: String,
    pub category: String,
    pub targets: Vec<String>,
    pub policy: String,
    pub supported_policies: Vec<String>,
    pub outcome: String,
    pub fired_count: usize,
    pub repaired_count: usize,
    pub contradicted_count: usize,
}

#[derive(Clone, Debug, Default)]
pub struct ConstraintTrace {
    pub fired_count: usize,
    pub repaired_count: usize,
    pub contradicted_count: usize,
}

#[derive(Clone, Debug)]
pub struct ActivityEntry {
    pub time: f64,
    pub kind: String,
    pub targets: Vec<String>,
    pub policy: String,
    pub action: String,
}

impl SimulationReport {
    pub fn to_json(&self, source: &str) -> String {
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str(&format!("  \"source\": \"{}\",\n", escape_json(source)));
        json.push_str("  \"analytics\": {\n");
        json.push_str(&format!(
            "    \"total_constraints\": {},\n",
            self.analytics.total_constraints
        ));
        json.push_str(&format!(
            "    \"invariant_constraints\": {},\n",
            self.analytics.invariant_constraints
        ));
        json.push_str(&format!(
            "    \"boundary_constraints\": {},\n",
            self.analytics.boundary_constraints
        ));
        json.push_str(&format!(
            "    \"interaction_constraints\": {},\n",
            self.analytics.interaction_constraints
        ));
        json.push_str(&format!(
            "    \"idle_constraints\": {},\n",
            self.analytics.idle_constraints
        ));
        json.push_str(&format!(
            "    \"fired_constraints\": {},\n",
            self.analytics.fired_constraints
        ));
        json.push_str(&format!(
            "    \"repaired_constraints\": {},\n",
            self.analytics.repaired_constraints
        ));
        json.push_str(&format!(
            "    \"contradicted_constraints\": {}\n",
            self.analytics.contradicted_constraints
        ));
        json.push_str("  },\n");
        json.push_str("  \"constraints\": [\n");
        for (index, constraint) in self.constraints.iter().enumerate() {
            json.push_str("    {\n");
            json.push_str(&format!(
                "      \"kind\": \"{}\",\n",
                escape_json(&constraint.kind)
            ));
            json.push_str(&format!(
                "      \"category\": \"{}\",\n",
                escape_json(&constraint.category)
            ));
            json.push_str("      \"targets\": [");
            for (target_index, target) in constraint.targets.iter().enumerate() {
                json.push_str(&format!("\"{}\"", escape_json(target)));
                if target_index + 1 != constraint.targets.len() {
                    json.push_str(", ");
                }
            }
            json.push_str("],\n");
            json.push_str(&format!(
                "      \"policy\": \"{}\",\n",
                escape_json(&constraint.policy)
            ));
            json.push_str("      \"supported_policies\": [");
            for (policy_index, supported_policy) in
                constraint.supported_policies.iter().enumerate()
            {
                json.push_str(&format!("\"{}\"", escape_json(supported_policy)));
                if policy_index + 1 != constraint.supported_policies.len() {
                    json.push_str(", ");
                }
            }
            json.push_str("],\n");
            json.push_str(&format!(
                "      \"outcome\": \"{}\",\n",
                escape_json(&constraint.outcome)
            ));
            json.push_str(&format!(
                "      \"fired_count\": {},\n",
                constraint.fired_count
            ));
            json.push_str(&format!(
                "      \"repaired_count\": {},\n",
                constraint.repaired_count
            ));
            json.push_str(&format!(
                "      \"contradicted_count\": {}\n",
                constraint.contradicted_count
            ));
            json.push_str("    }");
            if index + 1 != self.constraints.len() {
                json.push(',');
            }
            json.push('\n');
        }
        json.push_str("  ],\n");
        json.push_str("  \"convergence_analytics\": {\n");
        json.push_str(&format!(
            "    \"candidate_entities\": {},\n",
            self.convergence_analytics.candidate_entities
        ));
        json.push_str(&format!(
            "    \"direct_entities\": {},\n",
            self.convergence_analytics.direct_entities
        ));
        json.push_str(&format!(
            "    \"fallback_entities\": {},\n",
            self.convergence_analytics.fallback_entities
        ));
        json.push_str(&format!(
            "    \"repaired_entities\": {},\n",
            self.convergence_analytics.repaired_entities
        ));
        json.push_str(&format!(
            "    \"deferred_entities\": {},\n",
            self.convergence_analytics.deferred_entities
        ));
        json.push_str(&format!(
            "    \"tie_broken_entities\": {},\n",
            self.convergence_analytics.tie_broken_entities
        ));
        json.push_str(&format!(
            "    \"equivalent_tie_entities\": {},\n",
            self.convergence_analytics.equivalent_tie_entities
        ));
        json.push_str(&format!(
            "    \"determinate_entities\": {},\n",
            self.convergence_analytics.determinate_entities
        ));
        json.push_str(&format!(
            "    \"representative_entities\": {},\n",
            self.convergence_analytics.representative_entities
        ));
        json.push_str(&format!(
            "    \"ambiguous_entities\": {},\n",
            self.convergence_analytics.ambiguous_entities
        ));
        json.push_str(&format!(
            "    \"symbolically_underdetermined_entities\": {},\n",
            self.convergence_analytics.symbolically_underdetermined_entities
        ));
        json.push_str(&format!(
            "    \"observationally_underdetermined_entities\": {},\n",
            self.convergence_analytics.observationally_underdetermined_entities
        ));
        json.push_str(&format!(
            "    \"rejected_candidates_total\": {},\n",
            self.convergence_analytics.rejected_candidates_total
        ));
        json.push_str(&format!(
            "    \"skipped_candidates_total\": {}\n",
            self.convergence_analytics.skipped_candidates_total
        ));
        json.push_str("  },\n");
        json.push_str("  \"observation_summary\": {\n");
        json.push_str(&format!(
            "    \"status\": \"{}\",\n",
            escape_json(&self.observation_summary.status)
        ));
        json.push_str(&format!(
            "    \"representative_entities\": {},\n",
            self.observation_summary.representative_entities
        ));
        json.push_str(&format!(
            "    \"ambiguous_entities\": {}\n",
            self.observation_summary.ambiguous_entities
        ));
        json.push_str("  },\n");
        json.push_str("  \"candidate_resolutions\": [\n");
        for (index, candidate_resolution) in self.candidate_resolutions.iter().enumerate() {
            json.push_str("    {\n");
            json.push_str(&format!(
                "      \"entity\": \"{}\",\n",
                escape_json(&candidate_resolution.entity)
            ));
            json.push_str(&format!(
                "      \"total_candidates\": {},\n",
                candidate_resolution.total_candidates
            ));
                json.push_str(&format!(
                    "      \"rejected_candidates\": {},\n",
                    candidate_resolution.rejected_candidates
                ));
            json.push_str(&format!(
                "      \"skipped_candidates\": {},\n",
                candidate_resolution.skipped_candidates
            ));
            json.push_str(&format!(
                "      \"convergence_mode\": \"{}\",\n",
                escape_json(&candidate_resolution.convergence_mode)
            ));
            json.push_str(&format!(
                "      \"observation_mode\": \"{}\",\n",
                escape_json(&candidate_resolution.observation_mode)
            ));
            json.push_str("      \"observation_labels\": [");
            for (label_index, label) in candidate_resolution.observation_labels.iter().enumerate() {
                json.push_str(&format!("\"{}\"", escape_json(label)));
                if label_index + 1 != candidate_resolution.observation_labels.len() {
                    json.push_str(", ");
                }
            }
            json.push_str("],\n");
            json.push_str(&format!(
                "      \"symbolically_underdetermined\": {},\n",
                candidate_resolution.symbolically_underdetermined
            ));
            json.push_str(&format!(
                "      \"observationally_underdetermined\": {},\n",
                candidate_resolution.observationally_underdetermined
            ));
                match &candidate_resolution.selected_candidate {
                    Some(selected_candidate) => json.push_str(&format!(
                        "      \"selected_candidate\": \"{}\",\n",
                    escape_json(selected_candidate)
                )),
                None => json.push_str("      \"selected_candidate\": null,\n"),
            }
                match &candidate_resolution.selected_score {
                    Some(selected_score) => json.push_str(&format!(
                        "      \"selected_score\": \"{}\",\n",
                        escape_json(selected_score)
                    )),
                    None => json.push_str("      \"selected_score\": null,\n"),
                }
                json.push_str(&format!(
                    "      \"top_score\": \"{}\",\n",
                    escape_json(&candidate_resolution.top_score)
                ));
            json.push_str("      \"top_labels\": [");
            for (label_index, label) in candidate_resolution.top_labels.iter().enumerate() {
                json.push_str(&format!("\"{}\"", escape_json(label)));
                if label_index + 1 != candidate_resolution.top_labels.len() {
                    json.push_str(", ");
                }
            }
            json.push_str("],\n");
            json.push_str(&format!(
                "      \"tie_broken\": {},\n",
                candidate_resolution.tie_broken
            ));
            json.push_str("      \"equivalent_top_labels\": [");
            for (label_index, label) in candidate_resolution
                .equivalent_top_labels
                .iter()
                .enumerate()
            {
                json.push_str(&format!("\"{}\"", escape_json(label)));
                if label_index + 1 != candidate_resolution.equivalent_top_labels.len() {
                    json.push_str(", ");
                }
            }
            json.push_str("],\n");
            json.push_str(&format!(
                "      \"observationally_equivalent_tie\": {},\n",
                candidate_resolution.observationally_equivalent_tie
            ));
            json.push_str(&format!(
                "      \"repaired_after_selection\": {}\n",
                candidate_resolution.repaired_after_selection
            ));
            json.push_str("    }");
            if index + 1 != self.candidate_resolutions.len() {
                json.push(',');
            }
            json.push('\n');
        }
        json.push_str("  ],\n");
        json.push_str("  \"activities\": [\n");
        for (index, activity) in self.activities.iter().enumerate() {
            json.push_str("    {\n");
            json.push_str(&format!("      \"time\": {:.6},\n", activity.time));
            json.push_str(&format!(
                "      \"kind\": \"{}\",\n",
                escape_json(&activity.kind)
            ));
            json.push_str("      \"targets\": [");
            for (target_index, target) in activity.targets.iter().enumerate() {
                json.push_str(&format!("\"{}\"", escape_json(target)));
                if target_index + 1 != activity.targets.len() {
                    json.push_str(", ");
                }
            }
            json.push_str("],\n");
            json.push_str(&format!(
                "      \"policy\": \"{}\",\n",
                escape_json(&activity.policy)
            ));
            json.push_str(&format!(
                "      \"action\": \"{}\"\n",
                escape_json(&activity.action)
            ));
            json.push_str("    }");
            if index + 1 != self.activities.len() {
                json.push(',');
            }
            json.push('\n');
        }
        json.push_str("  ],\n");
        json.push_str("  \"snapshots\": [\n");

        for (snapshot_index, snapshot) in self.snapshots.iter().enumerate() {
            json.push_str("    {\n");
            json.push_str(&format!("      \"time\": {:.6},\n", snapshot.time));
            json.push_str("      \"spheres\": [\n");

            for (sphere_index, sphere) in snapshot.spheres.iter().enumerate() {
                json.push_str("        {\n");
                json.push_str(&format!(
                    "          \"name\": \"{}\",\n",
                    escape_json(&sphere.name)
                ));
                json.push_str(&format!(
                    "          \"position\": {{ \"x\": {:.6}, \"y\": {:.6}, \"z\": {:.6} }},\n",
                    sphere.position.x, sphere.position.y, sphere.position.z
                ));
                json.push_str(&format!(
                    "          \"velocity\": {{ \"x\": {:.6}, \"y\": {:.6}, \"z\": {:.6} }}\n",
                    sphere.velocity.x, sphere.velocity.y, sphere.velocity.z
                ));
                json.push_str("        }");
                if sphere_index + 1 != snapshot.spheres.len() {
                    json.push(',');
                }
                json.push('\n');
            }

            json.push_str("      ]\n");
            json.push_str("    }");
            if snapshot_index + 1 != self.snapshots.len() {
                json.push(',');
            }
            json.push('\n');
        }

        json.push_str("  ]\n");
        json.push('}');
        json
    }
}

impl LawInventory {
    pub fn to_json(&self, source: &str) -> String {
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str(&format!("  \"source\": \"{}\",\n", escape_json(source)));
        json.push_str("  \"analytics\": {\n");
        json.push_str(&format!(
            "    \"total_constraints\": {},\n",
            self.analytics.total_constraints
        ));
        json.push_str(&format!(
            "    \"invariant_constraints\": {},\n",
            self.analytics.invariant_constraints
        ));
        json.push_str(&format!(
            "    \"boundary_constraints\": {},\n",
            self.analytics.boundary_constraints
        ));
        json.push_str(&format!(
            "    \"interaction_constraints\": {},\n",
            self.analytics.interaction_constraints
        ));
        json.push_str(&format!(
            "    \"idle_constraints\": {},\n",
            self.analytics.idle_constraints
        ));
        json.push_str(&format!(
            "    \"fired_constraints\": {},\n",
            self.analytics.fired_constraints
        ));
        json.push_str(&format!(
            "    \"repaired_constraints\": {},\n",
            self.analytics.repaired_constraints
        ));
        json.push_str(&format!(
            "    \"contradicted_constraints\": {}\n",
            self.analytics.contradicted_constraints
        ));
        json.push_str("  },\n");
        json.push_str("  \"constraints\": [\n");
        for (index, constraint) in self.constraints.iter().enumerate() {
            json.push_str("    {\n");
            json.push_str(&format!(
                "      \"kind\": \"{}\",\n",
                escape_json(&constraint.kind)
            ));
            json.push_str(&format!(
                "      \"category\": \"{}\",\n",
                escape_json(&constraint.category)
            ));
            json.push_str("      \"targets\": [");
            for (target_index, target) in constraint.targets.iter().enumerate() {
                json.push_str(&format!("\"{}\"", escape_json(target)));
                if target_index + 1 != constraint.targets.len() {
                    json.push_str(", ");
                }
            }
            json.push_str("],\n");
            json.push_str(&format!(
                "      \"policy\": \"{}\",\n",
                escape_json(&constraint.policy)
            ));
            json.push_str("      \"supported_policies\": [");
            for (policy_index, supported_policy) in constraint.supported_policies.iter().enumerate()
            {
                json.push_str(&format!("\"{}\"", escape_json(supported_policy)));
                if policy_index + 1 != constraint.supported_policies.len() {
                    json.push_str(", ");
                }
            }
            json.push_str("],\n");
            json.push_str(&format!(
                "      \"outcome\": \"{}\",\n",
                escape_json(&constraint.outcome)
            ));
            json.push_str(&format!(
                "      \"fired_count\": {},\n",
                constraint.fired_count
            ));
            json.push_str(&format!(
                "      \"repaired_count\": {},\n",
                constraint.repaired_count
            ));
            json.push_str(&format!(
                "      \"contradicted_count\": {}\n",
                constraint.contradicted_count
            ));
            json.push_str("    }");
            if index + 1 != self.constraints.len() {
                json.push(',');
            }
            json.push('\n');
        }
        json.push_str("  ],\n");
        json.push_str("  \"candidate_inventory\": [\n");
        for (index, candidate_inventory) in self.candidate_inventory.iter().enumerate() {
            json.push_str("    {\n");
            json.push_str(&format!(
                "      \"entity\": \"{}\",\n",
                escape_json(&candidate_inventory.entity)
            ));
            json.push_str(&format!(
                "      \"total_candidates\": {},\n",
                candidate_inventory.total_candidates
            ));
            json.push_str("      \"labels\": [");
            for (label_index, label) in candidate_inventory.labels.iter().enumerate() {
                json.push_str(&format!("\"{}\"", escape_json(label)));
                if label_index + 1 != candidate_inventory.labels.len() {
                    json.push_str(", ");
                }
            }
            json.push_str("],\n");
            json.push_str(&format!(
                "      \"top_score\": \"{}\",\n",
                escape_json(&candidate_inventory.top_score)
            ));
            json.push_str("      \"top_labels\": [");
            for (label_index, label) in candidate_inventory.top_labels.iter().enumerate() {
                json.push_str(&format!("\"{}\"", escape_json(label)));
                if label_index + 1 != candidate_inventory.top_labels.len() {
                    json.push_str(", ");
                }
            }
            json.push_str("]\n");
            json.push_str("    }");
            if index + 1 != self.candidate_inventory.len() {
                json.push(',');
            }
            json.push('\n');
        }
        json.push_str("  ],\n");
        json.push_str("  \"action_directive_inventory\": [\n");
        for (index, directive) in self.action_directive_inventory.iter().enumerate() {
            json.push_str("    {\n");
            json.push_str(&format!(
                "      \"entity\": \"{}\",\n",
                escape_json(&directive.entity)
            ));
            json.push_str(&format!(
                "      \"kind\": \"{}\"\n",
                escape_json(&directive.kind)
            ));
            json.push_str("    }");
            if index + 1 != self.action_directive_inventory.len() {
                json.push(',');
            }
            json.push('\n');
        }
        json.push_str("  ]\n");
        json.push('}');
        json
    }
}

#[derive(Clone, Debug)]
pub struct SimulationEnvelope {
    pub source: String,
    pub status: String,
    pub report: Option<SimulationReport>,
    pub error: Option<String>,
}

impl SimulationEnvelope {
    pub fn success(source: &str, report: SimulationReport) -> Self {
        Self {
            source: source.to_string(),
            status: "ok".to_string(),
            report: Some(report),
            error: None,
        }
    }

    pub fn failure(source: &str, error: impl Into<String>) -> Self {
        Self {
            source: source.to_string(),
            status: "error".to_string(),
            report: None,
            error: Some(error.into()),
        }
    }

    pub fn failure_with_report(
        source: &str,
        error: impl Into<String>,
        report: SimulationReport,
    ) -> Self {
        Self {
            source: source.to_string(),
            status: "error".to_string(),
            report: Some(report),
            error: Some(error.into()),
        }
    }

    pub fn to_json(&self) -> String {
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str(&format!("  \"source\": \"{}\",\n", escape_json(&self.source)));
        json.push_str(&format!("  \"status\": \"{}\",\n", escape_json(&self.status)));
        match &self.report {
            Some(report) => {
                match &self.error {
                    Some(error) => json.push_str(&format!(
                        "  \"error\": \"{}\",\n",
                        escape_json(error)
                    )),
                    None => json.push_str("  \"error\": null,\n"),
                }
                json.push_str("  \"analytics\": {\n");
                json.push_str(&format!(
                    "    \"total_constraints\": {},\n",
                    report.analytics.total_constraints
                ));
                json.push_str(&format!(
                    "    \"invariant_constraints\": {},\n",
                    report.analytics.invariant_constraints
                ));
                json.push_str(&format!(
                    "    \"boundary_constraints\": {},\n",
                    report.analytics.boundary_constraints
                ));
                json.push_str(&format!(
                    "    \"interaction_constraints\": {},\n",
                    report.analytics.interaction_constraints
                ));
                json.push_str(&format!(
                    "    \"idle_constraints\": {},\n",
                    report.analytics.idle_constraints
                ));
                json.push_str(&format!(
                    "    \"fired_constraints\": {},\n",
                    report.analytics.fired_constraints
                ));
                json.push_str(&format!(
                    "    \"repaired_constraints\": {},\n",
                    report.analytics.repaired_constraints
                ));
                json.push_str(&format!(
                    "    \"contradicted_constraints\": {}\n",
                    report.analytics.contradicted_constraints
                ));
                json.push_str("  },\n");
                json.push_str("  \"constraints\": [\n");
                for (index, constraint) in report.constraints.iter().enumerate() {
                    json.push_str("    {\n");
                    json.push_str(&format!(
                        "      \"kind\": \"{}\",\n",
                        escape_json(&constraint.kind)
                    ));
                    json.push_str(&format!(
                        "      \"category\": \"{}\",\n",
                        escape_json(&constraint.category)
                    ));
                    json.push_str("      \"targets\": [");
                    for (target_index, target) in constraint.targets.iter().enumerate() {
                        json.push_str(&format!("\"{}\"", escape_json(target)));
                        if target_index + 1 != constraint.targets.len() {
                            json.push_str(", ");
                        }
                    }
                    json.push_str("],\n");
                    json.push_str(&format!(
                        "      \"policy\": \"{}\",\n",
                        escape_json(&constraint.policy)
                    ));
                    json.push_str("      \"supported_policies\": [");
                    for (policy_index, supported_policy) in
                        constraint.supported_policies.iter().enumerate()
                    {
                        json.push_str(&format!("\"{}\"", escape_json(supported_policy)));
                        if policy_index + 1 != constraint.supported_policies.len() {
                            json.push_str(", ");
                        }
                    }
                    json.push_str("],\n");
                    json.push_str(&format!(
                        "      \"outcome\": \"{}\",\n",
                        escape_json(&constraint.outcome)
                    ));
                    json.push_str(&format!(
                        "      \"fired_count\": {},\n",
                        constraint.fired_count
                    ));
                    json.push_str(&format!(
                        "      \"repaired_count\": {},\n",
                        constraint.repaired_count
                    ));
                    json.push_str(&format!(
                        "      \"contradicted_count\": {}\n",
                        constraint.contradicted_count
                    ));
                    json.push_str("    }");
                    if index + 1 != report.constraints.len() {
                        json.push(',');
                    }
                    json.push('\n');
                }
                json.push_str("  ],\n");
                json.push_str("  \"convergence_analytics\": {\n");
                json.push_str(&format!(
                    "    \"candidate_entities\": {},\n",
                    report.convergence_analytics.candidate_entities
                ));
                json.push_str(&format!(
                    "    \"direct_entities\": {},\n",
                    report.convergence_analytics.direct_entities
                ));
                json.push_str(&format!(
                    "    \"fallback_entities\": {},\n",
                    report.convergence_analytics.fallback_entities
                ));
                json.push_str(&format!(
                    "    \"repaired_entities\": {},\n",
                    report.convergence_analytics.repaired_entities
                ));
                json.push_str(&format!(
                    "    \"deferred_entities\": {},\n",
                    report.convergence_analytics.deferred_entities
                ));
                json.push_str(&format!(
                    "    \"tie_broken_entities\": {},\n",
                    report.convergence_analytics.tie_broken_entities
                ));
                json.push_str(&format!(
                    "    \"equivalent_tie_entities\": {},\n",
                    report.convergence_analytics.equivalent_tie_entities
                ));
                json.push_str(&format!(
                    "    \"determinate_entities\": {},\n",
                    report.convergence_analytics.determinate_entities
                ));
                json.push_str(&format!(
                    "    \"representative_entities\": {},\n",
                    report.convergence_analytics.representative_entities
                ));
                json.push_str(&format!(
                    "    \"ambiguous_entities\": {},\n",
                    report.convergence_analytics.ambiguous_entities
                ));
                json.push_str(&format!(
                    "    \"symbolically_underdetermined_entities\": {},\n",
                    report.convergence_analytics.symbolically_underdetermined_entities
                ));
                json.push_str(&format!(
                    "    \"observationally_underdetermined_entities\": {},\n",
                    report.convergence_analytics.observationally_underdetermined_entities
                ));
                json.push_str(&format!(
                    "    \"rejected_candidates_total\": {},\n",
                    report.convergence_analytics.rejected_candidates_total
                ));
                json.push_str(&format!(
                    "    \"skipped_candidates_total\": {}\n",
                    report.convergence_analytics.skipped_candidates_total
                ));
                json.push_str("  },\n");
                json.push_str("  \"observation_summary\": {\n");
                json.push_str(&format!(
                    "    \"status\": \"{}\",\n",
                    escape_json(&report.observation_summary.status)
                ));
                json.push_str(&format!(
                    "    \"representative_entities\": {},\n",
                    report.observation_summary.representative_entities
                ));
                json.push_str(&format!(
                    "    \"ambiguous_entities\": {}\n",
                    report.observation_summary.ambiguous_entities
                ));
                json.push_str("  },\n");
                json.push_str("  \"candidate_resolutions\": [\n");
                for (index, candidate_resolution) in report.candidate_resolutions.iter().enumerate()
                {
                    json.push_str("    {\n");
                    json.push_str(&format!(
                        "      \"entity\": \"{}\",\n",
                        escape_json(&candidate_resolution.entity)
                    ));
                    json.push_str(&format!(
                        "      \"total_candidates\": {},\n",
                        candidate_resolution.total_candidates
                    ));
                    json.push_str(&format!(
                        "      \"rejected_candidates\": {},\n",
                        candidate_resolution.rejected_candidates
                    ));
                    json.push_str(&format!(
                        "      \"skipped_candidates\": {},\n",
                        candidate_resolution.skipped_candidates
                    ));
                    json.push_str(&format!(
                        "      \"convergence_mode\": \"{}\",\n",
                        escape_json(&candidate_resolution.convergence_mode)
                    ));
                    json.push_str(&format!(
                        "      \"observation_mode\": \"{}\",\n",
                        escape_json(&candidate_resolution.observation_mode)
                    ));
                    json.push_str("      \"observation_labels\": [");
                    for (label_index, label) in
                        candidate_resolution.observation_labels.iter().enumerate()
                    {
                        json.push_str(&format!("\"{}\"", escape_json(label)));
                        if label_index + 1 != candidate_resolution.observation_labels.len() {
                            json.push_str(", ");
                        }
                    }
                    json.push_str("],\n");
                    json.push_str(&format!(
                        "      \"symbolically_underdetermined\": {},\n",
                        candidate_resolution.symbolically_underdetermined
                    ));
                    json.push_str(&format!(
                        "      \"observationally_underdetermined\": {},\n",
                        candidate_resolution.observationally_underdetermined
                    ));
                    match &candidate_resolution.selected_candidate {
                        Some(selected_candidate) => json.push_str(&format!(
                            "      \"selected_candidate\": \"{}\",\n",
                            escape_json(selected_candidate)
                        )),
                        None => json.push_str("      \"selected_candidate\": null,\n"),
                    }
                    match &candidate_resolution.selected_score {
                        Some(selected_score) => json.push_str(&format!(
                            "      \"selected_score\": \"{}\",\n",
                            escape_json(selected_score)
                        )),
                        None => json.push_str("      \"selected_score\": null,\n"),
                    }
                    json.push_str(&format!(
                        "      \"top_score\": \"{}\",\n",
                        escape_json(&candidate_resolution.top_score)
                    ));
                    json.push_str("      \"top_labels\": [");
                    for (label_index, label) in candidate_resolution.top_labels.iter().enumerate()
                    {
                        json.push_str(&format!("\"{}\"", escape_json(label)));
                        if label_index + 1 != candidate_resolution.top_labels.len() {
                            json.push_str(", ");
                        }
                    }
                    json.push_str("],\n");
                    json.push_str(&format!(
                        "      \"tie_broken\": {},\n",
                        candidate_resolution.tie_broken
                    ));
                    json.push_str("      \"equivalent_top_labels\": [");
                    for (label_index, label) in candidate_resolution
                        .equivalent_top_labels
                        .iter()
                        .enumerate()
                    {
                        json.push_str(&format!("\"{}\"", escape_json(label)));
                        if label_index + 1 != candidate_resolution.equivalent_top_labels.len() {
                            json.push_str(", ");
                        }
                    }
                    json.push_str("],\n");
                    json.push_str(&format!(
                        "      \"observationally_equivalent_tie\": {},\n",
                        candidate_resolution.observationally_equivalent_tie
                    ));
                    json.push_str(&format!(
                        "      \"repaired_after_selection\": {}\n",
                        candidate_resolution.repaired_after_selection
                    ));
                    json.push_str("    }");
                    if index + 1 != report.candidate_resolutions.len() {
                        json.push(',');
                    }
                    json.push('\n');
                }
                json.push_str("  ],\n");
                json.push_str("  \"activities\": [\n");
                for (index, activity) in report.activities.iter().enumerate() {
                    json.push_str("    {\n");
                    json.push_str(&format!("      \"time\": {:.6},\n", activity.time));
                    json.push_str(&format!(
                        "      \"kind\": \"{}\",\n",
                        escape_json(&activity.kind)
                    ));
                    json.push_str("      \"targets\": [");
                    for (target_index, target) in activity.targets.iter().enumerate() {
                        json.push_str(&format!("\"{}\"", escape_json(target)));
                        if target_index + 1 != activity.targets.len() {
                            json.push_str(", ");
                        }
                    }
                    json.push_str("],\n");
                    json.push_str(&format!(
                        "      \"policy\": \"{}\",\n",
                        escape_json(&activity.policy)
                    ));
                    json.push_str(&format!(
                        "      \"action\": \"{}\"\n",
                        escape_json(&activity.action)
                    ));
                    json.push_str("    }");
                    if index + 1 != report.activities.len() {
                        json.push(',');
                    }
                    json.push('\n');
                }
                json.push_str("  ],\n");
                json.push_str("  \"snapshots\": [\n");
                for (snapshot_index, snapshot) in report.snapshots.iter().enumerate() {
                    json.push_str("    {\n");
                    json.push_str(&format!("      \"time\": {:.6},\n", snapshot.time));
                    json.push_str("      \"spheres\": [\n");
                    for (sphere_index, sphere) in snapshot.spheres.iter().enumerate() {
                        json.push_str("        {\n");
                        json.push_str(&format!(
                            "          \"name\": \"{}\",\n",
                            escape_json(&sphere.name)
                        ));
                        json.push_str(&format!(
                            "          \"position\": {{ \"x\": {:.6}, \"y\": {:.6}, \"z\": {:.6} }},\n",
                            sphere.position.x, sphere.position.y, sphere.position.z
                        ));
                        json.push_str(&format!(
                            "          \"velocity\": {{ \"x\": {:.6}, \"y\": {:.6}, \"z\": {:.6} }}\n",
                            sphere.velocity.x, sphere.velocity.y, sphere.velocity.z
                        ));
                        json.push_str("        }");
                        if sphere_index + 1 != snapshot.spheres.len() {
                            json.push(',');
                        }
                        json.push('\n');
                    }
                    json.push_str("      ]\n");
                    json.push_str("    }");
                    if snapshot_index + 1 != report.snapshots.len() {
                        json.push(',');
                    }
                    json.push('\n');
                }
                json.push_str("  ]\n");
            }
            None => {
                json.push_str(&format!(
                    "  \"error\": \"{}\",\n",
                    escape_json(self.error.as_deref().unwrap_or("unknown error"))
                ));
                json.push_str("  \"analytics\": {\n");
                json.push_str("    \"total_constraints\": 0,\n");
                json.push_str("    \"invariant_constraints\": 0,\n");
                json.push_str("    \"boundary_constraints\": 0,\n");
                json.push_str("    \"interaction_constraints\": 0,\n");
                json.push_str("    \"idle_constraints\": 0,\n");
                json.push_str("    \"fired_constraints\": 0,\n");
                json.push_str("    \"repaired_constraints\": 0,\n");
                json.push_str("    \"contradicted_constraints\": 0\n");
                json.push_str("  },\n");
                json.push_str("  \"constraints\": [],\n");
                json.push_str("  \"convergence_analytics\": {\n");
                json.push_str("    \"candidate_entities\": 0,\n");
                json.push_str("    \"direct_entities\": 0,\n");
                json.push_str("    \"fallback_entities\": 0,\n");
                json.push_str("    \"repaired_entities\": 0,\n");
                json.push_str("    \"deferred_entities\": 0,\n");
                json.push_str("    \"tie_broken_entities\": 0,\n");
                json.push_str("    \"equivalent_tie_entities\": 0,\n");
                json.push_str("    \"determinate_entities\": 0,\n");
                json.push_str("    \"representative_entities\": 0,\n");
                json.push_str("    \"ambiguous_entities\": 0,\n");
                json.push_str("    \"symbolically_underdetermined_entities\": 0,\n");
                json.push_str("    \"observationally_underdetermined_entities\": 0,\n");
                json.push_str("    \"rejected_candidates_total\": 0,\n");
                json.push_str("    \"skipped_candidates_total\": 0\n");
                json.push_str("  },\n");
                json.push_str("  \"observation_summary\": {\n");
                json.push_str("    \"status\": \"determinate\",\n");
                json.push_str("    \"representative_entities\": 0,\n");
                json.push_str("    \"ambiguous_entities\": 0\n");
                json.push_str("  },\n");
                json.push_str("  \"candidate_resolutions\": [],\n");
                json.push_str("  \"activities\": [],\n");
                json.push_str("  \"snapshots\": []\n");
            }
        }
        json.push('}');
        json
    }
}

#[derive(Debug)]
pub enum SimulationError {
    MissingSphere,
    MissingPlane,
    MissingPosition(String),
    MissingVelocity(String),
    InvalidRadius(String),
    InvalidPlaneNormal,
    InvalidConstraint(String),
    MissingRegionBounds(String),
    VelocityLimitExceeded { sphere: String, speed: f64, limit: f64 },
    EnteredForbiddenRegion { sphere: String, region: String, time: f64 },
    SphereNotFound(String),
    InvalidActionCandidate(String),
}

impl fmt::Display for SimulationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSphere => write!(f, "program requires at least one sphere"),
            Self::MissingPlane => write!(f, "program requires exactly one plane"),
            Self::MissingPosition(name) => write!(f, "missing position for entity `{name}`"),
            Self::MissingVelocity(name) => write!(f, "missing velocity for entity `{name}`"),
            Self::InvalidRadius(name) => write!(f, "missing or invalid radius for sphere `{name}`"),
            Self::InvalidPlaneNormal => write!(f, "plane normal must be non-zero"),
            Self::InvalidConstraint(message) => write!(f, "{message}"),
            Self::MissingRegionBounds(name) => {
                write!(f, "region `{name}` requires both min and max vectors")
            }
            Self::VelocityLimitExceeded {
                sphere,
                speed,
                limit,
            } => write!(
                f,
                "sphere `{sphere}` exceeded velocity limit: speed {speed:.3} > limit {limit:.3}"
            ),
            Self::EnteredForbiddenRegion { sphere, region, time } => write!(
                f,
                "sphere `{sphere}` entered forbidden region `{region}` at t={time:.3}"
            ),
            Self::SphereNotFound(name) => write!(f, "unknown sphere `{name}`"),
            Self::InvalidActionCandidate(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for SimulationError {}

pub fn simulate_program(program: &Program) -> Result<SimulationReport, SimulationError> {
    let mut world = World::from_program(program)?;
    let mut snapshots = Vec::new();
    let mut observation_times = program.observe_times.clone();
    observation_times.sort_by(|a, b| a.total_cmp(b));

    for time in observation_times {
        world.advance_to(time)?;
        let mut spheres = world
            .spheres
            .iter()
            .map(|sphere| SphereSnapshot {
                name: sphere.name.clone(),
                position: sphere.position,
                velocity: sphere.velocity,
            })
            .collect::<Vec<_>>();
        spheres.sort_by(|a, b| a.name.cmp(&b.name));
        snapshots.push(Snapshot { time, spheres });
    }

    let constraints = world.constraint_summaries();
    Ok(SimulationReport {
        analytics: LawAnalytics::from_constraints(&constraints),
        constraints,
        convergence_analytics: ConvergenceAnalytics::from_candidate_resolutions(
            &world.candidate_resolutions,
        ),
        observation_summary: ObservationSummary::from_candidate_resolutions(
            &world.candidate_resolutions,
        ),
        candidate_resolutions: world.candidate_resolutions.clone(),
        activities: world.activity_log.clone(),
        snapshots,
    })
}

pub fn analyze_program(program: &Program) -> Result<LawInventory, SimulationError> {
    let world = World::from_program(program)?;
    let constraints = world.constraint_summaries();
    Ok(LawInventory {
        analytics: LawAnalytics::from_constraints(&constraints),
        constraints,
        candidate_inventory: candidate_inventory_from_program(program),
        action_directive_inventory: action_directive_inventory_from_program(program),
    })
}

pub fn simulate_program_envelope(program: &Program, source: &str) -> SimulationEnvelope {
    let mut world = match World::from_program(program) {
        Ok(world) => world,
        Err(error) => return SimulationEnvelope::failure(source, error.to_string()),
    };
    let mut snapshots = Vec::new();
    let mut observation_times = program.observe_times.clone();
    observation_times.sort_by(|a, b| a.total_cmp(b));

    for time in observation_times {
        if let Err(error) = world.advance_to(time) {
            let constraints = world.constraint_summaries();
            return SimulationEnvelope::failure_with_report(
                source,
                error.to_string(),
                SimulationReport {
                    analytics: LawAnalytics::from_constraints(&constraints),
                    constraints,
                    convergence_analytics: ConvergenceAnalytics::from_candidate_resolutions(
                        &world.candidate_resolutions,
                    ),
                    observation_summary: ObservationSummary::from_candidate_resolutions(
                        &world.candidate_resolutions,
                    ),
                    candidate_resolutions: world.candidate_resolutions.clone(),
                    activities: world.activity_log.clone(),
                    snapshots,
                },
            );
        }
        let mut spheres = world
            .spheres
            .iter()
            .map(|sphere| SphereSnapshot {
                name: sphere.name.clone(),
                position: sphere.position,
                velocity: sphere.velocity,
            })
            .collect::<Vec<_>>();
        spheres.sort_by(|a, b| a.name.cmp(&b.name));
        snapshots.push(Snapshot { time, spheres });
    }

    let constraints = world.constraint_summaries();
    SimulationEnvelope::success(
        source,
        SimulationReport {
            analytics: LawAnalytics::from_constraints(&constraints),
            constraints,
            convergence_analytics: ConvergenceAnalytics::from_candidate_resolutions(
                &world.candidate_resolutions,
            ),
            observation_summary: ObservationSummary::from_candidate_resolutions(
                &world.candidate_resolutions,
            ),
            candidate_resolutions: world.candidate_resolutions.clone(),
            activities: world.activity_log.clone(),
            snapshots,
        },
    )
}

impl World {
    pub fn from_program(program: &Program) -> Result<Self, SimulationError> {
        let sphere_decls = program
            .entities
            .iter()
            .filter(|entity| entity.kind == "sphere")
            .collect::<Vec<_>>();
        if sphere_decls.is_empty() {
            return Err(SimulationError::MissingSphere);
        }

        let plane_decl = program
            .entities
            .iter()
            .find(|entity| entity.kind == "plane")
            .ok_or(SimulationError::MissingPlane)?;
        let region_decl = program.entities.iter().find(|entity| entity.kind == "region");

        let mut spheres = Vec::new();
        for sphere_decl in sphere_decls {
            let sphere_name = sphere_decl.name.clone();
            spheres.push(Sphere {
                name: sphere_name.clone(),
                position: program
                    .vec3_property("position", &sphere_name)
                    .ok_or_else(|| SimulationError::MissingPosition(sphere_name.clone()))?,
                velocity: program
                    .vec3_property("velocity", &sphere_name)
                    .ok_or_else(|| SimulationError::MissingVelocity(sphere_name.clone()))?,
                radius: program
                    .number_property("radius", &sphere_name)
                    .ok_or_else(|| SimulationError::InvalidRadius(sphere_name.clone()))?,
                last_update_time: 0.0,
            });
        }

        let plane_name = plane_decl.name.clone();
        let plane = Plane {
            name: plane_name.clone(),
            normal: program
                .vec3_property("normal", &plane_name)
                .unwrap_or(Vec3::new(0.0, 1.0, 0.0))
                .normalized()?,
            offset: program.number_property("offset", &plane_name).unwrap_or(0.0),
        };

        let region = match region_decl {
            Some(region_decl) => {
                let region_name = region_decl.name.clone();
                Some(Region {
                    name: region_name.clone(),
                    min: program
                        .vec3_property("min", &region_name)
                        .ok_or_else(|| SimulationError::MissingRegionBounds(region_name.clone()))?,
                    max: program
                        .vec3_property("max", &region_name)
                        .ok_or_else(|| SimulationError::MissingRegionBounds(region_name.clone()))?,
                })
            }
            None => None,
        };

        let build_context = ConstraintBuildContext {
            spheres: &spheres,
            plane_name: &plane.name,
            region_name: region.as_ref().map(|decl| decl.name.as_str()),
        };
        let mut constraints = Vec::new();
        for constraint in &program.constraints {
            constraints.push(Constraint::from_parts(constraint, &build_context)?);
        }

        let mut world = Self {
            spheres,
            plane,
            region,
            constraints,
                constraint_traces: vec![ConstraintTrace::default(); program.constraints.len()],
                candidate_resolutions: Vec::new(),
                activity_log: Vec::new(),
            };

        world.resolve_initial_action_candidates(
            &program.action_candidates,
            &program.action_directives,
        )?;
        world.validated()
    }

    pub fn advance_to(&mut self, target_time: f64) -> Result<(), SimulationError> {
        loop {
            let current_time = self
                .spheres
                .iter()
                .map(|sphere| sphere.last_update_time)
                .fold(0.0, f64::max);
            if target_time <= current_time + EPSILON {
                break;
            }

            let max_dt = target_time - current_time;
            let next_event = self.next_event(max_dt);
            match next_event {
                Some(event) => {
                    self.advance_all_by(event.dt);
                    self.handle_event(event)?;
                }
                None => {
                    self.advance_all_by(max_dt);
                    self.enforce_all_constraints()?;
                    break;
                }
            }
        }

        Ok(())
    }

    fn validated(mut self) -> Result<Self, SimulationError> {
        self.enforce_all_constraints()?;
        Ok(self)
    }

    fn enforce_all_constraints(&mut self) -> Result<(), SimulationError> {
        let constraints = self.constraints.clone();
        for (index, constraint) in constraints.iter().enumerate() {
            match constraint.enforce(self) {
                Ok(repaired) => {
                    if repaired {
                        self.constraint_traces[index].repaired_count += 1;
                        self.activity_log.push(constraint.activity_entry(self, "repaired"));
                    }
                }
                Err(error) => {
                    self.constraint_traces[index].contradicted_count += 1;
                    self.activity_log
                        .push(constraint.activity_entry(self, "contradicted"));
                    return Err(error);
                }
            }
        }
        Ok(())
    }

    fn next_event(&self, max_dt: f64) -> Option<Event> {
        let mut best: Option<Event> = None;
        for (constraint_index, constraint) in self.constraints.iter().enumerate() {
            if let Some(candidate) = constraint.candidate_event(self, constraint_index) {
                best = choose_earlier(best, candidate);
            }
        }

        match best {
            Some(event) if event.dt <= max_dt + EPSILON => Some(event),
            _ => None,
        }
    }

    fn advance_all_by(&mut self, dt: f64) {
        if dt <= EPSILON {
            return;
        }

        for sphere in &mut self.spheres {
            sphere.position = sphere.position + sphere.velocity * dt;
            sphere.last_update_time += dt;
        }
    }

    fn handle_event(&mut self, event: Event) -> Result<(), SimulationError> {
        self.constraint_traces[event.constraint_index].fired_count += 1;
        self.activity_log.push(
            self.constraints[event.constraint_index].activity_entry(self, "fired"),
        );
        event.kind.apply(self)?;
        self.enforce_all_constraints()?;
        Ok(())
    }

    fn apply_elastic_sphere_collision(
        &mut self,
        left_index: usize,
        right_index: usize,
    ) -> Result<(), SimulationError> {
        let left = self.spheres[left_index].clone();
        let right = self.spheres[right_index].clone();
        let collision_normal = (left.position - right.position).normalized()?;
        let relative = left.velocity - right.velocity;
        let impulse = relative.dot(collision_normal);
        if impulse >= 0.0 {
            return Ok(());
        }

        self.spheres[left_index].velocity = left.velocity - collision_normal * impulse;
        self.spheres[right_index].velocity = right.velocity + collision_normal * impulse;
        Ok(())
    }

    fn constraint_summaries(&self) -> Vec<ConstraintSummary> {
        self.constraints
            .iter()
            .enumerate()
            .map(|(index, constraint)| constraint.summary(self, &self.constraint_traces[index]))
            .collect()
    }

    fn current_time(&self) -> f64 {
        self.spheres
            .iter()
            .map(|sphere| sphere.last_update_time)
            .fold(0.0, f64::max)
    }

    fn resolve_initial_action_candidates(
        &mut self,
        action_candidates: &[ActionCandidateDecl],
        action_directives: &[ActionDirectiveDecl],
    ) -> Result<(), SimulationError> {
        if action_candidates.is_empty() {
            return Ok(());
        }

        let deferred_entities = action_directives
            .iter()
            .filter(|directive| directive.kind == "defer_on_ambiguous_top")
            .map(|directive| directive.entity.clone())
            .collect::<std::collections::BTreeSet<_>>();

        let mut grouped = BTreeMap::<String, Vec<ActionCandidateDecl>>::new();
        for candidate in action_candidates {
            grouped
                .entry(candidate.entity.clone())
                .or_default()
                .push(candidate.clone());
        }

        for (sphere_name, mut candidates) in grouped {
            let sphere_index = ensure_sphere_exists(&self.spheres, &sphere_name)?;
            candidates.sort_by(|left, right| {
                right
                    .score
                    .total_cmp(&left.score)
                    .then_with(|| left.label.cmp(&right.label))
            });
            let total_candidates = candidates.len();
            let top_score = candidates
                .first()
                .map(|candidate| candidate.score)
                .unwrap_or(0.0);
            let top_labels = candidates
                .iter()
                .filter(|candidate| (candidate.score - top_score).abs() <= EPSILON)
                .map(|candidate| candidate.label.clone())
                .collect::<Vec<_>>();
            let top_candidate_specs = candidates
                .iter()
                .filter(|candidate| (candidate.score - top_score).abs() <= EPSILON)
                .cloned()
                .collect::<Vec<_>>();

            let mut selected = false;
            let mut rejected_candidates = 0usize;
            let mut selected_candidate = None;
            let mut selected_score = None;
            let mut selected_score_value = None;
            let mut repaired_after_selection = false;
            let mut selected_signature = None;
            let mut selected_spheres = None;
            let mut selected_activity_log = Vec::new();
            for candidate in candidates {
                let mut probe = self.clone();
                probe.activity_log.clear();
                probe.spheres[sphere_index].velocity = candidate.velocity;

                match probe.enforce_all_constraints() {
                    Ok(_) => {
                        let probe_signature = world_signature(&probe);
                        let probe_repaired = probe
                            .activity_log
                            .iter()
                            .any(|activity| activity.action == "repaired");
                        selected_spheres = Some(probe.spheres);
                        selected_activity_log = probe.activity_log;
                        selected_candidate = Some(candidate.label.clone());
                        selected_score = Some(format!("score={:.3}", candidate.score));
                        selected_score_value = Some(candidate.score);
                        selected_signature = Some(probe_signature);
                        repaired_after_selection = probe_repaired;
                        selected = true;
                        break;
                    }
                    Err(_) => {
                        rejected_candidates += 1;
                        self.activity_log.push(candidate_activity_entry(
                            self.current_time(),
                            &sphere_name,
                            &candidate.label,
                            candidate.score,
                            "rejected_by_hard_law",
                        ));
                    }
                }
            }

            if !selected {
                return Err(SimulationError::InvalidActionCandidate(format!(
                    "all candidate actions for sphere `{}` were rejected by hard laws",
                    sphere_name
                )));
            }

            let mut equivalent_top_labels = Vec::new();
            if let Some(selected_signature) = &selected_signature {
                for candidate in &top_candidate_specs {
                    let mut probe = self.clone();
                    probe.activity_log.clear();
                    probe.spheres[sphere_index].velocity = candidate.velocity;
                    if probe.enforce_all_constraints().is_ok()
                        && &world_signature(&probe) == selected_signature
                    {
                        equivalent_top_labels.push(candidate.label.clone());
                    }
                }
                equivalent_top_labels.sort();
            }

            let tie_broken = top_labels.len() > 1;
            let observationally_equivalent_tie = equivalent_top_labels.len() > 1;
            let deferred = tie_broken
                && !observationally_equivalent_tie
                && deferred_entities.contains(&sphere_name);
            if deferred {
                self.activity_log.push(candidate_activity_entry(
                    self.current_time(),
                    &sphere_name,
                    top_labels.first().map(String::as_str).unwrap_or(""),
                    top_score,
                    "deferred_due_to_tie",
                ));
                selected_candidate = None;
                selected_score = None;
                repaired_after_selection = false;
            } else {
                if let Some(spheres) = selected_spheres {
                    self.spheres = spheres;
                }
                if let Some(label) = &selected_candidate {
                    self.activity_log.push(candidate_activity_entry(
                        self.current_time(),
                        &sphere_name,
                        label,
                        selected_score_value.unwrap_or(top_score),
                        "selected",
                    ));
                }
                self.activity_log.extend(selected_activity_log);
            }

            let convergence_mode = if deferred {
                "deferred"
            } else if observationally_equivalent_tie {
                "equivalent_tie"
            } else if tie_broken {
                "tie_broken"
            } else if repaired_after_selection {
                "repaired"
            } else if rejected_candidates > 0 {
                "fallback"
            } else {
                "direct"
            };
            let observation_mode = if deferred {
                "ambiguous"
            } else if observationally_equivalent_tie {
                "representative"
            } else if tie_broken {
                "ambiguous"
            } else {
                "determinate"
            };
            let observation_labels = if observationally_equivalent_tie {
                equivalent_top_labels.clone()
            } else if tie_broken {
                top_labels.clone()
            } else {
                selected_candidate
                    .as_ref()
                    .map(|label| vec![label.clone()])
                    .unwrap_or_default()
            };

            self.candidate_resolutions.push(CandidateResolution {
                entity: sphere_name,
                total_candidates,
                rejected_candidates,
                skipped_candidates: if deferred {
                    total_candidates.saturating_sub(rejected_candidates)
                } else {
                    total_candidates - rejected_candidates - 1
                },
                convergence_mode: convergence_mode.to_string(),
                observation_mode: observation_mode.to_string(),
                observation_labels,
                symbolically_underdetermined: tie_broken,
                observationally_underdetermined: tie_broken && !observationally_equivalent_tie,
                selected_candidate,
                selected_score,
                top_score: format!("{top_score:.3}"),
                tie_broken,
                top_labels,
                observationally_equivalent_tie,
                equivalent_top_labels,
                repaired_after_selection,
            });
        }

        Ok(())
    }

}

#[derive(Clone, Copy, Debug)]
struct Event {
    constraint_index: usize,
    dt: f64,
    kind: EventKind,
}

impl Event {
    fn plane(constraint_index: usize, sphere_index: usize, dt: f64) -> Self {
        Self {
            constraint_index,
            dt,
            kind: EventKind::PlaneCollision { sphere_index },
        }
    }

    fn region(constraint_index: usize, sphere_index: usize, dt: f64) -> Self {
        Self {
            constraint_index,
            dt,
            kind: EventKind::ForbiddenRegionEntry { sphere_index },
        }
    }

    fn sphere_pair(
        constraint_index: usize,
        left_index: usize,
        right_index: usize,
        dt: f64,
    ) -> Self {
        Self {
            constraint_index,
            dt,
            kind: EventKind::SphereCollision {
                left_index,
                right_index,
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum EventKind {
    PlaneCollision { sphere_index: usize },
    ForbiddenRegionEntry { sphere_index: usize },
    SphereCollision { left_index: usize, right_index: usize },
}

impl EventKind {
    fn apply(self, world: &mut World) -> Result<(), SimulationError> {
        match self {
            Self::PlaneCollision { sphere_index } => {
                let normal = world.plane.normal;
                let sphere = &mut world.spheres[sphere_index];
                let reflected = sphere.velocity - normal * (2.0 * sphere.velocity.dot(normal));
                sphere.velocity = reflected;
                Ok(())
            }
            Self::ForbiddenRegionEntry { sphere_index } => {
                let _ = sphere_index;
                Ok(())
            }
            Self::SphereCollision {
                left_index,
                right_index,
            } => world.apply_elastic_sphere_collision(left_index, right_index),
        }
    }
}

struct ConstraintBuildContext<'a> {
    spheres: &'a [Sphere],
    plane_name: &'a str,
    region_name: Option<&'a str>,
}

#[derive(Clone, Copy, Debug)]
enum ConstraintCategory {
    Invariant,
    Boundary,
    Interaction,
}

impl Constraint {
    fn activity_entry(&self, world: &World, action: &str) -> ActivityEntry {
        let summary = self.summary(world, &ConstraintTrace::default());
        ActivityEntry {
            time: world.current_time(),
            kind: summary.kind,
            targets: summary.targets,
            policy: summary.policy,
            action: action.to_string(),
        }
    }

    fn summary(&self, world: &World, trace: &ConstraintTrace) -> ConstraintSummary {
        match self {
            Self::ReflectOnCollision { sphere_index } => ConstraintSummary {
                kind: "reflect_on_collision".to_string(),
                category: self.category().as_str().to_string(),
                targets: vec![
                    world.spheres[*sphere_index].name.clone(),
                    world.plane.name.clone(),
                ],
                policy: "implicit".to_string(),
                supported_policies: self
                    .supported_policies()
                    .into_iter()
                    .map(|policy| policy.as_str().to_string())
                    .collect(),
                outcome: trace.outcome().to_string(),
                fired_count: trace.fired_count,
                repaired_count: trace.repaired_count,
                contradicted_count: trace.contradicted_count,
            },
            Self::VelocityLimit {
                sphere_index,
                policy,
                ..
            } => ConstraintSummary {
                kind: "velocity_limit".to_string(),
                category: self.category().as_str().to_string(),
                targets: vec![world.spheres[*sphere_index].name.clone()],
                policy: policy.as_str().to_string(),
                supported_policies: self
                    .supported_policies()
                    .into_iter()
                    .map(|policy| policy.as_str().to_string())
                    .collect(),
                outcome: trace.outcome().to_string(),
                fired_count: trace.fired_count,
                repaired_count: trace.repaired_count,
                contradicted_count: trace.contradicted_count,
            },
            Self::NotInside {
                sphere_index,
                policy,
            } => ConstraintSummary {
                kind: "not_inside".to_string(),
                category: self.category().as_str().to_string(),
                targets: vec![
                    world.spheres[*sphere_index].name.clone(),
                    world
                        .region
                        .as_ref()
                        .map(|region| region.name.clone())
                        .unwrap_or_else(|| "region".to_string()),
                ],
                policy: policy.as_str().to_string(),
                supported_policies: self
                    .supported_policies()
                    .into_iter()
                    .map(|policy| policy.as_str().to_string())
                    .collect(),
                outcome: trace.outcome().to_string(),
                fired_count: trace.fired_count,
                repaired_count: trace.repaired_count,
                contradicted_count: trace.contradicted_count,
            },
            Self::ElasticCollision {
                left_index,
                right_index,
            } => ConstraintSummary {
                kind: "elastic_collision".to_string(),
                category: self.category().as_str().to_string(),
                targets: vec![
                    world.spheres[*left_index].name.clone(),
                    world.spheres[*right_index].name.clone(),
                ],
                policy: "implicit".to_string(),
                supported_policies: self
                    .supported_policies()
                    .into_iter()
                    .map(|policy| policy.as_str().to_string())
                    .collect(),
                outcome: trace.outcome().to_string(),
                fired_count: trace.fired_count,
                repaired_count: trace.repaired_count,
                contradicted_count: trace.contradicted_count,
            },
        }
    }

    fn category(&self) -> ConstraintCategory {
        match self {
            Self::VelocityLimit { .. } => ConstraintCategory::Invariant,
            Self::ReflectOnCollision { .. } | Self::NotInside { .. } => ConstraintCategory::Boundary,
            Self::ElasticCollision { .. } => ConstraintCategory::Interaction,
        }
    }

    fn supported_policies(&self) -> Vec<RepairPolicy> {
        match self {
            Self::VelocityLimit { .. } => vec![RepairPolicy::Reject, RepairPolicy::Clamp],
            Self::NotInside { .. } => {
                vec![RepairPolicy::Reject, RepairPolicy::Clamp, RepairPolicy::Reflect]
            }
            Self::ReflectOnCollision { .. } | Self::ElasticCollision { .. } => Vec::new(),
        }
    }

    fn from_parts(
        parts: &[String],
        context: &ConstraintBuildContext<'_>,
    ) -> Result<Self, SimulationError> {
        match parts {
            [name, sphere_ref, plane_ref] if name == "reflect_on_collision" => {
                if plane_ref != context.plane_name {
                    return Err(SimulationError::InvalidConstraint(format!(
                        "unknown plane in reflect_on_collision: {plane_ref}"
                    )));
                }
                Ok(Self::ReflectOnCollision {
                    sphere_index: ensure_sphere_exists(context.spheres, sphere_ref)?,
                })
            }
            [name, sphere_ref, limit] if name == "velocity_limit" => {
                let max_speed = limit.parse::<f64>().map_err(|_| {
                    SimulationError::InvalidConstraint(format!(
                        "invalid velocity limit value: {limit}"
                    ))
                })?;
                Ok(Self::VelocityLimit {
                    sphere_index: ensure_sphere_exists(context.spheres, sphere_ref)?,
                    max_speed,
                    policy: RepairPolicy::Reject,
                })
            }
            [name, sphere_ref, limit, policy] if name == "velocity_limit" => {
                let max_speed = limit.parse::<f64>().map_err(|_| {
                    SimulationError::InvalidConstraint(format!(
                        "invalid velocity limit value: {limit}"
                    ))
                })?;
                let policy = parse_repair_policy(policy)?;
                ensure_policy_supported(
                    "velocity_limit",
                    policy,
                    &[RepairPolicy::Reject, RepairPolicy::Clamp],
                )?;
                Ok(Self::VelocityLimit {
                    sphere_index: ensure_sphere_exists(context.spheres, sphere_ref)?,
                    max_speed,
                    policy,
                })
            }
            [name, sphere_ref, region_ref] if name == "not_inside" => {
                let Some(region_name) = context.region_name else {
                    return Err(SimulationError::InvalidConstraint(
                        "not_inside requires a declared region".to_string(),
                    ));
                };
                if region_ref != region_name {
                    return Err(SimulationError::InvalidConstraint(format!(
                        "unknown region in not_inside: {region_ref}"
                    )));
                }
                Ok(Self::NotInside {
                    sphere_index: ensure_sphere_exists(context.spheres, sphere_ref)?,
                    policy: RepairPolicy::Reject,
                })
            }
            [name, sphere_ref, region_ref, policy] if name == "not_inside" => {
                let Some(region_name) = context.region_name else {
                    return Err(SimulationError::InvalidConstraint(
                        "not_inside requires a declared region".to_string(),
                    ));
                };
                if region_ref != region_name {
                    return Err(SimulationError::InvalidConstraint(format!(
                        "unknown region in not_inside: {region_ref}"
                    )));
                }
                let policy = parse_repair_policy(policy)?;
                ensure_policy_supported(
                    "not_inside",
                    policy,
                    &[
                        RepairPolicy::Reject,
                        RepairPolicy::Clamp,
                        RepairPolicy::Reflect,
                    ],
                )?;
                Ok(Self::NotInside {
                    sphere_index: ensure_sphere_exists(context.spheres, sphere_ref)?,
                    policy,
                })
            }
            [name, left, right] if name == "elastic_collision" => Ok(Self::ElasticCollision {
                left_index: ensure_sphere_exists(context.spheres, left)?,
                right_index: ensure_sphere_exists(context.spheres, right)?,
            }),
            _ => Err(SimulationError::InvalidConstraint(format!(
                "unsupported constraint: {:?}",
                parts
            ))),
        }
    }

    fn enforce(&self, world: &mut World) -> Result<bool, SimulationError> {
        match self {
            Self::ReflectOnCollision { .. } | Self::ElasticCollision { .. } => Ok(false),
            Self::VelocityLimit {
                sphere_index,
                max_speed,
                policy,
            } => {
                let sphere = &mut world.spheres[*sphere_index];
                let speed = sphere.velocity.magnitude();
                if speed > *max_speed + EPSILON {
                    match policy {
                        RepairPolicy::Reject => {
                            return Err(SimulationError::VelocityLimitExceeded {
                                sphere: sphere.name.clone(),
                                speed,
                                limit: *max_speed,
                            });
                        }
                        RepairPolicy::Clamp => {
                            let direction = sphere.velocity * (1.0 / speed);
                            sphere.velocity = direction * *max_speed;
                            return Ok(true);
                        }
                        RepairPolicy::Reflect => {
                            return Err(SimulationError::InvalidConstraint(
                                "velocity_limit does not support reflect policy".to_string(),
                            ));
                        }
                    }
                }
                Ok(false)
            }
            Self::NotInside {
                sphere_index,
                policy,
            } => {
                let Some(region) = world.region.as_ref() else {
                    return Ok(false);
                };
                let region_min = region.min;
                let region_max = region.max;
                let region_name = region.name.clone();
                let sphere = &mut world.spheres[*sphere_index];
                if point_inside_box(sphere.position, region_min, region_max) {
                    match policy {
                        RepairPolicy::Reject => {
                            return Err(SimulationError::EnteredForbiddenRegion {
                                sphere: sphere.name.clone(),
                                region: region_name,
                                time: sphere.last_update_time,
                            });
                        }
                        RepairPolicy::Clamp => {
                            clamp_sphere_outside_box(sphere, region_min, region_max);
                            return Ok(true);
                        }
                        RepairPolicy::Reflect => {
                            reflect_sphere_outside_box(sphere, region_min, region_max);
                            return Ok(true);
                        }
                    }
                }
                Ok(false)
            }
        }
    }

    fn candidate_event(&self, world: &World, constraint_index: usize) -> Option<Event> {
        match self {
            Self::ReflectOnCollision { sphere_index } => {
                let sphere = &world.spheres[*sphere_index];
                time_to_plane_collision(sphere, &world.plane)
                    .map(|dt| Event::plane(constraint_index, *sphere_index, dt))
            }
            Self::VelocityLimit { .. } => None,
            Self::NotInside { sphere_index, .. } => {
                let region = world.region.as_ref()?;
                let sphere = &world.spheres[*sphere_index];
                time_to_box_entry(sphere.position, sphere.velocity, region.min, region.max)
                    .map(|dt| Event::region(constraint_index, *sphere_index, dt))
            }
            Self::ElasticCollision {
                left_index,
                right_index,
            } => time_to_sphere_collision(&world.spheres[*left_index], &world.spheres[*right_index])
                .map(|dt| Event::sphere_pair(constraint_index, *left_index, *right_index, dt)),
        }
    }
}

impl ConstraintCategory {
    fn as_str(self) -> &'static str {
        match self {
            Self::Invariant => "invariant",
            Self::Boundary => "boundary",
            Self::Interaction => "interaction",
        }
    }
}

impl ConstraintTrace {
    fn outcome(&self) -> &'static str {
        if self.contradicted_count > 0 {
            "contradicted"
        } else if self.repaired_count > 0 {
            "repaired"
        } else if self.fired_count > 0 {
            "fired"
        } else {
            "idle"
        }
    }
}

impl LawAnalytics {
    fn from_constraints(constraints: &[ConstraintSummary]) -> Self {
        let mut analytics = Self {
            total_constraints: constraints.len(),
            invariant_constraints: 0,
            boundary_constraints: 0,
            interaction_constraints: 0,
            idle_constraints: 0,
            fired_constraints: 0,
            repaired_constraints: 0,
            contradicted_constraints: 0,
        };

        for constraint in constraints {
            match constraint.category.as_str() {
                "invariant" => analytics.invariant_constraints += 1,
                "boundary" => analytics.boundary_constraints += 1,
                "interaction" => analytics.interaction_constraints += 1,
                _ => {}
            }

            match constraint.outcome.as_str() {
                "idle" => analytics.idle_constraints += 1,
                "fired" => analytics.fired_constraints += 1,
                "repaired" => analytics.repaired_constraints += 1,
                "contradicted" => analytics.contradicted_constraints += 1,
                _ => {}
            }
        }

        analytics
    }
}

impl ConvergenceAnalytics {
    pub fn from_candidate_resolutions(candidate_resolutions: &[CandidateResolution]) -> Self {
        let mut analytics = Self {
            candidate_entities: candidate_resolutions.len(),
            direct_entities: 0,
            fallback_entities: 0,
            repaired_entities: 0,
            deferred_entities: 0,
            tie_broken_entities: 0,
            equivalent_tie_entities: 0,
            determinate_entities: 0,
            representative_entities: 0,
            ambiguous_entities: 0,
            symbolically_underdetermined_entities: 0,
            observationally_underdetermined_entities: 0,
            rejected_candidates_total: 0,
            skipped_candidates_total: 0,
        };

        for resolution in candidate_resolutions {
            analytics.rejected_candidates_total += resolution.rejected_candidates;
            analytics.skipped_candidates_total += resolution.skipped_candidates;
            match resolution.convergence_mode.as_str() {
                "direct" => analytics.direct_entities += 1,
                "fallback" => analytics.fallback_entities += 1,
                "repaired" => analytics.repaired_entities += 1,
                "deferred" => analytics.deferred_entities += 1,
                "tie_broken" => analytics.tie_broken_entities += 1,
                "equivalent_tie" => analytics.equivalent_tie_entities += 1,
                _ => {}
            }
            if resolution.symbolically_underdetermined {
                analytics.symbolically_underdetermined_entities += 1;
            }
            if resolution.observationally_underdetermined {
                analytics.observationally_underdetermined_entities += 1;
            }
            match resolution.observation_mode.as_str() {
                "determinate" => analytics.determinate_entities += 1,
                "representative" => analytics.representative_entities += 1,
                "ambiguous" => analytics.ambiguous_entities += 1,
                _ => {}
            }
        }

        analytics
    }
}

impl ObservationSummary {
    pub fn from_candidate_resolutions(candidate_resolutions: &[CandidateResolution]) -> Self {
        let representative_entities = candidate_resolutions
            .iter()
            .filter(|resolution| resolution.observation_mode == "representative")
            .count();
        let ambiguous_entities = candidate_resolutions
            .iter()
            .filter(|resolution| resolution.observation_mode == "ambiguous")
            .count();

        let status = if ambiguous_entities > 0 {
            "unresolved"
        } else if representative_entities > 0 {
            "representative"
        } else {
            "determinate"
        };

        Self {
            status: status.to_string(),
            representative_entities,
            ambiguous_entities,
        }
    }
}

impl RepairPolicy {
    fn as_str(self) -> &'static str {
        match self {
            Self::Reject => "reject",
            Self::Clamp => "clamp",
            Self::Reflect => "reflect",
        }
    }
}

fn ensure_sphere_exists(spheres: &[Sphere], sphere_name: &str) -> Result<usize, SimulationError> {
    spheres
        .iter()
        .position(|sphere| sphere.name == sphere_name)
        .ok_or_else(|| SimulationError::SphereNotFound(sphere_name.to_string()))
}

fn parse_repair_policy(policy: &str) -> Result<RepairPolicy, SimulationError> {
    match policy {
        "reject" => Ok(RepairPolicy::Reject),
        "clamp" => Ok(RepairPolicy::Clamp),
        "reflect" => Ok(RepairPolicy::Reflect),
        _ => Err(SimulationError::InvalidConstraint(format!(
            "unknown repair policy: {policy}"
        ))),
    }
}

fn ensure_policy_supported(
    constraint_name: &str,
    selected_policy: RepairPolicy,
    supported_policies: &[RepairPolicy],
) -> Result<(), SimulationError> {
    if supported_policies.contains(&selected_policy) {
        return Ok(());
    }

    let supported = supported_policies
        .iter()
        .map(|policy| policy.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    Err(SimulationError::InvalidConstraint(format!(
        "{} does not support {}; supported policies: {}",
        constraint_name,
        selected_policy.as_str(),
        supported
    )))
}

fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

fn choose_earlier(current: Option<Event>, candidate: Event) -> Option<Event> {
    if candidate.dt < 0.0 {
        return current;
    }

    match current {
        Some(existing) if existing.dt <= candidate.dt => Some(existing),
        _ => Some(candidate),
    }
}

fn candidate_activity_entry(
    time: f64,
    sphere_name: &str,
    label: &str,
    score: f64,
    action: &str,
) -> ActivityEntry {
    ActivityEntry {
        time,
        kind: "candidate_velocity".to_string(),
        targets: vec![sphere_name.to_string(), label.to_string()],
        policy: format!("score={score:.3}"),
        action: action.to_string(),
    }
}

fn world_signature(world: &World) -> Vec<String> {
    let mut entries = world
        .spheres
        .iter()
        .map(|sphere| {
            format!(
                "{}|{:.6}|{:.6}|{:.6}|{:.6}|{:.6}|{:.6}",
                sphere.name,
                sphere.position.x,
                sphere.position.y,
                sphere.position.z,
                sphere.velocity.x,
                sphere.velocity.y,
                sphere.velocity.z
            )
        })
        .collect::<Vec<_>>();
    entries.sort();
    entries
}

fn candidate_inventory_from_program(program: &Program) -> Vec<CandidateInventorySummary> {
    let mut grouped = BTreeMap::<String, Vec<ActionCandidateDecl>>::new();
    for candidate in &program.action_candidates {
        grouped
            .entry(candidate.entity.clone())
            .or_default()
            .push(candidate.clone());
    }

    grouped
        .into_iter()
        .map(|(entity, mut candidates)| {
            candidates.sort_by(|left, right| left.label.cmp(&right.label));
            let top_score = candidates
                .iter()
                .map(|candidate| candidate.score)
                .max_by(|left, right| left.total_cmp(right))
                .unwrap_or(0.0);
            let mut labels = candidates
                .iter()
                .map(|candidate| candidate.label.clone())
                .collect::<Vec<_>>();
            labels.sort();
            let mut top_labels = candidates
                .iter()
                .filter(|candidate| (candidate.score - top_score).abs() <= EPSILON)
                .map(|candidate| candidate.label.clone())
                .collect::<Vec<_>>();
            top_labels.sort();

            CandidateInventorySummary {
                entity,
                total_candidates: candidates.len(),
                labels,
                top_score: format!("{top_score:.3}"),
                top_labels,
            }
        })
        .collect()
}

fn action_directive_inventory_from_program(program: &Program) -> Vec<ActionDirectiveSummary> {
    let mut directives = program
        .action_directives
        .iter()
        .map(|directive| ActionDirectiveSummary {
            entity: directive.entity.clone(),
            kind: directive.kind.clone(),
        })
        .collect::<Vec<_>>();
    directives.sort_by(|left, right| {
        left.entity
            .cmp(&right.entity)
            .then_with(|| left.kind.cmp(&right.kind))
    });
    directives
}

fn time_to_plane_collision(sphere: &Sphere, plane: &Plane) -> Option<f64> {
    let signed_distance = plane.normal.dot(sphere.position) - plane.offset - sphere.radius;
    let approach_speed = plane.normal.dot(sphere.velocity);

    if signed_distance <= EPSILON || approach_speed >= -EPSILON {
        return None;
    }

    let hit_dt = -signed_distance / approach_speed;
    if hit_dt >= 0.0 { Some(hit_dt) } else { None }
}

fn time_to_sphere_collision(left: &Sphere, right: &Sphere) -> Option<f64> {
    let delta_position = left.position - right.position;
    let delta_velocity = left.velocity - right.velocity;
    let combined_radius = left.radius + right.radius;

    let a = delta_velocity.dot(delta_velocity);
    if a <= EPSILON {
        return None;
    }

    let b = 2.0 * delta_position.dot(delta_velocity);
    let c = delta_position.dot(delta_position) - combined_radius * combined_radius;

    if c <= EPSILON {
        return None;
    }

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }

    let sqrt_disc = discriminant.sqrt();
    let t1 = (-b - sqrt_disc) / (2.0 * a);
    let t2 = (-b + sqrt_disc) / (2.0 * a);
    [t1, t2]
        .into_iter()
        .filter(|dt| *dt >= 0.0)
        .min_by(|a, b| a.total_cmp(b))
}

fn point_inside_box(point: Vec3, min: Vec3, max: Vec3) -> bool {
    point.x >= min.x
        && point.x <= max.x
        && point.y >= min.y
        && point.y <= max.y
        && point.z >= min.z
        && point.z <= max.z
}

fn time_to_box_entry(position: Vec3, velocity: Vec3, min: Vec3, max: Vec3) -> Option<f64> {
    if point_inside_box(position, min, max) {
        return Some(0.0);
    }

    let mut t_min: f64 = 0.0;
    let mut t_max = f64::INFINITY;

    for (pos, vel, axis_min, axis_max) in [
        (position.x, velocity.x, min.x, max.x),
        (position.y, velocity.y, min.y, max.y),
        (position.z, velocity.z, min.z, max.z),
    ] {
        if vel.abs() <= EPSILON {
            if pos < axis_min || pos > axis_max {
                return None;
            }
            continue;
        }

        let t1 = (axis_min - pos) / vel;
        let t2 = (axis_max - pos) / vel;
        let axis_entry = t1.min(t2);
        let axis_exit = t1.max(t2);
        t_min = t_min.max(axis_entry);
        t_max = t_max.min(axis_exit);
        if t_min > t_max {
            return None;
        }
    }

    if t_max < 0.0 {
        None
    } else if t_min <= 0.0 {
        Some(0.0)
    } else {
        Some(t_min)
    }
}

fn clamp_sphere_outside_box(sphere: &mut Sphere, min: Vec3, max: Vec3) {
    let distances = [
        (sphere.position.x - min.x, 0usize, min.x - EPSILON),
        (max.x - sphere.position.x, 0usize, max.x + EPSILON),
        (sphere.position.y - min.y, 1usize, min.y - EPSILON),
        (max.y - sphere.position.y, 1usize, max.y + EPSILON),
        (sphere.position.z - min.z, 2usize, min.z - EPSILON),
        (max.z - sphere.position.z, 2usize, max.z + EPSILON),
    ];

    let (_, axis, target) = distances
        .into_iter()
        .min_by(|left, right| left.0.total_cmp(&right.0))
        .expect("box face distances are non-empty");

    match axis {
        0 => {
            sphere.position.x = target;
            sphere.velocity.x = 0.0;
        }
        1 => {
            sphere.position.y = target;
            sphere.velocity.y = 0.0;
        }
        2 => {
            sphere.position.z = target;
            sphere.velocity.z = 0.0;
        }
        _ => unreachable!("axis index is bounded above"),
    }
}

fn reflect_sphere_outside_box(sphere: &mut Sphere, min: Vec3, max: Vec3) {
    let distances = [
        (sphere.position.x - min.x, 0usize, min.x - EPSILON, -1.0),
        (max.x - sphere.position.x, 0usize, max.x + EPSILON, 1.0),
        (sphere.position.y - min.y, 1usize, min.y - EPSILON, -1.0),
        (max.y - sphere.position.y, 1usize, max.y + EPSILON, 1.0),
        (sphere.position.z - min.z, 2usize, min.z - EPSILON, -1.0),
        (max.z - sphere.position.z, 2usize, max.z + EPSILON, 1.0),
    ];

    let (_, axis, target, direction) = distances
        .into_iter()
        .min_by(|left, right| left.0.total_cmp(&right.0))
        .expect("box face distances are non-empty");

    match axis {
        0 => {
            sphere.position.x = target;
            sphere.velocity.x = sphere.velocity.x.abs() * direction;
        }
        1 => {
            sphere.position.y = target;
            sphere.velocity.y = sphere.velocity.y.abs() * direction;
        }
        2 => {
            sphere.position.z = target;
            sphere.velocity.z = sphere.velocity.z.abs() * direction;
        }
        _ => unreachable!("axis index is bounded above"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        SimulationEnvelope, analyze_program, parse_program, simulate_program,
        simulate_program_envelope,
    };

    #[test]
    fn bounce_reflects_on_floor() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 10, 0)
velocity(A) = (1, -3, 0)
radius(A) = 1
constraint:
    reflect_on_collision(A, floor)
observe:
    snapshot at 3
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        assert_eq!(report.snapshots[0].spheres[0].position.y, 1.0);
        assert_eq!(report.snapshots[0].spheres[0].velocity.y, 3.0);
    }

    #[test]
    fn elastic_collision_swaps_velocities() {
        let source = r#"
sphere A
sphere B
plane floor
position(A) = (0, 2, 0)
velocity(A) = (1, 0, 0)
radius(A) = 1
position(B) = (4, 2, 0)
velocity(B) = (-1, 0, 0)
radius(B) = 1
constraint:
    elastic_collision(A, B)
observe:
    snapshot at 3
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let snapshot = &report.snapshots[0];
        let a = snapshot
            .spheres
            .iter()
            .find(|sphere| sphere.name == "A")
            .expect("sphere A exists");
        let b = snapshot
            .spheres
            .iter()
            .find(|sphere| sphere.name == "B")
            .expect("sphere B exists");
        assert_eq!(a.velocity.x, -1.0);
        assert_eq!(b.velocity.x, 1.0);
    }

    #[test]
    fn velocity_limit_rejects_fast_sphere() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 10, 0)
velocity(A) = (3, 4, 0)
radius(A) = 1
constraint:
    velocity_limit(A, 4)
"#;
        let program = parse_program(source).expect("program should parse");
        let error = simulate_program(&program).expect_err("simulation should fail");
        assert!(error.to_string().contains("velocity limit"));
    }

    #[test]
    fn velocity_limit_can_clamp_speed() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 10, 0)
velocity(A) = (6, 8, 0)
radius(A) = 1
constraint:
    clamp speed(A) <= 5
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let sphere = &report.snapshots[0].spheres[0];
        assert!((sphere.velocity.magnitude() - 5.0).abs() < 1e-9);
    }

    #[test]
    fn velocity_limit_rejects_unsupported_reflect_policy_at_build_time() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 10, 0)
velocity(A) = (6, 8, 0)
radius(A) = 1
constraint:
    reflect speed(A) <= 5
"#;
        let program = parse_program(source).expect("program should parse");
        let error = simulate_program(&program).expect_err("simulation should fail");
        assert!(error
            .to_string()
            .contains("velocity_limit does not support reflect"));
    }

    #[test]
    fn forbidden_region_stops_world() {
        let source = r#"
sphere A
plane floor
region zone
position(A) = (0, 0, 0)
velocity(A) = (1, 0, 0)
radius(A) = 1
min(zone) = (2, -1, -1)
max(zone) = (4, 1, 1)
constraint:
    not inside(A, zone)
observe:
    snapshot at 3
"#;
        let program = parse_program(source).expect("program should parse");
        let error = simulate_program(&program).expect_err("simulation should fail");
        assert!(error.to_string().contains("forbidden region"));
    }

    #[test]
    fn forbidden_region_can_clamp_to_boundary() {
        let source = r#"
sphere A
plane floor
region zone
position(A) = (0, 0, 0)
velocity(A) = (1, 0, 0)
radius(A) = 1
min(zone) = (2, -1, -1)
max(zone) = (4, 1, 1)
constraint:
    clamp not inside(A, zone)
observe:
    snapshot at 3
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let sphere = &report.snapshots[0].spheres[0];
        assert!(sphere.position.x < 2.0 || sphere.position.x > 4.0);
        assert_eq!(sphere.velocity.x, 0.0);
    }

    #[test]
    fn forbidden_region_can_reflect_from_boundary() {
        let source = r#"
sphere A
plane floor
region zone
position(A) = (0, 0, 0)
velocity(A) = (1, 0, 0)
radius(A) = 1
min(zone) = (2, -1, -1)
max(zone) = (4, 1, 1)
constraint:
    reflect not inside(A, zone)
observe:
    snapshot at 3
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let sphere = &report.snapshots[0].spheres[0];
        assert!(sphere.position.x < 2.0 || sphere.position.x > 4.0);
        assert_eq!(sphere.velocity.x, -1.0);
    }

    #[test]
    fn report_serializes_to_json() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 1, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
constraint:
    speed(A) <= 1
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let json = report.to_json("example.sk");
        assert!(json.contains("\"source\": \"example.sk\""));
        assert!(json.contains("\"analytics\""));
        assert!(json.contains("\"total_constraints\": 1"));
        assert!(json.contains("\"constraints\""));
        assert!(json.contains("\"velocity_limit\""));
        assert!(json.contains("\"category\": \"invariant\""));
        assert!(json.contains("\"supported_policies\": [\"reject\", \"clamp\"]"));
        assert!(json.contains("\"outcome\": \"idle\""));
        assert!(json.contains("\"fired_count\""));
        assert!(json.contains("\"repaired_count\""));
        assert!(json.contains("\"convergence_analytics\""));
        assert!(json.contains("\"determinate_entities\""));
        assert!(json.contains("\"observation_summary\""));
        assert!(json.contains("\"candidate_resolutions\": ["));
        assert!(json.contains("\"activities\""));
        assert!(json.contains("\"snapshots\""));
        assert!(json.contains("\"name\": \"A\""));
    }

    #[test]
    fn analyze_program_summarizes_declared_laws_without_simulating() {
        let source = r#"
sphere A
plane floor
region zone
position(A) = (0, 0, 0)
velocity(A) = (1, 0, 0)
radius(A) = 1
min(zone) = (2, -1, -1)
max(zone) = (4, 1, 1)
constraint:
    reflect not inside(A, zone)
"#;
        let program = parse_program(source).expect("program should parse");
        let inventory = analyze_program(&program).expect("analysis should succeed");
        let json = inventory.to_json("analyze.sk");
        assert!(json.contains("\"source\": \"analyze.sk\""));
        assert!(json.contains("\"analytics\""));
        assert!(json.contains("\"total_constraints\": 1"));
        assert!(json.contains("\"policy\": \"reflect\""));
        assert!(json.contains("\"outcome\": \"idle\""));
        assert!(json.contains("\"candidate_inventory\": ["));
        assert!(!json.contains("\"entity\":"));
        assert!(!json.contains("\"snapshots\""));
    }

    #[test]
    fn analyze_program_reports_candidate_inventory() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
action:
    candidate_velocity(A, fast) = (6, 0, 0) score 5
    candidate_velocity(A, safe) = (3, 0, 0) score 2
constraint:
    speed(A) <= 4
"#;
        let program = parse_program(source).expect("program should parse");
        let inventory = analyze_program(&program).expect("analysis should succeed");
        let json = inventory.to_json("candidate.sk");
        assert!(json.contains("\"candidate_inventory\""));
        assert!(json.contains("\"entity\": \"A\""));
        assert!(json.contains("\"total_candidates\": 2"));
        assert!(json.contains("\"labels\": [\"fast\", \"safe\"]"));
        assert!(json.contains("\"top_score\": \"5.000\""));
        assert!(json.contains("\"top_labels\": [\"fast\"]"));
    }

    #[test]
    fn analyze_program_reports_action_directives() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
action:
    candidate_velocity(A, alpha) = (3, 0, 0) score 5
    candidate_velocity(A, beta) = (2, 0, 0) score 5
    defer_on_ambiguous_top(A)
constraint:
    speed(A) <= 4
"#;
        let program = parse_program(source).expect("program should parse");
        let inventory = analyze_program(&program).expect("analysis should succeed");
        let json = inventory.to_json("deferred.sk");
        assert!(json.contains("\"action_directive_inventory\""));
        assert!(json.contains("\"entity\": \"A\""));
        assert!(json.contains("\"kind\": \"defer_on_ambiguous_top\""));
    }

    #[test]
    fn envelope_serializes_error_json() {
        let envelope = SimulationEnvelope::failure("bad.sk", "world contradiction");
        let json = envelope.to_json();
        assert!(json.contains("\"status\": \"error\""));
        assert!(json.contains("\"error\": \"world contradiction\""));
        assert!(json.contains("\"analytics\": {"));
        assert!(json.contains("\"constraints\": []"));
        assert!(json.contains("\"convergence_analytics\": {"));
        assert!(json.contains("\"observation_summary\": {"));
        assert!(json.contains("\"activities\": []"));
        assert!(json.contains("\"snapshots\": []"));
    }

    #[test]
    fn failure_envelope_can_keep_partial_report() {
        let source = r#"
sphere A
plane floor
region zone
position(A) = (0, 0, 0)
velocity(A) = (1, 0, 0)
radius(A) = 1
min(zone) = (2, -1, -1)
max(zone) = (4, 1, 1)
constraint:
    not inside(A, zone)
observe:
    snapshot at 1
    snapshot at 3
"#;
        let program = parse_program(source).expect("program should parse");
        let envelope = simulate_program_envelope(&program, "forbidden_region.sk");
        let json = envelope.to_json();
        assert!(json.contains("\"status\": \"error\""));
        assert!(json.contains("\"constraints\""));
        assert!(json.contains("\"not_inside\""));
        assert!(json.contains("\"analytics\""));
        assert!(json.contains("\"boundary_constraints\": 1"));
        assert!(json.contains("\"outcome\": \"contradicted\""));
        assert!(json.contains("\"contradicted_count\": 1"));
        assert!(json.contains("\"action\": \"contradicted\""));
        assert!(json.contains("\"activities\""));
        assert!(json.contains("\"time\": 1.000000"));
    }

    #[test]
    fn candidate_velocity_selects_best_admissible_option() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
action:
    candidate_velocity(A, fast) = (6, 0, 0) score 5
    candidate_velocity(A, safe) = (3, 0, 0) score 2
constraint:
    speed(A) <= 4
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let sphere = &report.snapshots[0].spheres[0];
        assert_eq!(sphere.velocity.x, 3.0);
        assert!(report
            .activities
            .iter()
            .any(|entry| entry.kind == "candidate_velocity" && entry.action == "selected"));
        assert!(report.activities.iter().any(|entry| {
            entry.kind == "candidate_velocity"
                && entry.action == "rejected_by_hard_law"
                && entry.targets.iter().any(|target| target == "fast")
        }));
        let candidate_resolution = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("candidate resolution should be recorded");
        assert_eq!(candidate_resolution.entity, "A");
        assert_eq!(candidate_resolution.total_candidates, 2);
        assert_eq!(candidate_resolution.rejected_candidates, 1);
        assert_eq!(candidate_resolution.skipped_candidates, 0);
        assert_eq!(candidate_resolution.convergence_mode, "fallback");
        assert_eq!(candidate_resolution.observation_mode, "determinate");
        assert_eq!(candidate_resolution.observation_labels, vec!["safe".to_string()]);
        assert!(!candidate_resolution.symbolically_underdetermined);
        assert!(!candidate_resolution.observationally_underdetermined);
        assert_eq!(
            candidate_resolution.selected_candidate.as_deref(),
            Some("safe")
        );
        assert_eq!(candidate_resolution.top_score, "5.000");
        assert_eq!(candidate_resolution.top_labels, vec!["fast".to_string()]);
        assert!(!candidate_resolution.tie_broken);
        assert!(candidate_resolution.equivalent_top_labels.is_empty());
        assert!(!candidate_resolution.observationally_equivalent_tie);
    }

    #[test]
    fn candidate_velocity_can_select_repaired_option() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
action:
    candidate_velocity(A, fast) = (6, 0, 0) score 5
    candidate_velocity(A, safe) = (3, 0, 0) score 2
constraint:
    clamp speed(A) <= 4
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let sphere = &report.snapshots[0].spheres[0];
        assert_eq!(sphere.velocity.x, 4.0);
        assert!(report.activities.iter().any(|entry| {
            entry.kind == "candidate_velocity"
                && entry.action == "selected"
                && entry.targets.iter().any(|target| target == "fast")
        }));
        assert!(report
            .activities
            .iter()
            .any(|entry| entry.kind == "velocity_limit" && entry.action == "repaired"));
        let candidate_resolution = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("candidate resolution should be recorded");
        assert_eq!(
            candidate_resolution.selected_candidate.as_deref(),
            Some("fast")
        );
        assert_eq!(candidate_resolution.convergence_mode, "repaired");
        assert_eq!(candidate_resolution.observation_mode, "determinate");
        assert_eq!(candidate_resolution.observation_labels, vec!["fast".to_string()]);
        assert_eq!(candidate_resolution.skipped_candidates, 1);
        assert_eq!(candidate_resolution.top_score, "5.000");
        assert_eq!(candidate_resolution.top_labels, vec!["fast".to_string()]);
        assert!(!candidate_resolution.tie_broken);
        assert_eq!(
            candidate_resolution.equivalent_top_labels,
            vec!["fast".to_string()]
        );
        assert!(!candidate_resolution.observationally_equivalent_tie);
        assert!(candidate_resolution.repaired_after_selection);
    }

    #[test]
    fn candidate_velocity_can_resolve_multiple_entities() {
        let source = r#"
sphere A
sphere B
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
position(B) = (5, 2, 0)
velocity(B) = (0, 0, 0)
radius(B) = 1
action:
    candidate_velocity(A, fast) = (6, 0, 0) score 5
    candidate_velocity(A, safe) = (3, 0, 0) score 2
    candidate_velocity(B, sprint) = (5, 0, 0) score 4
    candidate_velocity(B, coast) = (2, 0, 0) score 1
constraint:
    speed(A) <= 4
    speed(B) <= 3
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let a = report
            .snapshots[0]
            .spheres
            .iter()
            .find(|sphere| sphere.name == "A")
            .expect("sphere A exists");
        let b = report
            .snapshots[0]
            .spheres
            .iter()
            .find(|sphere| sphere.name == "B")
            .expect("sphere B exists");
        assert_eq!(a.velocity.x, 3.0);
        assert_eq!(b.velocity.x, 2.0);
        assert_eq!(report.candidate_resolutions.len(), 2);
        assert!(report
            .candidate_resolutions
            .iter()
            .any(|resolution| resolution.entity == "A"));
        assert!(report
            .candidate_resolutions
            .iter()
            .any(|resolution| resolution.entity == "B"));
    }

    #[test]
    fn candidate_velocity_reports_top_score_ties() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
action:
    candidate_velocity(A, alpha) = (3, 0, 0) score 5
    candidate_velocity(A, beta) = (2, 0, 0) score 5
constraint:
    speed(A) <= 4
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let candidate_resolution = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("candidate resolution should be recorded");
        assert_eq!(candidate_resolution.selected_candidate.as_deref(), Some("alpha"));
        assert_eq!(candidate_resolution.top_score, "5.000");
        assert_eq!(
            candidate_resolution.top_labels,
            vec!["alpha".to_string(), "beta".to_string()]
        );
        assert_eq!(candidate_resolution.skipped_candidates, 1);
        assert_eq!(candidate_resolution.convergence_mode, "tie_broken");
        assert_eq!(candidate_resolution.observation_mode, "ambiguous");
        assert_eq!(
            candidate_resolution.observation_labels,
            vec!["alpha".to_string(), "beta".to_string()]
        );
        assert!(candidate_resolution.symbolically_underdetermined);
        assert!(candidate_resolution.observationally_underdetermined);
        assert!(candidate_resolution.tie_broken);
        assert_eq!(
            candidate_resolution.equivalent_top_labels,
            vec!["alpha".to_string()]
        );
        assert!(!candidate_resolution.observationally_equivalent_tie);
    }

    #[test]
    fn candidate_velocity_can_report_observationally_equivalent_ties() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
action:
    candidate_velocity(A, alpha) = (3, 0, 0) score 5
    candidate_velocity(A, beta) = (3, 0, 0) score 5
constraint:
    speed(A) <= 4
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let candidate_resolution = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("candidate resolution should be recorded");
        assert!(candidate_resolution.tie_broken);
        assert_eq!(candidate_resolution.convergence_mode, "equivalent_tie");
        assert_eq!(candidate_resolution.observation_mode, "representative");
        assert_eq!(
            candidate_resolution.observation_labels,
            vec!["alpha".to_string(), "beta".to_string()]
        );
        assert!(candidate_resolution.symbolically_underdetermined);
        assert!(!candidate_resolution.observationally_underdetermined);
        assert!(candidate_resolution.observationally_equivalent_tie);
        assert_eq!(
            candidate_resolution.equivalent_top_labels,
            vec!["alpha".to_string(), "beta".to_string()]
        );
    }

    #[test]
    fn candidate_velocity_reports_convergence_analytics() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
action:
    candidate_velocity(A, alpha) = (3, 0, 0) score 5
    candidate_velocity(A, beta) = (3, 0, 0) score 5
constraint:
    speed(A) <= 4
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        assert_eq!(report.convergence_analytics.candidate_entities, 1);
        assert_eq!(report.convergence_analytics.equivalent_tie_entities, 1);
        assert_eq!(report.convergence_analytics.direct_entities, 0);
        assert_eq!(report.convergence_analytics.tie_broken_entities, 0);
        assert_eq!(report.convergence_analytics.determinate_entities, 0);
        assert_eq!(report.convergence_analytics.representative_entities, 1);
        assert_eq!(report.convergence_analytics.ambiguous_entities, 0);
        assert_eq!(report.observation_summary.status, "representative");
        assert_eq!(report.observation_summary.representative_entities, 1);
        assert_eq!(report.observation_summary.ambiguous_entities, 0);
        assert_eq!(
            report.convergence_analytics.symbolically_underdetermined_entities,
            1
        );
        assert_eq!(
            report
                .convergence_analytics
                .observationally_underdetermined_entities,
            0
        );
    }

    #[test]
    fn candidate_velocity_can_surface_unresolved_observation() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
action:
    candidate_velocity(A, alpha) = (3, 0, 0) score 5
    candidate_velocity(A, beta) = (2, 0, 0) score 5
constraint:
    speed(A) <= 4
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        assert_eq!(report.observation_summary.status, "unresolved");
        assert_eq!(report.observation_summary.representative_entities, 0);
        assert_eq!(report.observation_summary.ambiguous_entities, 1);
    }

    #[test]
    fn candidate_velocity_can_defer_ambiguous_top_choice() {
        let source = r#"
sphere A
plane floor
position(A) = (0, 2, 0)
velocity(A) = (0, 0, 0)
radius(A) = 1
action:
    candidate_velocity(A, alpha) = (3, 0, 0) score 5
    candidate_velocity(A, beta) = (2, 0, 0) score 5
    defer_on_ambiguous_top(A)
constraint:
    speed(A) <= 4
observe:
    snapshot at 0
"#;
        let program = parse_program(source).expect("program should parse");
        let report = simulate_program(&program).expect("simulation should succeed");
        let sphere = &report.snapshots[0].spheres[0];
        assert_eq!(sphere.velocity.x, 0.0);
        let candidate_resolution = report
            .candidate_resolutions
            .iter()
            .find(|resolution| resolution.entity == "A")
            .expect("candidate resolution should be recorded");
        assert_eq!(candidate_resolution.convergence_mode, "deferred");
        assert_eq!(candidate_resolution.observation_mode, "ambiguous");
        assert_eq!(candidate_resolution.selected_candidate, None);
        assert_eq!(candidate_resolution.skipped_candidates, 2);
        assert_eq!(report.convergence_analytics.deferred_entities, 1);
        assert_eq!(report.observation_summary.status, "unresolved");
        assert!(report.activities.iter().any(|entry| {
            entry.kind == "candidate_velocity" && entry.action == "deferred_due_to_tie"
        }));
    }
}
