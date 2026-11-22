#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChildcareCenter {
    pub address: Option<String>,
    pub county_code: Option<String>,
    pub facility_id: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EconIndicator {
    pub month: Option<String>,
    pub series: Option<String>,
    pub value: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncomeTown {
    pub county_code: Option<String>,
    pub households: Option<i64>,
    pub income_k: Option<i64>,
    pub mean_k: Option<i64>,
    pub median_k: Option<i64>,
    pub town: Option<String>,
    pub year: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PopulationYear {
    pub county_code: Option<String>,
    pub female: Option<i64>,
    pub households: Option<i64>,
    pub male: Option<i64>,
    pub population: Option<i64>,
    pub year: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct School {
    pub address: Option<String>,
    pub code: Option<String>,
    pub county_code: Option<String>,
    pub is_public: Option<bool>,
    pub level: Option<String>,
    pub name: Option<String>,
    pub school_id: Option<String>,
    pub school_year: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShelterAnimal {
    pub age: Option<String>,
    pub animal_id: Option<i64>,
    pub body_type: Option<String>,
    pub colour: Option<String>,
    pub county_code: Option<String>,
    pub fetched_at: Option<String>,
    pub kind: Option<String>,
    pub open_date: Option<String>,
    pub sex: Option<String>,
    pub shelter: Option<String>,
    pub sterilized: Option<bool>,
    pub variety: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShelterStats {
    pub accepted: Option<i64>,
    pub adopted: Option<i64>,
    pub county_code: Option<String>,
    pub died: Option<i64>,
    pub euthanized: Option<i64>,
    pub month: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TourismAttraction {
    pub address: Option<String>,
    pub attraction_id: Option<String>,
    pub classes: Option<String>,
    pub county_code: Option<String>,
    pub description: Option<String>,
    pub free: Option<bool>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub name: Option<String>,
    pub status: Option<i64>,
    pub town: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TourismHotel {
    pub address: Option<String>,
    pub capacity: Option<i64>,
    pub class: Option<i64>,
    pub county_code: Option<String>,
    pub highest_price: Option<i64>,
    pub hotel_id: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub lowest_price: Option<i64>,
    pub name: Option<String>,
    pub rooms: Option<i64>,
    pub stars: Option<i64>,
    pub town: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VitalMonth {
    pub births: Option<i64>,
    pub county_code: Option<String>,
    pub deaths: Option<i64>,
    pub divorces: Option<i64>,
    pub marriages: Option<i64>,
    pub month: Option<String>,
}

pub const POPULATION_YEAR_ID: &str = "population_year";

pub type PopulationYearLatestResponse =
    super::common::LatestResponse<super::society::PopulationYear>;

pub type PopulationYearHistoryResponse =
    super::common::HistoryResponse<super::society::PopulationYear>;

pub const VITAL_MONTH_ID: &str = "vital_month";

pub type VitalMonthLatestResponse = super::common::LatestResponse<super::society::VitalMonth>;

pub type VitalMonthHistoryResponse = super::common::HistoryResponse<super::society::VitalMonth>;

pub const ECON_INDICATOR_ID: &str = "econ_indicator";

pub type EconIndicatorLatestResponse = super::common::LatestResponse<super::society::EconIndicator>;

pub type EconIndicatorHistoryResponse =
    super::common::HistoryResponse<super::society::EconIndicator>;

pub const INCOME_TOWN_ID: &str = "income_town";

pub type IncomeTownLatestResponse = super::common::LatestResponse<super::society::IncomeTown>;

pub type IncomeTownHistoryResponse = super::common::HistoryResponse<super::society::IncomeTown>;

pub const SCHOOL_ID: &str = "school";

pub type SchoolLatestResponse = super::common::LatestResponse<super::society::School>;

pub const CHILDCARE_CENTER_ID: &str = "childcare_center";

pub type ChildcareCenterLatestResponse =
    super::common::LatestResponse<super::society::ChildcareCenter>;

pub const TOURISM_ATTRACTION_ID: &str = "tourism_attraction";

pub type TourismAttractionLatestResponse =
    super::common::LatestResponse<super::society::TourismAttraction>;

pub const TOURISM_HOTEL_ID: &str = "tourism_hotel";

pub type TourismHotelLatestResponse = super::common::LatestResponse<super::society::TourismHotel>;

pub const SHELTER_ANIMAL_ID: &str = "shelter_animal";

pub type ShelterAnimalLatestResponse = super::common::LatestResponse<super::society::ShelterAnimal>;

pub const SHELTER_STATS_ID: &str = "shelter_stats";

pub type ShelterStatsLatestResponse = super::common::LatestResponse<super::society::ShelterStats>;

pub type ShelterStatsHistoryResponse = super::common::HistoryResponse<super::society::ShelterStats>;
