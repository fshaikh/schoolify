use geojson::PolygonType;

use crate::models::object_base::ObjectBase;

pub enum GeometryValue {
    Polygon(PolygonType),
    MultiPolygon(Vec<PolygonType>),
}

/// Data structure representing a catchment area
pub struct CatchmentArea {
    pub meta: ObjectBase,
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
