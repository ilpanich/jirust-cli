/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-2776b5c6be42695cc76ed18bb9006304d509a541
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonNode {
    #[serde(rename = "array", skip_serializing_if = "Option::is_none")]
    pub array: Option<bool>,
    #[serde(rename = "bigDecimal", skip_serializing_if = "Option::is_none")]
    pub big_decimal: Option<bool>,
    #[serde(rename = "bigInteger", skip_serializing_if = "Option::is_none")]
    pub big_integer: Option<bool>,
    #[serde(rename = "bigIntegerValue", skip_serializing_if = "Option::is_none")]
    pub big_integer_value: Option<i32>,
    #[serde(rename = "binary", skip_serializing_if = "Option::is_none")]
    pub binary: Option<bool>,
    #[serde(rename = "binaryValue", skip_serializing_if = "Option::is_none")]
    pub binary_value: Option<Vec<String>>,
    #[serde(rename = "boolean", skip_serializing_if = "Option::is_none")]
    pub boolean: Option<bool>,
    #[serde(rename = "booleanValue", skip_serializing_if = "Option::is_none")]
    pub boolean_value: Option<bool>,
    #[serde(rename = "containerNode", skip_serializing_if = "Option::is_none")]
    pub container_node: Option<bool>,
    #[serde(rename = "decimalValue", skip_serializing_if = "Option::is_none")]
    pub decimal_value: Option<f64>,
    #[serde(rename = "double", skip_serializing_if = "Option::is_none")]
    pub double: Option<bool>,
    #[serde(rename = "doubleValue", skip_serializing_if = "Option::is_none")]
    pub double_value: Option<f64>,
    #[serde(rename = "elements", skip_serializing_if = "Option::is_none")]
    pub elements: Option<serde_json::Value>,
    #[serde(rename = "fieldNames", skip_serializing_if = "Option::is_none")]
    pub field_names: Option<serde_json::Value>,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<serde_json::Value>,
    #[serde(rename = "floatingPointNumber", skip_serializing_if = "Option::is_none")]
    pub floating_point_number: Option<bool>,
    #[serde(rename = "int", skip_serializing_if = "Option::is_none")]
    pub int: Option<bool>,
    #[serde(rename = "intValue", skip_serializing_if = "Option::is_none")]
    pub int_value: Option<i32>,
    #[serde(rename = "integralNumber", skip_serializing_if = "Option::is_none")]
    pub integral_number: Option<bool>,
    #[serde(rename = "long", skip_serializing_if = "Option::is_none")]
    pub long: Option<bool>,
    #[serde(rename = "longValue", skip_serializing_if = "Option::is_none")]
    pub long_value: Option<i64>,
    #[serde(rename = "missingNode", skip_serializing_if = "Option::is_none")]
    pub missing_node: Option<bool>,
    #[serde(rename = "null", skip_serializing_if = "Option::is_none")]
    pub null: Option<bool>,
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<bool>,
    #[serde(rename = "numberType", skip_serializing_if = "Option::is_none")]
    pub number_type: Option<NumberType>,
    #[serde(rename = "numberValue", skip_serializing_if = "Option::is_none")]
    pub number_value: Option<f64>,
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<bool>,
    #[serde(rename = "pojo", skip_serializing_if = "Option::is_none")]
    pub pojo: Option<bool>,
    #[serde(rename = "textValue", skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    #[serde(rename = "textual", skip_serializing_if = "Option::is_none")]
    pub textual: Option<bool>,
    #[serde(rename = "valueAsBoolean", skip_serializing_if = "Option::is_none")]
    pub value_as_boolean: Option<bool>,
    #[serde(rename = "valueAsDouble", skip_serializing_if = "Option::is_none")]
    pub value_as_double: Option<f64>,
    #[serde(rename = "valueAsInt", skip_serializing_if = "Option::is_none")]
    pub value_as_int: Option<i32>,
    #[serde(rename = "valueAsLong", skip_serializing_if = "Option::is_none")]
    pub value_as_long: Option<i64>,
    #[serde(rename = "valueAsText", skip_serializing_if = "Option::is_none")]
    pub value_as_text: Option<String>,
    #[serde(rename = "valueNode", skip_serializing_if = "Option::is_none")]
    pub value_node: Option<bool>,
}

impl JsonNode {
    pub fn new() -> JsonNode {
        JsonNode {
            array: None,
            big_decimal: None,
            big_integer: None,
            big_integer_value: None,
            binary: None,
            binary_value: None,
            boolean: None,
            boolean_value: None,
            container_node: None,
            decimal_value: None,
            double: None,
            double_value: None,
            elements: None,
            field_names: None,
            fields: None,
            floating_point_number: None,
            int: None,
            int_value: None,
            integral_number: None,
            long: None,
            long_value: None,
            missing_node: None,
            null: None,
            number: None,
            number_type: None,
            number_value: None,
            object: None,
            pojo: None,
            text_value: None,
            textual: None,
            value_as_boolean: None,
            value_as_double: None,
            value_as_int: None,
            value_as_long: None,
            value_as_text: None,
            value_node: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NumberType {
    #[serde(rename = "INT")]
    Int,
    #[serde(rename = "LONG")]
    Long,
    #[serde(rename = "BIG_INTEGER")]
    BigInteger,
    #[serde(rename = "FLOAT")]
    Float,
    #[serde(rename = "DOUBLE")]
    Double,
    #[serde(rename = "BIG_DECIMAL")]
    BigDecimal,
}

impl Default for NumberType {
    fn default() -> NumberType {
        Self::Int
    }
}

