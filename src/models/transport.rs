#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BikeAvailability {
    pub bikes: Option<i64>,
    pub county_code: Option<String>,
    pub docks: Option<i64>,
    pub empty_stations: Option<i64>,
    pub full_stations: Option<i64>,
    pub observed_at: Option<String>,
    pub stations: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BikeAvailabilityNow {
    pub bikes: Option<i64>,
    pub docks: Option<i64>,
    pub ebikes: Option<i64>,
    pub fetched_at: Option<String>,
    pub station_uid: Option<String>,
    pub status: Option<i64>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BikeAvailabilitySite {
    pub address: Option<String>,
    pub capacity: Option<i64>,
    pub city: Option<String>,
    pub county_code: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub name: Option<String>,
    pub service_type: Option<i64>,
    pub station_uid: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvLive {
    pub broken: Option<i64>,
    pub county_code: Option<String>,
    pub free: Option<i64>,
    pub in_use: Option<i64>,
    pub observed_at: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvLiveNow {
    pub broken: Option<i64>,
    pub fetched_at: Option<String>,
    pub free: Option<i64>,
    pub in_use: Option<i64>,
    pub station_id: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvLiveSite {
    pub ac_connectors: Option<i64>,
    pub charging_points: Option<i64>,
    pub charging_rate: Option<String>,
    pub connector_types: Option<String>,
    pub county_code: Option<String>,
    pub dc_connectors: Option<i64>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub name: Option<String>,
    pub operation_type: Option<i64>,
    pub parking_rate: Option<String>,
    pub service_time: Option<String>,
    pub spaces: Option<i64>,
    pub station_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Flight {
    pub actual_at: Option<String>,
    pub airline_id: Option<String>,
    pub airline_name: Option<String>,
    pub airport_id: Option<String>,
    pub cargo: Option<bool>,
    pub direction: Option<String>,
    pub estimated_at: Option<String>,
    pub flight_date: Option<String>,
    pub flight_no: Option<String>,
    pub last_seen_at: Option<String>,
    pub other_airport: Option<String>,
    pub remark: Option<String>,
    pub scheduled_at: Option<String>,
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FlightSite {
    pub airport_id: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub county_code: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FreewayTraffic {
    pub level: Option<i64>,
    pub observed_at: Option<String>,
    pub section_id: Option<String>,
    pub speed_kmh: Option<i64>,
    pub travel_time_s: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FreewayTrafficSite {
    pub county_code: Option<String>,
    pub direction: Option<String>,
    pub end_name: Option<String>,
    pub length_km: Option<f64>,
    pub name: Option<String>,
    pub road_name: Option<String>,
    pub section_id: Option<String>,
    pub speed_limit: Option<i64>,
    pub start_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FuelPrice {
    pub effective_at: Option<String>,
    pub price: Option<f64>,
    pub product: Option<String>,
    pub unit: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FuelStation {
    pub address: Option<String>,
    pub county_code: Option<String>,
    pub freeway: Option<bool>,
    pub has_98: Option<bool>,
    pub has_diesel: Option<bool>,
    pub kind: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub name: Option<String>,
    pub open_hours: Option<String>,
    pub operating: Option<bool>,
    pub self_service: Option<bool>,
    pub station_id: Option<String>,
    pub town: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RailAlert {
    pub alert_id: Option<String>,
    pub description: Option<String>,
    pub ends_at: Option<String>,
    pub first_seen_at: Option<String>,
    pub last_seen_at: Option<String>,
    pub level: Option<i64>,
    pub lines: Option<String>,
    pub operator: Option<String>,
    pub published_at: Option<String>,
    pub reason: Option<String>,
    pub starts_at: Option<String>,
    pub stations: Option<String>,
    pub status: Option<i64>,
    pub title: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TraLive {
    pub avg_delay_min: Option<f64>,
    pub delayed: Option<i64>,
    pub max_delay_min: Option<i64>,
    pub observed_at: Option<String>,
    pub running: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TraLiveNow {
    pub delay_min: Option<i64>,
    pub station_id: Option<String>,
    pub station_name: Option<String>,
    pub station_status: Option<i64>,
    pub train_no: Option<String>,
    pub train_type: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TraLiveSite {
    pub address: Option<String>,
    pub county_code: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub name: Option<String>,
    pub station_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrafficAccident {
    pub accident_id: Option<String>,
    pub accident_type: Option<String>,
    pub cause: Option<String>,
    pub county_code: Option<String>,
    pub deaths: Option<i64>,
    pub injuries: Option<i64>,
    pub lat: Option<f64>,
    pub light: Option<String>,
    pub lon: Option<f64>,
    pub occurred_at: Option<String>,
    pub place: Option<String>,
    pub police: Option<String>,
    pub road_class: Option<String>,
    pub speed_limit: Option<i64>,
    pub vehicle: Option<String>,
    pub weather: Option<String>,
}

pub const FREEWAY_TRAFFIC_ID: &str = "freeway_traffic";

pub type FreewayTrafficLatestResponse =
    super::common::LatestResponse<super::transport::FreewayTraffic>;

pub type FreewayTrafficHistoryResponse =
    super::common::HistoryResponse<super::transport::FreewayTraffic>;

pub type FreewayTrafficSitesResponse =
    super::common::SitesResponse<super::transport::FreewayTrafficSite>;

pub const FUEL_PRICE_ID: &str = "fuel_price";

pub type FuelPriceLatestResponse = super::common::LatestResponse<super::transport::FuelPrice>;

pub type FuelPriceHistoryResponse = super::common::HistoryResponse<super::transport::FuelPrice>;

pub const FUEL_STATION_ID: &str = "fuel_station";

pub type FuelStationLatestResponse = super::common::LatestResponse<super::transport::FuelStation>;

pub const RAIL_ALERT_ID: &str = "rail_alert";

pub type RailAlertLatestResponse = super::common::LatestResponse<super::transport::RailAlert>;

pub type RailAlertHistoryResponse = super::common::HistoryResponse<super::transport::RailAlert>;

pub const TRA_LIVE_ID: &str = "tra_live";

pub type TraLiveLatestResponse = super::common::LatestResponse<super::transport::TraLiveNow>;

pub type TraLiveHistoryResponse = super::common::HistoryResponse<super::transport::TraLive>;

pub type TraLiveSitesResponse = super::common::SitesResponse<super::transport::TraLiveSite>;

pub const BIKE_AVAILABILITY_ID: &str = "bike_availability";

pub type BikeAvailabilityLatestResponse =
    super::common::LatestResponse<super::transport::BikeAvailabilityNow>;

pub type BikeAvailabilityHistoryResponse =
    super::common::HistoryResponse<super::transport::BikeAvailability>;

pub type BikeAvailabilitySitesResponse =
    super::common::SitesResponse<super::transport::BikeAvailabilitySite>;

pub const EV_LIVE_ID: &str = "ev_live";

pub type EvLiveLatestResponse = super::common::LatestResponse<super::transport::EvLiveNow>;

pub type EvLiveHistoryResponse = super::common::HistoryResponse<super::transport::EvLive>;

pub type EvLiveSitesResponse = super::common::SitesResponse<super::transport::EvLiveSite>;

pub const FLIGHT_ID: &str = "flight";

pub type FlightLatestResponse = super::common::LatestResponse<super::transport::Flight>;

pub type FlightHistoryResponse = super::common::HistoryResponse<super::transport::Flight>;

pub type FlightSitesResponse = super::common::SitesResponse<super::transport::FlightSite>;

pub const TRAFFIC_ACCIDENT_ID: &str = "traffic_accident";

pub type TrafficAccidentLatestResponse =
    super::common::LatestResponse<super::transport::TrafficAccident>;

pub type TrafficAccidentHistoryResponse =
    super::common::HistoryResponse<super::transport::TrafficAccident>;
