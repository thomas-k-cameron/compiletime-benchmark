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
pub struct DeleteVerifiedAccessEndpointInput {
    /// <p>The ID of the Amazon Web Services Verified Access endpoint.</p>
    #[doc(hidden)]
    pub verified_access_endpoint_id: std::option::Option<std::string::String>,
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    #[doc(hidden)]
    pub client_token: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl DeleteVerifiedAccessEndpointInput {
    /// <p>The ID of the Amazon Web Services Verified Access endpoint.</p>
    pub fn verified_access_endpoint_id(&self) -> std::option::Option<&str> {
        self.verified_access_endpoint_id.as_deref()
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(&self) -> std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl DeleteVerifiedAccessEndpointInput {
    /// Creates a new builder-style object to manufacture [`DeleteVerifiedAccessEndpointInput`](crate::operation::delete_verified_access_endpoint::DeleteVerifiedAccessEndpointInput).
    pub fn builder() -> crate::operation::delete_verified_access_endpoint::builders::DeleteVerifiedAccessEndpointInputBuilder{
        crate::operation::delete_verified_access_endpoint::builders::DeleteVerifiedAccessEndpointInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::delete_verified_access_endpoint::DeleteVerifiedAccessEndpointInput;
/// A builder for [`DeleteVerifiedAccessEndpointInput`](crate::operation::delete_verified_access_endpoint::DeleteVerifiedAccessEndpointInput).
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
pub struct DeleteVerifiedAccessEndpointInputBuilder {
    pub(crate) verified_access_endpoint_id: std::option::Option<std::string::String>,
    pub(crate) client_token: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl DeleteVerifiedAccessEndpointInputBuilder {
    /// <p>The ID of the Amazon Web Services Verified Access endpoint.</p>
    pub fn verified_access_endpoint_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.verified_access_endpoint_id = Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services Verified Access endpoint.</p>
    pub fn set_verified_access_endpoint_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.verified_access_endpoint_id = input;
        self
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.client_token = Some(input.into());
        self
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteVerifiedAccessEndpointInput`](crate::operation::delete_verified_access_endpoint::DeleteVerifiedAccessEndpointInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_verified_access_endpoint::DeleteVerifiedAccessEndpointInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::delete_verified_access_endpoint::DeleteVerifiedAccessEndpointInput {
                verified_access_endpoint_id: self.verified_access_endpoint_id,
                client_token: self.client_token,
                dry_run: self.dry_run,
            },
        )
    }
}
