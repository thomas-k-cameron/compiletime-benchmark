// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for ModifyNetworkInterfaceAttribute.</p>
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
pub struct ModifyNetworkInterfaceAttributeInput {
    /// <p>Information about the interface attachment. If modifying the <code>delete on termination</code> attribute, you must specify the ID of the interface attachment.</p>
    #[doc(hidden)]
    pub attachment: std::option::Option<crate::types::NetworkInterfaceAttachmentChanges>,
    /// <p>A description for the network interface.</p>
    #[doc(hidden)]
    pub description: std::option::Option<crate::types::AttributeValue>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>Changes the security groups for the network interface. The new set of groups you specify replaces the current set. You must specify at least one group, even if it's just the default security group in the VPC. You must specify the ID of the security group, not the name.</p>
    #[doc(hidden)]
    pub groups: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The ID of the network interface.</p>
    #[doc(hidden)]
    pub network_interface_id: std::option::Option<std::string::String>,
    /// <p>Enable or disable source/destination checks, which ensure that the instance is either the source or the destination of any traffic that it receives. If the value is <code>true</code>, source/destination checks are enabled; otherwise, they are disabled. The default value is <code>true</code>. You must disable source/destination checks if the instance runs services such as network address translation, routing, or firewalls.</p>
    #[doc(hidden)]
    pub source_dest_check: std::option::Option<crate::types::AttributeBooleanValue>,
    /// <p>Updates the ENA Express configuration for the network interface that’s attached to the instance.</p>
    #[doc(hidden)]
    pub ena_srd_specification: std::option::Option<crate::types::EnaSrdSpecification>,
}
impl ModifyNetworkInterfaceAttributeInput {
    /// <p>Information about the interface attachment. If modifying the <code>delete on termination</code> attribute, you must specify the ID of the interface attachment.</p>
    pub fn attachment(
        &self,
    ) -> std::option::Option<&crate::types::NetworkInterfaceAttachmentChanges> {
        self.attachment.as_ref()
    }
    /// <p>A description for the network interface.</p>
    pub fn description(&self) -> std::option::Option<&crate::types::AttributeValue> {
        self.description.as_ref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>Changes the security groups for the network interface. The new set of groups you specify replaces the current set. You must specify at least one group, even if it's just the default security group in the VPC. You must specify the ID of the security group, not the name.</p>
    pub fn groups(&self) -> std::option::Option<&[std::string::String]> {
        self.groups.as_deref()
    }
    /// <p>The ID of the network interface.</p>
    pub fn network_interface_id(&self) -> std::option::Option<&str> {
        self.network_interface_id.as_deref()
    }
    /// <p>Enable or disable source/destination checks, which ensure that the instance is either the source or the destination of any traffic that it receives. If the value is <code>true</code>, source/destination checks are enabled; otherwise, they are disabled. The default value is <code>true</code>. You must disable source/destination checks if the instance runs services such as network address translation, routing, or firewalls.</p>
    pub fn source_dest_check(&self) -> std::option::Option<&crate::types::AttributeBooleanValue> {
        self.source_dest_check.as_ref()
    }
    /// <p>Updates the ENA Express configuration for the network interface that’s attached to the instance.</p>
    pub fn ena_srd_specification(&self) -> std::option::Option<&crate::types::EnaSrdSpecification> {
        self.ena_srd_specification.as_ref()
    }
}
impl ModifyNetworkInterfaceAttributeInput {
    /// Creates a new builder-style object to manufacture [`ModifyNetworkInterfaceAttributeInput`](crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeInput).
    pub fn builder() -> crate::operation::modify_network_interface_attribute::builders::ModifyNetworkInterfaceAttributeInputBuilder{
        crate::operation::modify_network_interface_attribute::builders::ModifyNetworkInterfaceAttributeInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeInput;
/// A builder for [`ModifyNetworkInterfaceAttributeInput`](crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeInput).
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
pub struct ModifyNetworkInterfaceAttributeInputBuilder {
    pub(crate) attachment: std::option::Option<crate::types::NetworkInterfaceAttachmentChanges>,
    pub(crate) description: std::option::Option<crate::types::AttributeValue>,
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) groups: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) network_interface_id: std::option::Option<std::string::String>,
    pub(crate) source_dest_check: std::option::Option<crate::types::AttributeBooleanValue>,
    pub(crate) ena_srd_specification: std::option::Option<crate::types::EnaSrdSpecification>,
}
impl ModifyNetworkInterfaceAttributeInputBuilder {
    /// <p>Information about the interface attachment. If modifying the <code>delete on termination</code> attribute, you must specify the ID of the interface attachment.</p>
    pub fn attachment(mut self, input: crate::types::NetworkInterfaceAttachmentChanges) -> Self {
        self.attachment = Some(input);
        self
    }
    /// <p>Information about the interface attachment. If modifying the <code>delete on termination</code> attribute, you must specify the ID of the interface attachment.</p>
    pub fn set_attachment(
        mut self,
        input: std::option::Option<crate::types::NetworkInterfaceAttachmentChanges>,
    ) -> Self {
        self.attachment = input;
        self
    }
    /// <p>A description for the network interface.</p>
    pub fn description(mut self, input: crate::types::AttributeValue) -> Self {
        self.description = Some(input);
        self
    }
    /// <p>A description for the network interface.</p>
    pub fn set_description(
        mut self,
        input: std::option::Option<crate::types::AttributeValue>,
    ) -> Self {
        self.description = input;
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
    /// Appends an item to `groups`.
    ///
    /// To override the contents of this collection use [`set_groups`](Self::set_groups).
    ///
    /// <p>Changes the security groups for the network interface. The new set of groups you specify replaces the current set. You must specify at least one group, even if it's just the default security group in the VPC. You must specify the ID of the security group, not the name.</p>
    pub fn groups(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.groups.unwrap_or_default();
        v.push(input.into());
        self.groups = Some(v);
        self
    }
    /// <p>Changes the security groups for the network interface. The new set of groups you specify replaces the current set. You must specify at least one group, even if it's just the default security group in the VPC. You must specify the ID of the security group, not the name.</p>
    pub fn set_groups(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.groups = input;
        self
    }
    /// <p>The ID of the network interface.</p>
    pub fn network_interface_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.network_interface_id = Some(input.into());
        self
    }
    /// <p>The ID of the network interface.</p>
    pub fn set_network_interface_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.network_interface_id = input;
        self
    }
    /// <p>Enable or disable source/destination checks, which ensure that the instance is either the source or the destination of any traffic that it receives. If the value is <code>true</code>, source/destination checks are enabled; otherwise, they are disabled. The default value is <code>true</code>. You must disable source/destination checks if the instance runs services such as network address translation, routing, or firewalls.</p>
    pub fn source_dest_check(mut self, input: crate::types::AttributeBooleanValue) -> Self {
        self.source_dest_check = Some(input);
        self
    }
    /// <p>Enable or disable source/destination checks, which ensure that the instance is either the source or the destination of any traffic that it receives. If the value is <code>true</code>, source/destination checks are enabled; otherwise, they are disabled. The default value is <code>true</code>. You must disable source/destination checks if the instance runs services such as network address translation, routing, or firewalls.</p>
    pub fn set_source_dest_check(
        mut self,
        input: std::option::Option<crate::types::AttributeBooleanValue>,
    ) -> Self {
        self.source_dest_check = input;
        self
    }
    /// <p>Updates the ENA Express configuration for the network interface that’s attached to the instance.</p>
    pub fn ena_srd_specification(mut self, input: crate::types::EnaSrdSpecification) -> Self {
        self.ena_srd_specification = Some(input);
        self
    }
    /// <p>Updates the ENA Express configuration for the network interface that’s attached to the instance.</p>
    pub fn set_ena_srd_specification(
        mut self,
        input: std::option::Option<crate::types::EnaSrdSpecification>,
    ) -> Self {
        self.ena_srd_specification = input;
        self
    }
    /// Consumes the builder and constructs a [`ModifyNetworkInterfaceAttributeInput`](crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeInput {
                attachment: self.attachment
                ,
                description: self.description
                ,
                dry_run: self.dry_run
                ,
                groups: self.groups
                ,
                network_interface_id: self.network_interface_id
                ,
                source_dest_check: self.source_dest_check
                ,
                ena_srd_specification: self.ena_srd_specification
                ,
            }
        )
    }
}
