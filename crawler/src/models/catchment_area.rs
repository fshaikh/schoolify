use crate::models::object_base::ObjectBase;
use geojson::PolygonType;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub enum GeometryValue {
    Polygon(PolygonType),
    MultiPolygon(Vec<PolygonType>),
}

/// Data structure representing a catchment area
#[derive(Serialize, Deserialize)]
pub struct CatchmentArea {
    pub meta: ObjectBase,
    pub region_id: String,
    /// Catchment area Id provided by the data source
    pub area_key: String,
    /// District key of the catchment area
    pub district_key: String,
    /// District name of the catchment area
    pub district_name: String,
    /// Geometry of the catchment area in the form of array of lat/lon.
    /// We can standardize this to a MultiPolygon data structure
    pub geometry: GeometryValue,
}

impl Default for CatchmentArea {
    fn default() -> Self {
        CatchmentArea {
            meta: Default::default(),
            region_id: "".to_string(),
            area_key: "".to_string(),
            district_key: "".to_string(),
            district_name: "".to_string(),
            geometry: GeometryValue::Polygon(vec![]),
        }
    }
}

impl fmt::Debug for CatchmentArea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CatchmentArea")
            .field("meta", &self.meta)
            .field("Region Id", &self.region_id)
            .field("Area Key", &self.area_key)
            .field("District Key", &self.district_key)
            .field("District Name", &self.district_name)
            .finish()
    }
}
