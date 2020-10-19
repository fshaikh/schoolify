use crate::models::catchment_area::GeometryValue;

fn getlinestring(polygon: &Vec<Vec<f64>>) -> String {
    let mut index = 0;
    polygon
        .iter()
        .flat_map(|positions| {
            if index + 1 == polygon.len() {
                return vec![format!("{} {}", positions[0], positions[1])];
            } else {
                index = index + 1;
                return vec![format!("{} {}, ", positions[0], positions[1])];
            }
        })
        .collect()
}

// ST_GeomFromText('POLYGON( (), () )')
fn to_polygon_string(polygon: &Vec<Vec<Vec<f64>>>) -> String {
    let mut polygonconstant = "POLYGON(".to_string();
    polygon
        .iter()
        .flat_map(|positions| vec![getlinestring(&positions)])
        .for_each(|encoded| {
            let aa = format!("({})", encoded);
            polygonconstant.push_str(&aa);
        });
    polygonconstant.push_str(")");
    return polygonconstant;
}
// MULTIPOLYGON( ((), ()), ((), ()) )
fn to_polygon_string_in_multipolygon(polygon: &Vec<Vec<Vec<f64>>>) -> String {
    let mut polygoncontatnt = "(".to_string();
    polygon
        .iter()
        .flat_map(|positions| vec![getlinestring(&positions)])
        .for_each(|encoded| {
            let aa = format!("({})", encoded);
            polygoncontatnt.push_str(&aa);
        });
    polygoncontatnt.push_str(")");
    return polygoncontatnt;
}

// MULTIPOLYGON( ((), ()), ((),()) )
fn to_multipolygon_string(multipolygon: &Vec<Vec<Vec<Vec<f64>>>>) -> String {
    let multipolygon_string_prefix = "MULTIPOLYGON(".to_string();
    let polygon_string_vector: Vec<String> = multipolygon
        .iter()
        .flat_map(|positions| vec![to_polygon_string_in_multipolygon(&positions)])
        .collect();
    let length = polygon_string_vector.len();
    let mut multipolygon_string = polygon_string_vector.iter().enumerate().fold(
        multipolygon_string_prefix,
        |mut acc, (index, polygon_str)| {
            acc.push_str(&polygon_str);
            if index + 1 < length {
                acc.push_str(",");
            }
            return acc;
        },
    );
    multipolygon_string.push_str(")");
    return multipolygon_string;
}

pub fn tosql_polygon_geometry(geometry: &GeometryValue) -> String {
    match geometry {
        GeometryValue::Polygon(polygon) => to_polygon_string(&polygon),
        GeometryValue::MultiPolygon(multi_polygon) => to_multipolygon_string(&multi_polygon),
        _ => panic!("found others"),
    }
}
