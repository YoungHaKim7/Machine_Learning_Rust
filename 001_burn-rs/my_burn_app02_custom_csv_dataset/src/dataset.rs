use burn::data::dataset::InMemDataset;
use serde::Serialize;

#[derive(Serialize)]
struct DiabetesPatient {
    #[serde(rename = "AGE")]
    pub age: u8,

    #[serde(rename = "SEX")]
    pub sex: u8,

    #[serde(rename = "BMI")]
    pub bmi: f32,

    #[serde(rename = "BP")]
    pub bp: f32,

    #[serde(rename = "S1")]
    pub tc: u16,

    #[serde(rename = "S2")]
    pub ldl: u16,

    #[serde(rename = "S3")]
    pub hdl: u16,

    #[serde(rename = "S4")]
    pub tch: u16,

    #[serde(rename = "S5")]
    pub ltg: u16,

    #[serde(rename = "S6")]
    pub glu: u16,

    #[serde(rename = "Y")]
    pub response: u16,
}

struct DiabetesDataset {
    dataset: InMemDataset<DiabetesPatient>,
}

impl DiabetesDataset {
    pub fn new() -> Result<Self, std::io::Error> {}
}
