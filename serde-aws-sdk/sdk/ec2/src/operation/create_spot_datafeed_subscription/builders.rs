// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_spot_datafeed_subscription::_create_spot_datafeed_subscription_output::CreateSpotDatafeedSubscriptionOutputBuilder;

pub use crate::operation::create_spot_datafeed_subscription::_create_spot_datafeed_subscription_input::CreateSpotDatafeedSubscriptionInputBuilder;

/// Fluent builder constructing a request to `CreateSpotDatafeedSubscription`.
///
/// <p>Creates a data feed for Spot Instances, enabling you to view Spot Instance usage logs. You can create one data feed per Amazon Web Services account. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/spot-data-feeds.html">Spot Instance data feed</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateSpotDatafeedSubscriptionFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::create_spot_datafeed_subscription::builders::CreateSpotDatafeedSubscriptionInputBuilder
            }
impl CreateSpotDatafeedSubscriptionFluentBuilder {
    /// Creates a new `CreateSpotDatafeedSubscription`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::create_spot_datafeed_subscription::CreateSpotDatafeedSubscription, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::create_spot_datafeed_subscription::CreateSpotDatafeedSubscriptionError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::create_spot_datafeed_subscription::CreateSpotDatafeedSubscriptionOutput, aws_smithy_http::result::SdkError<crate::operation::create_spot_datafeed_subscription::CreateSpotDatafeedSubscriptionError>>
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
    ///     let deserialized_parameters: crate::operation::create_spot_datafeed_subscription::builders::CreateSpotDatafeedSubscriptionInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.create_spot_datafeed_subscription().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::create_spot_datafeed_subscription::builders::CreateSpotDatafeedSubscriptionInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The name of the Amazon S3 bucket in which to store the Spot Instance data feed. For more information about bucket names, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/BucketRestrictions.html#bucketnamingrules">Rules for bucket naming</a> in the <i>Amazon S3 Developer Guide</i>.</p>
    pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.bucket(input.into());
        self
    }
    /// <p>The name of the Amazon S3 bucket in which to store the Spot Instance data feed. For more information about bucket names, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/BucketRestrictions.html#bucketnamingrules">Rules for bucket naming</a> in the <i>Amazon S3 Developer Guide</i>.</p>
    pub fn set_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_bucket(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>The prefix for the data feed file names.</p>
    pub fn prefix(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.prefix(input.into());
        self
    }
    /// <p>The prefix for the data feed file names.</p>
    pub fn set_prefix(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_prefix(input);
        self
    }
}