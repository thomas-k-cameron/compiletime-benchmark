// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`EnableAddressTransfer`](crate::operation::enable_address_transfer::builders::EnableAddressTransferFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`allocation_id(impl Into<String>)`](crate::operation::enable_address_transfer::builders::EnableAddressTransferFluentBuilder::allocation_id) / [`set_allocation_id(Option<String>)`](crate::operation::enable_address_transfer::builders::EnableAddressTransferFluentBuilder::set_allocation_id): <p>The allocation ID of an Elastic IP address.</p>
    ///   - [`transfer_account_id(impl Into<String>)`](crate::operation::enable_address_transfer::builders::EnableAddressTransferFluentBuilder::transfer_account_id) / [`set_transfer_account_id(Option<String>)`](crate::operation::enable_address_transfer::builders::EnableAddressTransferFluentBuilder::set_transfer_account_id): <p>The ID of the account that you want to transfer the Elastic IP address to.</p>
    ///   - [`dry_run(bool)`](crate::operation::enable_address_transfer::builders::EnableAddressTransferFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::enable_address_transfer::builders::EnableAddressTransferFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`EnableAddressTransferOutput`](crate::operation::enable_address_transfer::EnableAddressTransferOutput) with field(s):
    ///   - [`address_transfer(Option<AddressTransfer>)`](crate::operation::enable_address_transfer::EnableAddressTransferOutput::address_transfer): <p>An Elastic IP address transfer.</p>
    /// - On failure, responds with [`SdkError<EnableAddressTransferError>`](crate::operation::enable_address_transfer::EnableAddressTransferError)
    pub fn enable_address_transfer(
        &self,
    ) -> crate::operation::enable_address_transfer::builders::EnableAddressTransferFluentBuilder
    {
        crate::operation::enable_address_transfer::builders::EnableAddressTransferFluentBuilder::new(
            self.handle.clone(),
        )
    }
}