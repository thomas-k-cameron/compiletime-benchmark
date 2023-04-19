// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
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
pub struct PutMultiRegionAccessPointPolicyInput {
    /// <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    #[doc(hidden)]
    pub account_id: std::option::Option<std::string::String>,
    /// <p>An idempotency token used to identify the request and guarantee that requests are unique.</p>
    #[doc(hidden)]
    pub client_token: std::option::Option<std::string::String>,
    /// <p>A container element containing the details of the policy for the Multi-Region Access Point.</p>
    #[doc(hidden)]
    pub details: std::option::Option<crate::types::PutMultiRegionAccessPointPolicyInput>,
}
impl PutMultiRegionAccessPointPolicyInput {
    /// <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    pub fn account_id(&self) -> std::option::Option<&str> {
        self.account_id.as_deref()
    }
    /// <p>An idempotency token used to identify the request and guarantee that requests are unique.</p>
    pub fn client_token(&self) -> std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>A container element containing the details of the policy for the Multi-Region Access Point.</p>
    pub fn details(
        &self,
    ) -> std::option::Option<&crate::types::PutMultiRegionAccessPointPolicyInput> {
        self.details.as_ref()
    }
}
impl PutMultiRegionAccessPointPolicyInput {
    /// Creates a new builder-style object to manufacture [`PutMultiRegionAccessPointPolicyInput`](crate::operation::put_multi_region_access_point_policy::PutMultiRegionAccessPointPolicyInput).
    pub fn builder() -> crate::operation::put_multi_region_access_point_policy::builders::PutMultiRegionAccessPointPolicyInputBuilder{
        crate::operation::put_multi_region_access_point_policy::builders::PutMultiRegionAccessPointPolicyInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::put_multi_region_access_point_policy::PutMultiRegionAccessPointPolicyInput;
/// A builder for [`PutMultiRegionAccessPointPolicyInput`](crate::operation::put_multi_region_access_point_policy::PutMultiRegionAccessPointPolicyInput).
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
pub struct PutMultiRegionAccessPointPolicyInputBuilder {
    pub(crate) account_id: std::option::Option<std::string::String>,
    pub(crate) client_token: std::option::Option<std::string::String>,
    pub(crate) details: std::option::Option<crate::types::PutMultiRegionAccessPointPolicyInput>,
}
impl PutMultiRegionAccessPointPolicyInputBuilder {
    /// <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.account_id = Some(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.account_id = input;
        self
    }
    /// <p>An idempotency token used to identify the request and guarantee that requests are unique.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.client_token = Some(input.into());
        self
    }
    /// <p>An idempotency token used to identify the request and guarantee that requests are unique.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>A container element containing the details of the policy for the Multi-Region Access Point.</p>
    pub fn details(mut self, input: crate::types::PutMultiRegionAccessPointPolicyInput) -> Self {
        self.details = Some(input);
        self
    }
    /// <p>A container element containing the details of the policy for the Multi-Region Access Point.</p>
    pub fn set_details(
        mut self,
        input: std::option::Option<crate::types::PutMultiRegionAccessPointPolicyInput>,
    ) -> Self {
        self.details = input;
        self
    }
    /// Consumes the builder and constructs a [`PutMultiRegionAccessPointPolicyInput`](crate::operation::put_multi_region_access_point_policy::PutMultiRegionAccessPointPolicyInput).
    pub fn build(self) -> Result<crate::operation::put_multi_region_access_point_policy::PutMultiRegionAccessPointPolicyInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::put_multi_region_access_point_policy::PutMultiRegionAccessPointPolicyInput {
                account_id: self.account_id
                ,
                client_token: self.client_token
                ,
                details: self.details
                ,
            }
        )
    }
}