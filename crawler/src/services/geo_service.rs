use crate::models::catchment_area::GeometryValue;
use crate::models::location::Location;
/// Provides geo-spatial related features
use geo::algorithm::contains::Contains;
use geo::{Coordinate, Point};
use geojson::{PolygonType, Position};

/// Determines if a given point is contained within a polygon. Supports :
/// Supports:
///    polygon with no internal ring
///    polygon with internal rings
///    multi polygons
///    multi polygons with internal rings
/// NOTE: Does not work when point is on the edge of the polygon. It muset be enclosed within the polygon
pub fn point_within_polygon(location: &Location, geometry_value: &GeometryValue) -> bool {
    let geo_point = Point::new(location.longitude, location.latitude);
    match geometry_value {
        GeometryValue::Polygon(polygon_type) => polygon_contains(&geo_point, polygon_type),
        GeometryValue::MultiPolygon(multi_polygon) => {
            multipolygon_contains(&geo_point, multi_polygon)
        }
    }
}

fn polygon_contains(point: &Point<f64>, polygon_type: &PolygonType) -> bool {
    return get_polygon_from_polygon_type(polygon_type).contains(point);
}

fn multipolygon_contains(point: &Point<f64>, multi_polygon: &Vec<PolygonType>) -> bool {
    return get_multipolygon_from_polygon_type(multi_polygon).contains(point);
}

pub fn get_polygon_from_polygon_type(polygon_type: &PolygonType) -> geo::Polygon<f64> {
    // PolygonType is a Vec<Vec<Vec<f64>>>
    let polygon = geo::Polygon::new(
        geo::LineString::from(get_linestring(&polygon_type[0])),
        get_internal_rings(polygon_type),
    );
    println!("{:?}", polygon);
    return polygon;
}

fn get_linestring(polygon_type: &[Position]) -> geo::LineString<f64> {
    return geo::LineString::from(
        polygon_type
            .iter()
            .flat_map(|positions| {
                vec![Coordinate::<f64> {
                    x: positions[0],
                    y: positions[1],
                }]
            })
            .collect::<Vec<Coordinate<f64>>>(),
    );
}

pub fn get_internal_rings(polygon_type: &PolygonType) -> Vec<geo::LineString<f64>> {
    if polygon_type.iter().count() == 1 {
        return vec![];
    }
    // internal rings are represented as Vec<Vec<Vec<f64>>>
    // fetch all elements from 2nd index to end
    println!("get_internal_rings : {}", polygon_type.iter().count());
    return polygon_type[1..]
        .iter()
        .map(|positions| get_linestring(positions))
        .collect();

    // TODO: Make this pattern matching work
    // match &polygon_type[..] {
    //     [_a] => {println!("first"); return vec![];},
    //     [_a, _b] => {println!("second"); return polygon_type[1..]
    //         .iter()
    //         .map(|positions| get_linestring(positions))
    //         .collect();},
    //     _ => {println!("third"); return vec![];},
    // }
}

fn get_multipolygon_from_polygon_type(
    multipolygon_type: &[PolygonType],
) -> geo::MultiPolygon<f64> {
    // Input is of type: // Vec<Vec<Vec<Vec<f64>>>>
    // Output: Vec<Polygon>

    return geo::MultiPolygon::from(
        multipolygon_type[..]
            .iter()
            .map(|polygon| get_polygon_from_polygon_type(polygon))
            .collect::<geo::MultiPolygon<f64>>(),
    );
}

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::catchment_area::GeometryValue;
    use crate::models::location::Location;
    use geojson::PolygonType;

    #[test]
    fn test_point_polygon_with_no_internal_rings() {
        let location = Location {
            longitude: 13.426008,
            latitude: 52.536780,
        };
        let location_outside = Location {
            longitude: 23.426008,
            latitude: 52.536780,
        };
        let positions = vec![
            vec![13.425781731803541, 52.534321477991661],
            vec![13.422081412762472, 52.535570557220687],
            vec![13.423027639945467, 52.537143026809822],
            vec![13.424210560058448, 52.53917959650277],
            vec![13.433057967203794, 52.53617852176172],
            vec![13.431807020385165, 52.535120557971489],
            vec![13.429434731727476, 52.533061419699571],
            vec![13.425781731803541, 52.534321477991661],
        ];
        let polygon = GeometryValue::Polygon(vec![positions]);

        assert_eq!(point_within_polygon(&location, &polygon), true);

        assert_eq!(point_within_polygon(&location_outside, &polygon), true);
    }

    #[test]
    fn test_point_polygon_with_internal_rings() {
        let location = Location {
            longitude: 13.426008,
            latitude: 52.536780,
        };
        let location_outside = Location {
            longitude: 23.426008,
            latitude: 52.536780,
        };
        let external_ring = vec![
            vec![13.425781731803541, 52.534321477991661],
            vec![13.422081412762472, 52.535570557220687],
            vec![13.423027639945467, 52.537143026809822],
            vec![13.424210560058448, 52.53917959650277],
            vec![13.433057967203794, 52.53617852176172],
            vec![13.431807020385165, 52.535120557971489],
            vec![13.429434731727476, 52.533061419699571],
            vec![13.425781731803541, 52.534321477991661],
        ];

        let internal_ring_1 = vec![
            vec![13.429219722747801, 52.53593369274464],
            vec![13.428919315338135, 52.535215831051765],
            vec![13.43003511428833, 52.53518972676906],
            vec![13.430378437042236, 52.535724861462676],
            vec![13.429219722747801, 52.53593369274464],
        ];
        let polygon_with_rings = GeometryValue::Polygon(vec![external_ring, internal_ring_1]);

        assert_eq!(point_within_polygon(&location, &polygon_with_rings), true);
        assert_eq!(
            point_within_polygon(&location_outside, &polygon_with_rings),
            false
        );
    }

    #[test]
    fn test_linestring_with_no_internal_ring() {
        let external_ring = vec![
            vec![13.425781731803541, 52.534321477991661],
            vec![13.422081412762472, 52.535570557220687],
            vec![13.423027639945467, 52.537143026809822],
            vec![13.424210560058448, 52.53917959650277],
            vec![13.433057967203794, 52.53617852176172],
            vec![13.431807020385165, 52.535120557971489],
            vec![13.429434731727476, 52.533061419699571],
            vec![13.425781731803541, 52.534321477991661],
        ];
        let por = vec![external_ring];
        let linestring = get_internal_rings(&por);
        assert_eq!(linestring.iter().count(), 0);
    }

    #[test]
    fn test_linestring_with_2_internal_rings() {
        let external_ring = vec![
            vec![13.425781731803541, 52.534321477991661],
            vec![13.422081412762472, 52.535570557220687],
            vec![13.423027639945467, 52.537143026809822],
            vec![13.424210560058448, 52.53917959650277],
            vec![13.433057967203794, 52.53617852176172],
            vec![13.431807020385165, 52.535120557971489],
            vec![13.429434731727476, 52.533061419699571],
            vec![13.425781731803541, 52.534321477991661],
        ];

        let internal_ring_1 = vec![
            vec![13.429219722747801, 52.53593369274464],
            vec![13.428919315338135, 52.535215831051765],
            vec![13.43003511428833, 52.53518972676906],
            vec![13.430378437042236, 52.535724861462676],
            vec![13.429219722747801, 52.53593369274464],
        ];

        let internal_ring_2 = vec![
            vec![23.429219722747801, 62.53593369274464],
            vec![23.428919315338135, 62.535215831051765],
            vec![23.43003511428833, 62.53518972676906],
            vec![23.430378437042236, 62.535724861462676],
            vec![23.429219722747801, 62.53593369274464],
        ];

        let por = vec![external_ring, internal_ring_1, internal_ring_2];
        let linestring = get_internal_rings(&por);
        assert_eq!(linestring.iter().count(), 2);
    }

    #[test]
    fn test_point_multipolygon_with_no_rings() {
        let location = Location {
            longitude: 13.426008,
            latitude: 52.536780,
        };
        let location_outside = Location {
            longitude: 23.426008,
            latitude: 52.536780,
        };
        let point_outside = vec![23.426008, 52.536780];

        let polygon1 = vec![
            vec![13.425781731803541, 52.534321477991661],
            vec![13.422081412762472, 52.535570557220687],
            vec![13.423027639945467, 52.537143026809822],
            vec![13.424210560058448, 52.53917959650277],
            vec![13.433057967203794, 52.53617852176172],
            vec![13.431807020385165, 52.535120557971489],
            vec![13.429434731727476, 52.533061419699571],
            vec![13.425781731803541, 52.534321477991661],
        ];
        let polygon2 = vec![
            vec![23.425781731803541, 62.534321477991661],
            vec![23.422081412762472, 62.535570557220687],
            vec![23.423027639945467, 62.537143026809822],
            vec![23.424210560058448, 62.53917959650277],
            vec![23.433057967203794, 62.53617852176172],
            vec![23.431807020385165, 62.535120557971489],
            vec![23.429434731727476, 62.533061419699571],
            vec![23.425781731803541, 62.534321477991661],
        ];
        let multi_polygon = GeometryValue::MultiPolygon(vec![vec![polygon1], vec![polygon2]]);

        assert_eq!(point_within_polygon(&location, &multi_polygon), true);
        assert_eq!(
            point_within_polygon(&location_outside, &multi_polygon),
            false
        );
    }

    #[test]
    fn test_point_multipolygon_with_rings() {
        let location = Location {
            longitude: 13.426008,
            latitude: 52.536780,
        };
        let location_outside = Location {
            longitude: 23.426008,
            latitude: 52.536780,
        };

        let external_ring = vec![
            vec![13.425781731803541, 52.534321477991661],
            vec![13.422081412762472, 52.535570557220687],
            vec![13.423027639945467, 52.537143026809822],
            vec![13.424210560058448, 52.53917959650277],
            vec![13.433057967203794, 52.53617852176172],
            vec![13.431807020385165, 52.535120557971489],
            vec![13.429434731727476, 52.533061419699571],
            vec![13.425781731803541, 52.534321477991661],
        ];

        let internal_ring_1 = vec![
            vec![13.429219722747801, 52.53593369274464],
            vec![13.428919315338135, 52.535215831051765],
            vec![13.43003511428833, 52.53518972676906],
            vec![13.430378437042236, 52.535724861462676],
            vec![13.429219722747801, 52.53593369274464],
        ];
        let polygon1 = vec![external_ring, internal_ring_1];

        let polygon2 = vec![
            vec![23.425781731803541, 62.534321477991661],
            vec![23.422081412762472, 62.535570557220687],
            vec![23.423027639945467, 62.537143026809822],
            vec![23.424210560058448, 62.53917959650277],
            vec![23.433057967203794, 62.53617852176172],
            vec![23.431807020385165, 62.535120557971489],
            vec![23.429434731727476, 62.533061419699571],
            vec![23.425781731803541, 62.534321477991661],
        ];
        let multi_polygon = GeometryValue::MultiPolygon(vec![polygon1, vec![polygon2]]);

        assert_eq!(point_within_polygon(&location, &multi_polygon), true);
        assert_eq!(
            point_within_polygon(&location_outside, &multi_polygon),
            false
        );
    }
}
//#endregion Tests
