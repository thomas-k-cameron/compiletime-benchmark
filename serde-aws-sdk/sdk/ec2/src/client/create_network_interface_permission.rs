// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateNetworkInterfacePermission`](crate::operation::create_network_interface_permission::builders::CreateNetworkInterfacePermissionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`network_interface_id(impl Into<String>)`](crate::operation::create_network_interface_permission::builders::CreateNetworkInterfacePermissionFluentBuilder::network_interface_id) / [`set_network_interface_id(Option<String>)`](crate::operation::create_network_interface_permission::builders::CreateNetworkInterfacePermissionFluentBuilder::set_network_interface_id): <p>The ID of the network interface.</p>
    ///   - [`aws_account_id(impl Into<String>)`](crate::operation::create_network_interface_permission::builders::CreateNetworkInterfacePermissionFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::create_network_interface_permission::builders::CreateNetworkInterfacePermissionFluentBuilder::set_aws_account_id): <p>The Amazon Web Services account ID.</p>
    ///   - [`aws_service(impl Into<String>)`](crate::operation::create_network_interface_permission::builders::CreateNetworkInterfacePermissionFluentBuilder::aws_service) / [`set_aws_service(Option<String>)`](crate::operation::create_network_interface_permission::builders::CreateNetworkInterfacePermissionFluentBuilder::set_aws_service): <p>The Amazon Web Service. Currently not supported.</p>
    ///   - [`permission(InterfacePermissionType)`](crate::operation::create_network_interface_permission::builders::CreateNetworkInterfacePermissionFluentBuilder::permission) / [`set_permission(Option<InterfacePermissionType>)`](crate::operation::create_network_interface_permission::builders::CreateNetworkInterfacePermissionFluentBuilder::set_permission): <p>The type of permission to grant.</p>
    ///   - [`dry_run(bool)`](crate::operation::create_network_interface_permission::builders::CreateNetworkInterfacePermissionFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_network_interface_permission::builders::CreateNetworkInterfacePermissionFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`CreateNetworkInterfacePermissionOutput`](crate::operation::create_network_interface_permission::CreateNetworkInterfacePermissionOutput) with field(s):
    ///   - [`interface_permission(Option<NetworkInterfacePermission>)`](crate::operation::create_network_interface_permission::CreateNetworkInterfacePermissionOutput::interface_permission): <p>Information about the permission for the network interface.</p>
    /// - On failure, responds with [`SdkError<CreateNetworkInterfacePermissionError>`](crate::operation::create_network_interface_permission::CreateNetworkInterfacePermissionError)
    pub fn create_network_interface_permission(&self) -> crate::operation::create_network_interface_permission::builders::CreateNetworkInterfacePermissionFluentBuilder{
        crate::operation::create_network_interface_permission::builders::CreateNetworkInterfacePermissionFluentBuilder::new(self.handle.clone())
    }
}
