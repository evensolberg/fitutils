use super::types;

pub fn print_session(my_session: &types::Session) {
    println!("Manufacturer: {}", my_session.manufacturer);
    println!("Time created: {}", my_session.time_created);
    println!("Sessions:     {:5}", my_session.num_sessions.unwrap());
    println!("Laps:         {:5}", my_session.num_laps.unwrap());
    println!("Records:      {:5}", my_session.num_records.unwrap());
    println!("\nTotal duration:  {}", my_session.duration);
    println!("Calories burned: {:8}", my_session.calories.unwrap());
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
