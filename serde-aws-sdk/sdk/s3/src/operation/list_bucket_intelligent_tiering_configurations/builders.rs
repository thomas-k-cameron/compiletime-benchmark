// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_bucket_intelligent_tiering_configurations::_list_bucket_intelligent_tiering_configurations_output::ListBucketIntelligentTieringConfigurationsOutputBuilder;

pub use crate::operation::list_bucket_intelligent_tiering_configurations::_list_bucket_intelligent_tiering_configurations_input::ListBucketIntelligentTieringConfigurationsInputBuilder;

/// Fluent builder constructing a request to `ListBucketIntelligentTieringConfigurations`.
///
/// <p>Lists the S3 Intelligent-Tiering configuration from the specified bucket.</p>
/// <p>The S3 Intelligent-Tiering storage class is designed to optimize storage costs by automatically moving data to the most cost-effective storage access tier, without performance impact or operational overhead. S3 Intelligent-Tiering delivers automatic cost savings in three low latency and high throughput access tiers. To get the lowest storage cost on data that can be accessed in minutes to hours, you can choose to activate additional archiving capabilities.</p>
/// <p>The S3 Intelligent-Tiering storage class is the ideal storage class for data with unknown, changing, or unpredictable access patterns, independent of object size or retention period. If the size of an object is less than 128 KB, it is not monitored and not eligible for auto-tiering. Smaller objects can be stored, but they are always charged at the Frequent Access tier rates in the S3 Intelligent-Tiering storage class.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html#sc-dynamic-data-access">Storage class for automatically optimizing frequently and infrequently accessed objects</a>.</p>
/// <p>Operations related to <code>ListBucketIntelligentTieringConfigurations</code> include: </p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketIntelligentTieringConfiguration.html">DeleteBucketIntelligentTieringConfiguration</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketIntelligentTieringConfiguration.html">PutBucketIntelligentTieringConfiguration</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketIntelligentTieringConfiguration.html">GetBucketIntelligentTieringConfiguration</a> </p> </li>
/// </ul>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListBucketIntelligentTieringConfigurationsFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::list_bucket_intelligent_tiering_configurations::builders::ListBucketIntelligentTieringConfigurationsInputBuilder
            }
impl ListBucketIntelligentTieringConfigurationsFluentBuilder {
    /// Creates a new `ListBucketIntelligentTieringConfigurations`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::list_bucket_intelligent_tiering_configurations::ListBucketIntelligentTieringConfigurations, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::list_bucket_intelligent_tiering_configurations::ListBucketIntelligentTieringConfigurationsError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::list_bucket_intelligent_tiering_configurations::ListBucketIntelligentTieringConfigurationsOutput, aws_smithy_http::result::SdkError<crate::operation::list_bucket_intelligent_tiering_configurations::ListBucketIntelligentTieringConfigurationsError>>
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
    ///     let deserialized_parameters: crate::operation::list_bucket_intelligent_tiering_configurations::builders::ListBucketIntelligentTieringConfigurationsInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.list_bucket_intelligent_tiering_configurations().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::list_bucket_intelligent_tiering_configurations::builders::ListBucketIntelligentTieringConfigurationsInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.bucket(input.into());
        self
    }
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    pub fn set_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_bucket(input);
        self
    }
    /// <p>The <code>ContinuationToken</code> that represents a placeholder from where this request should begin.</p>
    pub fn continuation_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.continuation_token(input.into());
        self
    }
    /// <p>The <code>ContinuationToken</code> that represents a placeholder from where this request should begin.</p>
    pub fn set_continuation_token(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_continuation_token(input);
        self
    }
}
