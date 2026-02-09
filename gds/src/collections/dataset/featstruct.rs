//! Feature structure DSL core types.
//!
//! This models NLTK-style feature structures (dict/list), but does not include
//! unification algorithms.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FeatReentranceId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FeatPathSegment {
    Key(String),
    Index(usize),
}

impl FeatPathSegment {
    pub fn key(value: impl Into<String>) -> Self {
        FeatPathSegment::Key(value.into())
    }

    pub fn index(value: usize) -> Self {
        FeatPathSegment::Index(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FeatPath {
    segments: Vec<FeatPathSegment>,
}

impl FeatPath {
    pub fn new(segments: impl Into<Vec<FeatPathSegment>>) -> Self {
        Self {
            segments: segments.into(),
        }
    }

    pub fn segments(&self) -> &[FeatPathSegment] {
        &self.segments
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FeatValue {
    Text(String),
    Number(i64),
    Bool(bool),
    Null,
    BytesRange {
        start: usize,
        end: usize,
    },
    Empty,
    Variable(String),
    Reentrant(FeatReentranceId),
    ReentranceDef {
        id: FeatReentranceId,
        value: Box<FeatValue>,
    },
    Struct(FeatStruct),
    List(FeatList),
}

impl FeatValue {
    pub fn text(value: impl Into<String>) -> Self {
        FeatValue::Text(value.into())
    }

    pub fn variable(name: impl Into<String>) -> Self {
        FeatValue::Variable(name.into())
    }

    pub fn bytes_range(start: usize, end: usize) -> Self {
        FeatValue::BytesRange { start, end }
    }
}

impl From<String> for FeatValue {
    fn from(value: String) -> Self {
        FeatValue::Text(value)
    }
}

impl From<&str> for FeatValue {
    fn from(value: &str) -> Self {
        FeatValue::Text(value.to_string())
    }
}

impl From<i64> for FeatValue {
    fn from(value: i64) -> Self {
        FeatValue::Number(value)
    }
}

impl From<bool> for FeatValue {
    fn from(value: bool) -> Self {
        FeatValue::Bool(value)
    }
}

pub type FeatDict = BTreeMap<String, FeatValue>;

pub type FeatList = Vec<FeatValue>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FeatStruct {
    Dict(FeatDict),
    List(FeatList),
}

impl FeatStruct {
    pub fn dict(values: FeatDict) -> Self {
        FeatStruct::Dict(values)
    }

    pub fn list(values: FeatList) -> Self {
        FeatStruct::List(values)
    }

    pub fn get_path(&self, path: &FeatPath) -> Option<&FeatValue> {
        get_path_struct(self, path.segments())
    }
}

#[derive(Debug, Clone, Default)]
pub struct FeatStructSet {
    reentrances: BTreeMap<FeatReentranceId, FeatValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatStructParseError {
    message: String,
}

impl FeatStructParseError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FeatBindings {
    bindings: BTreeMap<String, FeatValue>,
}

impl FeatBindings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, name: impl Into<String>, value: FeatValue) {
        self.bindings.insert(name.into(), value);
    }

    pub fn get(&self, name: &str) -> Option<&FeatValue> {
        self.bindings.get(name)
    }

    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }

    pub fn bindings(&self) -> &BTreeMap<String, FeatValue> {
        &self.bindings
    }
}

pub fn parse_featstruct(input: &str) -> Result<FeatStruct, FeatStructParseError> {
    match parse_featvalue(input)? {
        FeatValue::Struct(value) => Ok(value),
        FeatValue::ReentranceDef { value, .. } => match *value {
            FeatValue::Struct(value) => Ok(value),
            _ => Err(FeatStructParseError::new("expected feature structure")),
        },
        _ => Err(FeatStructParseError::new("expected feature structure")),
    }
}

pub fn parse_featvalue(input: &str) -> Result<FeatValue, FeatStructParseError> {
    let mut parser = FeatStructParser::new(input);
    let value = parser.parse_value()?;
    parser.skip_ws();
    if !parser.is_eof() {
        return Err(FeatStructParseError::new("unexpected trailing input"));
    }
    Ok(value)
}

pub fn format_featstruct(value: &FeatStruct) -> String {
    match value {
        FeatStruct::Dict(dict) => format_featdict(dict),
        FeatStruct::List(list) => format_featlist(list),
    }
}

pub fn unify_featstruct(
    left: &FeatStruct,
    right: &FeatStruct,
    bindings: Option<&mut FeatBindings>,
) -> Option<FeatStruct> {
    let left_vars = collect_variables_struct(left);
    let mut used_vars = left_vars.clone();
    if let Some(bindings) = bindings.as_ref() {
        used_vars.extend(bindings.bindings.keys().cloned());
        for value in bindings.bindings.values() {
            used_vars.extend(collect_variables_value(value));
        }
    }

    let mut var_mapping = BTreeMap::new();
    let right_renamed = rename_variables_struct(right, &mut used_vars, &mut var_mapping);

    let mut left_reentrances = HashMap::new();
    let mut right_reentrances = HashMap::new();
    let left_node = resolve_struct(left, &mut left_reentrances)?;
    let right_node = resolve_struct(&right_renamed, &mut right_reentrances)?;

    let mut internal_bindings = BTreeMap::new();
    if let Some(bindings) = bindings.as_ref() {
        internal_bindings = resolve_bindings(&bindings.bindings)?;
    }

    let mut forward = HashMap::new();
    let result_node = unify_nodes(left_node, right_node, &mut internal_bindings, &mut forward)?;
    resolve_aliases(&mut internal_bindings);

    let counts = count_references(&result_node, &internal_bindings, &forward);
    let mut materializer = Materializer::new(&counts, &internal_bindings, &forward);
    let result_value = materializer.materialize_value(&result_node);
    let result_struct = featstruct_from_value(result_value)?;

    if let Some(bindings) = bindings {
        let mut new_bindings = BTreeMap::new();
        for (name, node) in internal_bindings.iter() {
            let value = materializer.materialize_value(node);
            new_bindings.insert(name.clone(), value);
        }
        bindings.bindings = new_bindings;
    }

    Some(result_struct)
}

pub fn subsumes_featstruct(left: &FeatStruct, right: &FeatStruct) -> bool {
    unify_featstruct(left, right, None)
        .map(|value| value == *right)
        .unwrap_or(false)
}

impl FeatStructSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_reentrance(&mut self, id: FeatReentranceId, value: FeatValue) {
        self.reentrances.insert(id, value);
    }

    pub fn get_reentrance(&self, id: FeatReentranceId) -> Option<&FeatValue> {
        self.reentrances.get(&id)
    }

    pub fn reentrances(&self) -> &BTreeMap<FeatReentranceId, FeatValue> {
        &self.reentrances
    }
}

fn get_path_struct<'a>(value: &'a FeatStruct, path: &[FeatPathSegment]) -> Option<&'a FeatValue> {
    if path.is_empty() {
        return None;
    }
    match (path.first().unwrap(), value) {
        (FeatPathSegment::Key(key), FeatStruct::Dict(dict)) => {
            let next = dict.get(key)?;
            get_path_value(next, &path[1..])
        }
        (FeatPathSegment::Index(index), FeatStruct::List(list)) => {
            let next = list.get(*index)?;
            get_path_value(next, &path[1..])
        }
        _ => None,
    }
}

fn get_path_value<'a>(value: &'a FeatValue, rest: &[FeatPathSegment]) -> Option<&'a FeatValue> {
    if rest.is_empty() {
        return Some(value);
    }
    match value {
        FeatValue::Struct(inner) => get_path_struct(inner, rest),
        FeatValue::List(list) => match rest.first().unwrap() {
            FeatPathSegment::Index(index) => {
                let next = list.get(*index)?;
                get_path_value(next, &rest[1..])
            }
            _ => None,
        },
        _ => None,
    }
}

fn format_featdict(values: &FeatDict) -> String {
    let mut prefix = String::new();
    let mut suffix = String::new();
    let mut parts: Vec<String> = Vec::new();

    for (key, value) in values.iter() {
        if key == FEAT_PREFIX_KEY {
            if let Some(text) = format_prefix_value(value) {
                prefix = text;
            }
            continue;
        }
        if key == FEAT_SLASH_KEY {
            if let Some(text) = format_prefix_value(value) {
                suffix = format!("/{}", text);
            }
            continue;
        }
        if let FeatValue::Bool(value) = value {
            parts.push(format!("{}{}", if *value { '+' } else { '-' }, key));
        } else {
            parts.push(format!("{}={}", key, format_featvalue(value)));
        }
    }

    parts.sort();
    format!("{}[{}]{}", prefix, parts.join(", "), suffix)
}

fn format_featlist(values: &FeatList) -> String {
    let parts: Vec<String> = values.iter().map(format_featvalue).collect();
    format!("[{}]", parts.join(", "))
}

fn format_featvalue(value: &FeatValue) -> String {
    match value {
        FeatValue::Text(text) => format_symbol_or_string(text),
        FeatValue::Number(value) => value.to_string(),
        FeatValue::Bool(value) => format_bool(*value),
        FeatValue::Null => "None".to_string(),
        FeatValue::BytesRange { start, end } => format!("{}:{}", start, end),
        FeatValue::Empty => "_".to_string(),
        FeatValue::Variable(name) => format!("?{}", name),
        FeatValue::Reentrant(id) => format!("->({})", id.0),
        FeatValue::ReentranceDef { id, value } => {
            format!("({}){}", id.0, format_featvalue(value))
        }
        FeatValue::Struct(structure) => format_featstruct(structure),
        FeatValue::List(list) => format_featlist(list),
    }
}

fn format_prefix_value(value: &FeatValue) -> Option<String> {
    match value {
        FeatValue::Variable(name) => Some(format!("?{}", name)),
        FeatValue::Text(text) => Some(format_symbol_or_string(text)),
        FeatValue::Number(value) => Some(value.to_string()),
        FeatValue::Bool(value) => Some(format_bool(*value)),
        FeatValue::Null => Some("None".to_string()),
        _ => None,
    }
}

fn format_symbol_or_string(text: &str) -> String {
    if is_symbol_token(text) {
        text.to_string()
    } else {
        format!("\"{}\"", escape_string(text))
    }
}

fn escape_string(text: &str) -> String {
    let mut out = String::new();
    for ch in text.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(ch),
        }
    }
    out
}

fn format_bool(value: bool) -> String {
    if value {
        "True".to_string()
    } else {
        "False".to_string()
    }
}

type NodeRef = Rc<RefCell<ResolvedValue>>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ResolvedValue {
    Text(String),
    Number(i64),
    Bool(bool),
    Null,
    BytesRange { start: usize, end: usize },
    Empty,
    Variable(String),
    Struct(BTreeMap<String, NodeRef>),
    List(Vec<NodeRef>),
}

fn is_symbol_token(text: &str) -> bool {
    let mut chars = text.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
}

fn collect_variables_struct(value: &FeatStruct) -> BTreeSet<String> {
    match value {
        FeatStruct::Dict(dict) => dict.values().flat_map(collect_variables_value).collect(),
        FeatStruct::List(list) => list.iter().flat_map(collect_variables_value).collect(),
    }
}

fn collect_variables_value(value: &FeatValue) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    match value {
        FeatValue::Variable(name) => {
            out.insert(name.clone());
        }
        FeatValue::Struct(value) => {
            out.extend(collect_variables_struct(value));
        }
        FeatValue::List(list) => {
            for item in list {
                out.extend(collect_variables_value(item));
            }
        }
        FeatValue::ReentranceDef { value, .. } => {
            out.extend(collect_variables_value(value));
        }
        _ => {}
    }
    out
}

fn rename_variables_struct(
    value: &FeatStruct,
    used: &mut BTreeSet<String>,
    mapping: &mut BTreeMap<String, String>,
) -> FeatStruct {
    match value {
        FeatStruct::Dict(dict) => {
            let mut out = BTreeMap::new();
            for (key, value) in dict {
                out.insert(key.clone(), rename_variables_value(value, used, mapping));
            }
            FeatStruct::Dict(out)
        }
        FeatStruct::List(list) => FeatStruct::List(
            list.iter()
                .map(|value| rename_variables_value(value, used, mapping))
                .collect(),
        ),
    }
}

fn rename_variables_value(
    value: &FeatValue,
    used: &mut BTreeSet<String>,
    mapping: &mut BTreeMap<String, String>,
) -> FeatValue {
    match value {
        FeatValue::Variable(name) => {
            let renamed = if let Some(mapped) = mapping.get(name) {
                mapped.clone()
            } else if used.contains(name) {
                let new_name = next_variable_name(name, used);
                mapping.insert(name.clone(), new_name.clone());
                new_name
            } else {
                used.insert(name.clone());
                name.clone()
            };
            FeatValue::Variable(renamed)
        }
        FeatValue::Struct(value) => {
            FeatValue::Struct(rename_variables_struct(value, used, mapping))
        }
        FeatValue::List(list) => FeatValue::List(
            list.iter()
                .map(|value| rename_variables_value(value, used, mapping))
                .collect(),
        ),
        FeatValue::ReentranceDef { id, value } => FeatValue::ReentranceDef {
            id: id.clone(),
            value: Box::new(rename_variables_value(value, used, mapping)),
        },
        _ => value.clone(),
    }
}

fn next_variable_name(name: &str, used: &BTreeSet<String>) -> String {
    let mut base = name
        .trim_end_matches(|ch: char| ch.is_ascii_digit())
        .to_string();
    if base.is_empty() {
        base = "x".to_string();
    }
    let mut index = 2;
    loop {
        let candidate = format!("{}{}", base, index);
        if !used.contains(&candidate) {
            return candidate;
        }
        index += 1;
    }
}

fn resolve_struct(value: &FeatStruct, reentrances: &mut HashMap<u64, NodeRef>) -> Option<NodeRef> {
    match value {
        FeatStruct::Dict(dict) => {
            let mut out = BTreeMap::new();
            for (key, value) in dict {
                let node = resolve_value(value, reentrances)?;
                out.insert(key.clone(), node);
            }
            Some(Rc::new(RefCell::new(ResolvedValue::Struct(out))))
        }
        FeatStruct::List(list) => {
            let mut out = Vec::new();
            for value in list {
                out.push(resolve_value(value, reentrances)?);
            }
            Some(Rc::new(RefCell::new(ResolvedValue::List(out))))
        }
    }
}

fn resolve_value(value: &FeatValue, reentrances: &mut HashMap<u64, NodeRef>) -> Option<NodeRef> {
    match value {
        FeatValue::ReentranceDef { id, value } => {
            if reentrances.contains_key(&id.0) {
                return None;
            }
            let placeholder = Rc::new(RefCell::new(ResolvedValue::Empty));
            reentrances.insert(id.0, placeholder.clone());
            let inner = resolve_value_inner(value, reentrances)?;
            *placeholder.borrow_mut() = inner;
            Some(placeholder)
        }
        FeatValue::Reentrant(id) => reentrances.get(&id.0).cloned(),
        _ => Some(Rc::new(RefCell::new(resolve_value_inner(
            value,
            reentrances,
        )?))),
    }
}

fn resolve_value_inner(
    value: &FeatValue,
    reentrances: &mut HashMap<u64, NodeRef>,
) -> Option<ResolvedValue> {
    match value {
        FeatValue::Text(value) => Some(ResolvedValue::Text(value.clone())),
        FeatValue::Number(value) => Some(ResolvedValue::Number(*value)),
        FeatValue::Bool(value) => Some(ResolvedValue::Bool(*value)),
        FeatValue::Null => Some(ResolvedValue::Null),
        FeatValue::BytesRange { start, end } => Some(ResolvedValue::BytesRange {
            start: *start,
            end: *end,
        }),
        FeatValue::Empty => Some(ResolvedValue::Empty),
        FeatValue::Variable(name) => Some(ResolvedValue::Variable(name.clone())),
        FeatValue::Struct(value) => match value {
            FeatStruct::Dict(dict) => {
                let mut out = BTreeMap::new();
                for (key, value) in dict {
                    out.insert(key.clone(), resolve_value(value, reentrances)?);
                }
                Some(ResolvedValue::Struct(out))
            }
            FeatStruct::List(list) => {
                let mut out = Vec::new();
                for value in list {
                    out.push(resolve_value(value, reentrances)?);
                }
                Some(ResolvedValue::List(out))
            }
        },
        FeatValue::List(list) => {
            let mut out = Vec::new();
            for value in list {
                out.push(resolve_value(value, reentrances)?);
            }
            Some(ResolvedValue::List(out))
        }
        FeatValue::Reentrant(_) | FeatValue::ReentranceDef { .. } => None,
    }
}

fn resolve_bindings(values: &BTreeMap<String, FeatValue>) -> Option<BTreeMap<String, NodeRef>> {
    let mut out = BTreeMap::new();
    for (name, value) in values {
        let mut reentrances = HashMap::new();
        let resolved = resolve_value(value, &mut reentrances)?;
        out.insert(name.clone(), resolved);
    }
    Some(out)
}

fn unify_nodes(
    left: NodeRef,
    right: NodeRef,
    bindings: &mut BTreeMap<String, NodeRef>,
    forward: &mut HashMap<usize, NodeRef>,
) -> Option<NodeRef> {
    let left = follow_forward(left, forward);
    let right = follow_forward(right, forward);
    if Rc::ptr_eq(&left, &right) {
        return Some(left);
    }

    let (left_value, left_bound) = deref_bound(left.clone(), bindings);
    let (right_value, right_bound) = deref_bound(right.clone(), bindings);

    let left_kind = resolved_kind(&left_value);
    let right_kind = resolved_kind(&right_value);

    let result = match (left_kind, right_kind) {
        (ResolvedKind::Struct, ResolvedKind::Struct) | (ResolvedKind::List, ResolvedKind::List) => {
            unify_containers(left_value.clone(), right_value.clone(), bindings, forward)?
        }
        (ResolvedKind::Variable(name1), ResolvedKind::Variable(name2)) => {
            if name1 != name2 {
                bindings.insert(name2, left_value.clone());
            }
            left_value.clone()
        }
        (ResolvedKind::Variable(name), _) => {
            bindings.insert(name, right_value.clone());
            left_value.clone()
        }
        (_, ResolvedKind::Variable(name)) => {
            bindings.insert(name, left_value.clone());
            right_value.clone()
        }
        (ResolvedKind::Struct, _)
        | (ResolvedKind::List, _)
        | (_, ResolvedKind::Struct)
        | (_, ResolvedKind::List) => {
            return None;
        }
        _ => {
            let equal = {
                let left_borrow = left_value.borrow();
                let right_borrow = right_value.borrow();
                *left_borrow == *right_borrow
            };
            if equal {
                left_value.clone()
            } else {
                return None;
            }
        }
    };

    if let Some(name) = left_bound.as_deref() {
        bindings.insert(name.to_string(), result.clone());
    }
    if let Some(name) = right_bound.as_deref() {
        bindings.insert(name.to_string(), result.clone());
    }

    Some(result)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ResolvedKind {
    Struct,
    List,
    Variable(String),
    Base,
}

fn resolved_kind(node: &NodeRef) -> ResolvedKind {
    match &*node.borrow() {
        ResolvedValue::Struct(_) => ResolvedKind::Struct,
        ResolvedValue::List(_) => ResolvedKind::List,
        ResolvedValue::Variable(name) => ResolvedKind::Variable(name.clone()),
        _ => ResolvedKind::Base,
    }
}

fn unify_containers(
    left: NodeRef,
    right: NodeRef,
    bindings: &mut BTreeMap<String, NodeRef>,
    forward: &mut HashMap<usize, NodeRef>,
) -> Option<NodeRef> {
    let left = follow_forward(left, forward);
    let right = follow_forward(right, forward);
    if Rc::ptr_eq(&left, &right) {
        return Some(left);
    }
    forward.insert(node_id(&right), left.clone());

    let right_snapshot = {
        let borrow = right.borrow();
        match &*borrow {
            ResolvedValue::Struct(map) => Some(ContainerSnapshot::Struct(
                map.iter()
                    .map(|(key, value)| (key.clone(), value.clone()))
                    .collect(),
            )),
            ResolvedValue::List(list) => {
                Some(ContainerSnapshot::List(list.iter().cloned().collect()))
            }
            _ => None,
        }
    }?;

    match right_snapshot {
        ContainerSnapshot::Struct(entries) => {
            let mut left_borrow = left.borrow_mut();
            let ResolvedValue::Struct(left_map) = &mut *left_borrow else {
                return None;
            };
            for (key, right_value) in entries {
                if let Some(left_value) = left_map.get(&key) {
                    let unified = unify_nodes(left_value.clone(), right_value, bindings, forward)?;
                    left_map.insert(key, unified);
                } else {
                    left_map.insert(key, right_value);
                }
            }
            Some(left.clone())
        }
        ContainerSnapshot::List(entries) => {
            let mut left_borrow = left.borrow_mut();
            let ResolvedValue::List(left_list) = &mut *left_borrow else {
                return None;
            };
            if left_list.len() != entries.len() {
                return None;
            }
            for (index, right_value) in entries.into_iter().enumerate() {
                let unified =
                    unify_nodes(left_list[index].clone(), right_value, bindings, forward)?;
                left_list[index] = unified;
            }
            Some(left.clone())
        }
    }
}

enum ContainerSnapshot {
    Struct(Vec<(String, NodeRef)>),
    List(Vec<NodeRef>),
}

fn deref_bound(node: NodeRef, bindings: &BTreeMap<String, NodeRef>) -> (NodeRef, Option<String>) {
    let mut current = node;
    let mut bound = None;
    let mut seen = BTreeSet::new();
    loop {
        let name = match &*current.borrow() {
            ResolvedValue::Variable(name) => name.clone(),
            _ => break,
        };
        if seen.contains(&name) {
            break;
        }
        seen.insert(name.clone());
        if let Some(bound_value) = bindings.get(&name) {
            bound = Some(name);
            current = bound_value.clone();
        } else {
            break;
        }
    }
    (current, bound)
}

fn resolve_aliases(bindings: &mut BTreeMap<String, NodeRef>) {
    let keys: Vec<String> = bindings.keys().cloned().collect();
    for key in keys {
        let mut current = bindings.get(&key).cloned();
        let mut seen = BTreeSet::new();
        while let Some(node) = current.clone() {
            let name = match &*node.borrow() {
                ResolvedValue::Variable(name) => name.clone(),
                _ => break,
            };
            if !seen.insert(name.clone()) {
                break;
            }
            if let Some(next) = bindings.get(&name) {
                current = Some(next.clone());
            } else {
                break;
            }
        }
        if let Some(node) = current {
            bindings.insert(key.clone(), node);
        }
    }
}

fn follow_forward(node: NodeRef, forward: &HashMap<usize, NodeRef>) -> NodeRef {
    let mut current = node;
    let mut seen = BTreeSet::new();
    loop {
        let id = node_id(&current);
        if !seen.insert(id) {
            return current;
        }
        if let Some(next) = forward.get(&id) {
            current = next.clone();
        } else {
            return current;
        }
    }
}

fn node_id(node: &NodeRef) -> usize {
    Rc::as_ptr(node) as usize
}

fn count_references(
    node: &NodeRef,
    bindings: &BTreeMap<String, NodeRef>,
    forward: &HashMap<usize, NodeRef>,
) -> HashMap<usize, usize> {
    let mut counts = HashMap::new();
    let mut visited = BTreeSet::new();
    count_node(node, bindings, forward, &mut counts, &mut visited);
    counts
}

fn count_node(
    node: &NodeRef,
    bindings: &BTreeMap<String, NodeRef>,
    forward: &HashMap<usize, NodeRef>,
    counts: &mut HashMap<usize, usize>,
    visited: &mut BTreeSet<usize>,
) {
    let resolved = resolve_binding(node.clone(), bindings, forward);
    let id = node_id(&resolved);
    *counts.entry(id).or_insert(0) += 1;
    if !visited.insert(id) {
        return;
    }
    let snapshot = {
        let borrow = resolved.borrow();
        match &*borrow {
            ResolvedValue::Struct(values) => {
                Some(NodeSnapshot::Struct(values.values().cloned().collect()))
            }
            ResolvedValue::List(values) => Some(NodeSnapshot::List(values.clone())),
            _ => None,
        }
    };
    match snapshot {
        Some(NodeSnapshot::Struct(values)) => {
            for value in values {
                count_node(&value, bindings, forward, counts, visited);
            }
        }
        Some(NodeSnapshot::List(values)) => {
            for value in values {
                count_node(&value, bindings, forward, counts, visited);
            }
        }
        None => {}
    }
}

enum NodeSnapshot {
    Struct(Vec<NodeRef>),
    List(Vec<NodeRef>),
}

fn resolve_binding(
    node: NodeRef,
    bindings: &BTreeMap<String, NodeRef>,
    forward: &HashMap<usize, NodeRef>,
) -> NodeRef {
    let mut current = follow_forward(node, forward);
    let mut seen = BTreeSet::new();
    loop {
        let name = {
            let borrow = current.borrow();
            match &*borrow {
                ResolvedValue::Variable(name) => Some(name.clone()),
                _ => None,
            }
        };
        let Some(name) = name else {
            return current;
        };
        if !seen.insert(name.clone()) {
            return current;
        }
        if let Some(bound) = bindings.get(&name) {
            current = follow_forward(bound.clone(), forward);
        } else {
            return current;
        }
    }
}

struct Materializer<'a> {
    counts: &'a HashMap<usize, usize>,
    bindings: &'a BTreeMap<String, NodeRef>,
    forward: &'a HashMap<usize, NodeRef>,
    ids: HashMap<usize, FeatReentranceId>,
    next_id: u64,
}

impl<'a> Materializer<'a> {
    fn new(
        counts: &'a HashMap<usize, usize>,
        bindings: &'a BTreeMap<String, NodeRef>,
        forward: &'a HashMap<usize, NodeRef>,
    ) -> Self {
        Self {
            counts,
            bindings,
            forward,
            ids: HashMap::new(),
            next_id: 1,
        }
    }
    fn materialize_value(&mut self, node: &NodeRef) -> FeatValue {
        let resolved = resolve_binding(node.clone(), self.bindings, self.forward);
        let id = node_id(&resolved);
        if self.counts.get(&id).copied().unwrap_or(0) > 1 {
            if let Some(existing) = self.ids.get(&id) {
                return FeatValue::Reentrant(existing.clone());
            }
            let assigned = FeatReentranceId(self.next_id);
            self.next_id += 1;
            self.ids.insert(id, assigned.clone());
            let inner = self.materialize_inner(&resolved);
            return FeatValue::ReentranceDef {
                id: assigned,
                value: Box::new(inner),
            };
        }
        self.materialize_inner(&resolved)
    }

    fn materialize_inner(&mut self, node: &NodeRef) -> FeatValue {
        match &*node.borrow() {
            ResolvedValue::Text(value) => FeatValue::Text(value.clone()),
            ResolvedValue::Number(value) => FeatValue::Number(*value),
            ResolvedValue::Bool(value) => FeatValue::Bool(*value),
            ResolvedValue::Null => FeatValue::Null,
            ResolvedValue::BytesRange { start, end } => FeatValue::BytesRange {
                start: *start,
                end: *end,
            },
            ResolvedValue::Empty => FeatValue::Empty,
            ResolvedValue::Variable(name) => FeatValue::Variable(name.clone()),
            ResolvedValue::Struct(values) => {
                let mut out = BTreeMap::new();
                for (key, value) in values.iter() {
                    out.insert(key.clone(), self.materialize_value(value));
                }
                FeatValue::Struct(FeatStruct::Dict(out))
            }
            ResolvedValue::List(values) => {
                let out = values
                    .iter()
                    .map(|value| self.materialize_value(value))
                    .collect();
                FeatValue::List(out)
            }
        }
    }
}

fn featstruct_from_value(value: FeatValue) -> Option<FeatStruct> {
    match value {
        FeatValue::Struct(value) => Some(value),
        FeatValue::ReentranceDef { value, .. } => match *value {
            FeatValue::Struct(value) => Some(value),
            _ => None,
        },
        _ => None,
    }
}

struct FeatStructParser {
    chars: Vec<char>,
    pos: usize,
}

impl FeatStructParser {
    fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn parse_struct(&mut self) -> Result<FeatStruct, FeatStructParseError> {
        self.skip_ws();
        let prefix = if self.peek() == Some('[') {
            None
        } else if self.lookahead_token_followed_by('[') {
            Some(self.parse_token()?)
        } else {
            return Err(FeatStructParseError::new("expected '['"));
        };
        self.skip_ws();
        self.expect('[')?;
        self.skip_ws();
        if self.consume_if(']') {
            let mut dict = BTreeMap::new();
            if let Some(prefix) = prefix {
                insert_prefix_value(&mut dict, prefix);
            }
            return Ok(FeatStruct::Dict(dict));
        }

        let mut mode = if prefix.is_some() {
            FeatStructMode::Dict
        } else {
            FeatStructMode::Unknown
        };
        let mut dict = BTreeMap::new();
        let mut list = Vec::new();

        loop {
            self.skip_ws();
            let entry = self.parse_entry()?;
            match entry {
                FeatStructEntry::KeyValue(key, value) => {
                    mode = mode.ensure_dict()?;
                    dict.insert(key, value);
                }
                FeatStructEntry::Value(value) => {
                    mode = mode.ensure_list()?;
                    list.push(value);
                }
            }
            self.skip_ws();
            if self.consume_if(']') {
                break;
            }
            self.expect(',')?;
        }

        if let Some(prefix) = prefix {
            insert_prefix_value(&mut dict, prefix);
        }

        if mode == FeatStructMode::Dict {
            self.skip_ws();
        }
        if mode == FeatStructMode::Dict && self.consume_if('/') {
            let value = self.parse_value()?;
            dict.insert(FEAT_SLASH_KEY.to_string(), value);
        }

        match mode {
            FeatStructMode::Dict => Ok(FeatStruct::Dict(dict)),
            FeatStructMode::List => Ok(FeatStruct::List(list)),
            FeatStructMode::Unknown => Ok(FeatStruct::Dict(dict)),
        }
    }

    fn parse_entry(&mut self) -> Result<FeatStructEntry, FeatStructParseError> {
        self.skip_ws();
        if self.peek() == Some('[') {
            return Ok(FeatStructEntry::Value(FeatValue::Struct(
                self.parse_struct()?,
            )));
        }

        if (self.peek() == Some('+') || self.peek() == Some('-'))
            && !self.lookahead_reentrance_ref()
        {
            let start_pos = self.pos;
            let token = self.parse_token()?;
            if token.len() > 1 {
                let sign = token.chars().next().unwrap();
                let name = token[1..].to_string();
                return Ok(FeatStructEntry::KeyValue(
                    name,
                    FeatValue::Bool(sign == '+'),
                ));
            }
            self.pos = start_pos;
        }

        let start_pos = self.pos;
        let token = self.parse_token()?;
        self.skip_ws();
        if self.consume_str("->") {
            let id = self.parse_reentrance_id()?;
            return Ok(FeatStructEntry::KeyValue(token, FeatValue::Reentrant(id)));
        }
        if self.consume_if('=') {
            let value = self.parse_value()?;
            return Ok(FeatStructEntry::KeyValue(token, value));
        }
        self.pos = start_pos;
        let value = self.parse_value()?;
        Ok(FeatStructEntry::Value(value))
    }

    fn parse_value(&mut self) -> Result<FeatValue, FeatStructParseError> {
        self.skip_ws();
        match self.peek() {
            Some('(') if self.lookahead_reentrance_id() => {
                let id = self.parse_reentrance_id()?;
                let value = self.parse_value()?;
                Ok(FeatValue::ReentranceDef {
                    id,
                    value: Box::new(value),
                })
            }
            Some('-') if self.lookahead_reentrance_ref() => {
                self.consume_str("->");
                let id = self.parse_reentrance_id()?;
                Ok(FeatValue::Reentrant(id))
            }
            Some('[') => Ok(FeatValue::Struct(self.parse_struct()?)),
            Some(_) if self.lookahead_token_followed_by('[') => {
                Ok(FeatValue::Struct(self.parse_struct()?))
            }
            Some('"') | Some('\'') => Ok(FeatValue::Text(self.parse_quoted()?)),
            Some('?') => {
                self.pos += 1;
                let name = self.parse_token()?;
                Ok(FeatValue::Variable(name))
            }
            Some(_) => {
                let token = self.parse_token()?;
                if token == "_" {
                    return Ok(FeatValue::Empty);
                }
                if token == "None" {
                    return Ok(FeatValue::Null);
                }
                if token == "True" || token == "False" {
                    return Ok(FeatValue::Bool(token == "True"));
                }
                if token == "true" || token == "false" {
                    return Ok(FeatValue::Bool(token == "true"));
                }
                if let Ok(num) = token.parse::<i64>() {
                    return Ok(FeatValue::Number(num));
                }
                if let Some(range) = parse_range_token(&token) {
                    return Ok(range);
                }
                Ok(FeatValue::Text(token))
            }
            None => Err(FeatStructParseError::new("unexpected end of input")),
        }
    }

    fn parse_quoted(&mut self) -> Result<String, FeatStructParseError> {
        let quote = self
            .next()
            .ok_or_else(|| FeatStructParseError::new("expected quote"))?;
        let mut out = String::new();
        while let Some(ch) = self.next() {
            if ch == quote {
                return Ok(out);
            }
            if ch == '\\' {
                if let Some(escaped) = self.next() {
                    let decoded = match escaped {
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        '\\' => '\\',
                        '"' => '"',
                        '\'' => '\'',
                        other => other,
                    };
                    out.push(decoded);
                } else {
                    return Err(FeatStructParseError::new("unterminated string"));
                }
                continue;
            }
            out.push(ch);
        }
        Err(FeatStructParseError::new("unterminated string"))
    }

    fn parse_token(&mut self) -> Result<String, FeatStructParseError> {
        self.skip_ws();
        let mut out = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() || matches!(ch, '[' | ']' | ',' | '=') {
                break;
            }
            out.push(ch);
            self.pos += 1;
        }
        if out.is_empty() {
            return Err(FeatStructParseError::new("expected token"));
        }
        Ok(out)
    }

    fn skip_ws(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn consume_if(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn consume_str(&mut self, expected: &str) -> bool {
        let mut index = self.pos;
        for ch in expected.chars() {
            if self.chars.get(index).copied() != Some(ch) {
                return false;
            }
            index += 1;
        }
        self.pos = index;
        true
    }

    fn expect(&mut self, expected: char) -> Result<(), FeatStructParseError> {
        if self.consume_if(expected) {
            Ok(())
        } else {
            Err(FeatStructParseError::new(format!(
                "expected '{}'",
                expected
            )))
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn next(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += 1;
        Some(ch)
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.chars.len()
    }

    fn lookahead_token_followed_by(&self, expected: char) -> bool {
        let mut index = self.pos;
        while let Some(ch) = self.chars.get(index).copied() {
            if ch.is_whitespace() {
                index += 1;
            } else {
                break;
            }
        }
        if self.chars.get(index).copied() == Some(expected) {
            return true;
        }
        let start = index;
        while let Some(ch) = self.chars.get(index).copied() {
            if ch.is_whitespace() || matches!(ch, '[' | ']' | ',' | '=') {
                break;
            }
            index += 1;
        }
        if index == start {
            return false;
        }
        while let Some(ch) = self.chars.get(index).copied() {
            if ch.is_whitespace() {
                index += 1;
            } else {
                break;
            }
        }
        self.chars.get(index).copied() == Some(expected)
    }

    fn lookahead_reentrance_id(&self) -> bool {
        let mut index = self.pos;
        if self.chars.get(index).copied() != Some('(') {
            return false;
        }
        index += 1;
        let mut saw_digit = false;
        while let Some(ch) = self.chars.get(index).copied() {
            if ch.is_ascii_digit() {
                saw_digit = true;
                index += 1;
            } else {
                break;
            }
        }
        saw_digit && self.chars.get(index).copied() == Some(')')
    }

    fn lookahead_reentrance_ref(&self) -> bool {
        let mut index = self.pos;
        if self.chars.get(index).copied() != Some('-') {
            return false;
        }
        index += 1;
        if self.chars.get(index).copied() != Some('>') {
            return false;
        }
        index += 1;
        while let Some(ch) = self.chars.get(index).copied() {
            if ch.is_whitespace() {
                index += 1;
            } else {
                break;
            }
        }
        self.chars.get(index).copied() == Some('(')
    }

    fn parse_reentrance_id(&mut self) -> Result<FeatReentranceId, FeatStructParseError> {
        self.skip_ws();
        self.expect('(')?;
        let mut digits = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                digits.push(ch);
                self.pos += 1;
            } else {
                break;
            }
        }
        if digits.is_empty() {
            return Err(FeatStructParseError::new("expected reentrance id"));
        }
        self.expect(')')?;
        let id = digits
            .parse::<u64>()
            .map_err(|_| FeatStructParseError::new("invalid reentrance id"))?;
        Ok(FeatReentranceId(id))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FeatStructMode {
    Unknown,
    Dict,
    List,
}

impl FeatStructMode {
    fn ensure_dict(self) -> Result<Self, FeatStructParseError> {
        match self {
            FeatStructMode::Unknown | FeatStructMode::Dict => Ok(FeatStructMode::Dict),
            FeatStructMode::List => {
                Err(FeatStructParseError::new("cannot mix dict and list values"))
            }
        }
    }

    fn ensure_list(self) -> Result<Self, FeatStructParseError> {
        match self {
            FeatStructMode::Unknown | FeatStructMode::List => Ok(FeatStructMode::List),
            FeatStructMode::Dict => {
                Err(FeatStructParseError::new("cannot mix dict and list values"))
            }
        }
    }
}

enum FeatStructEntry {
    KeyValue(String, FeatValue),
    Value(FeatValue),
}

fn parse_range_token(token: &str) -> Option<FeatValue> {
    let mut parts = token.split(':');
    let start = parts.next()?.parse::<usize>().ok()?;
    let end = parts.next()?.parse::<usize>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some(FeatValue::bytes_range(start, end))
}

const FEAT_PREFIX_KEY: &str = "type";
const FEAT_SLASH_KEY: &str = "slash";

fn insert_prefix_value(dict: &mut FeatDict, prefix: String) {
    if prefix.starts_with('?') {
        dict.insert(
            FEAT_PREFIX_KEY.to_string(),
            FeatValue::Variable(prefix.trim_start_matches('?').to_string()),
        );
    } else {
        dict.insert(FEAT_PREFIX_KEY.to_string(), FeatValue::Text(prefix));
    }
}
