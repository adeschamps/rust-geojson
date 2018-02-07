#![feature(concat_idents)]
#![feature(test)]

extern crate geojson;
extern crate serde_json;
extern crate test;

#[cfg(test)]
mod decode_benchmarks {
    use std::fs::File;
    use std::io::prelude::*;
    use geojson::GeoJson;
    use test::Bencher;

    macro_rules! decode_test {
        ($name:ident: $file_name:expr) => {
            #[bench]
            fn $name(b: &mut Bencher) {
                let fixture_dir_path = "tests/fixtures/canonical/";
                let mut file_path = fixture_dir_path.to_owned();
                file_path.push_str($file_name.to_owned().as_str());

                bench_decode(b, &file_path);
            }
        }
    }

    macro_rules! decode_tests {
        ( $($name:ident: $file_name:expr,)* ) => {
            $(
                decode_test!($name: $file_name);
             )*
        }
    }

    decode_tests! {
        bench_decode_good_feature_with_id: "good-feature-with-id.geojson",
        bench_decode_good_feature_with_string_id: "good-feature-with-string-id.geojson",
        bench_decode_good_feature: "good-feature.geojson",
        bench_decode_good_feature_collection_bbox: "good-featurecollection-bbox.geojson",
        bench_decode_good_feature_collection_bbox3d: "good-featurecollection-bbox3d.geojson",
        bench_decode_good_feature_collection_extensions: "good-featurecollection-extensions.geojson",
        bench_decode_good_feature_collection: "good-featurecollection.geojson",
        bench_decode_good_geometry_collection: "good-geometrycollection.geojson",
        bench_decode_good_linestring: "good-linestring.geojson",
        bench_decode_good_multilinestring: "good-multilinestring.geojson",
        bench_decode_good_multipoint: "good-multipoint.geojson",
        bench_decode_good_point_3d: "good-point-3d.geojson",
        bench_decode_good_point: "good-point.geojson",
        bench_decode_good_polygon: "good-polygon.geojson",
        bench_decode_multipolygon: "multipolygon.geojson",
        bench_decode_null_geometry: "nullgeometry.geojson",
    }

    fn bench_decode(b: &mut Bencher, file_path: &str) {
        let mut file = File::open(&file_path).unwrap();
        let mut file_contents = String::new();
        let _ = file.read_to_string(&mut file_contents);

        b.iter(|| file_contents.parse::<GeoJson>().expect("unable to parse"));
    }
}
