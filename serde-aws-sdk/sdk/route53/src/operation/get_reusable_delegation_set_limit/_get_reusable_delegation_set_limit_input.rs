// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A complex type that contains information about the request to create a hosted zone.</p>
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
pub struct GetReusableDelegationSetLimitInput {
    /// <p>Specify <code>MAX_ZONES_BY_REUSABLE_DELEGATION_SET</code> to get the maximum number of hosted zones that you can associate with the specified reusable delegation set.</p>
    #[doc(hidden)]
    pub r#type: std::option::Option<crate::types::ReusableDelegationSetLimitType>,
    /// <p>The ID of the delegation set that you want to get the limit for.</p>
    #[doc(hidden)]
    pub delegation_set_id: std::option::Option<std::string::String>,
}
impl GetReusableDelegationSetLimitInput {
    /// <p>Specify <code>MAX_ZONES_BY_REUSABLE_DELEGATION_SET</code> to get the maximum number of hosted zones that you can associate with the specified reusable delegation set.</p>
    pub fn r#type(&self) -> std::option::Option<&crate::types::ReusableDelegationSetLimitType> {
        self.r#type.as_ref()
    }
    /// <p>The ID of the delegation set that you want to get the limit for.</p>
    pub fn delegation_set_id(&self) -> std::option::Option<&str> {
        self.delegation_set_id.as_deref()
    }
}
impl GetReusableDelegationSetLimitInput {
    /// Creates a new builder-style object to manufacture [`GetReusableDelegationSetLimitInput`](crate::operation::get_reusable_delegation_set_limit::GetReusableDelegationSetLimitInput).
    pub fn builder() -> crate::operation::get_reusable_delegation_set_limit::builders::GetReusableDelegationSetLimitInputBuilder{
        crate::operation::get_reusable_delegation_set_limit::builders::GetReusableDelegationSetLimitInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::get_reusable_delegation_set_limit::GetReusableDelegationSetLimitInput;
/// A builder for [`GetReusableDelegationSetLimitInput`](crate::operation::get_reusable_delegation_set_limit::GetReusableDelegationSetLimitInput).
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
pub struct GetReusableDelegationSetLimitInputBuilder {
    pub(crate) r#type: std::option::Option<crate::types::ReusableDelegationSetLimitType>,
    pub(crate) delegation_set_id: std::option::Option<std::string::String>,
}
impl GetReusableDelegationSetLimitInputBuilder {
    /// <p>Specify <code>MAX_ZONES_BY_REUSABLE_DELEGATION_SET</code> to get the maximum number of hosted zones that you can associate with the specified reusable delegation set.</p>
    pub fn r#type(mut self, input: crate::types::ReusableDelegationSetLimitType) -> Self {
        self.r#type = Some(input);
        self
    }
    /// <p>Specify <code>MAX_ZONES_BY_REUSABLE_DELEGATION_SET</code> to get the maximum number of hosted zones that you can associate with the specified reusable delegation set.</p>
    pub fn set_type(
        mut self,
        input: std::option::Option<crate::types::ReusableDelegationSetLimitType>,
    ) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The ID of the delegation set that you want to get the limit for.</p>
    pub fn delegation_set_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.delegation_set_id = Some(input.into());
        self
    }
    /// <p>The ID of the delegation set that you want to get the limit for.</p>
    pub fn set_delegation_set_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.delegation_set_id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetReusableDelegationSetLimitInput`](crate::operation::get_reusable_delegation_set_limit::GetReusableDelegationSetLimitInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::get_reusable_delegation_set_limit::GetReusableDelegationSetLimitInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::get_reusable_delegation_set_limit::GetReusableDelegationSetLimitInput {
                r#type: self.r#type
                ,
                delegation_set_id: self.delegation_set_id
                ,
            }
        )
    }
}
