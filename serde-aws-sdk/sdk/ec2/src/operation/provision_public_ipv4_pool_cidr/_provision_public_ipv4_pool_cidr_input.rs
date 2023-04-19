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
pub struct ProvisionPublicIpv4PoolCidrInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>The ID of the IPAM pool you would like to use to allocate this CIDR.</p>
    #[doc(hidden)]
    pub ipam_pool_id: std::option::Option<std::string::String>,
    /// <p>The ID of the public IPv4 pool you would like to use for this CIDR.</p>
    #[doc(hidden)]
    pub pool_id: std::option::Option<std::string::String>,
    /// <p>The netmask length of the CIDR you would like to allocate to the public IPv4 pool.</p>
    #[doc(hidden)]
    pub netmask_length: std::option::Option<i32>,
}
impl ProvisionPublicIpv4PoolCidrInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of the IPAM pool you would like to use to allocate this CIDR.</p>
    pub fn ipam_pool_id(&self) -> std::option::Option<&str> {
        self.ipam_pool_id.as_deref()
    }
    /// <p>The ID of the public IPv4 pool you would like to use for this CIDR.</p>
    pub fn pool_id(&self) -> std::option::Option<&str> {
        self.pool_id.as_deref()
    }
    /// <p>The netmask length of the CIDR you would like to allocate to the public IPv4 pool.</p>
    pub fn netmask_length(&self) -> std::option::Option<i32> {
        self.netmask_length
    }
}
impl ProvisionPublicIpv4PoolCidrInput {
    /// Creates a new builder-style object to manufacture [`ProvisionPublicIpv4PoolCidrInput`](crate::operation::provision_public_ipv4_pool_cidr::ProvisionPublicIpv4PoolCidrInput).
    pub fn builder() -> crate::operation::provision_public_ipv4_pool_cidr::builders::ProvisionPublicIpv4PoolCidrInputBuilder{
        crate::operation::provision_public_ipv4_pool_cidr::builders::ProvisionPublicIpv4PoolCidrInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::provision_public_ipv4_pool_cidr::ProvisionPublicIpv4PoolCidrInput;
/// A builder for [`ProvisionPublicIpv4PoolCidrInput`](crate::operation::provision_public_ipv4_pool_cidr::ProvisionPublicIpv4PoolCidrInput).
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
pub struct ProvisionPublicIpv4PoolCidrInputBuilder {
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) ipam_pool_id: std::option::Option<std::string::String>,
    pub(crate) pool_id: std::option::Option<std::string::String>,
    pub(crate) netmask_length: std::option::Option<i32>,
}
impl ProvisionPublicIpv4PoolCidrInputBuilder {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>The ID of the IPAM pool you would like to use to allocate this CIDR.</p>
    pub fn ipam_pool_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.ipam_pool_id = Some(input.into());
        self
    }
    /// <p>The ID of the IPAM pool you would like to use to allocate this CIDR.</p>
    pub fn set_ipam_pool_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.ipam_pool_id = input;
        self
    }
    /// <p>The ID of the public IPv4 pool you would like to use for this CIDR.</p>
    pub fn pool_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.pool_id = Some(input.into());
        self
    }
    /// <p>The ID of the public IPv4 pool you would like to use for this CIDR.</p>
    pub fn set_pool_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.pool_id = input;
        self
    }
    /// <p>The netmask length of the CIDR you would like to allocate to the public IPv4 pool.</p>
    pub fn netmask_length(mut self, input: i32) -> Self {
        self.netmask_length = Some(input);
        self
    }
    /// <p>The netmask length of the CIDR you would like to allocate to the public IPv4 pool.</p>
    pub fn set_netmask_length(mut self, input: std::option::Option<i32>) -> Self {
        self.netmask_length = input;
        self
    }
    /// Consumes the builder and constructs a [`ProvisionPublicIpv4PoolCidrInput`](crate::operation::provision_public_ipv4_pool_cidr::ProvisionPublicIpv4PoolCidrInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::provision_public_ipv4_pool_cidr::ProvisionPublicIpv4PoolCidrInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::provision_public_ipv4_pool_cidr::ProvisionPublicIpv4PoolCidrInput {
                dry_run: self.dry_run,
                ipam_pool_id: self.ipam_pool_id,
                pool_id: self.pool_id,
                netmask_length: self.netmask_length,
            },
        )
    }
}