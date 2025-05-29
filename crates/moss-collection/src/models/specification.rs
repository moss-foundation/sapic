// use moss_kdl::foundations::http::{
//     HeaderParamBody, HeaderParamOptions as KdlHeaderParamOptions, HttpRequestFile, PathParamBody,
//     PathParamOptions as KdlPathParamOptions, QueryParamBody,
//     QueryParamOptions as KdlQueryParamOptions, UrlBlock,
// };
use serde::Serialize;

use anyhow::anyhow;
use uuid::Uuid;
// use moss_kdl::foundations::body::{
//     FormDataBodyItem, FormDataOptions as KdlFormDataOptions, FormDataValue as KdlFormDataValue,
//     RawBodyType as KdlRawBodyType, RequestBodyBlock, UrlEncodedBodyItem,
//     UrlEncodedOptions as KdlUrlEncodedOptions,
// };
// use moss_kdl::spec_models::dir_spec::DirContentByClass;
// use moss_kdl::spec_models::item_spec::ItemContentByClass;
// use moss_kdl::spec_models::item_spec::request::RequestContent;
use std::path::PathBuf;
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct SpecificationMetadata {
    pub id: Uuid,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub enum RequestItemSpecificationInfo {
    Http {
        url: UrlItem,
        query_params: Vec<QueryParamItem>,
        path_params: Vec<PathParamItem>,
        headers: Vec<HeaderParamItem>,
        body: Option<RequestBody>,
    },
}

#[derive(Clone, Debug, Serialize, TS)]
// #[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub enum RequestDirSpecificationInfo {}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub enum ItemSpecificationInfoInner {
    Request(RequestItemSpecificationInfo),
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub enum DirSpecificationInfoInner {
    Request(RequestDirSpecificationInfo),
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ItemSpecificationInfo {
    pub metadata: SpecificationMetadata,
    pub inner: ItemSpecificationInfoInner,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DirSpecificationInfo {
    pub metadata: SpecificationMetadata,
    pub inner: DirSpecificationInfoInner,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub enum SpecificationInfo {
    Item(ItemSpecificationInfo),
    Dir(DirSpecificationInfo),
}

impl SpecificationInfo {
    pub fn is_dir(&self) -> bool {
        matches!(self, SpecificationInfo::Dir(_))
    }

    pub fn is_item(&self) -> bool {
        matches!(self, SpecificationInfo::Item(_))
    }
}

// impl TryInto<DirContentByClass> for SpecificationContent {
//     type Error = anyhow::Error;

//     fn try_into(self) -> Result<DirContentByClass, Self::Error> {
//         match self {
//             SpecificationContent::Http(_) => {
//                 Err(anyhow!("Invalid specification content for directory"))
//             }
//         }
//     }
// }

// impl TryInto<ItemContentByClass> for SpecificationContent {
//     type Error = anyhow::Error;

//     fn try_into(self) -> Result<ItemContentByClass, Self::Error> {
//         match self {
//             SpecificationContent::Http(http) => Ok(ItemContentByClass::Request(
//                 RequestContent::Http(http.into()),
//             )),
//         }
//     }
// }

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct HttpRequestSpecification {
    pub url: UrlItem,
    pub query_params: Vec<QueryParamItem>,
    pub path_params: Vec<PathParamItem>,
    pub headers: Vec<HeaderParamItem>,
    pub body: Option<RequestBody>,
}

// impl Into<HttpRequestFile> for HttpRequestSpecification {
//     fn into(self) -> HttpRequestFile {
//         HttpRequestFile {
//             url: self.url.into(),
//             query_params: self
//                 .query_params
//                 .into_iter()
//                 .map(|item| {
//                     (
//                         item.key.clone(),
//                         QueryParamBody {
//                             value: item.value,
//                             desc: item.desc,
//                             order: item.order,
//                             disabled: item.disabled,
//                             options: KdlQueryParamOptions {
//                                 propagate: item.options.propagate,
//                             },
//                         },
//                     )
//                 })
//                 .collect(),
//             path_params: self
//                 .path_params
//                 .into_iter()
//                 .map(|item| {
//                     (
//                         item.key.clone(),
//                         PathParamBody {
//                             value: item.value,
//                             desc: item.desc,
//                             order: item.order,
//                             disabled: item.disabled,
//                             options: KdlPathParamOptions {
//                                 propagate: item.options.propagate,
//                             },
//                         },
//                     )
//                 })
//                 .collect(),
//             headers: self
//                 .headers
//                 .into_iter()
//                 .map(|item| {
//                     (
//                         item.key.clone(),
//                         HeaderParamBody {
//                             value: item.value,
//                             desc: item.desc,
//                             order: item.order,
//                             disabled: item.disabled,
//                             options: KdlHeaderParamOptions {
//                                 propagate: item.options.propagate,
//                             },
//                         },
//                     )
//                 })
//                 .collect(),
//             body: self.body.map(|body| body.into()),
//         }
//     }
// }

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UrlItem {
    pub raw: Option<String>,
    pub host: Option<String>,
}

// impl Into<UrlBlock> for UrlItem {
//     fn into(self) -> UrlBlock {
//         UrlBlock {
//             raw: self.raw,
//             host: self.host,
//         }
//     }
// }

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct QueryParamOptions {
    pub propagate: bool,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct QueryParamItem {
    pub key: String,
    pub value: String,
    #[ts(optional)]
    pub order: Option<usize>,
    #[ts(optional)]
    pub desc: Option<String>,
    pub disabled: bool,
    pub options: QueryParamOptions,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct PathParamOptions {
    pub propagate: bool,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct PathParamItem {
    pub key: String,
    pub value: String,
    #[ts(optional)]
    pub order: Option<usize>,
    #[ts(optional)]
    pub desc: Option<String>,
    pub disabled: bool,
    pub options: PathParamOptions,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct HeaderParamOptions {
    pub propagate: bool,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct HeaderParamItem {
    pub key: String,
    pub value: String,
    #[ts(optional)]
    pub order: Option<usize>,
    #[ts(optional)]
    pub desc: Option<String>,
    pub disabled: bool,
    pub options: HeaderParamOptions,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum RequestBody {
    Raw(RawBodyType),
    FormData(Vec<FormDataItem>),
    UrlEncoded(Vec<UrlEncodedItem>),
    Binary(PathBuf),
}

// impl Into<RequestBodyBlock> for RequestBody {
//     fn into(self) -> RequestBodyBlock {
//         match self {
//             RequestBody::Raw(raw) => RequestBodyBlock::Raw(match raw {
//                 RawBodyType::Text(s) => KdlRawBodyType::Text(s),
//                 RawBodyType::Json(s) => KdlRawBodyType::Json(s),
//                 RawBodyType::Html(s) => KdlRawBodyType::Html(s),
//                 RawBodyType::Xml(s) => KdlRawBodyType::Xml(s),
//             }),
//             RequestBody::FormData(formdata) => RequestBodyBlock::FormData(
//                 formdata
//                     .into_iter()
//                     .map(|item| {
//                         (
//                             item.key,
//                             FormDataBodyItem {
//                                 value: match item.value {
//                                     FormDataValue::Text(s) => KdlFormDataValue::Text(s),
//                                     FormDataValue::File(s) => KdlFormDataValue::File(s),
//                                 },
//                                 desc: item.desc,
//                                 order: item.order,
//                                 disabled: item.disabled,
//                                 options: KdlFormDataOptions {
//                                     propagate: item.options.propagate,
//                                 },
//                             },
//                         )
//                     })
//                     .collect(),
//             ),
//             RequestBody::UrlEncoded(urlencoded) => RequestBodyBlock::UrlEncoded(
//                 urlencoded
//                     .into_iter()
//                     .map(|item| {
//                         (
//                             item.key,
//                             UrlEncodedBodyItem {
//                                 value: item.value,
//                                 desc: item.desc,
//                                 order: item.order,
//                                 disabled: item.disabled,
//                                 options: KdlUrlEncodedOptions {
//                                     propagate: item.options.propagate,
//                                 },
//                             },
//                         )
//                     })
//                     .collect(),
//             ),
//             RequestBody::Binary(path) => RequestBodyBlock::Binary(path),
//         }
//     }
// }

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct FormDataItem {
    pub key: String,
    pub value: FormDataValue,
    #[ts(optional)]
    pub order: Option<usize>,
    #[ts(optional)]
    pub desc: Option<String>,
    pub disabled: bool,
    pub options: FormDataOptions,
}

#[derive(Clone, Debug, Serialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum FormDataValue {
    Text(String),
    File(PathBuf),
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct FormDataOptions {
    pub propagate: bool,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct UrlEncodedItem {
    pub key: String,
    pub value: String,
    #[ts(optional)]
    pub order: Option<usize>,
    #[ts(optional)]
    pub desc: Option<String>,
    pub disabled: bool,
    pub options: UrlEncodedOptions,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct UrlEncodedOptions {
    pub propagate: bool,
}

#[derive(Clone, Debug, Serialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum RawBodyType {
    Text(String),
    Json(String),
    Html(String),
    Xml(String),
}
