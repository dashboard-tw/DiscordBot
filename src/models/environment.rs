#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AqiForecast {
    pub aqi: Option<i64>,
    pub area: Option<String>,
    pub content: Option<String>,
    pub forecast_date: Option<String>,
    pub major_pollutant: Option<String>,
    pub minor_aqi: Option<i64>,
    pub minor_pollutant: Option<String>,
    pub published_at: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AqiObs {
    pub aqi: Option<i64>,
    pub co: Option<f64>,
    pub co_8hr: Option<f64>,
    pub no: Option<f64>,
    pub no2: Option<f64>,
    pub nox: Option<f64>,
    pub o3: Option<f64>,
    pub o3_8hr: Option<f64>,
    pub observed_at: Option<String>,
    pub pm10: Option<f64>,
    pub pm10_avg: Option<f64>,
    pub pm25: Option<f64>,
    pub pm25_avg: Option<f64>,
    pub pollutant: Option<String>,
    pub site_id: Option<i64>,
    pub so2: Option<f64>,
    pub so2_avg: Option<f64>,
    pub status: Option<String>,
    pub wind_dir: Option<f64>,
    pub wind_speed: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AqiObsSite {
    pub county_code: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub name: Option<String>,
    pub site_id: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PowerSales {
    pub county_code: Option<String>,
    pub kwh: Option<i64>,
    pub month: Option<String>,
    pub sector: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RenewableMonth {
    pub gen_mwh: Option<f64>,
    pub month: Option<String>,
    pub source: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReservoirObs {
    pub effective_storage: Option<f64>,
    pub inflow: Option<f64>,
    pub observed_at: Option<String>,
    pub rainfall: Option<f64>,
    pub reservoir_id: Option<String>,
    pub status_type: Option<i64>,
    pub storage_pct: Option<f64>,
    pub total_outflow: Option<f64>,
    pub water_level: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReservoirObsSite {
    pub county_code: Option<String>,
    pub effective_capacity: Option<f64>,
    pub is_major: Option<bool>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub name: Option<String>,
    pub reservoir_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RiverLevel {
    pub observed_at: Option<String>,
    pub station_id: Option<String>,
    pub water_level_m: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RiverLevelSite {
    pub alert1_m: Option<f64>,
    pub alert2_m: Option<f64>,
    pub alert3_m: Option<f64>,
    pub county_code: Option<String>,
    pub is_active: Option<bool>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub name: Option<String>,
    pub river_name: Option<String>,
    pub station_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RiverQuality {
    pub rpi: Option<f64>,
    pub sampled_at: Option<String>,
    pub site_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RiverQualitySite {
    pub basin: Option<String>,
    pub county_code: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub name: Option<String>,
    pub river: Option<String>,
    pub site_id: Option<String>,
    pub township: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UvObs {
    pub observed_at: Option<String>,
    pub site_id: Option<i64>,
    pub uvi: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UvObsSite {
    pub agency: Option<String>,
    pub county_code: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub name: Option<String>,
    pub site_id: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Waste {
    pub county_code: Option<String>,
    pub food_t: Option<f64>,
    pub garbage_t: Option<f64>,
    pub per_capita_kg: Option<f64>,
    pub period: Option<String>,
    pub period_type: Option<String>,
    pub recycled_t: Option<f64>,
    pub total_t: Option<f64>,
}

pub const AQI_OBS_ID: &str = "aqi_obs";

pub type AqiObsLatestResponse = super::common::LatestResponse<super::environment::AqiObs>;

pub type AqiObsHistoryResponse = super::common::HistoryResponse<super::environment::AqiObs>;

pub type AqiObsSitesResponse = super::common::SitesResponse<super::environment::AqiObsSite>;

pub const UV_OBS_ID: &str = "uv_obs";

pub type UvObsLatestResponse = super::common::LatestResponse<super::environment::UvObs>;

pub type UvObsHistoryResponse = super::common::HistoryResponse<super::environment::UvObs>;

pub type UvObsSitesResponse = super::common::SitesResponse<super::environment::UvObsSite>;

pub const AQI_FORECAST_ID: &str = "aqi_forecast";

pub type AqiForecastLatestResponse = super::common::LatestResponse<super::environment::AqiForecast>;

pub type AqiForecastHistoryResponse =
    super::common::HistoryResponse<super::environment::AqiForecast>;

pub const RESERVOIR_OBS_ID: &str = "reservoir_obs";

pub type ReservoirObsLatestResponse =
    super::common::LatestResponse<super::environment::ReservoirObs>;

pub type ReservoirObsHistoryResponse =
    super::common::HistoryResponse<super::environment::ReservoirObs>;

pub type ReservoirObsSitesResponse =
    super::common::SitesResponse<super::environment::ReservoirObsSite>;

pub const RIVER_LEVEL_ID: &str = "river_level";

pub type RiverLevelLatestResponse = super::common::LatestResponse<super::environment::RiverLevel>;

pub type RiverLevelHistoryResponse = super::common::HistoryResponse<super::environment::RiverLevel>;

pub type RiverLevelSitesResponse = super::common::SitesResponse<super::environment::RiverLevelSite>;

pub const RIVER_QUALITY_ID: &str = "river_quality";

pub type RiverQualityLatestResponse =
    super::common::LatestResponse<super::environment::RiverQuality>;

pub type RiverQualityHistoryResponse =
    super::common::HistoryResponse<super::environment::RiverQuality>;

pub type RiverQualitySitesResponse =
    super::common::SitesResponse<super::environment::RiverQualitySite>;

pub const RENEWABLE_MONTH_ID: &str = "renewable_month";

pub type RenewableMonthLatestResponse =
    super::common::LatestResponse<super::environment::RenewableMonth>;

pub type RenewableMonthHistoryResponse =
    super::common::HistoryResponse<super::environment::RenewableMonth>;

pub const WASTE_ID: &str = "waste";

pub type WasteLatestResponse = super::common::LatestResponse<super::environment::Waste>;

pub type WasteHistoryResponse = super::common::HistoryResponse<super::environment::Waste>;

pub const POWER_SALES_ID: &str = "power_sales";

pub type PowerSalesLatestResponse = super::common::LatestResponse<super::environment::PowerSales>;

pub type PowerSalesHistoryResponse = super::common::HistoryResponse<super::environment::PowerSales>;
