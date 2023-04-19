// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateQueryLoggingConfigInput {
    /// <p>The ID of the hosted zone that you want to log queries for. You can log queries only for public hosted zones.</p>
    #[doc(hidden)]
    pub hosted_zone_id: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) for the log group that you want to Amazon Route 53 to send query logs to. This is the format of the ARN:</p>
    /// <p>arn:aws:logs:<i>region</i>:<i>account-id</i>:log-group:<i>log_group_name</i> </p>
    /// <p>To get the ARN for a log group, you can use the CloudWatch console, the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeLogGroups.html">DescribeLogGroups</a> API action, the <a href="https://docs.aws.amazon.com/cli/latest/reference/logs/describe-log-groups.html">describe-log-groups</a> command, or the applicable command in one of the Amazon Web Services SDKs.</p>
    #[doc(hidden)]
    pub cloud_watch_logs_log_group_arn: std::option::Option<std::string::String>,
}
impl CreateQueryLoggingConfigInput {
    /// <p>The ID of the hosted zone that you want to log queries for. You can log queries only for public hosted zones.</p>
    pub fn hosted_zone_id(&self) -> std::option::Option<&str> {
        self.hosted_zone_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) for the log group that you want to Amazon Route 53 to send query logs to. This is the format of the ARN:</p>
    /// <p>arn:aws:logs:<i>region</i>:<i>account-id</i>:log-group:<i>log_group_name</i> </p>
    /// <p>To get the ARN for a log group, you can use the CloudWatch console, the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeLogGroups.html">DescribeLogGroups</a> API action, the <a href="https://docs.aws.amazon.com/cli/latest/reference/logs/describe-log-groups.html">describe-log-groups</a> command, or the applicable command in one of the Amazon Web Services SDKs.</p>
    pub fn cloud_watch_logs_log_group_arn(&self) -> std::option::Option<&str> {
        self.cloud_watch_logs_log_group_arn.as_deref()
    }
}
impl CreateQueryLoggingConfigInput {
    /// Creates a new builder-style object to manufacture [`CreateQueryLoggingConfigInput`](crate::operation::create_query_logging_config::CreateQueryLoggingConfigInput).
    pub fn builder(
    ) -> crate::operation::create_query_logging_config::builders::CreateQueryLoggingConfigInputBuilder
    {
        crate::operation::create_query_logging_config::builders::CreateQueryLoggingConfigInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_query_logging_config::CreateQueryLoggingConfigInput;
/// A builder for [`CreateQueryLoggingConfigInput`](crate::operation::create_query_logging_config::CreateQueryLoggingConfigInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct CreateQueryLoggingConfigInputBuilder {
    pub(crate) hosted_zone_id: std::option::Option<std::string::String>,
    pub(crate) cloud_watch_logs_log_group_arn: std::option::Option<std::string::String>,
}
impl CreateQueryLoggingConfigInputBuilder {
    /// <p>The ID of the hosted zone that you want to log queries for. You can log queries only for public hosted zones.</p>
    pub fn hosted_zone_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.hosted_zone_id = Some(input.into());
        self
    }
    /// <p>The ID of the hosted zone that you want to log queries for. You can log queries only for public hosted zones.</p>
    pub fn set_hosted_zone_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.hosted_zone_id = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the log group that you want to Amazon Route 53 to send query logs to. This is the format of the ARN:</p>
    /// <p>arn:aws:logs:<i>region</i>:<i>account-id</i>:log-group:<i>log_group_name</i> </p>
    /// <p>To get the ARN for a log group, you can use the CloudWatch console, the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeLogGroups.html">DescribeLogGroups</a> API action, the <a href="https://docs.aws.amazon.com/cli/latest/reference/logs/describe-log-groups.html">describe-log-groups</a> command, or the applicable command in one of the Amazon Web Services SDKs.</p>
    pub fn cloud_watch_logs_log_group_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.cloud_watch_logs_log_group_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the log group that you want to Amazon Route 53 to send query logs to. This is the format of the ARN:</p>
    /// <p>arn:aws:logs:<i>region</i>:<i>account-id</i>:log-group:<i>log_group_name</i> </p>
    /// <p>To get the ARN for a log group, you can use the CloudWatch console, the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeLogGroups.html">DescribeLogGroups</a> API action, the <a href="https://docs.aws.amazon.com/cli/latest/reference/logs/describe-log-groups.html">describe-log-groups</a> command, or the applicable command in one of the Amazon Web Services SDKs.</p>
    pub fn set_cloud_watch_logs_log_group_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.cloud_watch_logs_log_group_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateQueryLoggingConfigInput`](crate::operation::create_query_logging_config::CreateQueryLoggingConfigInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::create_query_logging_config::CreateQueryLoggingConfigInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::create_query_logging_config::CreateQueryLoggingConfigInput {
                hosted_zone_id: self.hosted_zone_id,
                cloud_watch_logs_log_group_arn: self.cloud_watch_logs_log_group_arn,
            },
        )
    }
}