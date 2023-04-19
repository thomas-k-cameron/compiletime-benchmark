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
pub struct CreateCoipCidrInput {
    /// <p> A customer-owned IP address range to create. </p>
    #[doc(hidden)]
    pub cidr: std::option::Option<std::string::String>,
    /// <p> The ID of the address pool. </p>
    #[doc(hidden)]
    pub coip_pool_id: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl CreateCoipCidrInput {
    /// <p> A customer-owned IP address range to create. </p>
    pub fn cidr(&self) -> std::option::Option<&str> {
        self.cidr.as_deref()
    }
    /// <p> The ID of the address pool. </p>
    pub fn coip_pool_id(&self) -> std::option::Option<&str> {
        self.coip_pool_id.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl CreateCoipCidrInput {
    /// Creates a new builder-style object to manufacture [`CreateCoipCidrInput`](crate::operation::create_coip_cidr::CreateCoipCidrInput).
    pub fn builder() -> crate::operation::create_coip_cidr::builders::CreateCoipCidrInputBuilder {
        crate::operation::create_coip_cidr::builders::CreateCoipCidrInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_coip_cidr::CreateCoipCidrInput;
/// A builder for [`CreateCoipCidrInput`](crate::operation::create_coip_cidr::CreateCoipCidrInput).
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
pub struct CreateCoipCidrInputBuilder {
    pub(crate) cidr: std::option::Option<std::string::String>,
    pub(crate) coip_pool_id: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl CreateCoipCidrInputBuilder {
    /// <p> A customer-owned IP address range to create. </p>
    pub fn cidr(mut self, input: impl Into<std::string::String>) -> Self {
        self.cidr = Some(input.into());
        self
    }
    /// <p> A customer-owned IP address range to create. </p>
    pub fn set_cidr(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.cidr = input;
        self
    }
    /// <p> The ID of the address pool. </p>
    pub fn coip_pool_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.coip_pool_id = Some(input.into());
        self
    }
    /// <p> The ID of the address pool. </p>
    pub fn set_coip_pool_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.coip_pool_id = input;
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
    /// Consumes the builder and constructs a [`CreateCoipCidrInput`](crate::operation::create_coip_cidr::CreateCoipCidrInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::create_coip_cidr::CreateCoipCidrInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::create_coip_cidr::CreateCoipCidrInput {
            cidr: self.cidr,
            coip_pool_id: self.coip_pool_id,
            dry_run: self.dry_run,
        })
    }
}
