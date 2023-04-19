// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A complex type that contains the type of limit that you specified in the request and the current value for that limit.</p>
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ReusableDelegationSetLimit {
    /// <p>The limit that you requested: <code>MAX_ZONES_BY_REUSABLE_DELEGATION_SET</code>, the maximum number of hosted zones that you can associate with the specified reusable delegation set.</p>
    #[doc(hidden)]
    pub r#type: std::option::Option<crate::types::ReusableDelegationSetLimitType>,
    /// <p>The current value for the <code>MAX_ZONES_BY_REUSABLE_DELEGATION_SET</code> limit.</p>
    #[doc(hidden)]
    pub value: i64,
}
impl ReusableDelegationSetLimit {
    /// <p>The limit that you requested: <code>MAX_ZONES_BY_REUSABLE_DELEGATION_SET</code>, the maximum number of hosted zones that you can associate with the specified reusable delegation set.</p>
    pub fn r#type(&self) -> std::option::Option<&crate::types::ReusableDelegationSetLimitType> {
        self.r#type.as_ref()
    }
    /// <p>The current value for the <code>MAX_ZONES_BY_REUSABLE_DELEGATION_SET</code> limit.</p>
    pub fn value(&self) -> i64 {
        self.value
    }
}
impl ReusableDelegationSetLimit {
    /// Creates a new builder-style object to manufacture [`ReusableDelegationSetLimit`](crate::types::ReusableDelegationSetLimit).
    pub fn builder() -> crate::types::builders::ReusableDelegationSetLimitBuilder {
        crate::types::builders::ReusableDelegationSetLimitBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ReusableDelegationSetLimit;
/// A builder for [`ReusableDelegationSetLimit`](crate::types::ReusableDelegationSetLimit).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct ReusableDelegationSetLimitBuilder {
    pub(crate) r#type: std::option::Option<crate::types::ReusableDelegationSetLimitType>,
    pub(crate) value: std::option::Option<i64>,
}
impl ReusableDelegationSetLimitBuilder {
    /// <p>The limit that you requested: <code>MAX_ZONES_BY_REUSABLE_DELEGATION_SET</code>, the maximum number of hosted zones that you can associate with the specified reusable delegation set.</p>
    pub fn r#type(mut self, input: crate::types::ReusableDelegationSetLimitType) -> Self {
        self.r#type = Some(input);
        self
    }
    /// <p>The limit that you requested: <code>MAX_ZONES_BY_REUSABLE_DELEGATION_SET</code>, the maximum number of hosted zones that you can associate with the specified reusable delegation set.</p>
    pub fn set_type(
        mut self,
        input: std::option::Option<crate::types::ReusableDelegationSetLimitType>,
    ) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The current value for the <code>MAX_ZONES_BY_REUSABLE_DELEGATION_SET</code> limit.</p>
    pub fn value(mut self, input: i64) -> Self {
        self.value = Some(input);
        self
    }
    /// <p>The current value for the <code>MAX_ZONES_BY_REUSABLE_DELEGATION_SET</code> limit.</p>
    pub fn set_value(mut self, input: std::option::Option<i64>) -> Self {
        self.value = input;
        self
    }
    /// Consumes the builder and constructs a [`ReusableDelegationSetLimit`](crate::types::ReusableDelegationSetLimit).
    pub fn build(self) -> crate::types::ReusableDelegationSetLimit {
        crate::types::ReusableDelegationSetLimit {
            r#type: self.r#type,
            value: self.value.unwrap_or_default(),
        }
    }
}
