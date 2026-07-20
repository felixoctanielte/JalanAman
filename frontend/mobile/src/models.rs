use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct GeoPoint {
    pub(crate) lat: f64,
    pub(crate) lng: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct LocationEval {
    pub(crate) lat: Option<f64>,
    pub(crate) lng: Option<f64>,
    pub(crate) error: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct MapSelectionEval {
    pub(crate) lat: f64,
    pub(crate) lng: f64,
}
