use super::types;

pub fn print_session(my_session: &types::Session) {
    println!("\n{} summary:\n", my_session.filename.as_ref().unwrap());
    println!(
        "Manufacturer: {}    Time created: {}",
        my_session.manufacturer.as_ref().unwrap(),
        my_session.time_created.as_ref().unwrap()
    );
    println!(
        "Sessions: {}      Laps: {:2}      Records: {}",
        my_session.num_sessions.unwrap(),
        my_session.num_laps.unwrap(),
        my_session.num_records.unwrap()
    );
    println!(
        "Total duration:  {}      Calories Burned: {}",
        my_session.duration.unwrap(),
        my_session.calories.unwrap()
    );
    println!("\nTime in Zones:");
    println!(
        "Speed/Power: {}",
        my_session.time_in_hr_zones.hr_zone_4.unwrap()
    );
    println!(
        "Anaerobic:   {}",
        my_session.time_in_hr_zones.hr_zone_3.unwrap()
    );
    println!(
        "Aerobic:     {}",
        my_session.time_in_hr_zones.hr_zone_2.unwrap()
    );
    println!(
        "Fat Burning: {}",
        my_session.time_in_hr_zones.hr_zone_1.unwrap()
    );
    println!(
        "Warmup:      {}",
        my_session.time_in_hr_zones.hr_zone_0.unwrap()
    );
}
