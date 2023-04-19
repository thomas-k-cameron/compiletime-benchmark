// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisableAddressTransfer`](crate::operation::disable_address_transfer::builders::DisableAddressTransferFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`allocation_id(impl Into<String>)`](crate::operation::disable_address_transfer::builders::DisableAddressTransferFluentBuilder::allocation_id) / [`set_allocation_id(Option<String>)`](crate::operation::disable_address_transfer::builders::DisableAddressTransferFluentBuilder::set_allocation_id): <p>The allocation ID of an Elastic IP address.</p>
    ///   - [`dry_run(bool)`](crate::operation::disable_address_transfer::builders::DisableAddressTransferFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::disable_address_transfer::builders::DisableAddressTransferFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`DisableAddressTransferOutput`](crate::operation::disable_address_transfer::DisableAddressTransferOutput) with field(s):
    ///   - [`address_transfer(Option<AddressTransfer>)`](crate::operation::disable_address_transfer::DisableAddressTransferOutput::address_transfer): <p>An Elastic IP address transfer.</p>
    /// - On failure, responds with [`SdkError<DisableAddressTransferError>`](crate::operation::disable_address_transfer::DisableAddressTransferError)
    pub fn disable_address_transfer(
        &self,
    ) -> crate::operation::disable_address_transfer::builders::DisableAddressTransferFluentBuilder
    {
        crate::operation::disable_address_transfer::builders::DisableAddressTransferFluentBuilder::new(self.handle.clone())
    }
}
