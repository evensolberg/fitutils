use utilities::FITActivity;

pub fn print_activity(act: &FITActivity, detailed: bool) {
    let unknown = "Unknown".to_string();

    println!("\n:: FILE: {}", act.session.filename.as_ref().unwrap());
    println!(
        "Manufacturer:  {}",
        act.session.manufacturer.as_ref().unwrap_or(&unknown)
    );
    println!(
        "Product:       {}",
        act.session.product.as_ref().unwrap_or(&unknown)
    );
    println!(
        "Serial number: {}",
        act.session.serial_number.as_ref().unwrap_or(&unknown)
    );
    println!(
        "Time created:  {}",
        act.session.time_created.as_ref().unwrap()
    );
    println!(
        "Activity type:   {}",
        act.session.activity_type.as_ref().unwrap_or(&unknown)
    );
    println!(
        "Activity detail: {}",
        act.session.activity_detailed.as_ref().unwrap_or(&unknown)
    );
    println!(
        "Sessions:           {:5}",
        act.session.num_sessions.unwrap_or_default()
    );
    println!(
        "Laps:               {:5}",
        act.session.num_laps.unwrap_or_default()
    );
    println!(
        "Records:            {:5}",
        act.session.num_records.unwrap_or_default()
    );
    println!("Total duration:  {}", act.session.duration.unwrap());
    println!(
        "Calories Burned:    {:5}",
        act.session.calories.unwrap_or_default()
    );

    println!(
        "Cadence Avg: {}, Max: {}",
        act.session.cadence_avg.unwrap_or_default(),
        act.session.cadence_max.unwrap_or_default()
    );
    println!(
        "Heart Rate Min: {}, Avg: {}, Max: {}",
        act.session.heartrate_min.unwrap_or_default(),
        act.session.heartrate_avg.unwrap_or_default(),
        act.session.heartrate_max.unwrap_or_default()
    );

    println!(
        "Speed Avg: {:?} m/s, Max: {:?} m/s",
        act.session.speed_avg.unwrap_or_default().value,
        act.session.speed_max.unwrap_or_default().value
    );
    println!(
        "Power Avg: {}, Max: {}, Threshold: {}",
        act.session.power_avg.unwrap_or_default(),
        act.session.power_max.unwrap_or_default(),
        act.session.power_threshold.unwrap_or_default()
    );
    println!(
        "Ascent:   {:5} m",
        act.session.ascent.unwrap_or_default().value
    );
    println!(
        "Descent:  {:5} m",
        act.session.descent.unwrap_or_default().value
    );
    println!(
        "Distance: {:5} m",
        act.session.distance.unwrap_or_default().value
    );
    if detailed {
        println!(
            "North East Latitude: {}, Longitude: {}",
            act.session.nec_lat.unwrap_or_default(),
            act.session.nec_lon.unwrap_or_default()
        );
        println!(
            "South West Latitude: {}, Longitude: {}",
            act.session.swc_lat.unwrap_or_default(),
            act.session.swc_lon.unwrap_or_default()
        );
        println!(
            "Stance time avg: {}s",
            act.session.stance_time_avg.unwrap_or_default()
        );
        println!(
            "Vertical Oscillation Avg: {} cm",
            act.session.vertical_oscillation_avg.unwrap_or_default()
        );
        println!(
            "Duration Active: {}",
            act.session.duration_active.unwrap_or_default()
        );
        println!(
            "Duration Moving: {}",
            act.session.duration_moving.unwrap_or_default()
        );
        println!("Start time:  {}", act.session.start_time.as_ref().unwrap());
        println!("Finish time: {}", act.session.finish_time.as_ref().unwrap());
        println!("Time in Zones:");
        println!(
            "  Speed/Power: {}",
            act.session.time_in_hr_zones.hr_zone_4.unwrap()
        );
        println!(
            "  Anaerobic:   {}",
            act.session.time_in_hr_zones.hr_zone_3.unwrap()
        );
        println!(
            "  Aerobic:     {}",
            act.session.time_in_hr_zones.hr_zone_2.unwrap()
        );
        println!(
            "  Fat Burning: {}",
            act.session.time_in_hr_zones.hr_zone_1.unwrap()
        );
        println!(
            "  Warmup:      {}",
            act.session.time_in_hr_zones.hr_zone_0.unwrap()
        );
    }
}
