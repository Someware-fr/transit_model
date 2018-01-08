extern crate csv;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate get_corresponding_derive;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate chrono;

pub mod collection;
pub mod objects;
pub mod relations;
pub mod ntfs;
pub(crate) mod utils;
pub mod gtfs;

use std::ops;

use std::collections::HashMap;
use collection::{Collection, Idx};
use objects::*;
use relations::{IdxSet, ManyToMany, OneToMany, Relation};

#[derive(Derivative, Serialize, Deserialize, Debug)]
#[derivative(Default)]
pub struct Collections {
    pub contributors: Collection<Contributor>,
    pub datasets: Collection<Dataset>,
    pub networks: Collection<Network>,
    pub commercial_modes: Collection<CommercialMode>,
    pub lines: Collection<Line>,
    pub routes: Collection<Route>,
    pub vehicle_journeys: Collection<VehicleJourney>,
    pub physical_modes: Collection<PhysicalMode>,
    pub stop_areas: Collection<StopArea>,
    pub stop_points: Collection<StopPoint>,
    pub feed_infos: HashMap<String, String>,
    pub calendars: Collection<Calendar>,
    pub companies: Collection<Company>,
}

#[derive(GetCorresponding)]
pub struct PtObjects {
    collections: Collections,

    // original relations
    network_to_lines: OneToMany<Network, Line>,
    commercial_modes_to_lines: OneToMany<CommercialMode, Line>,
    lines_to_routes: OneToMany<Line, Route>,
    routes_to_vehicle_journeys: OneToMany<Route, VehicleJourney>,
    physical_modes_to_vehicle_journeys: OneToMany<PhysicalMode, VehicleJourney>,
    stop_areas_to_stop_points: OneToMany<StopArea, StopPoint>,
    contributors_to_datasets: OneToMany<Contributor, Dataset>,
    datasets_to_vehicle_journeys: OneToMany<Dataset, VehicleJourney>,
    vehicle_journeys_to_stop_points: ManyToMany<VehicleJourney, StopPoint>,

    // shortcuts
    #[get_corresponding(weight = "1.9")] routes_to_stop_points: ManyToMany<Route, StopPoint>,
    #[get_corresponding(weight = "1.9")]
    physical_modes_to_stop_points: ManyToMany<PhysicalMode, StopPoint>,
    #[get_corresponding(weight = "1.9")] physical_modes_to_routes: ManyToMany<PhysicalMode, Route>,
    #[get_corresponding(weight = "1.9")] datasets_to_stop_points: ManyToMany<Dataset, StopPoint>,
    #[get_corresponding(weight = "1.9")] datasets_to_routes: ManyToMany<Dataset, Route>,
    #[get_corresponding(weight = "1.9")]
    datasets_to_physical_modes: ManyToMany<Dataset, PhysicalMode>,
}
impl PtObjects {
    pub fn new(c: Collections) -> Self {
        let forward_vj_to_sp = c.vehicle_journeys
            .iter()
            .map(|(idx, vj)| {
                (
                    idx,
                    vj.stop_times.iter().map(|st| st.stop_point_idx).collect(),
                )
            })
            .collect();
        let vehicle_journeys_to_stop_points = ManyToMany::from_forward(forward_vj_to_sp);
        let routes_to_vehicle_journeys = OneToMany::new(&c.routes, &c.vehicle_journeys);
        let physical_modes_to_vehicle_journeys =
            OneToMany::new(&c.physical_modes, &c.vehicle_journeys);
        let datasets_to_vehicle_journeys = OneToMany::new(&c.datasets, &c.vehicle_journeys);
        PtObjects {
            routes_to_stop_points: ManyToMany::from_relations_chain(
                &routes_to_vehicle_journeys,
                &vehicle_journeys_to_stop_points,
            ),
            physical_modes_to_stop_points: ManyToMany::from_relations_chain(
                &physical_modes_to_vehicle_journeys,
                &vehicle_journeys_to_stop_points,
            ),
            physical_modes_to_routes: ManyToMany::from_relations_sink(
                &physical_modes_to_vehicle_journeys,
                &routes_to_vehicle_journeys,
            ),
            datasets_to_stop_points: ManyToMany::from_relations_chain(
                &datasets_to_vehicle_journeys,
                &vehicle_journeys_to_stop_points,
            ),
            datasets_to_routes: ManyToMany::from_relations_sink(
                &datasets_to_vehicle_journeys,
                &routes_to_vehicle_journeys,
            ),
            datasets_to_physical_modes: ManyToMany::from_relations_sink(
                &datasets_to_vehicle_journeys,
                &physical_modes_to_vehicle_journeys,
            ),
            network_to_lines: OneToMany::new(&c.networks, &c.lines),
            commercial_modes_to_lines: OneToMany::new(&c.commercial_modes, &c.lines),
            lines_to_routes: OneToMany::new(&c.lines, &c.routes),
            routes_to_vehicle_journeys: routes_to_vehicle_journeys,
            physical_modes_to_vehicle_journeys: physical_modes_to_vehicle_journeys,
            stop_areas_to_stop_points: OneToMany::new(&c.stop_areas, &c.stop_points),
            contributors_to_datasets: OneToMany::new(&c.contributors, &c.datasets),
            datasets_to_vehicle_journeys: datasets_to_vehicle_journeys,
            vehicle_journeys_to_stop_points: vehicle_journeys_to_stop_points,
            collections: c,
        }
    }
}
impl ::serde::Serialize for PtObjects {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.collections.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for PtObjects {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::serde::Deserialize::deserialize(deserializer).map(PtObjects::new)
    }
}
impl ops::Deref for PtObjects {
    type Target = Collections;
    fn deref(&self) -> &Self::Target {
        &self.collections
    }
}
