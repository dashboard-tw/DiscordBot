#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CloudLayer {
    pub east: Option<f64>,
    pub height: Option<i64>,
    pub north: Option<f64>,
    pub observed_at: Option<String>,
    pub south: Option<f64>,
    pub url: Option<String>,
    pub west: Option<f64>,
    pub width: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CountyForecast {
    pub comfort: Option<String>,
    pub county_code: Option<String>,
    pub ends_at: Option<String>,
    pub max_temp_c: Option<i64>,
    pub min_temp_c: Option<i64>,
    pub rain_chance_pct: Option<i64>,
    pub starts_at: Option<String>,
    pub weather: Option<String>,
    pub weather_code: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DebrisAlert {
    pub alert_id: Option<String>,
    pub alert_type: Option<String>,
    pub county_code: Option<String>,
    pub first_seen_at: Option<String>,
    pub last_seen_at: Option<String>,
    pub level: Option<String>,
    pub name: Option<String>,
    pub town: Option<String>,
    pub updated_at: Option<String>,
    pub village: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DisasterAlert {
    pub alert_id: Option<String>,
    pub color: Option<String>,
    pub county_codes: Option<String>,
    pub description: Option<String>,
    pub effective_at: Option<String>,
    pub expires_at: Option<String>,
    pub first_seen_at: Option<String>,
    pub headline: Option<String>,
    pub kind: Option<String>,
    pub last_seen_at: Option<String>,
    pub lat: Option<f64>,
    pub level: Option<String>,
    pub lon: Option<f64>,
    pub msg_type: Option<String>,
    pub sender: Option<String>,
    pub sent_at: Option<String>,
    pub supersedes: Option<String>,
    pub url: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Earthquake {
    pub content: Option<String>,
    pub county_intensities: Option<String>,
    pub depth_km: Option<f64>,
    pub earthquake_no: Option<i64>,
    pub image_url: Option<String>,
    pub issued_at: Option<String>,
    pub lat: Option<f64>,
    pub location: Option<String>,
    pub lon: Option<f64>,
    pub magnitude: Option<f64>,
    pub max_intensity: Option<String>,
    pub origin_time: Option<String>,
    pub report_color: Option<String>,
    pub report_id: Option<String>,
    pub report_url: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EarthquakeLocal {
    pub content: Option<String>,
    pub county_intensities: Option<String>,
    pub depth_km: Option<f64>,
    pub earthquake_no: Option<i64>,
    pub image_url: Option<String>,
    pub issued_at: Option<String>,
    pub lat: Option<f64>,
    pub location: Option<String>,
    pub lon: Option<f64>,
    pub magnitude: Option<f64>,
    pub max_intensity: Option<String>,
    pub origin_time: Option<String>,
    pub report_color: Option<String>,
    pub report_id: Option<String>,
    pub report_url: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FireIncident {
    pub arrived_at: Option<String>,
    pub case_id: Option<String>,
    pub cause: Option<String>,
    pub county_code: Option<String>,
    pub deaths: Option<i64>,
    pub injuries: Option<i64>,
    pub reported_at: Option<String>,
    pub town: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FloodSensor {
    pub depth_cm: Option<f64>,
    pub observed_at: Option<String>,
    pub station_code: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FloodSensorSite {
    pub authority: Option<String>,
    pub county_code: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub name: Option<String>,
    pub station_code: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RadarImage {
    pub height: Option<i64>,
    pub observed_at: Option<String>,
    pub url: Option<String>,
    pub width: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RadarLayer {
    pub east: Option<f64>,
    pub height: Option<i64>,
    pub north: Option<f64>,
    pub observed_at: Option<String>,
    pub south: Option<f64>,
    pub url: Option<String>,
    pub west: Option<f64>,
    pub width: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RainObs {
    pub observed_at: Option<String>,
    pub past10min_mm: Option<f64>,
    pub past12hr_mm: Option<f64>,
    pub past1hr_mm: Option<f64>,
    pub past24hr_mm: Option<f64>,
    pub past2days_mm: Option<f64>,
    pub past3days_mm: Option<f64>,
    pub past3hr_mm: Option<f64>,
    pub past6hr_mm: Option<f64>,
    pub station_id: Option<String>,
    pub today_mm: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RainObsSite {
    pub altitude_m: Option<f64>,
    pub county_code: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub maintainer: Option<String>,
    pub name: Option<String>,
    pub station_id: Option<String>,
    pub town: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SatelliteImage {
    pub height: Option<i64>,
    pub observed_at: Option<String>,
    pub url: Option<String>,
    pub width: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Shelter {
    pub address: Option<String>,
    pub code: Option<String>,
    pub county_code: Option<String>,
    pub disaster_types: Option<String>,
    pub for_weaker: Option<bool>,
    pub indoor: Option<bool>,
    pub is_open: Option<bool>,
    pub last_seen_at: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub name: Option<String>,
    pub outdoor: Option<bool>,
    pub shelter_id: Option<String>,
    pub town: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShelterReport {
    pub capacity: Option<i64>,
    pub closed_at: Option<String>,
    pub county_code: Option<String>,
    pub first_seen_at: Option<String>,
    pub last_seen_at: Option<String>,
    pub name: Option<String>,
    pub opened_at: Option<String>,
    pub project: Option<String>,
    pub refugees: Option<i64>,
    pub refugees_total: Option<i64>,
    pub reported_at: Option<String>,
    pub shelter_id: Option<String>,
    pub town: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TyphoonTrack {
    pub forecast_hour: Option<i64>,
    pub issued_at: Option<String>,
    pub kind: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub max_gust_ms: Option<i64>,
    pub max_wind_ms: Option<i64>,
    pub moving_dir: Option<String>,
    pub moving_speed_kmh: Option<i64>,
    pub name_en: Option<String>,
    pub name_zh: Option<String>,
    pub note: Option<String>,
    pub pressure_hpa: Option<i64>,
    pub radius10_km: Option<i64>,
    pub radius70_km: Option<i64>,
    pub radius7_km: Option<i64>,
    pub ty_no: Option<i64>,
    pub typhoon_id: Option<String>,
    pub valid_at: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TyphoonWarning {
    pub alert: Option<String>,
    pub areas: Option<String>,
    pub effective_at: Option<String>,
    pub expires_at: Option<String>,
    pub name_en: Option<String>,
    pub name_zh: Option<String>,
    pub sections: Option<String>,
    pub ty_no: Option<i64>,
    pub url: Option<String>,
    pub warning_no: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WeatherObs {
    pub gust_ms: Option<f64>,
    pub high_c: Option<f64>,
    pub humidity_pct: Option<f64>,
    pub low_c: Option<f64>,
    pub observed_at: Option<String>,
    pub pressure_hpa: Option<f64>,
    pub station_id: Option<String>,
    pub temp_c: Option<f64>,
    pub weather: Option<String>,
    pub wind_dir_deg: Option<f64>,
    pub wind_ms: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WeatherObsSite {
    pub altitude_m: Option<f64>,
    pub county_code: Option<String>,
    pub kind: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub name: Option<String>,
    pub station_id: Option<String>,
    pub town: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WeatherWarning {
    pub county_code: Option<String>,
    pub ends_at: Option<String>,
    pub first_seen_at: Option<String>,
    pub last_seen_at: Option<String>,
    pub phenomena: Option<String>,
    pub significance: Option<String>,
    pub starts_at: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WindLayer {
    pub east: Option<f64>,
    pub height: Option<i64>,
    pub north: Option<f64>,
    pub observed_at: Option<String>,
    pub south: Option<f64>,
    pub url: Option<String>,
    pub west: Option<f64>,
    pub width: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WorkClosure {
    pub announced_at: Option<String>,
    pub county_code: Option<String>,
    pub day: Option<String>,
    pub first_seen_at: Option<String>,
    pub last_seen_at: Option<String>,
    pub level: Option<i64>,
    pub status: Option<String>,
}

pub const TYPHOON_TRACK_ID: &str = "typhoon_track";

pub type TyphoonTrackLatestResponse = super::common::LatestResponse<super::weather::TyphoonTrack>;

pub type TyphoonTrackHistoryResponse = super::common::HistoryResponse<super::weather::TyphoonTrack>;

pub const TYPHOON_WARNING_ID: &str = "typhoon_warning";

pub type TyphoonWarningLatestResponse =
    super::common::LatestResponse<super::weather::TyphoonWarning>;

pub type TyphoonWarningHistoryResponse =
    super::common::HistoryResponse<super::weather::TyphoonWarning>;

pub const WEATHER_WARNING_ID: &str = "weather_warning";

pub type WeatherWarningLatestResponse =
    super::common::LatestResponse<super::weather::WeatherWarning>;

pub type WeatherWarningHistoryResponse =
    super::common::HistoryResponse<super::weather::WeatherWarning>;

pub const RAIN_OBS_ID: &str = "rain_obs";

pub type RainObsLatestResponse = super::common::LatestResponse<super::weather::RainObs>;

pub type RainObsHistoryResponse = super::common::HistoryResponse<super::weather::RainObs>;

pub type RainObsSitesResponse = super::common::SitesResponse<super::weather::RainObsSite>;

pub const WEATHER_OBS_ID: &str = "weather_obs";

pub type WeatherObsLatestResponse = super::common::LatestResponse<super::weather::WeatherObs>;

pub type WeatherObsHistoryResponse = super::common::HistoryResponse<super::weather::WeatherObs>;

pub type WeatherObsSitesResponse = super::common::SitesResponse<super::weather::WeatherObsSite>;

pub const COUNTY_FORECAST_ID: &str = "county_forecast";

pub type CountyForecastLatestResponse =
    super::common::LatestResponse<super::weather::CountyForecast>;

pub type CountyForecastHistoryResponse =
    super::common::HistoryResponse<super::weather::CountyForecast>;

pub const EARTHQUAKE_ID: &str = "earthquake";

pub type EarthquakeLatestResponse = super::common::LatestResponse<super::weather::Earthquake>;

pub type EarthquakeHistoryResponse = super::common::HistoryResponse<super::weather::Earthquake>;

pub const EARTHQUAKE_LOCAL_ID: &str = "earthquake_local";

pub type EarthquakeLocalLatestResponse =
    super::common::LatestResponse<super::weather::EarthquakeLocal>;

pub type EarthquakeLocalHistoryResponse =
    super::common::HistoryResponse<super::weather::EarthquakeLocal>;

pub const DISASTER_ALERT_ID: &str = "disaster_alert";

pub type DisasterAlertLatestResponse = super::common::LatestResponse<super::weather::DisasterAlert>;

pub type DisasterAlertHistoryResponse =
    super::common::HistoryResponse<super::weather::DisasterAlert>;

pub const DEBRIS_ALERT_ID: &str = "debris_alert";

pub type DebrisAlertLatestResponse = super::common::LatestResponse<super::weather::DebrisAlert>;

pub type DebrisAlertHistoryResponse = super::common::HistoryResponse<super::weather::DebrisAlert>;

pub const FLOOD_SENSOR_ID: &str = "flood_sensor";

pub type FloodSensorLatestResponse = super::common::LatestResponse<super::weather::FloodSensor>;

pub type FloodSensorHistoryResponse = super::common::HistoryResponse<super::weather::FloodSensor>;

pub type FloodSensorSitesResponse = super::common::SitesResponse<super::weather::FloodSensorSite>;

pub const WORK_CLOSURE_ID: &str = "work_closure";

pub type WorkClosureLatestResponse = super::common::LatestResponse<super::weather::WorkClosure>;

pub type WorkClosureHistoryResponse = super::common::HistoryResponse<super::weather::WorkClosure>;

pub const SHELTER_ID: &str = "shelter";

pub type ShelterLatestResponse = super::common::LatestResponse<super::weather::Shelter>;

pub const SHELTER_REPORT_ID: &str = "shelter_report";

pub type ShelterReportLatestResponse = super::common::LatestResponse<super::weather::ShelterReport>;

pub type ShelterReportHistoryResponse =
    super::common::HistoryResponse<super::weather::ShelterReport>;

pub const RADAR_IMAGE_ID: &str = "radar_image";

pub type RadarImageLatestResponse = super::common::LatestResponse<super::weather::RadarImage>;

pub type RadarImageHistoryResponse = super::common::HistoryResponse<super::weather::RadarImage>;

pub const SATELLITE_IMAGE_ID: &str = "satellite_image";

pub type SatelliteImageLatestResponse =
    super::common::LatestResponse<super::weather::SatelliteImage>;

pub type SatelliteImageHistoryResponse =
    super::common::HistoryResponse<super::weather::SatelliteImage>;

pub const RADAR_LAYER_ID: &str = "radar_layer";

pub type RadarLayerLatestResponse = super::common::LatestResponse<super::weather::RadarLayer>;

pub type RadarLayerHistoryResponse = super::common::HistoryResponse<super::weather::RadarLayer>;

pub const CLOUD_LAYER_ID: &str = "cloud_layer";

pub type CloudLayerLatestResponse = super::common::LatestResponse<super::weather::CloudLayer>;

pub type CloudLayerHistoryResponse = super::common::HistoryResponse<super::weather::CloudLayer>;

pub const WIND_LAYER_ID: &str = "wind_layer";

pub type WindLayerLatestResponse = super::common::LatestResponse<super::weather::WindLayer>;

pub type WindLayerHistoryResponse = super::common::HistoryResponse<super::weather::WindLayer>;

pub const FIRE_INCIDENT_ID: &str = "fire_incident";

pub type FireIncidentLatestResponse = super::common::LatestResponse<super::weather::FireIncident>;

pub type FireIncidentHistoryResponse = super::common::HistoryResponse<super::weather::FireIncident>;
