// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListVaults`](crate::operation::list_vaults::builders::ListVaultsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_vaults::builders::ListVaultsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::operation::list_vaults::builders::ListVaultsFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::list_vaults::builders::ListVaultsFluentBuilder::set_account_id): <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID.</p>
    ///   - [`marker(impl Into<String>)`](crate::operation::list_vaults::builders::ListVaultsFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_vaults::builders::ListVaultsFluentBuilder::set_marker): <p>A string used for pagination. The marker specifies the vault ARN after which the listing of vaults should begin.</p>
    ///   - [`limit(i32)`](crate::operation::list_vaults::builders::ListVaultsFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::list_vaults::builders::ListVaultsFluentBuilder::set_limit): <p>The maximum number of vaults to be returned. The default limit is 10. The number of vaults returned might be fewer than the specified limit, but the number of returned vaults never exceeds the limit.</p>
    /// - On success, responds with [`ListVaultsOutput`](crate::operation::list_vaults::ListVaultsOutput) with field(s):
    ///   - [`vault_list(Option<Vec<DescribeVaultOutput>>)`](crate::operation::list_vaults::ListVaultsOutput::vault_list): <p>List of vaults.</p>
    ///   - [`marker(Option<String>)`](crate::operation::list_vaults::ListVaultsOutput::marker): <p>The vault ARN at which to continue pagination of the results. You use the marker in another List Vaults request to obtain more vaults in the list.</p>
    /// - On failure, responds with [`SdkError<ListVaultsError>`](crate::operation::list_vaults::ListVaultsError)
    pub fn list_vaults(&self) -> crate::operation::list_vaults::builders::ListVaultsFluentBuilder {
        crate::operation::list_vaults::builders::ListVaultsFluentBuilder::new(self.handle.clone())
    }
}