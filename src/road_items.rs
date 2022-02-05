use crate::traits::{DynamicRoadItemTraits, StaticRoadItemTraits};

pub struct StopSign {}
impl StaticRoadItemTraits for StopSign {}

pub struct IntersectionSign {}
impl StaticRoadItemTraits for IntersectionSign {}

pub struct SpeedLimitSign {}
impl StaticRoadItemTraits for SpeedLimitSign {}

pub struct YieldSign {}
impl StaticRoadItemTraits for YieldSign {}

pub struct TrafficLight {}
impl DynamicRoadItemTraits for TrafficLight {}
