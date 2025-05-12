#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "README.md" ) ) ]
#![deny(missing_docs)]

use std::{borrow::Cow, marker::PhantomData};
#[cfg(feature = "recording")]
use std::{
    collections::{btree_map, BTreeMap},
    mem,
    sync::{atomic, Arc},
};

#[cfg(feature = "recording")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "recording")]
mod recording;
#[cfg(feature = "recording")]
pub use recording::{InputKind, LineState, Snapshot, StepState, StepStateInfo};
#[cfg(feature = "recording")]
use serde_json::Value;

/// The process global state
pub mod global {
    #[cfg(feature = "exporter")]
    const DEFAULT_PORT: u16 = 9001;

    #[cfg(feature = "exporter")]
    use std::net::{IpAddr, ToSocketAddrs};

    use super::{Processor, Rack};
    use once_cell::sync::Lazy;

    #[cfg(feature = "locking-default")]
    use parking_lot::Mutex;

    #[cfg(feature = "locking-rt")]
    use parking_lot_rt::Mutex;

    #[cfg(feature = "locking-rt-safe")]
    use rtsc::pi::Mutex;

    static GLOBAL_LADDER: Lazy<Mutex<Rack>> = Lazy::new(|| Mutex::new(Rack::new()));

    /// Sets the recording state for the global rack state
    #[cfg(feature = "recording")]
    pub fn set_recording(recording: bool) {
        GLOBAL_LADDER.lock().set_recording(recording);
    }

    /// Creates a snapshot of the global state
    #[cfg(feature = "recording")]
    pub fn snapshot() -> super::Snapshot {
        GLOBAL_LADDER.lock().snapshot()
    }

    /// Creates a new processor for the global state
    pub fn processor() -> Processor {
        GLOBAL_LADDER.lock().processor()
    }

    /// Stores the state of the processor in the global state
    #[cfg(feature = "recording")]
    pub fn ingress(processor: &mut Processor) {
        GLOBAL_LADDER.lock().ingress(processor);
    }

    #[cfg(not(feature = "recording"))]
    /// When the recording feature is disabled, this function does nothing
    pub fn ingress(_processor: &mut Processor) {}

    /// Installs the exporter (HTTP server) on the default address (all interfaces, 9001)
    #[cfg(feature = "exporter")]
    pub fn install_exporter() -> Result<(), Box<dyn std::error::Error>> {
        install_exporter_on((IpAddr::from([0, 0, 0, 0]), DEFAULT_PORT))
    }

    /// Installs the exporter (HTTP server) on the specified address
    #[cfg(feature = "exporter")]
    pub fn install_exporter_on<A: ToSocketAddrs>(
        addr: A,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let server = rouille::Server::new(addr, move |request| {
            if request.method() != "GET" {
                return rouille::Response::empty_406();
            }
            if request.url() == "/state" {
                return rouille::Response::json(&snapshot())
                    .with_additional_header("Access-Control-Allow-Origin", "*")
                    .with_additional_header("Access-Control-Allow-Methods", "GET, OPTIONS")
                    .with_additional_header("Access-Control-Allow-Headers", "Content-Type");
            }
            #[cfg(feature = "exporter-ui")]
            if request.url() == "/" {
                return rouille::Response::html(include_str!("../ll-default-view/dist/index.html"));
            }
            rouille::Response::empty_404()
        })
        .map_err(|e| e.to_string())?;
        std::thread::Builder::new()
            .name("exporter".to_string())
            .spawn(move || {
                server.run();
            })?;
        Ok(())
    }
}

/// Logical step in the line
pub struct Step<'p, INPUT> {
    active: bool,
    input: Option<INPUT>,
    processor: Option<&'p mut Processor>,
    line_name: Option<Cow<'static, str>>,
}

/// Operation helpers
pub mod ops {

    /// Logical NOT operation. In case if the input is `Some`, returns `None`, otherwise returns
    /// `Some(())`
    pub fn not(input: Option<()>) -> Option<()> {
        if input.is_some() {
            None
        } else {
            Some(())
        }
    }
}

#[cfg(feature = "recording")]
/// When the recording feature is enabled, inputs must implement the [`serde::Serialize`] trait
pub trait StepInput: Serialize {}

#[cfg(feature = "recording")]
impl<T> StepInput for T where T: Serialize {}

#[cfg(not(feature = "recording"))]
/// When the recording feature is disabled, the trait is empty
pub trait StepInput {}

#[cfg(not(feature = "recording"))]
impl<T> StepInput for T {}

impl<'p, INPUT> Step<'p, INPUT>
where
    INPUT: StepInput,
{
    /// Returns if the step is active (can be passed)
    pub fn is_active(&self) -> bool {
        self.active
    }
    /// Creates a new step
    pub fn new(value: INPUT) -> Self {
        Step {
            input: Some(value),
            active: true,
            processor: None,
            line_name: None,
        }
    }

    #[cfg(feature = "recording")]
    fn processor_is_recording(&self) -> bool {
        self.processor
            .as_ref()
            .is_some_and(|processor| processor.is_recording())
    }

    #[cfg(feature = "recording")]
    fn line_state_mut(&mut self) -> Option<&mut LineState> {
        let processor = self.processor.as_mut()?;
        let line_name = self.line_name.as_ref()?;
        processor.result.get_mut(line_name)
    }

    /// Passes the step in case if any of the actions returns `Some`
    #[allow(clippy::missing_panics_doc)]
    pub fn then_any<OUTPUT, A, A2, F, F2>(mut self, action1: A, action2: A2) -> Step<'p, OUTPUT>
    where
        A: Into<Action<'p, F, INPUT, OUTPUT>>,
        F: FnOnce(INPUT) -> Option<OUTPUT>,
        A2: Into<Action<'p, F2, INPUT, OUTPUT>>,
        F: FnOnce(INPUT) -> Option<OUTPUT>,
        F2: FnOnce(INPUT) -> Option<OUTPUT>,
        INPUT: Clone,
    {
        #[allow(unused_mut)]
        let mut action1 = action1.into();
        #[cfg(feature = "recording")]
        let input_kind1 = action1.input_kind();
        #[cfg(feature = "recording")]
        let recorded_input1 = self
            .processor_is_recording()
            .then(|| action1.take_recorded_input_serialized(self.input.as_ref()))
            .unwrap_or_default();
        #[allow(unused_mut)]
        let mut action2 = action2.into();
        #[cfg(feature = "recording")]
        let input_kind2 = action2.input_kind();
        #[cfg(feature = "recording")]
        let recorded_input2 = self
            .processor_is_recording()
            .then(|| action2.take_recorded_input_serialized(self.input.as_ref()))
            .unwrap_or_default();
        if !self.active || self.input.is_none() {
            #[cfg(feature = "recording")]
            {
                if let Some(l) = self.line_state_mut() {
                    let step_states = vec![
                        StepStateInfo::new(action1.name, None::<()>, input_kind1, false),
                        StepStateInfo::new(action2.name, None::<()>, input_kind2, false),
                    ];
                    l.extend(step_states);
                }
            }
            return Step {
                input: None,
                active: false,
                processor: self.processor,
                line_name: self.line_name,
            };
        }
        let action_input = self.input.take().unwrap();
        let mut next_input = None;
        #[cfg(feature = "recording")]
        let mut step_states = Vec::with_capacity(2);
        if let Some(output) = (action1.f)(action_input.clone()) {
            next_input = Some(output);
            #[cfg(feature = "recording")]
            step_states.push(StepStateInfo::new_with_serialized_input(
                action1.name,
                recorded_input1,
                input_kind1,
                true,
            ));
        } else {
            #[cfg(feature = "recording")]
            step_states.push(StepStateInfo::new_with_serialized_input(
                action1.name,
                recorded_input1,
                input_kind1,
                false,
            ));
        }
        if let Some(output) = (action2.f)(action_input) {
            if next_input.is_none() {
                next_input = Some(output);
            }
            #[cfg(feature = "recording")]
            step_states.push(StepStateInfo::new_with_serialized_input(
                action2.name,
                recorded_input2,
                input_kind2,
                true,
            ));
        } else {
            #[cfg(feature = "recording")]
            step_states.push(StepStateInfo::new_with_serialized_input(
                action2.name,
                recorded_input2,
                input_kind2,
                false,
            ));
        }
        #[cfg(feature = "recording")]
        if let Some(l) = self.line_state_mut() {
            l.extend(step_states);
        }
        Step {
            active: next_input.is_some(),
            input: next_input,
            processor: self.processor,
            line_name: self.line_name,
        }
    }

    /// Passes the step in case if the action returns `Some`
    #[allow(clippy::missing_panics_doc)]
    pub fn then<OUTPUT, A, F>(mut self, action: A) -> Step<'p, OUTPUT>
    where
        A: Into<Action<'p, F, INPUT, OUTPUT>>,
        F: FnOnce(INPUT) -> Option<OUTPUT>,
    {
        #[allow(unused_mut)]
        let mut action = action.into();
        #[cfg(feature = "recording")]
        let input_kind = action.input_kind();
        #[cfg(feature = "recording")]
        macro_rules! record_processed {
            ($name:expr, $passed:expr, $input:expr) => {
                if let Some(l) = self.line_state_mut() {
                    l.push_step_state(action.name, $input, input_kind, $passed);
                }
            };
        }
        if !self.active || self.input.is_none() {
            #[cfg(feature = "recording")]
            record_processed!(action.name, false, Value::Null);
            return Step {
                input: None,
                active: false,
                processor: self.processor,
                line_name: self.line_name,
            };
        }
        #[cfg(feature = "recording")]
        let recorded_input = self
            .processor_is_recording()
            .then(|| action.take_recorded_input_serialized(self.input.as_ref()))
            .unwrap_or_default();
        if let Some(output) = (action.f)(self.input.take().unwrap()) {
            #[cfg(feature = "recording")]
            record_processed!(action.name, true, recorded_input);
            Step {
                input: Some(output),
                active: true,
                processor: self.processor,
                line_name: self.line_name,
            }
        } else {
            #[cfg(feature = "recording")]
            record_processed!(action.name, false, recorded_input);
            Step {
                input: None,
                active: false,
                processor: self.processor,
                line_name: self.line_name,
            }
        }
    }
}

#[allow(dead_code)]
/// Action is a function wrapper that can be used in a step
pub struct Action<'a, F, INPUT, OUTPUT>
where
    F: FnOnce(INPUT) -> Option<OUTPUT>,
{
    f: F,
    name: Cow<'static, str>,
    #[cfg(feature = "recording")]
    recorded_input: Option<&'a dyn erased_serde::Serialize>,
    #[cfg(not(feature = "recording"))]
    _recorded_input: PhantomData<&'a ()>,
    _input: PhantomData<INPUT>,
}

impl<F, INPUT, OUTPUT> From<F> for Action<'_, F, INPUT, OUTPUT>
where
    F: FnOnce(INPUT) -> Option<OUTPUT>,
{
    fn from(function: F) -> Self {
        Action::new("", function)
    }
}

/// Creates a new action, in case if the name is not provided, the function name will be used as
/// the name of the action (in case of closures it is recommended to always provide a name to get
/// it clear and readable).
#[macro_export]
macro_rules! action {
    ($f: expr) => {
        $crate::Action::new(stringify!($f), $f)
    };
    ($name: expr, $f: expr) => {
        $crate::Action::new($name, $f)
    };
}

impl<'a, F, INPUT, OUTPUT> Action<'a, F, INPUT, OUTPUT>
where
    F: FnOnce(INPUT) -> Option<OUTPUT>,
{
    /// Creates a new action
    pub fn new(name: impl Into<Cow<'static, str>>, f: F) -> Self {
        Action {
            f,
            name: name.into(),
            #[cfg(feature = "recording")]
            recorded_input: None,
            #[cfg(not(feature = "recording"))]
            _recorded_input: PhantomData,
            _input: PhantomData,
        }
    }
    /// Sets the recorded (actual) input for the action function
    #[cfg(feature = "recording")]
    pub fn with_recorded_input<V>(mut self, input: &'a V) -> Self
    where
        V: Serialize,
    {
        self.recorded_input = Some(input);
        self
    }
    #[cfg(not(feature = "recording"))]
    #[allow(unused_mut)]
    /// When the recording feature is disabled, this function does nothing
    pub fn with_recorded_input<V>(mut self, _input: &'a V) -> Self {
        self
    }
    #[cfg(feature = "recording")]
    // WARNING: must be called before the input is taken
    fn input_kind(&self) -> InputKind {
        if self.recorded_input.is_some() {
            InputKind::External
        } else {
            InputKind::Flow
        }
    }
    #[cfg(feature = "recording")]
    fn take_recorded_input_serialized(&mut self, fallback: Option<&INPUT>) -> Value
    where
        INPUT: StepInput,
    {
        if let Some(i) = self.recorded_input.take() {
            serde_json::to_value(i).unwrap_or_default()
        } else {
            serde_json::to_value(fallback).unwrap_or_default()
        }
    }
}

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "recording", derive(Serialize, Deserialize))]
/// State of the process or a group of logic lines. Acts as a factory for [`Processor`] instances.
/// Shares recording state with the created processors.
pub struct Rack {
    #[cfg(feature = "recording")]
    lines: BTreeMap<Cow<'static, str>, LineState>,
    #[serde(skip)]
    #[cfg(feature = "recording")]
    recording: Arc<atomic::AtomicBool>,
}

impl Rack {
    /// Creates a new state
    pub fn new() -> Self {
        Self::default()
    }
    /// Record the state of the lines and reset the processor
    #[allow(unused_variables)]
    pub fn ingress(&mut self, processor: &mut Processor) {
        #[cfg(feature = "recording")]
        self.lines.extend(mem::take(&mut processor.result));
        #[cfg(not(feature = "recording"))]
        processor.reset();
    }
    /// Returns the state of the line
    #[cfg(feature = "recording")]
    pub fn line_state(&self, name: &str) -> Option<&LineState> {
        self.lines.get(name)
    }
    /// Returns all states of the lines
    #[cfg(feature = "recording")]
    pub fn lines(&self) -> &BTreeMap<Cow<'static, str>, LineState> {
        &self.lines
    }
    /// Creates a snapshot of the current state of the lines
    #[cfg(feature = "recording")]
    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            lines: self.lines.clone(),
        }
    }
    /// Creates a new processor
    pub fn processor(&self) -> Processor {
        Processor {
            #[cfg(feature = "recording")]
            result: BTreeMap::new(),
            #[cfg(feature = "recording")]
            recording: Arc::clone(&self.recording),
        }
    }

    /// Enables recording for the state
    #[cfg(feature = "recording")]
    pub fn with_recording_enabled(self) -> Self {
        self.recording.store(true, atomic::Ordering::SeqCst);
        self
    }

    /// Sets the recording state for the state
    #[cfg(feature = "recording")]
    pub fn set_recording(&mut self, recording: bool) {
        self.recording.store(recording, atomic::Ordering::SeqCst);
    }
}

/// Processor is an instance which creates logical lines
#[derive(Default)]
pub struct Processor {
    #[cfg(feature = "recording")]
    result: BTreeMap<Cow<'static, str>, LineState>,
    #[cfg(feature = "recording")]
    recording: Arc<atomic::AtomicBool>,
}

impl Processor {
    /// Creates a new processor (state-independent)
    pub fn new() -> Self {
        Self::default()
    }
    /// Resets the processor recordings
    pub fn reset(&mut self) {
        #[cfg(feature = "recording")]
        self.result.clear();
    }
    /// Returns the state of the line
    #[cfg(feature = "recording")]
    pub fn line_state(&self, name: &str) -> Option<&LineState> {
        self.result.get(name)
    }
    /// Returns `true` if the processor is recording
    #[cfg(feature = "recording")]
    pub fn is_recording(&self) -> bool {
        self.recording.load(atomic::Ordering::SeqCst)
    }
    /// Creates a new logical line
    pub fn line<INPUT>(&mut self, name: impl Into<Cow<'static, str>>, input: INPUT) -> Step<INPUT> {
        let name = name.into();
        #[cfg(feature = "recording")]
        if self.is_recording() {
            match self.result.entry(name.clone()) {
                btree_map::Entry::Vacant(entry) => {
                    entry.insert(LineState::new(name.clone()));
                }
                btree_map::Entry::Occupied(mut entry) => {
                    entry.get_mut().clear();
                }
            }
        }
        Step {
            input: Some(input),
            active: true,
            processor: Some(self),
            line_name: Some(name),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Rack;
    #[cfg(feature = "recording")]
    use serde::Serialize;

    #[test]
    fn test_lines() {
        #[allow(clippy::cast_lossless, clippy::unnecessary_wraps)]
        fn get_temp1(data: &ModbusData) -> Option<f32> {
            Some(data.temperature_1 as f32 / 10.0)
        }

        #[allow(clippy::cast_lossless, clippy::unnecessary_wraps)]
        fn get_temp2(data: &ModbusData) -> Option<f32> {
            Some(data.temperature_2 as f32 / 10.0)
        }

        #[allow(clippy::unnecessary_wraps)]
        fn temperature_critical(temp: f32) -> Option<()> {
            if temp > 90. {
                Some(())
            } else {
                None
            }
        }

        fn voltage_critical(voltage: u16) -> Option<()> {
            if voltage > 300 {
                Some(())
            } else {
                None
            }
        }

        #[cfg_attr(feature = "recording", derive(Serialize))]
        struct ModbusData {
            temperature_1: u16,
            voltage_1: u16,
            temperature_2: u16,
            voltage_2: u16,
        }

        let modbus_data = ModbusData {
            temperature_1: 950,
            voltage_1: 395,
            temperature_2: 250,
            voltage_2: 295,
        };
        let mut state = Rack::new();
        let mut processor = state.processor();
        let mut line1_active = true;
        let mut line2_active = true;
        #[cfg(feature = "recording")]
        state.set_recording(true);
        assert!(processor
            .line("line1", &modbus_data)
            .then(action!(get_temp1))
            .then(action!(temperature_critical))
            .then(
                action!("voltage", |()| Some(modbus_data.voltage_1))
                    .with_recorded_input(&modbus_data.voltage_1)
            )
            .then(action!(voltage_critical))
            .then(action!("OFF", |()| {
                line1_active = false;
                Some(())
            }))
            .is_active());
        assert!(!processor
            .line("line2", &modbus_data)
            .then(get_temp2)
            .then(action!(temperature_critical))
            .then(
                action!("voltage", |()| Some(modbus_data.voltage_2))
                    .with_recorded_input(&modbus_data.voltage_2)
            )
            .then(action!(voltage_critical))
            .then(action!("OFF", |()| {
                line2_active = false;
                Some(())
            }))
            .is_active());
        assert!(!line1_active);
        assert!(line2_active);
        state.ingress(&mut processor);
    }
}
