// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetStorageLensConfiguration`](crate::operation::get_storage_lens_configuration::builders::GetStorageLensConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`config_id(impl Into<String>)`](crate::operation::get_storage_lens_configuration::builders::GetStorageLensConfigurationFluentBuilder::config_id) / [`set_config_id(Option<String>)`](crate::operation::get_storage_lens_configuration::builders::GetStorageLensConfigurationFluentBuilder::set_config_id): <p>The ID of the Amazon S3 Storage Lens configuration.</p>
    ///   - [`account_id(impl Into<String>)`](crate::operation::get_storage_lens_configuration::builders::GetStorageLensConfigurationFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::get_storage_lens_configuration::builders::GetStorageLensConfigurationFluentBuilder::set_account_id): <p>The account ID of the requester.</p>
    /// - On success, responds with [`GetStorageLensConfigurationOutput`](crate::operation::get_storage_lens_configuration::GetStorageLensConfigurationOutput) with field(s):
    ///   - [`storage_lens_configuration(Option<StorageLensConfiguration>)`](crate::operation::get_storage_lens_configuration::GetStorageLensConfigurationOutput::storage_lens_configuration): <p>The S3 Storage Lens configuration requested.</p>
    /// - On failure, responds with [`SdkError<GetStorageLensConfigurationError>`](crate::operation::get_storage_lens_configuration::GetStorageLensConfigurationError)
    pub fn get_storage_lens_configuration(&self) -> crate::operation::get_storage_lens_configuration::builders::GetStorageLensConfigurationFluentBuilder{
        crate::operation::get_storage_lens_configuration::builders::GetStorageLensConfigurationFluentBuilder::new(self.handle.clone())
    }
}
