// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteStorageLensConfigurationTagging`](crate::operation::delete_storage_lens_configuration_tagging::builders::DeleteStorageLensConfigurationTaggingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`config_id(impl Into<String>)`](crate::operation::delete_storage_lens_configuration_tagging::builders::DeleteStorageLensConfigurationTaggingFluentBuilder::config_id) / [`set_config_id(Option<String>)`](crate::operation::delete_storage_lens_configuration_tagging::builders::DeleteStorageLensConfigurationTaggingFluentBuilder::set_config_id): <p>The ID of the S3 Storage Lens configuration.</p>
    ///   - [`account_id(impl Into<String>)`](crate::operation::delete_storage_lens_configuration_tagging::builders::DeleteStorageLensConfigurationTaggingFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::delete_storage_lens_configuration_tagging::builders::DeleteStorageLensConfigurationTaggingFluentBuilder::set_account_id): <p>The account ID of the requester.</p>
    /// - On success, responds with [`DeleteStorageLensConfigurationTaggingOutput`](crate::operation::delete_storage_lens_configuration_tagging::DeleteStorageLensConfigurationTaggingOutput)
    /// - On failure, responds with [`SdkError<DeleteStorageLensConfigurationTaggingError>`](crate::operation::delete_storage_lens_configuration_tagging::DeleteStorageLensConfigurationTaggingError)
    pub fn delete_storage_lens_configuration_tagging(&self) -> crate::operation::delete_storage_lens_configuration_tagging::builders::DeleteStorageLensConfigurationTaggingFluentBuilder{
        crate::operation::delete_storage_lens_configuration_tagging::builders::DeleteStorageLensConfigurationTaggingFluentBuilder::new(self.handle.clone())
    }
}
