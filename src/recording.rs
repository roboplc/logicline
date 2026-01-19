use core::fmt;
use std::{borrow::Cow, collections::BTreeMap, sync::Arc};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Rack;

/// Input kind, flow: taken from the previous action, external: specified by the user
#[derive(Deserialize, Serialize, Debug, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InputKind {
    /// Taken from the previous action
    Flow,
    /// Specified by the user
    External,
}

/// State of the logical line
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LineState {
    name: Cow<'static, str>,
    steps: Vec<StepState>,
}

impl fmt::Display for LineState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: ", self.name)?;
        let mut passed = true;
        for (step_no, step) in self.steps.iter().enumerate() {
            if step_no > 0 {
                write!(f, " -> ")?;
            }
            match step {
                StepState::Single(s) => {
                    write!(f, "{}", s.name())?;
                    if s.input() != &Value::Null {
                        write!(f, "({})", s.input())?;
                    }
                }
                StepState::Multi(ss) => {
                    write!(f, "( ")?;
                    for (s_no, s) in ss.iter().enumerate() {
                        if s_no > 0 {
                            write!(f, " | ")?;
                        }
                        write!(f, "{}", s.name())?;
                        if s.input() != &Value::Null {
                            write!(f, "(")?;
                            match s.input_kind() {
                                InputKind::Flow => {}
                                InputKind::External => {
                                    write!(f, "\\->")?;
                                }
                            }
                            write!(f, "{})", s.input())?;
                        }
                    }
                    write!(f, " )")?;
                }
            }
            if passed && !step.passed() {
                passed = false;
                write!(f, " !")?;
            }
        }
        Ok(())
    }
}

impl LineState {
    pub(crate) fn new(name: impl Into<Cow<'static, str>>) -> Self {
        LineState {
            name: name.into(),
            steps: Vec::new(),
        }
    }
    /// Name of the line
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
    /// Steps states of the line
    pub fn steps(&self) -> &[StepState] {
        &self.steps
    }
    /// Steps states of the line, mutable
    pub fn steps_mut(&mut self) -> &mut [StepState] {
        &mut self.steps
    }
    //pub(crate) fn push_step_state<INPUT: Serialize>(
    //&mut self,
    //name: impl Into<Cow<'static, str>>,
    //input: INPUT,
    //passed: bool,
    //) {
    //self.steps
    //.push(StepState::Single(StepStateSingle::new(name, input, passed)));
    //}
    pub(crate) fn extend<I>(&mut self, step_states: I)
    where
        I: IntoIterator<Item = StepStateInfo>,
    {
        let steps = step_states.into_iter().collect();
        self.steps.push(StepState::Multi(steps));
    }
    pub(crate) fn push_step_state(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        input: Value,
        input_kind: InputKind,
        passed: bool,
    ) {
        self.steps
            .push(StepState::Single(StepStateInfo::new_with_serialized_input(
                name, input, input_kind, passed,
            )));
    }
    pub(crate) fn clear(&mut self) {
        self.steps.clear();
    }
}

/// Line step state
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum StepState {
    /// Single step state (single flow)
    Single(StepStateInfo),
    /// Multiple step state (logical OR)
    Multi(Vec<StepStateInfo>),
}

impl StepState {
    /// Has the step been passed
    pub fn passed(&self) -> bool {
        match self {
            StepState::Single(single) => single.passed(),
            #[allow(clippy::redundant_closure_for_method_calls)]
            StepState::Multi(multi) => multi.iter().all(|s| s.passed()),
        }
    }
    /// Step state info, single-value vector for single step state, multi-value vector for multi step state
    pub fn info(&self) -> Vec<&StepStateInfo> {
        match self {
            StepState::Single(single) => vec![single],
            StepState::Multi(multi) => multi.iter().collect::<Vec<_>>(),
        }
    }
    /// Step state info mutable
    pub fn info_mut(&mut self) -> Vec<&mut StepStateInfo> {
        match self {
            StepState::Single(single) => vec![single],
            StepState::Multi(multi) => multi.iter_mut().collect::<Vec<_>>(),
        }
    }
}

/// Single step state information
#[derive(Serialize, Deserialize)]
pub struct StepStateInfo {
    #[serde(flatten)]
    inner: Arc<StepStateInner>,
}

impl Clone for StepStateInfo {
    fn clone(&self) -> Self {
        StepStateInfo {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl fmt::Debug for StepStateInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("StepState")
            .field("name", &self.inner.name)
            .field("input", &self.inner.input)
            .field("passed", &self.inner.passed)
            .finish()
    }
}

#[derive(Serialize, Deserialize)]
struct StepStateInner {
    name: Cow<'static, str>,
    input: Value,
    input_kind: InputKind,
    passed: bool,
}

impl StepStateInfo {
    /// Returns modified version of the step state info
    pub fn to_modified(
        &self,
        name: Option<&str>,
        input: Option<Value>,
        input_kind: Option<InputKind>,
        passed: Option<bool>,
    ) -> Self {
        StepStateInfo {
            inner: Arc::new(StepStateInner {
                name: name.map_or_else(|| self.inner.name.clone(), |n| n.to_owned().into()),
                input: input.unwrap_or_else(|| self.inner.input.clone()),
                input_kind: input_kind.unwrap_or(self.inner.input_kind),
                passed: passed.unwrap_or(self.inner.passed),
            }),
        }
    }
    pub(crate) fn new<INPUT: Serialize>(
        name: impl Into<Cow<'static, str>>,
        input: INPUT,
        input_kind: InputKind,
        passed: bool,
    ) -> Self {
        StepStateInfo {
            inner: Arc::new(StepStateInner {
                name: name.into(),
                input: serde_json::to_value(input).unwrap_or_default(),
                input_kind,
                passed,
            }),
        }
    }
    pub(crate) fn new_with_serialized_input(
        name: impl Into<Cow<'static, str>>,
        input: Value,
        input_kind: InputKind,
        passed: bool,
    ) -> Self {
        StepStateInfo {
            inner: Arc::new(StepStateInner {
                name: name.into(),
                input,
                input_kind,
                passed,
            }),
        }
    }
    /// Step name
    pub fn name(&self) -> &str {
        self.inner.name.as_ref()
    }
    /// Step input (serialized as [`serde_json::Value`])
    pub fn input(&self) -> &Value {
        &self.inner.input
    }
    /// Step input kind (flow or external)
    pub fn input_kind(&self) -> InputKind {
        self.inner.input_kind
    }
    /// Has the step been passed
    pub fn passed(&self) -> bool {
        self.inner.passed
    }
}

impl fmt::Display for Rack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, line) in self.lines.values().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{}", line)?;
        }
        Ok(())
    }
}

/// Modifies snapshots before serving/displaying
pub trait SnapshotFormatter: Send + Sync {
    /// Format the snapshot
    fn format(&self, snapshot: Snapshot) -> Snapshot;
}

/// State snapshot
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub(crate) lines: BTreeMap<Cow<'static, str>, LineState>,
}

impl Snapshot {
    /// State of the line
    pub fn line_state(&self, name: &str) -> Option<&LineState> {
        self.lines.get(name)
    }
    /// Lines map
    pub fn lines(&self) -> &BTreeMap<Cow<'static, str>, LineState> {
        &self.lines
    }
    /// Mutable state of the line
    pub fn line_state_mut(&mut self, name: &str) -> Option<&mut LineState> {
        self.lines.get_mut(name)
    }
    /// Lines map
    pub fn lines_mut(&mut self) -> &mut BTreeMap<Cow<'static, str>, LineState> {
        &mut self.lines
    }
}

impl fmt::Display for Snapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, line) in self.lines.values().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{}", line)?;
        }
        Ok(())
    }
}
