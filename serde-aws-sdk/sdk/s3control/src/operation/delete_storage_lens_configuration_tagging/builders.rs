// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_storage_lens_configuration_tagging::_delete_storage_lens_configuration_tagging_output::DeleteStorageLensConfigurationTaggingOutputBuilder;

pub use crate::operation::delete_storage_lens_configuration_tagging::_delete_storage_lens_configuration_tagging_input::DeleteStorageLensConfigurationTaggingInputBuilder;

/// Fluent builder constructing a request to `DeleteStorageLensConfigurationTagging`.
///
/// <p>Deletes the Amazon S3 Storage Lens configuration tags. For more information about S3 Storage Lens, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage_lens.html">Assessing your storage activity and usage with Amazon S3 Storage Lens </a> in the <i>Amazon S3 User Guide</i>.</p> <note>
/// <p>To use this action, you must have permission to perform the <code>s3:DeleteStorageLensConfigurationTagging</code> action. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage_lens_iam_permissions.html">Setting permissions to use Amazon S3 Storage Lens</a> in the <i>Amazon S3 User Guide</i>.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteStorageLensConfigurationTaggingFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::delete_storage_lens_configuration_tagging::builders::DeleteStorageLensConfigurationTaggingInputBuilder
            }
impl DeleteStorageLensConfigurationTaggingFluentBuilder {
    /// Creates a new `DeleteStorageLensConfigurationTagging`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::delete_storage_lens_configuration_tagging::DeleteStorageLensConfigurationTagging, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::delete_storage_lens_configuration_tagging::DeleteStorageLensConfigurationTaggingError>
    >{
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::operation::delete_storage_lens_configuration_tagging::DeleteStorageLensConfigurationTaggingOutput, aws_smithy_http::result::SdkError<crate::operation::delete_storage_lens_configuration_tagging::DeleteStorageLensConfigurationTaggingError>>
                     {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    #[cfg(aws_sdk_unstable)]
    /// This function replaces the parameter with new one.
    /// It is useful when you want to replace the existing data with de-serialized data.
    /// ```compile_fail
    /// let result_future = async {
    ///     let deserialized_parameters: crate::operation::delete_storage_lens_configuration_tagging::builders::DeleteStorageLensConfigurationTaggingInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.delete_storage_lens_configuration_tagging().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::delete_storage_lens_configuration_tagging::builders::DeleteStorageLensConfigurationTaggingInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID of the S3 Storage Lens configuration.</p>
    pub fn config_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.config_id(input.into());
        self
    }
    /// <p>The ID of the S3 Storage Lens configuration.</p>
    pub fn set_config_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_config_id(input);
        self
    }
    /// <p>The account ID of the requester.</p>
    pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The account ID of the requester.</p>
    pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
}
