use logicline::{Rack, action};

fn main() {
    let mut state = Rack::new().with_recording_enabled();
    let mut processor = state.processor();

    // Some fan state
    let mut fan = false;

    // A temperature sensor value
    let temperature = 31.0;

    processor
        // a sequence to turn on the fan on if the temperature is above 30 degrees
        .line("fan_on", temperature)
        .then(action!("temp_high", |t| (t > 30.0).then_some(())))
        .then(action!("fan_on", |()| {
            fan = true;
            Some(())
        }));
    processor
        // a sequence to turn off the fan if the temperature is below 25 degrees
        .line("fan_off", temperature)
        .then(action!("temp_low", |t| (t < 25.0).then_some(())))
        .then(action!("fan_off", |()| {
            fan = false;
            Some(())
        }));

    state.ingress(&mut processor);
    println!("{}", state);

    println!("{}", serde_json::to_string_pretty(&state).unwrap());
}
