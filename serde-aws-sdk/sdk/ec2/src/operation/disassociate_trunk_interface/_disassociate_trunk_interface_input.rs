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
pub struct DisassociateTrunkInterfaceInput {
    /// <p>The ID of the association</p>
    #[doc(hidden)]
    pub association_id: std::option::Option<std::string::String>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to Ensure Idempotency</a>.</p>
    #[doc(hidden)]
    pub client_token: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl DisassociateTrunkInterfaceInput {
    /// <p>The ID of the association</p>
    pub fn association_id(&self) -> std::option::Option<&str> {
        self.association_id.as_deref()
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to Ensure Idempotency</a>.</p>
    pub fn client_token(&self) -> std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl DisassociateTrunkInterfaceInput {
    /// Creates a new builder-style object to manufacture [`DisassociateTrunkInterfaceInput`](crate::operation::disassociate_trunk_interface::DisassociateTrunkInterfaceInput).
    pub fn builder() -> crate::operation::disassociate_trunk_interface::builders::DisassociateTrunkInterfaceInputBuilder{
        crate::operation::disassociate_trunk_interface::builders::DisassociateTrunkInterfaceInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::disassociate_trunk_interface::DisassociateTrunkInterfaceInput;
/// A builder for [`DisassociateTrunkInterfaceInput`](crate::operation::disassociate_trunk_interface::DisassociateTrunkInterfaceInput).
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
pub struct DisassociateTrunkInterfaceInputBuilder {
    pub(crate) association_id: std::option::Option<std::string::String>,
    pub(crate) client_token: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl DisassociateTrunkInterfaceInputBuilder {
    /// <p>The ID of the association</p>
    pub fn association_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.association_id = Some(input.into());
        self
    }
    /// <p>The ID of the association</p>
    pub fn set_association_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.association_id = input;
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to Ensure Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.client_token = Some(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to Ensure Idempotency</a>.</p>
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
    /// Consumes the builder and constructs a [`DisassociateTrunkInterfaceInput`](crate::operation::disassociate_trunk_interface::DisassociateTrunkInterfaceInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::disassociate_trunk_interface::DisassociateTrunkInterfaceInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::disassociate_trunk_interface::DisassociateTrunkInterfaceInput {
                association_id: self.association_id,
                client_token: self.client_token,
                dry_run: self.dry_run,
            },
        )
    }
}
