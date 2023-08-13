
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Google Custom Search Engine URL
pub const URL: &str = "https://customsearch.googleapis.com/customsearch/v1";

/// Error details
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ErrorDetail {
    /// Error code
    pub code: u16,
    /// Error message
    pub message: String,
    /// Status
    pub status: String,
}

/// Error body
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ErrorBody {
    /// Error details
    pub error: ErrorDetail,
}

/// Enable Simplified/Traditional Chinese
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum C2COff {
    /// Disabled
    Off,
    /// Enable Simplified and Traditional Chinese Search
    On,
}

impl Display for C2COff {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            C2COff::Off => write!(f, "0"),
            C2COff::On => write!(f, "1"),
        }
    }
}

/// Country Restriction
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Cr {
    /// Country code - See https://developers.google.com/custom-search/docs/json_api_reference#countryCollections
    Country(String),
    /// Not
    Not(Box<Cr>),
    /// And
    And(Box<Cr>, Box<Cr>),
    /// Or
    Or(Box<Cr>, Box<Cr>),
}

impl Display for Cr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cr::Country(country) => write!(f, "{}", country),
            Cr::Not(cr) => write!(f, "-{}", cr),
            Cr::And(cr1, cr2) => write!(f, "({}).({})", cr1, cr2),
            Cr::Or(cr1, cr2) => write!(f, "({}|{})", cr1, cr2),
        }
    }
}

/// Date Restrict
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum DateRestrict {
    /// From the specified number of past days
    Day(u32),
    /// From the specified number of past weeks
    Week(u32),
    /// From the specified number of past months
    Month(u32),
    /// From the specified number of past years
    Year(u32),
}

impl Display for DateRestrict {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DateRestrict::Day(day) => write!(f, "d{}", day),
            DateRestrict::Week(week) => write!(f, "w{}", week),
            DateRestrict::Month(month) => write!(f, "m{}", month),
            DateRestrict::Year(year) => write!(f, "y{}", year),
        }
    }
}

/// Duplicate Content Filter
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum DuplicateContentFilter {
    /// Off
    Off,
    /// On
    On,
}

impl Display for DuplicateContentFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DuplicateContentFilter::Off => write!(f, "0"),
            DuplicateContentFilter::On => write!(f, "1"),
        }
    }
}

/// Geolocation of end user
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Gl {
    /// Country code - See https://developers.google.com/custom-search/docs/json_api_reference#country-codes
    /// (2 letters lower case).
    CountryCode(String),
}

impl Display for Gl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Gl::CountryCode(country_code) => write!(f, "{}", country_code),
        }
    }
}

/// Image Color Type
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ImgColorType {
    /// Color
    Color,
    /// Gray
    Gray,
    /// Mono
    Mono,
    /// Transparent
    Trans,
}

impl Display for ImgColorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ImgColorType::Color => {
                write!(f, "color")
            }
            ImgColorType::Gray => {
                write!(f, "gray")
            }
            ImgColorType::Mono => {
                write!(f, "mono")
            }
            ImgColorType::Trans => {
                write!(f, "trans")
            }
        }
    }
}

/// Image dominant color
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ImgDominantColor {
    /// Black
    Black,
    /// Blue
    Blue,
    /// Brown
    Brown,
    /// Gray
    Gray,
    /// Green
    Green,
    /// Orange
    Orange,
    /// Pink
    Pink,
    /// Purple
    Purple,
    /// Red
    Red,
    /// Teal
    Teal,
    /// White
    White,
    /// Yellow
    Yellow,
}

impl Display for ImgDominantColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ImgDominantColor::Black => {
                write!(f, "black")
            }
            ImgDominantColor::Blue => {
                write!(f, "blue")
            }
            ImgDominantColor::Brown => {
                write!(f, "brown")
            }
            ImgDominantColor::Gray => {
                write!(f, "gray")
            }
            ImgDominantColor::Green => {
                write!(f, "green")
            }
            ImgDominantColor::Orange => {
                write!(f, "orange")
            }
            ImgDominantColor::Pink => {
                write!(f, "pink")
            }
            ImgDominantColor::Purple => {
                write!(f, "purple")
            }
            ImgDominantColor::Red => {
                write!(f, "red")
            }
            ImgDominantColor::Teal => {
                write!(f, "teal")
            }
            ImgDominantColor::White => {
                write!(f, "white")
            }
            ImgDominantColor::Yellow => {
                write!(f, "yellow")
            }
        }
    }
}

/// Image Size
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ImgSize {
    /// Icon
    Icon,
    /// Small
    Small,
    /// Medium
    Medium,
    /// Large
    Large,
    /// Xlarge
    Xlarge,
    /// Xxlarge
    Xxlarge,
    /// Huge
    Huge,
}

impl Display for ImgSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ImgSize::Icon => {
                write!(f, "icon")
            }
            ImgSize::Small => {
                write!(f, "small")
            }
            ImgSize::Medium => {
                write!(f, "medium")
            }
            ImgSize::Large => {
                write!(f, "large")
            }
            ImgSize::Xlarge => {
                write!(f, "xlarge")
            }
            ImgSize::Xxlarge => {
                write!(f, "xxlarge")
            }
            ImgSize::Huge => {
                write!(f, "huge")
            }
        }
    }
}

/// Image Type
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ImgType {
    /// Face
    Face,
    /// Photo
    Photo,
    /// Clipart
    Clipart,
    /// Lineart
    Lineart,
    /// Animated
    Animated,
    /// Stock
    Stock,
}

impl Display for ImgType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ImgType::Face => {
                write!(f, "face")
            }
            ImgType::Photo => {
                write!(f, "photo")
            }
            ImgType::Clipart => {
                write!(f, "clipart")
            }
            ImgType::Lineart => {
                write!(f, "lineart")
            }
            ImgType::Animated => {
                write!(f, "animated")
            }
            ImgType::Stock => {
                write!(f, "stock")
            }
        }
    }
}

/// Language Restriction
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Lr {
    LangAr,
    LangBg,
    LangCa,
    LangCs,
    LangDa,
    LangDe,
    LangEl,
    LangEn,
    LangEs,
    LangEt,
    LangFi,
    LangFr,
    LangHr,
    LangHu,
    LangId,
    LangIs,
    LangIt,
    LangIw,
    LangJa,
    LangKo,
    LangLt,
    LangLv,
    LangNl,
    LangNo,
    LangPl,
    LangPt,
    LangRo,
    LangRu,
    LangSk,
    LangSl,
    LangSr,
    LangSv,
    LangTr,
    LangZhCN,
    LangZhTW,
}

impl Display for Lr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Lr::LangAr => write!(f, "lang_ar"),
            Lr::LangBg => write!(f, "lang_bg"),
            Lr::LangCa => write!(f, "lang_ca"),
            Lr::LangCs => write!(f, "lang_cs"),
            Lr::LangDa => write!(f, "lang_da"),
            Lr::LangDe => write!(f, "lang_de"),
            Lr::LangEl => write!(f, "lang_el"),
            Lr::LangEn => write!(f, "lang_en"),
            Lr::LangEs => write!(f, "lang_es"),
            Lr::LangEt => write!(f, "lang_et"),
            Lr::LangFi => write!(f, "lang_fi"),
            Lr::LangFr => write!(f, "lang_fr"),
            Lr::LangHr => write!(f, "lang_hr"),
            Lr::LangHu => write!(f, "lang_hu"),
            Lr::LangId => write!(f, "lang_id"),
            Lr::LangIs => write!(f, "lang_is"),
            Lr::LangIt => write!(f, "lang_it"),
            Lr::LangIw => write!(f, "lang_iw"),
            Lr::LangJa => write!(f, "lang_ja"),
            Lr::LangKo => write!(f, "lang_ko"),
            Lr::LangLt => write!(f, "lang_lt"),
            Lr::LangLv => write!(f, "lang_lv"),
            Lr::LangNl => write!(f, "lang_nl"),
            Lr::LangNo => write!(f, "lang_no"),
            Lr::LangPl => write!(f, "lang_pl"),
            Lr::LangPt => write!(f, "lang_pt"),
            Lr::LangRo => write!(f, "lang_ro"),
            Lr::LangRu => write!(f, "lang_ru"),
            Lr::LangSk => write!(f, "lang_sk"),
            Lr::LangSl => write!(f, "lang_sl"),
            Lr::LangSr => write!(f, "lang_sr"),
            Lr::LangSv => write!(f, "lang_sv"),
            Lr::LangTr => write!(f, "lang_tr"),
            Lr::LangZhCN => write!(f, "lang_zh-CN"),
            Lr::LangZhTW => write!(f, "lang_zh-TW"),
        }
    }
}

impl TryFrom<&str> for Lr {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "lang_ar" => Ok(Lr::LangAr),
            "lang_bg" => Ok(Lr::LangBg),
            "lang_ca" => Ok(Lr::LangCa),
            "lang_cs" => Ok(Lr::LangCs),
            "lang_da" => Ok(Lr::LangDa),
            "lang_de" => Ok(Lr::LangDe),
            "lang_el" => Ok(Lr::LangEl),
            "lang_en" => Ok(Lr::LangEn),
            "lang_es" => Ok(Lr::LangEs),
            "lang_et" => Ok(Lr::LangEt),
            "lang_fi" => Ok(Lr::LangFi),
            "lang_fr" => Ok(Lr::LangFr),
            "lang_hr" => Ok(Lr::LangHr),
            "lang_hu" => Ok(Lr::LangHu),
            "lang_id" => Ok(Lr::LangId),
            "lang_is" => Ok(Lr::LangIs),
            "lang_it" => Ok(Lr::LangIt),
            "lang_iw" => Ok(Lr::LangIw),
            "lang_ja" => Ok(Lr::LangJa),
            "lang_ko" => Ok(Lr::LangKo),
            "lang_lt" => Ok(Lr::LangLt),
            "lang_lv" => Ok(Lr::LangLv),
            "lang_nl" => Ok(Lr::LangNl),
            "lang_no" => Ok(Lr::LangNo),
            "lang_pl" => Ok(Lr::LangPl),
            "lang_pt" => Ok(Lr::LangPt),
            "lang_ro" => Ok(Lr::LangRo),
            "lang_ru" => Ok(Lr::LangRu),
            "lang_sk" => Ok(Lr::LangSk),
            "lang_sl" => Ok(Lr::LangSl),
            "lang_sr" => Ok(Lr::LangSr),
            "lang_sv" => Ok(Lr::LangSv),
            "lang_tr" => Ok(Lr::LangTr),
            "lang_zh-CN" => Ok(Lr::LangZhCN),
            "lang_zh-TW" => Ok(Lr::LangZhTW),
            _ => Err("Invalid value"),
        }
    }
}
