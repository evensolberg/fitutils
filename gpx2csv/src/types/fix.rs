use gpx;
use serde::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Type of the GPS fix.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Fix {
    /// The GPS had no fix. To signify "the fix info is unknown", leave out the Fix entirely.
    None,
    /// 2D fix gives only longitude and latitude. It needs a minimum of 3 satellites.
    TwoDimensional,
    /// 3D fix gives longitude, latitude and altitude. It needs a minimum of 4 satellites.
    ThreeDimensional,
    /// Differential Global Positioning System.
    DGPS,
    /// Military signal.
    PPS,
    /// Other values that are not in the specification.
    Other(String),
}

impl Fix {
    fn from_gpx_fix(src: &gpx::Fix) -> Self {
        match src {
            gpx::Fix::None => Fix::None,
            gpx::Fix::TwoDimensional => Fix::TwoDimensional,
            gpx::Fix::ThreeDimensional => Fix::ThreeDimensional,
            gpx::Fix::DGPS => Fix::DGPS,
            gpx::Fix::PPS => Fix::PPS,
            gpx::Fix::Other(st) => Fix::Other(st.to_string()),
        }
    }
}

impl ToString for Fix {
    fn to_string(&self) -> String {
        match self {
            Fix::None => "None".to_string(),
            Fix::TwoDimensional => "TwoDimensional".to_string(),
            Fix::ThreeDimensional => "ThreeDimensional".to_string(),
            Fix::DGPS => "DGPS".to_string(),
            Fix::PPS => "PPS".to_string(),
            Fix::Other(_) => "Other".to_string(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix() {
        let src_fix = gpx::Fix::TwoDimensional;

        let dest_fix = Fix::from_gpx_fix(&src_fix);
        assert_eq!(dest_fix, Fix::TwoDimensional);

        let dest_fix_str = Fix::to_string(&dest_fix);
        assert_eq!("TwoDimensional".to_string(), dest_fix_str);
    }
}
