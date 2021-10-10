use crate::types::duration::Duration;

use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Detailed information about how much time is spent in each heart rate zone.
///
/// The actual zones are defined as the training levels based on your maximum heart rate, which is usually calculated
/// as 220 - your age in years.
///
/// **HR Zones:**
///
///    **0**: Warmup<br>
///    **1**: Fat Burn<br>
///    **2**: Aerobic<br>
///    **3**: Anaerobic<br>
///    **4**: Speed/Power<br>
///
/// **Reference:**
///
///    <https://www.heart.org/en/healthy-living/fitness/fitness-basics/target-heart-rates>
#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(default)]
pub struct HrZones {
    pub hr_zone_0: Option<Duration>,
    pub hr_zone_1: Option<Duration>,
    pub hr_zone_2: Option<Duration>,
    pub hr_zone_3: Option<Duration>,
    pub hr_zone_4: Option<Duration>,
}

impl HrZones {
    /// Initialize HrZones with default empty values
    pub fn new() -> Self {
        HrZones::default()
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// Initialize HrZones from a HrZone array from the fitparser
    ///
    /// **Parameters:**
    ///
    ///    `src: Option<&&fitparser::Value>` -- A fitparser array value containing the HR Zone information: <https://docs.rs/fitparser/0.4.0/fitparser/enum.Value.html>
    ///
    /// **Returns:**
    ///
    ///   `Self` -- Returns a new HrZones struct, which may or may not have values in its elements.
    ///
    /// **Example:**
    ///
    ///   ```rust
    ///   lap.time_in_hr_zones = HzZones::from(field_map.get("time_in_hr_zone"));
    ///   ```
    ///
    pub fn from(src: Option<&&fitparser::Value>) -> Self {
        let mut hr_zones = HrZones::new();

        if let Some(fitparser::Value::Array(tihz_vec)) = src {
            // Array[UInt32(23372), UInt32(31681), UInt32(32669), UInt32(447453), UInt32(1394934)]
            // FIXME: There HAS to be a better way to get the value
            let t2: Vec<Duration> = tihz_vec
                .iter()
                .map(|x| x.to_string().parse::<u64>().unwrap())
                .map(Duration::from_millis_u64)
                .collect();

            hr_zones.hr_zone_0 = Some(t2[0]);
            hr_zones.hr_zone_1 = Some(t2[1]);
            hr_zones.hr_zone_2 = Some(t2[2]);
            hr_zones.hr_zone_3 = Some(t2[3]);
            hr_zones.hr_zone_4 = Some(t2[4]);
        }

        // return it
        hr_zones
    }
}

impl Default for HrZones {
    /// Set defaults to be all `None`.
    fn default() -> Self {
        HrZones {
            hr_zone_0: None,
            hr_zone_1: None,
            hr_zone_2: None,
            hr_zone_3: None,
            hr_zone_4: None,
        }
    }
}

impl Serialize for HrZones {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("HrZones", 5)?;
        let dur_zero = Duration::from_millis_u64(0);
        state.serialize_field(
            "hr_zone_0_secs",
            &self.hr_zone_0.unwrap_or(dur_zero).0.as_secs_f32(),
        )?;
        state.serialize_field(
            "hr_zone_1_secs",
            &self.hr_zone_1.unwrap_or(dur_zero).0.as_secs_f32(),
        )?;
        state.serialize_field(
            "hr_zone_2_secs",
            &self.hr_zone_2.unwrap_or(dur_zero).0.as_secs_f32(),
        )?;
        state.serialize_field(
            "hr_zone_3_secs",
            &self.hr_zone_3.unwrap_or(dur_zero).0.as_secs_f32(),
        )?;
        state.serialize_field(
            "hr_zone_4_secs",
            &self.hr_zone_4.unwrap_or(dur_zero).0.as_secs_f32(),
        )?;
        state.end()
    }
}
