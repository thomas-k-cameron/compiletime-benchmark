// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_storage_lens_configuration::_put_storage_lens_configuration_output::PutStorageLensConfigurationOutputBuilder;

pub use crate::operation::put_storage_lens_configuration::_put_storage_lens_configuration_input::PutStorageLensConfigurationInputBuilder;

/// Fluent builder constructing a request to `PutStorageLensConfiguration`.
///
/// <p>Puts an Amazon S3 Storage Lens configuration. For more information about S3 Storage Lens, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage_lens.html">Working with Amazon S3 Storage Lens</a> in the <i>Amazon S3 User Guide</i>. For a complete list of S3 Storage Lens metrics, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/storage_lens_metrics_glossary.html">S3 Storage Lens metrics glossary</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
/// <p>To use this action, you must have permission to perform the <code>s3:PutStorageLensConfiguration</code> action. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage_lens_iam_permissions.html">Setting permissions to use Amazon S3 Storage Lens</a> in the <i>Amazon S3 User Guide</i>.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct PutStorageLensConfigurationFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::put_storage_lens_configuration::builders::PutStorageLensConfigurationInputBuilder
            }
impl PutStorageLensConfigurationFluentBuilder {
    /// Creates a new `PutStorageLensConfiguration`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::put_storage_lens_configuration::PutStorageLensConfiguration,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::put_storage_lens_configuration::PutStorageLensConfigurationError,
        >,
    > {
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
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::put_storage_lens_configuration::PutStorageLensConfigurationOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::put_storage_lens_configuration::PutStorageLensConfigurationError,
        >,
    > {
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
    ///     let deserialized_parameters: crate::operation::put_storage_lens_configuration::builders::PutStorageLensConfigurationInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.put_storage_lens_configuration().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::put_storage_lens_configuration::builders::PutStorageLensConfigurationInputBuilder,
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
    /// <p>The S3 Storage Lens configuration.</p>
    pub fn storage_lens_configuration(
        mut self,
        input: crate::types::StorageLensConfiguration,
    ) -> Self {
        self.inner = self.inner.storage_lens_configuration(input);
        self
    }
    /// <p>The S3 Storage Lens configuration.</p>
    pub fn set_storage_lens_configuration(
        mut self,
        input: std::option::Option<crate::types::StorageLensConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_storage_lens_configuration(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tag set of the S3 Storage Lens configuration.</p> <note>
    /// <p>You can set up to a maximum of 50 tags.</p>
    /// </note>
    pub fn tags(mut self, input: crate::types::StorageLensTag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The tag set of the S3 Storage Lens configuration.</p> <note>
    /// <p>You can set up to a maximum of 50 tags.</p>
    /// </note>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::StorageLensTag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
