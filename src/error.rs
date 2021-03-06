use serde::{Deserialize, Serialize};



#[derive(Debug, Deserialize, Serialize)]
pub enum AppError {
    BindErr,
    ScrapErr(ScrapError),
    TcpStreamCloneErr,
    ChromeOptionErr,
    CreateWebDriverErr(usize),
    DeserializeErr,
    SerializeErr,
    WriteErr,
    FlushErr,
    QuitDriverErr,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ScrapError {
    ErrFindingClass(String),
    ErrFindingTag(String),
    ErrTextParsing,
    InnerHtmlErr,
    ErrNavigateUrl(String),
    ErrFindingId(String),
    ErrFindClassName,
    ErrFindingImage(String),
}