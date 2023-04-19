// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ResetNetworkInterfaceAttribute`](crate::operation::reset_network_interface_attribute::builders::ResetNetworkInterfaceAttributeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::reset_network_interface_attribute::builders::ResetNetworkInterfaceAttributeFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::reset_network_interface_attribute::builders::ResetNetworkInterfaceAttributeFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`network_interface_id(impl Into<String>)`](crate::operation::reset_network_interface_attribute::builders::ResetNetworkInterfaceAttributeFluentBuilder::network_interface_id) / [`set_network_interface_id(Option<String>)`](crate::operation::reset_network_interface_attribute::builders::ResetNetworkInterfaceAttributeFluentBuilder::set_network_interface_id): <p>The ID of the network interface.</p>
    ///   - [`source_dest_check(impl Into<String>)`](crate::operation::reset_network_interface_attribute::builders::ResetNetworkInterfaceAttributeFluentBuilder::source_dest_check) / [`set_source_dest_check(Option<String>)`](crate::operation::reset_network_interface_attribute::builders::ResetNetworkInterfaceAttributeFluentBuilder::set_source_dest_check): <p>The source/destination checking attribute. Resets the value to <code>true</code>.</p>
    /// - On success, responds with [`ResetNetworkInterfaceAttributeOutput`](crate::operation::reset_network_interface_attribute::ResetNetworkInterfaceAttributeOutput)
    /// - On failure, responds with [`SdkError<ResetNetworkInterfaceAttributeError>`](crate::operation::reset_network_interface_attribute::ResetNetworkInterfaceAttributeError)
    pub fn reset_network_interface_attribute(&self) -> crate::operation::reset_network_interface_attribute::builders::ResetNetworkInterfaceAttributeFluentBuilder{
        crate::operation::reset_network_interface_attribute::builders::ResetNetworkInterfaceAttributeFluentBuilder::new(self.handle.clone())
    }
}
