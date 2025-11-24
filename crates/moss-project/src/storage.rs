use crate::models::primitives::{
    FormDataParamId, HeaderId, PathParamId, QueryParamId, ResourceId, UrlencodedParamId,
};

pub const KEY_EXPANDED_ENTRIES: &'static str = "expandedEntries";

pub const KEY_RESOURCE_PREFIX: &'static str = "resource";

pub fn key_resource(resource_id: &ResourceId) -> String {
    format!("{KEY_RESOURCE_PREFIX}.{resource_id}")
}

pub fn key_resource_order(resource_id: &ResourceId) -> String {
    format!("{KEY_RESOURCE_PREFIX}.{resource_id}.order")
}

// Header
pub fn key_resource_header(resource_id: &ResourceId, header_id: &HeaderId) -> String {
    format!("{KEY_RESOURCE_PREFIX}.{resource_id}.header.{header_id}")
}

pub fn key_resource_header_order(resource_id: &ResourceId, header_id: &HeaderId) -> String {
    format!("{KEY_RESOURCE_PREFIX}.{resource_id}.header.{header_id}.order")
}

// Path Param
pub fn key_resource_path_param(resource_id: &ResourceId, path_param_id: &PathParamId) -> String {
    format!("{KEY_RESOURCE_PREFIX}.{resource_id}.pathParam.{path_param_id}")
}

pub fn key_resource_path_param_order(
    resource_id: &ResourceId,
    path_param_id: &PathParamId,
) -> String {
    format!("{KEY_RESOURCE_PREFIX}.{resource_id}.pathParam.{path_param_id}.order")
}

// Query Param
pub fn key_resource_query_param(resource_id: &ResourceId, query_param_id: &QueryParamId) -> String {
    format!("{KEY_RESOURCE_PREFIX}.{resource_id}.queryParam.{query_param_id}")
}

pub fn key_resource_query_param_order(
    resource_id: &ResourceId,
    query_param_id: &QueryParamId,
) -> String {
    format!("{KEY_RESOURCE_PREFIX}.{resource_id}.queryParam.{query_param_id}.order")
}

// Body
pub fn key_resource_body(resource_id: &ResourceId) -> String {
    format!("{KEY_RESOURCE_PREFIX}.{resource_id}.body")
}

pub fn key_resource_body_urlencoded_param(
    resource_id: &ResourceId,
    urlencoded_param_id: &UrlencodedParamId,
) -> String {
    format!("{KEY_RESOURCE_PREFIX}.{resource_id}.body.urlencodedParam.{urlencoded_param_id}")
}

pub fn key_resource_body_urlencoded_param_order(
    resource_id: &ResourceId,
    urlencoded_param_id: &UrlencodedParamId,
) -> String {
    format!("{KEY_RESOURCE_PREFIX}.{resource_id}.body.urlencodedParam.{urlencoded_param_id}.order")
}

pub fn key_resource_body_formdata_param(
    resource_id: &ResourceId,
    formdata_param_id: &FormDataParamId,
) -> String {
    format!("{KEY_RESOURCE_PREFIX}.{resource_id}.body.formdataParam.{formdata_param_id}")
}

pub fn key_resource_body_formdata_param_order(
    resource_id: &ResourceId,
    formdata_param_id: &FormDataParamId,
) -> String {
    format!("{KEY_RESOURCE_PREFIX}.{resource_id}.body.formdataParam.{formdata_param_id}.order")
}
