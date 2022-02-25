use chrono::{Datelike, Local};
use std::path::Path;
use utilities::{Duration, GPXActivity, TimeStamp};

pub fn print_activity(act: &GPXActivity, detailed: bool) {
    let unknown = "Unknown".to_string();

    println!(
        "\nFile:              {}",
        act.metadata
            .filename
            .as_ref()
            .unwrap_or(&Path::new("unknown").to_path_buf())
            .to_string_lossy()
    );
    println!(
        "GPX Version:       {}",
        act.metadata.version.as_ref().unwrap_or(&unknown)
    );
    println!(
        "Creator:           {}",
        act.metadata.creator.as_ref().unwrap_or(&unknown)
    );
    println!(
        "Activity:          {}",
        act.metadata.activity.as_ref().unwrap_or(&unknown)
    );
    println!(
        "Time:              {}",
        act.metadata
            .time
            .as_ref()
            .unwrap_or(&TimeStamp(Local::now()))
    );
    println!(
        "Duration:          {}",
        act.metadata
            .duration
            .as_ref()
            .unwrap_or(&Duration::default())
    );
    println!(
        "Description:       {}",
        act.metadata.description.as_ref().unwrap_or(&unknown)
    );
    println!("Waypoints:         {}", act.metadata.num_waypoints);
    println!("Tracks:            {}", act.metadata.num_tracks);
    println!("Routes:            {}", act.metadata.num_routes);
    if detailed {
        println!(
            "Author Name:       {}",
            act.metadata.author_name.as_ref().unwrap_or(&unknown)
        );
        println!(
            "Author Email:      {}",
            act.metadata.author_email.as_ref().unwrap_or(&unknown)
        );
        println!(
            "Links Text:        {}",
            act.metadata.links_text.as_ref().unwrap_or(&unknown)
        );
        println!(
            "Links Href:        {}",
            act.metadata.links_href.as_ref().unwrap_or(&unknown)
        );
        println!(
            "Keywords:          {}",
            act.metadata.keywords.as_ref().unwrap_or(&unknown)
        );
        println!(
            "Copyright Author:  {}",
            act.metadata.copyright_author.as_ref().unwrap_or(&unknown)
        );
        let year = act
            .metadata
            .time
            .as_ref()
            .unwrap_or(&TimeStamp(Local::now()))
            .0
            .year();
        println!(
            "Copyright Year:    {}",
            act.metadata.copyright_year.as_ref().unwrap_or(&year)
        );
        println!(
            "Copyright License: {}",
            act.metadata.copyright_license.as_ref().unwrap_or(&unknown)
        );
    }
}
