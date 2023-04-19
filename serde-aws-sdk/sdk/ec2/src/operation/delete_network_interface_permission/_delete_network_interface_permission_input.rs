// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for DeleteNetworkInterfacePermission.</p>
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
pub struct DeleteNetworkInterfacePermissionInput {
    /// <p>The ID of the network interface permission.</p>
    #[doc(hidden)]
    pub network_interface_permission_id: std::option::Option<std::string::String>,
    /// <p>Specify <code>true</code> to remove the permission even if the network interface is attached to an instance.</p>
    #[doc(hidden)]
    pub force: std::option::Option<bool>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl DeleteNetworkInterfacePermissionInput {
    /// <p>The ID of the network interface permission.</p>
    pub fn network_interface_permission_id(&self) -> std::option::Option<&str> {
        self.network_interface_permission_id.as_deref()
    }
    /// <p>Specify <code>true</code> to remove the permission even if the network interface is attached to an instance.</p>
    pub fn force(&self) -> std::option::Option<bool> {
        self.force
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl DeleteNetworkInterfacePermissionInput {
    /// Creates a new builder-style object to manufacture [`DeleteNetworkInterfacePermissionInput`](crate::operation::delete_network_interface_permission::DeleteNetworkInterfacePermissionInput).
    pub fn builder() -> crate::operation::delete_network_interface_permission::builders::DeleteNetworkInterfacePermissionInputBuilder{
        crate::operation::delete_network_interface_permission::builders::DeleteNetworkInterfacePermissionInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::delete_network_interface_permission::DeleteNetworkInterfacePermissionInput;
/// A builder for [`DeleteNetworkInterfacePermissionInput`](crate::operation::delete_network_interface_permission::DeleteNetworkInterfacePermissionInput).
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
pub struct DeleteNetworkInterfacePermissionInputBuilder {
    pub(crate) network_interface_permission_id: std::option::Option<std::string::String>,
    pub(crate) force: std::option::Option<bool>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl DeleteNetworkInterfacePermissionInputBuilder {
    /// <p>The ID of the network interface permission.</p>
    pub fn network_interface_permission_id(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.network_interface_permission_id = Some(input.into());
        self
    }
    /// <p>The ID of the network interface permission.</p>
    pub fn set_network_interface_permission_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.network_interface_permission_id = input;
        self
    }
    /// <p>Specify <code>true</code> to remove the permission even if the network interface is attached to an instance.</p>
    pub fn force(mut self, input: bool) -> Self {
        self.force = Some(input);
        self
    }
    /// <p>Specify <code>true</code> to remove the permission even if the network interface is attached to an instance.</p>
    pub fn set_force(mut self, input: std::option::Option<bool>) -> Self {
        self.force = input;
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
    /// Consumes the builder and constructs a [`DeleteNetworkInterfacePermissionInput`](crate::operation::delete_network_interface_permission::DeleteNetworkInterfacePermissionInput).
    pub fn build(self) -> Result<crate::operation::delete_network_interface_permission::DeleteNetworkInterfacePermissionInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::delete_network_interface_permission::DeleteNetworkInterfacePermissionInput {
                network_interface_permission_id: self.network_interface_permission_id
                ,
                force: self.force
                ,
                dry_run: self.dry_run
                ,
            }
        )
    }
}
