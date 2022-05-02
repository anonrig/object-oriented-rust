use crate::road_items::{SpeedLimitSign, StopSign};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::any::Any;

pub trait RoadItemTraits {
    fn get_mile_marker(&self) -> f64;
    fn set_mile_marker(&mut self, distance: f64);
    fn as_any(&self) -> &dyn Any;
}

impl Serialize for Box<dyn RoadItemTraits> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_map(Some(3))?;
        if let Some(speed_limit) = self.as_any().downcast_ref::<SpeedLimitSign>() {
            seq.serialize_entry("Type", "SpeedLimit");
            seq.serialize_entry("MileMarker", &speed_limit.get_mile_marker());
            seq.serialize_entry("SpeedLimit", &speed_limit.get_speed_limit());
        } else if let Some(stop_sign) = self.as_any().downcast_ref::<StopSign>() {
            seq.serialize_entry("Type", "StopSign");
            seq.serialize_entry("MileMarker", &self.get_mile_marker());
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for Box<dyn RoadItemTraits> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json = serde_json::value::Value::deserialize(deserializer)?;

        let row_type = json
            .get("Type")
            .expect("Type is required")
            .as_str()
            .unwrap();

        match row_type {
            "SpeedLimit" => Ok(Box::new(SpeedLimitSign::new(
                json.get("SpeedLimit")
                    .expect("SpeedLimit is required")
                    .as_f64()
                    .unwrap(),
                json.get("MileMarker")
                    .expect("MileMarker is required")
                    .as_f64()
                    .unwrap(),
            ))),
            "StopSign" => Ok(Box::new(StopSign::new(
                json.get("MileMarker")
                    .expect("MileMarker is required")
                    .as_f64()
                    .unwrap(),
            ))),
            _ => panic!("Invalid type"),
        }
    }
}
