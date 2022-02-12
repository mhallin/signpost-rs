#![cfg(test)]

#[test]
fn emit_event_macro() {
    static LOGGER: signpost::OsLog = signpost::const_poi_logger!("mhallin.github.io");
    signpost::emit_event!(LOGGER, 10, "Event");
}

#[test]
fn emit_interval() {
    static LOGGER: signpost::OsLog = signpost::const_poi_logger!("mhallin.github.io");
    let _interval = signpost::begin_interval!(LOGGER, 11, "Interval");

    std::thread::sleep(std::time::Duration::from_millis(10));
}
