// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyNetworkInterfaceAttribute`](crate::operation::modify_network_interface_attribute::builders::ModifyNetworkInterfaceAttributeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`attachment(NetworkInterfaceAttachmentChanges)`](crate::operation::modify_network_interface_attribute::builders::ModifyNetworkInterfaceAttributeFluentBuilder::attachment) / [`set_attachment(Option<NetworkInterfaceAttachmentChanges>)`](crate::operation::modify_network_interface_attribute::builders::ModifyNetworkInterfaceAttributeFluentBuilder::set_attachment): <p>Information about the interface attachment. If modifying the <code>delete on termination</code> attribute, you must specify the ID of the interface attachment.</p>
    ///   - [`description(AttributeValue)`](crate::operation::modify_network_interface_attribute::builders::ModifyNetworkInterfaceAttributeFluentBuilder::description) / [`set_description(Option<AttributeValue>)`](crate::operation::modify_network_interface_attribute::builders::ModifyNetworkInterfaceAttributeFluentBuilder::set_description): <p>A description for the network interface.</p>
    ///   - [`dry_run(bool)`](crate::operation::modify_network_interface_attribute::builders::ModifyNetworkInterfaceAttributeFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_network_interface_attribute::builders::ModifyNetworkInterfaceAttributeFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`groups(Vec<String>)`](crate::operation::modify_network_interface_attribute::builders::ModifyNetworkInterfaceAttributeFluentBuilder::groups) / [`set_groups(Option<Vec<String>>)`](crate::operation::modify_network_interface_attribute::builders::ModifyNetworkInterfaceAttributeFluentBuilder::set_groups): <p>Changes the security groups for the network interface. The new set of groups you specify replaces the current set. You must specify at least one group, even if it's just the default security group in the VPC. You must specify the ID of the security group, not the name.</p>
    ///   - [`network_interface_id(impl Into<String>)`](crate::operation::modify_network_interface_attribute::builders::ModifyNetworkInterfaceAttributeFluentBuilder::network_interface_id) / [`set_network_interface_id(Option<String>)`](crate::operation::modify_network_interface_attribute::builders::ModifyNetworkInterfaceAttributeFluentBuilder::set_network_interface_id): <p>The ID of the network interface.</p>
    ///   - [`source_dest_check(AttributeBooleanValue)`](crate::operation::modify_network_interface_attribute::builders::ModifyNetworkInterfaceAttributeFluentBuilder::source_dest_check) / [`set_source_dest_check(Option<AttributeBooleanValue>)`](crate::operation::modify_network_interface_attribute::builders::ModifyNetworkInterfaceAttributeFluentBuilder::set_source_dest_check): <p>Enable or disable source/destination checks, which ensure that the instance is either the source or the destination of any traffic that it receives. If the value is <code>true</code>, source/destination checks are enabled; otherwise, they are disabled. The default value is <code>true</code>. You must disable source/destination checks if the instance runs services such as network address translation, routing, or firewalls.</p>
    ///   - [`ena_srd_specification(EnaSrdSpecification)`](crate::operation::modify_network_interface_attribute::builders::ModifyNetworkInterfaceAttributeFluentBuilder::ena_srd_specification) / [`set_ena_srd_specification(Option<EnaSrdSpecification>)`](crate::operation::modify_network_interface_attribute::builders::ModifyNetworkInterfaceAttributeFluentBuilder::set_ena_srd_specification): <p>Updates the ENA Express configuration for the network interface that’s attached to the instance.</p>
    /// - On success, responds with [`ModifyNetworkInterfaceAttributeOutput`](crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeOutput)
    /// - On failure, responds with [`SdkError<ModifyNetworkInterfaceAttributeError>`](crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeError)
    pub fn modify_network_interface_attribute(&self) -> crate::operation::modify_network_interface_attribute::builders::ModifyNetworkInterfaceAttributeFluentBuilder{
        crate::operation::modify_network_interface_attribute::builders::ModifyNetworkInterfaceAttributeFluentBuilder::new(self.handle.clone())
    }
}
