//! Defines the type of GPS GPXfix used in the file.

use gpx;
use serde::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Type of the GPS GPXfix.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GPXFix {
    /// The GPS had no GPXfix. To signify "the GPXfix info is unknown", leave out the GPXFix entirely.
    None,
    /// 2D GPXfix gives only longitude and latitude. It needs a minimum of 3 satellites.
    TwoDimensional,
    /// 3D GPXfix gives longitude, latitude and altitude. It needs a minimum of 4 satellites.
    ThreeDimensional,
    /// Differential Global Positioning System.
    DGPS,
    /// Military signal.
    PPS,
    /// Other values that are not in the specification.
    Other(String),
}

impl GPXFix {
    fn from_gpx_fix(src: &gpx::Fix) -> Self {
        match src {
            gpx::Fix::None => GPXFix::None,
            gpx::Fix::TwoDimensional => GPXFix::TwoDimensional,
            gpx::Fix::ThreeDimensional => GPXFix::ThreeDimensional,
            gpx::Fix::DGPS => GPXFix::DGPS,
            gpx::Fix::PPS => GPXFix::PPS,
            gpx::Fix::Other(st) => GPXFix::Other(st.to_string()),
        }
    }
}

impl ToString for GPXFix {
    fn to_string(&self) -> String {
        match self {
            GPXFix::None => "None".to_string(),
            GPXFix::TwoDimensional => "TwoDimensional".to_string(),
            GPXFix::ThreeDimensional => "ThreeDimensional".to_string(),
            GPXFix::DGPS => "DGPS".to_string(),
            GPXFix::PPS => "PPS".to_string(),
            GPXFix::Other(_) => "Other".to_string(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_GPXfix() {
        let src_GPXfix = gpx::Fix::TwoDimensional;

        let dest_GPXfix = GPXFix::from_gpx_fix(&src_GPXfix);
        assert_eq!(dest_GPXfix, GPXFix::TwoDimensional);

        let dest_GPXfix_str = dest_GPXfix.to_string();
        assert_eq!("TwoDimensional".to_string(), dest_GPXfix_str);
    }
}
