use std::{collections::HashMap, fmt};

use crate::world::Vec3;

#[derive(Clone, Debug)]
pub struct EntityDecl {
    pub kind: String,
    pub name: String,
}

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    Vec3(Vec3),
}

#[derive(Clone, Debug)]
pub struct ActionCandidateDecl {
    pub entity: String,
    pub label: String,
    pub velocity: Vec3,
    pub score: f64,
}

#[derive(Clone, Debug)]
pub struct ActionDirectiveDecl {
    pub entity: String,
    pub kind: String,
    pub argument: Option<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct Program {
    pub entities: Vec<EntityDecl>,
    pub properties: HashMap<(String, String), Value>,
    pub action_candidates: Vec<ActionCandidateDecl>,
    pub action_directives: Vec<ActionDirectiveDecl>,
    pub constraints: Vec<Vec<String>>,
    pub observe_times: Vec<f64>,
}

impl Program {
    pub fn vec3_property(&self, property: &str, entity: &str) -> Option<Vec3> {
        match self
            .properties
            .get(&(property.to_string(), entity.to_string()))
        {
            Some(Value::Vec3(value)) => Some(*value),
            _ => None,
        }
    }

    pub fn number_property(&self, property: &str, entity: &str) -> Option<f64> {
        match self
            .properties
            .get(&(property.to_string(), entity.to_string()))
        {
            Some(Value::Number(value)) => Some(*value),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    line: usize,
    message: String,
}

impl ParseError {
    fn new(line: usize, message: impl Into<String>) -> Self {
        Self {
            line,
            message: message.into(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}: {}", self.line, self.message)
    }
}

impl std::error::Error for ParseError {}

pub fn parse_program(source: &str) -> Result<Program, ParseError> {
    let mut program = Program::default();
    let mut mode = Mode::TopLevel;

    for (index, raw_line) in source.lines().enumerate() {
        let line_no = index + 1;
        let trimmed = raw_line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        match mode {
            Mode::TopLevel => {
                if trimmed == "constraint:" {
                    mode = Mode::Constraint;
                    continue;
                }
                if trimmed == "observe:" {
                    mode = Mode::Observe;
                    continue;
                }
                if trimmed == "action:" {
                    mode = Mode::Action;
                    continue;
                }
                if let Some(entity) = parse_entity_decl(trimmed) {
                    program.entities.push(entity);
                    continue;
                }
                if let Some((property, entity, value)) = parse_property(trimmed, line_no)? {
                    program.properties.insert((property, entity), value);
                    continue;
                }
                return Err(ParseError::new(
                    line_no,
                    format!("could not parse top-level statement `{trimmed}`"),
                ));
            }
            Mode::Constraint => {
                if !raw_line.starts_with(' ') && !raw_line.starts_with('\t') {
                    mode = Mode::TopLevel;
                    if trimmed == "observe:" {
                        mode = Mode::Observe;
                        continue;
                    }
                    if trimmed == "action:" {
                        mode = Mode::Action;
                        continue;
                    }
                    if trimmed == "constraint:" {
                        continue;
                    }
                    if let Some(entity) = parse_entity_decl(trimmed) {
                        program.entities.push(entity);
                        continue;
                    }
                    if let Some((property, entity, value)) = parse_property(trimmed, line_no)? {
                        program.properties.insert((property, entity), value);
                        continue;
                    }
                    return Err(ParseError::new(
                        line_no,
                        format!("could not parse statement `{trimmed}` after constraint block"),
                    ));
                }
                program.constraints.push(parse_constraint(trimmed, line_no)?);
            }
            Mode::Observe => {
                if !raw_line.starts_with(' ') && !raw_line.starts_with('\t') {
                    mode = Mode::TopLevel;
                    if let Some(entity) = parse_entity_decl(trimmed) {
                        program.entities.push(entity);
                        continue;
                    }
                    if let Some((property, entity, value)) = parse_property(trimmed, line_no)? {
                        program.properties.insert((property, entity), value);
                        continue;
                    }
                    if trimmed == "constraint:" {
                        mode = Mode::Constraint;
                        continue;
                    }
                    return Err(ParseError::new(
                        line_no,
                        format!("could not parse statement `{trimmed}` after observe block"),
                    ));
                }
                program.observe_times.push(parse_observe(trimmed, line_no)?);
            }
            Mode::Action => {
                if !raw_line.starts_with(' ') && !raw_line.starts_with('\t') {
                    mode = Mode::TopLevel;
                    if let Some(entity) = parse_entity_decl(trimmed) {
                        program.entities.push(entity);
                        continue;
                    }
                    if let Some((property, entity, value)) = parse_property(trimmed, line_no)? {
                        program.properties.insert((property, entity), value);
                        continue;
                    }
                    if trimmed == "constraint:" {
                        mode = Mode::Constraint;
                        continue;
                    }
                    if trimmed == "observe:" {
                        mode = Mode::Observe;
                        continue;
                    }
                    return Err(ParseError::new(
                        line_no,
                        format!("could not parse statement `{trimmed}` after action block"),
                    ));
                }
                if trimmed.starts_with("candidate_velocity(") {
                    program
                        .action_candidates
                        .push(parse_action_candidate(trimmed, line_no)?);
                    continue;
                }
                program
                    .action_directives
                    .push(parse_action_directive(trimmed, line_no)?);
            }
        }
    }

    if program.observe_times.is_empty() {
        program.observe_times = vec![0.0, 1.0, 2.0, 3.0];
    }

    Ok(program)
}

#[derive(Clone, Copy)]
enum Mode {
    TopLevel,
    Constraint,
    Observe,
    Action,
}

fn parse_entity_decl(line: &str) -> Option<EntityDecl> {
    let mut parts = line.split_whitespace();
    let kind = parts.next()?;
    let name = parts.next()?;
    if parts.next().is_some() {
        return None;
    }
    match kind {
        "sphere" | "plane" | "region" => Some(EntityDecl {
            kind: kind.to_string(),
            name: name.to_string(),
        }),
        _ => None,
    }
}

fn parse_property(
    line: &str,
    line_no: usize,
) -> Result<Option<(String, String, Value)>, ParseError> {
    let Some(eq_index) = line.find('=') else {
        return Ok(None);
    };
    let lhs = line[..eq_index].trim();
    let rhs = line[eq_index + 1..].trim();

    let open = lhs
        .find('(')
        .ok_or_else(|| ParseError::new(line_no, "property assignment is missing `(`"))?;
    let close = lhs
        .find(')')
        .ok_or_else(|| ParseError::new(line_no, "property assignment is missing `)`"))?;
    let property = lhs[..open].trim();
    let entity = lhs[open + 1..close].trim();
    if property.is_empty() || entity.is_empty() {
        return Err(ParseError::new(
            line_no,
            "property assignment must include property and entity names",
        ));
    }

    let value = if rhs.starts_with('(') {
        Value::Vec3(parse_vec3(rhs, line_no)?)
    } else {
        Value::Number(
            rhs.parse::<f64>()
                .map_err(|_| ParseError::new(line_no, format!("invalid number `{rhs}`")))?,
        )
    };

    Ok(Some((property.to_string(), entity.to_string(), value)))
}

fn parse_vec3(input: &str, line_no: usize) -> Result<Vec3, ParseError> {
    let inner = input
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))
        .ok_or_else(|| ParseError::new(line_no, format!("invalid vector `{input}`")))?;

    let parts = inner
        .split(',')
        .map(str::trim)
        .map(|part| {
            part.parse::<f64>()
                .map_err(|_| ParseError::new(line_no, format!("invalid vector component `{part}`")))
        })
        .collect::<Result<Vec<_>, _>>()?;

    if parts.len() != 3 {
        return Err(ParseError::new(
            line_no,
            format!("vector must have 3 components, found {}", parts.len()),
        ));
    }

    Ok(Vec3::new(parts[0], parts[1], parts[2]))
}

fn parse_constraint(line: &str, line_no: usize) -> Result<Vec<String>, ParseError> {
    if let Some(parsed) = parse_constraint_alias(line, line_no)? {
        return Ok(parsed);
    }

    let open = line
        .find('(')
        .ok_or_else(|| ParseError::new(line_no, "constraint is missing `(`"))?;
    let close = line
        .rfind(')')
        .ok_or_else(|| ParseError::new(line_no, "constraint is missing `)`"))?;
    let name = line[..open].trim();
    let args = line[open + 1..close]
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    if name.is_empty() {
        return Err(ParseError::new(line_no, "constraint name is empty"));
    }

    let mut parsed = vec![name.to_string()];
    parsed.extend(args);
    Ok(parsed)
}

fn parse_constraint_alias(line: &str, line_no: usize) -> Result<Option<Vec<String>>, ParseError> {
    let (policy, body) = parse_policy_prefix(line);

    if let Some(rest) = body.strip_prefix("not inside(") {
        let inner = rest
            .strip_suffix(')')
            .ok_or_else(|| ParseError::new(line_no, "constraint is missing `)`"))?;
        let args = inner
            .split(',')
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        if args.len() != 2 {
            return Err(ParseError::new(
                line_no,
                "not inside requires exactly 2 arguments",
            ));
        }
        let mut parsed = vec![
            "not_inside".to_string(),
            args[0].clone(),
            args[1].clone(),
        ];
        if let Some(policy) = policy {
            parsed.push(policy.to_string());
        }
        return Ok(Some(parsed));
    }

    if let Some(rest) = body.strip_prefix("speed(") {
        let close = rest
            .find(')')
            .ok_or_else(|| ParseError::new(line_no, "speed constraint is missing `)`"))?;
        let entity = rest[..close].trim();
        let tail = rest[close + 1..].trim();
        let limit = tail
            .strip_prefix("<=")
            .ok_or_else(|| ParseError::new(line_no, "speed constraint must use `<=`"))?
            .trim();
        if entity.is_empty() || limit.is_empty() {
            return Err(ParseError::new(
                line_no,
                "speed constraint requires an entity and a limit",
            ));
        }
        let mut parsed = vec![
            "velocity_limit".to_string(),
            entity.to_string(),
            limit.to_string(),
        ];
        if let Some(policy) = policy {
            parsed.push(policy.to_string());
        }
        return Ok(Some(parsed));
    }

    if let Some(rest) = body.strip_prefix("elastic collision(") {
        let inner = rest
            .strip_suffix(')')
            .ok_or_else(|| ParseError::new(line_no, "constraint is missing `)`"))?;
        let args = inner
            .split(',')
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        if args.len() != 2 {
            return Err(ParseError::new(
                line_no,
                "elastic collision requires exactly 2 arguments",
            ));
        }
        let mut parsed = vec![
            "elastic_collision".to_string(),
            args[0].clone(),
            args[1].clone(),
        ];
        if let Some(policy) = policy {
            parsed.push(policy.to_string());
        }
        return Ok(Some(parsed));
    }

    Ok(None)
}

fn parse_policy_prefix(line: &str) -> (Option<&str>, &str) {
    for policy in ["clamp", "reject", "reflect"] {
        if let Some(rest) = line.strip_prefix(policy)
            && let Some(body) = rest.strip_prefix(' ')
        {
            return (Some(policy), body.trim_start());
        }
    }
    (None, line)
}

fn parse_observe(line: &str, line_no: usize) -> Result<f64, ParseError> {
    let parts = line.split_whitespace().collect::<Vec<_>>();
    if parts.len() != 3 || parts[0] != "snapshot" || parts[1] != "at" {
        return Err(ParseError::new(
            line_no,
            format!("invalid observe statement `{line}`"),
        ));
    }
    parts[2]
        .parse::<f64>()
        .map_err(|_| ParseError::new(line_no, format!("invalid observation time `{}`", parts[2])))
}

fn parse_action_candidate(line: &str, line_no: usize) -> Result<ActionCandidateDecl, ParseError> {
    let Some(rest) = line.strip_prefix("candidate_velocity(") else {
        return Err(ParseError::new(
            line_no,
            format!("invalid action statement `{line}`"),
        ));
    };
    let close = rest
        .find(')')
        .ok_or_else(|| ParseError::new(line_no, "candidate_velocity is missing `)`"))?;
    let inner = &rest[..close];
    let tail = rest[close + 1..].trim();
    let args = inner
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();
    if args.len() != 2 {
        return Err(ParseError::new(
            line_no,
            "candidate_velocity requires an entity and a label",
        ));
    }

    let rhs = tail
        .strip_prefix('=')
        .ok_or_else(|| ParseError::new(line_no, "candidate_velocity must use `=`"))?
        .trim();
    let (velocity_text, score_text) = rhs
        .rsplit_once(" score ")
        .ok_or_else(|| ParseError::new(line_no, "candidate_velocity requires `score <number>`"))?;

    let velocity = parse_vec3(velocity_text.trim(), line_no)?;
    let score = score_text
        .trim()
        .parse::<f64>()
        .map_err(|_| ParseError::new(line_no, format!("invalid score `{}`", score_text.trim())))?;

    Ok(ActionCandidateDecl {
        entity: args[0].to_string(),
        label: args[1].to_string(),
        velocity,
        score,
    })
}

fn parse_action_directive(line: &str, line_no: usize) -> Result<ActionDirectiveDecl, ParseError> {
    if let Some(rest) = line.strip_prefix("defer_on_ambiguous_top(") {
        let close = rest
            .find(')')
            .ok_or_else(|| ParseError::new(line_no, "defer_on_ambiguous_top is missing `)`"))?;
        let entity = rest[..close].trim();
        if entity.is_empty() {
            return Err(ParseError::new(
                line_no,
                "defer_on_ambiguous_top requires an entity",
            ));
        }
        if !rest[close + 1..].trim().is_empty() {
            return Err(ParseError::new(
                line_no,
                "defer_on_ambiguous_top does not take trailing arguments",
            ));
        }
        return Ok(ActionDirectiveDecl {
            entity: entity.to_string(),
            kind: "defer_on_ambiguous_top".to_string(),
            argument: None,
        });
    }

    let Some(rest) = line.strip_prefix("resolve_deferred_at(") else {
        return Err(ParseError::new(
            line_no,
            format!("invalid action statement `{line}`"),
        ));
    };
    let close = rest
        .find(')')
        .ok_or_else(|| ParseError::new(line_no, "resolve_deferred_at is missing `)`"))?;
    let args = rest[..close]
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();
    if args.len() != 2 {
        return Err(ParseError::new(
            line_no,
            "resolve_deferred_at requires an entity and a time",
        ));
    }
    if !rest[close + 1..].trim().is_empty() {
        return Err(ParseError::new(
            line_no,
            "resolve_deferred_at does not take trailing arguments",
        ));
    }
    let time = args[1].parse::<f64>().map_err(|_| {
        ParseError::new(
            line_no,
            format!("invalid resolve_deferred_at time `{}`", args[1]),
        )
    })?;
    Ok(ActionDirectiveDecl {
        entity: args[0].to_string(),
        kind: "resolve_deferred_at".to_string(),
        argument: Some(time),
    })
}
