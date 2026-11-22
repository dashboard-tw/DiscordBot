#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DengueDaily {
    pub cases: Option<i64>,
    pub county_code: Option<String>,
    pub imported: Option<bool>,
    pub onset_date: Option<String>,
    pub town: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FarmPrice {
    pub avg_price: Option<f64>,
    pub category: Option<String>,
    pub crop: Option<String>,
    pub markets: Option<i64>,
    pub trade_date: Option<String>,
    pub volume: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FoodRejection {
    pub disposal: Option<String>,
    pub hs_code: Option<String>,
    pub origin: Option<String>,
    pub product: Option<String>,
    pub published_on: Option<String>,
    pub reason: Option<String>,
    pub received_on: Option<String>,
    pub rejection_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HogTrade {
    pub day: Option<String>,
    pub head: Option<i64>,
    pub market: Option<String>,
    pub price: Option<f64>,
    pub weight_kg: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LtcFacility {
    pub abc: Option<String>,
    pub address: Option<String>,
    pub beds: Option<i64>,
    pub contract_end: Option<String>,
    pub county_code: Option<String>,
    pub facility_id: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub name: Option<String>,
    pub residents: Option<i64>,
    pub services: Option<String>,
    pub town_code: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MedicalFacility {
    pub address: Option<String>,
    pub contract_end: Option<String>,
    pub county_code: Option<String>,
    pub departments: Option<String>,
    pub facility_id: Option<String>,
    pub kind: Option<String>,
    pub level: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub services: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NhiVisit {
    pub county_code: Option<String>,
    pub disease: Option<String>,
    pub total: Option<i64>,
    pub visit_type: Option<String>,
    pub visits: Option<i64>,
    pub week: Option<i64>,
    pub week_start: Option<String>,
    pub year: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PoultryPrice {
    pub broiler_large: Option<f64>,
    pub broiler_retail: Option<f64>,
    pub broiler_small: Option<f64>,
    pub day: Option<String>,
    pub egg_farm: Option<f64>,
    pub egg_transport: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PowerGen {
    pub capacity_mw: Option<f64>,
    pub net_gen_mw: Option<f64>,
    pub observed_at: Option<String>,
    pub remark: Option<String>,
    pub unit_id: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PowerGenSite {
    pub county_code: Option<String>,
    pub fuel_type: Option<String>,
    pub is_private: Option<bool>,
    pub plant_name: Option<String>,
    pub unit_id: Option<i64>,
    pub unit_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PowerLoad {
    pub fore_indicator: Option<String>,
    pub fore_peak_hours: Option<String>,
    pub fore_peak_load_mw: Option<f64>,
    pub fore_reserve_mw: Option<f64>,
    pub fore_reserve_pct: Option<f64>,
    pub fore_supply_mw: Option<f64>,
    pub load_mw: Option<f64>,
    pub observed_at: Option<String>,
    pub util_pct: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PowerOutage {
    pub area: Option<String>,
    pub county_code: Option<String>,
    pub district: Option<String>,
    pub expected_restore: Option<String>,
    pub households: Option<i64>,
    pub info: Option<String>,
    pub observed_at: Option<String>,
    pub seq: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PowerPeakDaily {
    pub date: Option<String>,
    pub indicator: Option<String>,
    pub peak_load_mw: Option<f64>,
    pub reserve_mw: Option<f64>,
    pub reserve_pct: Option<f64>,
    pub supply_mw: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PowerRegion {
    pub gen_mw: Option<f64>,
    pub load_mw: Option<f64>,
    pub observed_at: Option<String>,
    pub region: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PowerSupplyDaily {
    pub date: Option<String>,
    pub industrial_gwh: Option<f64>,
    pub net_peak_capacity_mw: Option<f64>,
    pub peak_load_mw: Option<f64>,
    pub reserve_mw: Option<f64>,
    pub reserve_pct: Option<f64>,
    pub residential_gwh: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PowerSupplyForecast {
    pub forecast_date: Option<String>,
    pub net_peak_capacity_mw: Option<f64>,
    pub peak_load_mw: Option<f64>,
    pub published_on: Option<String>,
    pub reserve_mw: Option<f64>,
    pub reserve_pct: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RealEstate {
    pub area_m2: Option<f64>,
    pub building_type: Option<String>,
    pub county_code: Option<String>,
    pub deal_date: Option<String>,
    pub district: Option<String>,
    pub kind: Option<String>,
    pub main_use: Option<String>,
    pub rooms: Option<i64>,
    pub season: Option<String>,
    pub serial: Option<String>,
    pub special: Option<bool>,
    pub target: Option<String>,
    pub total_price: Option<i64>,
    pub unit_price: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WaterOutage {
    pub area: Option<String>,
    pub case_id: Option<String>,
    pub county_code: Option<String>,
    pub daily_from: Option<String>,
    pub daily_to: Option<String>,
    pub district: Option<String>,
    pub ends_at: Option<String>,
    pub first_seen_at: Option<String>,
    pub households: Option<i64>,
    pub kind: Option<String>,
    pub last_seen_at: Option<String>,
    pub low_pressure_area: Option<String>,
    pub low_pressure_reason: Option<String>,
    pub published_on: Option<String>,
    pub reason: Option<String>,
    pub starts_at: Option<String>,
    pub url: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WaterOutageTaipei {
    pub area: Option<String>,
    pub case_id: Option<String>,
    pub county_code: Option<String>,
    pub daily_from: Option<String>,
    pub daily_to: Option<String>,
    pub district: Option<String>,
    pub ends_at: Option<String>,
    pub first_seen_at: Option<String>,
    pub households: Option<i64>,
    pub kind: Option<String>,
    pub last_seen_at: Option<String>,
    pub low_pressure_area: Option<String>,
    pub low_pressure_reason: Option<String>,
    pub published_on: Option<String>,
    pub reason: Option<String>,
    pub starts_at: Option<String>,
    pub url: Option<String>,
}

pub const WATER_OUTAGE_ID: &str = "water_outage";

pub type WaterOutageLatestResponse = super::common::LatestResponse<super::living::WaterOutage>;

pub type WaterOutageHistoryResponse = super::common::HistoryResponse<super::living::WaterOutage>;

pub const WATER_OUTAGE_TAIPEI_ID: &str = "water_outage_taipei";

pub type WaterOutageTaipeiLatestResponse =
    super::common::LatestResponse<super::living::WaterOutageTaipei>;

pub type WaterOutageTaipeiHistoryResponse =
    super::common::HistoryResponse<super::living::WaterOutageTaipei>;

pub const POWER_OUTAGE_ID: &str = "power_outage";

pub type PowerOutageLatestResponse = super::common::LatestResponse<super::living::PowerOutage>;

pub type PowerOutageHistoryResponse = super::common::HistoryResponse<super::living::PowerOutage>;

pub const POWER_GEN_ID: &str = "power_gen";

pub type PowerGenLatestResponse = super::common::LatestResponse<super::living::PowerGen>;

pub type PowerGenHistoryResponse = super::common::HistoryResponse<super::living::PowerGen>;

pub type PowerGenSitesResponse = super::common::SitesResponse<super::living::PowerGenSite>;

pub const POWER_REGION_ID: &str = "power_region";

pub type PowerRegionLatestResponse = super::common::LatestResponse<super::living::PowerRegion>;

pub type PowerRegionHistoryResponse = super::common::HistoryResponse<super::living::PowerRegion>;

pub const POWER_LOAD_ID: &str = "power_load";

pub type PowerLoadLatestResponse = super::common::LatestResponse<super::living::PowerLoad>;

pub type PowerLoadHistoryResponse = super::common::HistoryResponse<super::living::PowerLoad>;

pub const POWER_PEAK_DAILY_ID: &str = "power_peak_daily";

pub type PowerPeakDailyLatestResponse =
    super::common::LatestResponse<super::living::PowerPeakDaily>;

pub type PowerPeakDailyHistoryResponse =
    super::common::HistoryResponse<super::living::PowerPeakDaily>;

pub const POWER_SUPPLY_DAILY_ID: &str = "power_supply_daily";

pub type PowerSupplyDailyLatestResponse =
    super::common::LatestResponse<super::living::PowerSupplyDaily>;

pub type PowerSupplyDailyHistoryResponse =
    super::common::HistoryResponse<super::living::PowerSupplyDaily>;

pub const POWER_SUPPLY_FORECAST_ID: &str = "power_supply_forecast";

pub type PowerSupplyForecastLatestResponse =
    super::common::LatestResponse<super::living::PowerSupplyForecast>;

pub type PowerSupplyForecastHistoryResponse =
    super::common::HistoryResponse<super::living::PowerSupplyForecast>;

pub const MEDICAL_FACILITY_ID: &str = "medical_facility";

pub type MedicalFacilityLatestResponse =
    super::common::LatestResponse<super::living::MedicalFacility>;

pub const LTC_FACILITY_ID: &str = "ltc_facility";

pub type LtcFacilityLatestResponse = super::common::LatestResponse<super::living::LtcFacility>;

pub const FARM_PRICE_ID: &str = "farm_price";

pub type FarmPriceLatestResponse = super::common::LatestResponse<super::living::FarmPrice>;

pub type FarmPriceHistoryResponse = super::common::HistoryResponse<super::living::FarmPrice>;

pub const POULTRY_PRICE_ID: &str = "poultry_price";

pub type PoultryPriceLatestResponse = super::common::LatestResponse<super::living::PoultryPrice>;

pub type PoultryPriceHistoryResponse = super::common::HistoryResponse<super::living::PoultryPrice>;

pub const HOG_TRADE_ID: &str = "hog_trade";

pub type HogTradeLatestResponse = super::common::LatestResponse<super::living::HogTrade>;

pub type HogTradeHistoryResponse = super::common::HistoryResponse<super::living::HogTrade>;

pub const FOOD_REJECTION_ID: &str = "food_rejection";

pub type FoodRejectionLatestResponse = super::common::LatestResponse<super::living::FoodRejection>;

pub type FoodRejectionHistoryResponse =
    super::common::HistoryResponse<super::living::FoodRejection>;

pub const REAL_ESTATE_ID: &str = "real_estate";

pub type RealEstateLatestResponse = super::common::LatestResponse<super::living::RealEstate>;

pub type RealEstateHistoryResponse = super::common::HistoryResponse<super::living::RealEstate>;

pub const DENGUE_DAILY_ID: &str = "dengue_daily";

pub type DengueDailyLatestResponse = super::common::LatestResponse<super::living::DengueDaily>;

pub type DengueDailyHistoryResponse = super::common::HistoryResponse<super::living::DengueDaily>;

pub const NHI_VISIT_ID: &str = "nhi_visit";

pub type NhiVisitLatestResponse = super::common::LatestResponse<super::living::NhiVisit>;

pub type NhiVisitHistoryResponse = super::common::HistoryResponse<super::living::NhiVisit>;
