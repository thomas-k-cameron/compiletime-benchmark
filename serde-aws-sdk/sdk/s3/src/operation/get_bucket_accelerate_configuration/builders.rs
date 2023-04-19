// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_bucket_accelerate_configuration::_get_bucket_accelerate_configuration_output::GetBucketAccelerateConfigurationOutputBuilder;

pub use crate::operation::get_bucket_accelerate_configuration::_get_bucket_accelerate_configuration_input::GetBucketAccelerateConfigurationInputBuilder;

/// Fluent builder constructing a request to `GetBucketAccelerateConfiguration`.
///
/// <p>This implementation of the GET action uses the <code>accelerate</code> subresource to return the Transfer Acceleration state of a bucket, which is either <code>Enabled</code> or <code>Suspended</code>. Amazon S3 Transfer Acceleration is a bucket-level feature that enables you to perform faster data transfers to and from Amazon S3.</p>
/// <p>To use this operation, you must have permission to perform the <code>s3:GetAccelerateConfiguration</code> action. The bucket owner has this permission by default. The bucket owner can grant this permission to others. For more information about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to your Amazon S3 Resources</a> in the <i>Amazon S3 User Guide</i>.</p>
/// <p>You set the Transfer Acceleration state of an existing bucket to <code>Enabled</code> or <code>Suspended</code> by using the <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketAccelerateConfiguration.html">PutBucketAccelerateConfiguration</a> operation. </p>
/// <p>A GET <code>accelerate</code> request does not return a state value for a bucket that has no transfer acceleration state. A bucket has no Transfer Acceleration state if a state has never been set on the bucket. </p>
/// <p>For more information about transfer acceleration, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/transfer-acceleration.html">Transfer Acceleration</a> in the Amazon S3 User Guide.</p>
/// <p class="title"> <b>Related Resources</b> </p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketAccelerateConfiguration.html">PutBucketAccelerateConfiguration</a> </p> </li>
/// </ul>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetBucketAccelerateConfigurationFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::get_bucket_accelerate_configuration::builders::GetBucketAccelerateConfigurationInputBuilder
            }
impl GetBucketAccelerateConfigurationFluentBuilder {
    /// Creates a new `GetBucketAccelerateConfiguration`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::get_bucket_accelerate_configuration::GetBucketAccelerateConfiguration, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::get_bucket_accelerate_configuration::GetBucketAccelerateConfigurationError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::get_bucket_accelerate_configuration::GetBucketAccelerateConfigurationOutput, aws_smithy_http::result::SdkError<crate::operation::get_bucket_accelerate_configuration::GetBucketAccelerateConfigurationError>>
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
    ///     let deserialized_parameters: crate::operation::get_bucket_accelerate_configuration::builders::GetBucketAccelerateConfigurationInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.get_bucket_accelerate_configuration().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::get_bucket_accelerate_configuration::builders::GetBucketAccelerateConfigurationInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The name of the bucket for which the accelerate configuration is retrieved.</p>
    pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.bucket(input.into());
        self
    }
    /// <p>The name of the bucket for which the accelerate configuration is retrieved.</p>
    pub fn set_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_bucket(input);
        self
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.expected_bucket_owner(input.into());
        self
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn set_expected_bucket_owner(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_expected_bucket_owner(input);
        self
    }
}
