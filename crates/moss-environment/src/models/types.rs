use hcl::Expression as HclExpression;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value as JsonValue};
use ts_rs::TS;

pub type VariableName = String;
pub type EnvironmentName = String;

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize, TS, PartialEq, Eq)]
#[ts(export, export_to = "types.ts")]
#[serde(untagged)]
pub enum Expression {
    String(String),
    Number(i64),
    Boolean(bool),
    Variable(String),
    // TODO: Traversal(Traversal),
    // TODO: FuncCall(FuncCall),
}

impl TryFrom<Expression> for HclExpression {
    type Error = String;

    fn try_from(expr: Expression) -> Result<Self, Self::Error> {
        match expr {
            Expression::String(s) => Ok(HclExpression::String(s)),
            Expression::Number(n) => Ok(HclExpression::Number(n.into())),
            Expression::Boolean(b) => Ok(HclExpression::Bool(b)),
            Expression::Variable(v) => Ok(HclExpression::Variable(
                hcl::Variable::new(&v).map_err(|e| format!("failed to parse variable: {}", e))?,
            )),
            // TODO: Traversal(Traversal),
            // TODO: FuncCall(FuncCall),
        }
    }
}

impl TryFrom<HclExpression> for Expression {
    type Error = String;

    fn try_from(expr: HclExpression) -> Result<Self, Self::Error> {
        match expr {
            HclExpression::String(s) => Ok(Expression::String(s)),
            HclExpression::Number(n) => Ok(Expression::Number(n.as_i64().unwrap())),
            HclExpression::Bool(b) => Ok(Expression::Boolean(b)),
            HclExpression::Variable(v) => Ok(Expression::Variable(v.to_string())),
            _ => Err(format!("unsupported expression: {:?}", expr)),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "types.ts")]
#[serde(rename_all = "camelCase")]
pub struct VariableOptions {
    pub disabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "types.ts")]
#[serde(rename_all = "camelCase")]
pub struct AddVariableParams {
    pub name: VariableName,
    pub global_value: Option<Expression>,
    pub local_value: Option<Expression>,
    pub kind: Option<VariableKind>,
    pub order: isize,
    pub desc: Option<String>,
    pub options: VariableOptions,
}

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, TS, PartialEq, Eq)]
#[ts(export, export_to = "types.ts")]
pub enum VariableKind {
    #[serde(rename = "secret")]
    Secret,
    #[serde(rename = "default")]
    Default,
}

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct VariableInfo {
    pub name: VariableName,
    pub global_value: Option<Expression>,
    pub local_value: Option<Expression>,
    pub disabled: bool,
    pub kind: VariableKind,
    pub order: Option<isize>,
    pub desc: Option<String>,
}
