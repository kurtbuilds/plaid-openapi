use serde::{Serialize, Deserialize};
///Types of document formats. (Suggested values)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FdxContentTypes {
    #[serde(rename = "application/pdf")]
    ApplicationPdf,
    #[serde(rename = "image/gif")]
    ImageGif,
    #[serde(rename = "image/jpeg")]
    ImageJpeg,
    #[serde(rename = "image/tiff")]
    ImageTiff,
    #[serde(rename = "image/png")]
    ImagePng,
    #[serde(rename = "application/json")]
    ApplicationJson,
}
