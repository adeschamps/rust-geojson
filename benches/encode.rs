#![feature(concat_idents)]
#![feature(test)]

extern crate geojson;
extern crate serde_json;
extern crate test;

#[cfg(test)]
mod encode_benchmarks {
    use std::fs::File;
    use std::io::prelude::*;
    use geojson::GeoJson;
    use test::Bencher;

    macro_rules! encode_test {
        ($name:ident: $file_name:expr) => {
            #[bench]
            fn $name(b: &mut Bencher) {
                let fixture_dir_path = "tests/fixtures/canonical/";
                let mut file_path = fixture_dir_path.to_owned();
                file_path.push_str($file_name.to_owned().as_str());

                bench_encode(b, &file_path);
            }
        }
    }

    macro_rules! encode_tests {
        ( $($name:ident: $file_name:expr,)* ) => {
            $(
                encode_test!($name: $file_name);
             )*
        }
    }

    encode_tests! {
        bench_encode_good_feature_with_id: "good-feature-with-id.geojson",
        bench_encode_good_feature_with_string_id: "good-feature-with-string-id.geojson",
        bench_encode_good_feature: "good-feature.geojson",
        bench_encode_good_feature_collection_bbox: "good-featurecollection-bbox.geojson",
        bench_encode_good_feature_collection_bbox3d: "good-featurecollection-bbox3d.geojson",
        bench_encode_good_feature_collection_extensions: "good-featurecollection-extensions.geojson",
        bench_encode_good_feature_collection: "good-featurecollection.geojson",
        bench_encode_good_geometry_collection: "good-geometrycollection.geojson",
        bench_encode_good_linestring: "good-linestring.geojson",
        bench_encode_good_multilinestring: "good-multilinestring.geojson",
        bench_encode_good_multipoint: "good-multipoint.geojson",
        bench_encode_good_point_3d: "good-point-3d.geojson",
        bench_encode_good_point: "good-point.geojson",
        bench_encode_good_polygon: "good-polygon.geojson",
        bench_encode_multipolygon: "multipolygon.geojson",
        bench_encode_null_geometry: "nullgeometry.geojson",
    }

    fn bench_encode(b: &mut Bencher, file_path: &str) {
        let mut file = File::open(&file_path).unwrap();
        let mut file_contents = String::new();
        let _ = file.read_to_string(&mut file_contents);

        // Read and parse the geojson from the file's contents
        let geojson = file_contents.parse::<GeoJson>().expect("unable to parse");

        // Now that we've successfully decoded the geojson, re-encode it and compare to the
        // original to make sure nothing was lost.
        // let geojson_string = geojson.to_string();
        b.iter(|| geojson.to_string());

        // let original_json: serde_json::Value = serde_json::from_str(&file_contents).unwrap();
        // let roundtrip_json: serde_json::Value = serde_json::from_str(&geojson_string).unwrap();

        // assert_eq!(original_json, roundtrip_json)
    }
}
