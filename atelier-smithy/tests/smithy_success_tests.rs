use atelier_core::io::read_model_from_file;
use atelier_core::model::shapes::{HasTraits, ShapeKind};
use atelier_core::model::{HasIdentity, ShapeID};
use atelier_core::syntax::{
    SHAPE_APPLY, SHAPE_LIST, SHAPE_MAP, SHAPE_OPERATION, SHAPE_RESOURCE, SHAPE_SERVICE, SHAPE_SET,
    SHAPE_STRUCTURE, SHAPE_UNION,
};
use atelier_smithy::SmithyReader;
use std::path::PathBuf;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Test Cases
// ------------------------------------------------------------------------------------------------

#[test]
fn test_weather_example() {
    test_file_parses("weather");
}

#[test]
fn test_smithy_prelude() {
    test_file_parses("prelude-traits");
}

#[test]
fn test_waiters_example() {
    test_file_parses("waiters");
}

#[test]
fn test_mqtt_api_example() {
    test_file_parses("smithy-api-mqtt");
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn test_file_parses(file_name: &str) {
    let mut path = PathBuf::from_str(MANIFEST_DIR).unwrap();
    path.push(format!("tests/good/{}.smithy", file_name));
    println!("{:?}", path);

    let mut reader = SmithyReader::default();
    let result = read_model_from_file(&mut reader, path);

    let trait_trait = ShapeID::from_str("smithy.api#trait").unwrap();
    match result {
        Ok(parsed) => {
            let mut names = parsed
                .shapes()
                .map(|shape| {
                    format!(
                        "{:<32} -> {}{}",
                        shape.id(),
                        if shape.has_trait(&trait_trait) {
                            "trait "
                        } else {
                            ""
                        },
                        match shape.body() {
                            ShapeKind::Simple(v) => v.to_string(),
                            ShapeKind::List(_) => SHAPE_LIST.to_string(),
                            ShapeKind::Set(_) => SHAPE_SET.to_string(),
                            ShapeKind::Map(_) => SHAPE_MAP.to_string(),
                            ShapeKind::Structure(_) => SHAPE_STRUCTURE.to_string(),
                            ShapeKind::Union(_) => SHAPE_UNION.to_string(),
                            ShapeKind::Service(_) => SHAPE_SERVICE.to_string(),
                            ShapeKind::Operation(_) => SHAPE_OPERATION.to_string(),
                            ShapeKind::Resource(_) => SHAPE_RESOURCE.to_string(),
                            ShapeKind::Unresolved => SHAPE_APPLY.to_string(),
                        }
                    )
                })
                .collect::<Vec<String>>();
            names.sort();
            print!("{:#?}", names)
        }
        Err(err) => panic!("{}", err.to_string()),
    }
}
