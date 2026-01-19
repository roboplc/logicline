use logicline::{action, global};

fn main() {
    global::install_exporter().unwrap();
    global::set_recording(true);

    println!("Open browser to http://localhost:9001 to view the state");

    let mut temperature = 20.0;
    let mut direction_up = true;

    for _ in rtsc::time::interval_hz(1) {
        let mut processor = global::processor();

        // Some fan state
        let mut fan = false;

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

        global::ingress(&mut processor);

        if direction_up {
            temperature += 1.0;
            if temperature >= 35.0 {
                direction_up = false;
            }
        } else {
            temperature -= 1.0;
            if temperature <= 15.0 {
                direction_up = true;
            }
        }
    }
}
