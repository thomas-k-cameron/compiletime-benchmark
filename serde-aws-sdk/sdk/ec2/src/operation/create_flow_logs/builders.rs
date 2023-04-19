// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_flow_logs::_create_flow_logs_output::CreateFlowLogsOutputBuilder;

pub use crate::operation::create_flow_logs::_create_flow_logs_input::CreateFlowLogsInputBuilder;

/// Fluent builder constructing a request to `CreateFlowLogs`.
///
/// <p>Creates one or more flow logs to capture information about IP traffic for a specific network interface, subnet, or VPC. </p>
/// <p>Flow log data for a monitored network interface is recorded as flow log records, which are log events consisting of fields that describe the traffic flow. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/flow-logs.html#flow-log-records">Flow log records</a> in the <i>Amazon Virtual Private Cloud User Guide</i>.</p>
/// <p>When publishing to CloudWatch Logs, flow log records are published to a log group, and each network interface has a unique log stream in the log group. When publishing to Amazon S3, flow log records for all of the monitored network interfaces are published to a single log file object that is stored in the specified bucket.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/flow-logs.html">VPC Flow Logs</a> in the <i>Amazon Virtual Private Cloud User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateFlowLogsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_flow_logs::builders::CreateFlowLogsInputBuilder,
}
impl CreateFlowLogsFluentBuilder {
    /// Creates a new `CreateFlowLogs`.
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
            crate::operation::create_flow_logs::CreateFlowLogs,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::create_flow_logs::CreateFlowLogsError>,
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
        crate::operation::create_flow_logs::CreateFlowLogsOutput,
        aws_smithy_http::result::SdkError<crate::operation::create_flow_logs::CreateFlowLogsError>,
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
    ///     let deserialized_parameters: crate::operation::create_flow_logs::builders::CreateFlowLogsInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.create_flow_logs().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::create_flow_logs::builders::CreateFlowLogsInputBuilder,
    ) -> Self {
        self.inner = data;
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
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The ARN of the IAM role that allows Amazon EC2 to publish flow logs to a CloudWatch Logs log group in your account.</p>
    /// <p>This parameter is required if the destination type is <code>cloud-watch-logs</code> and unsupported otherwise.</p>
    pub fn deliver_logs_permission_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.deliver_logs_permission_arn(input.into());
        self
    }
    /// <p>The ARN of the IAM role that allows Amazon EC2 to publish flow logs to a CloudWatch Logs log group in your account.</p>
    /// <p>This parameter is required if the destination type is <code>cloud-watch-logs</code> and unsupported otherwise.</p>
    pub fn set_deliver_logs_permission_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_deliver_logs_permission_arn(input);
        self
    }
    /// <p>The ARN of the IAM role that allows Amazon EC2 to publish flow logs across accounts.</p>
    pub fn deliver_cross_account_role(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.deliver_cross_account_role(input.into());
        self
    }
    /// <p>The ARN of the IAM role that allows Amazon EC2 to publish flow logs across accounts.</p>
    pub fn set_deliver_cross_account_role(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_deliver_cross_account_role(input);
        self
    }
    /// <p>The name of a new or existing CloudWatch Logs log group where Amazon EC2 publishes your flow logs.</p>
    /// <p>This parameter is valid only if the destination type is <code>cloud-watch-logs</code>.</p>
    pub fn log_group_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.log_group_name(input.into());
        self
    }
    /// <p>The name of a new or existing CloudWatch Logs log group where Amazon EC2 publishes your flow logs.</p>
    /// <p>This parameter is valid only if the destination type is <code>cloud-watch-logs</code>.</p>
    pub fn set_log_group_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_log_group_name(input);
        self
    }
    /// Appends an item to `ResourceIds`.
    ///
    /// To override the contents of this collection use [`set_resource_ids`](Self::set_resource_ids).
    ///
    /// <p>The IDs of the resources to monitor. For example, if the resource type is <code>VPC</code>, specify the IDs of the VPCs.</p>
    /// <p>Constraints: Maximum of 25 for transit gateway resource types. Maximum of 1000 for the other resource types.</p>
    pub fn resource_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.resource_ids(input.into());
        self
    }
    /// <p>The IDs of the resources to monitor. For example, if the resource type is <code>VPC</code>, specify the IDs of the VPCs.</p>
    /// <p>Constraints: Maximum of 25 for transit gateway resource types. Maximum of 1000 for the other resource types.</p>
    pub fn set_resource_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_resource_ids(input);
        self
    }
    /// <p>The type of resource to monitor.</p>
    pub fn resource_type(mut self, input: crate::types::FlowLogsResourceType) -> Self {
        self.inner = self.inner.resource_type(input);
        self
    }
    /// <p>The type of resource to monitor.</p>
    pub fn set_resource_type(
        mut self,
        input: std::option::Option<crate::types::FlowLogsResourceType>,
    ) -> Self {
        self.inner = self.inner.set_resource_type(input);
        self
    }
    /// <p>The type of traffic to monitor (accepted traffic, rejected traffic, or all traffic). This parameter is not supported for transit gateway resource types. It is required for the other resource types.</p>
    pub fn traffic_type(mut self, input: crate::types::TrafficType) -> Self {
        self.inner = self.inner.traffic_type(input);
        self
    }
    /// <p>The type of traffic to monitor (accepted traffic, rejected traffic, or all traffic). This parameter is not supported for transit gateway resource types. It is required for the other resource types.</p>
    pub fn set_traffic_type(
        mut self,
        input: std::option::Option<crate::types::TrafficType>,
    ) -> Self {
        self.inner = self.inner.set_traffic_type(input);
        self
    }
    /// <p>The type of destination for the flow log data.</p>
    /// <p>Default: <code>cloud-watch-logs</code> </p>
    pub fn log_destination_type(mut self, input: crate::types::LogDestinationType) -> Self {
        self.inner = self.inner.log_destination_type(input);
        self
    }
    /// <p>The type of destination for the flow log data.</p>
    /// <p>Default: <code>cloud-watch-logs</code> </p>
    pub fn set_log_destination_type(
        mut self,
        input: std::option::Option<crate::types::LogDestinationType>,
    ) -> Self {
        self.inner = self.inner.set_log_destination_type(input);
        self
    }
    /// <p>The destination for the flow log data. The meaning of this parameter depends on the destination type.</p>
    /// <ul>
    /// <li> <p>If the destination type is <code>cloud-watch-logs</code>, specify the ARN of a CloudWatch Logs log group. For example:</p> <p>arn:aws:logs:<i>region</i>:<i>account_id</i>:log-group:<i>my_group</i> </p> <p>Alternatively, use the <code>LogGroupName</code> parameter.</p> </li>
    /// <li> <p>If the destination type is <code>s3</code>, specify the ARN of an S3 bucket. For example:</p> <p>arn:aws:s3:::<i>my_bucket</i>/<i>my_subfolder</i>/</p> <p>The subfolder is optional. Note that you can't use <code>AWSLogs</code> as a subfolder name.</p> </li>
    /// <li> <p>If the destination type is <code>kinesis-data-firehose</code>, specify the ARN of a Kinesis Data Firehose delivery stream. For example:</p> <p>arn:aws:firehose:<i>region</i>:<i>account_id</i>:deliverystream:<i>my_stream</i> </p> </li>
    /// </ul>
    pub fn log_destination(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.log_destination(input.into());
        self
    }
    /// <p>The destination for the flow log data. The meaning of this parameter depends on the destination type.</p>
    /// <ul>
    /// <li> <p>If the destination type is <code>cloud-watch-logs</code>, specify the ARN of a CloudWatch Logs log group. For example:</p> <p>arn:aws:logs:<i>region</i>:<i>account_id</i>:log-group:<i>my_group</i> </p> <p>Alternatively, use the <code>LogGroupName</code> parameter.</p> </li>
    /// <li> <p>If the destination type is <code>s3</code>, specify the ARN of an S3 bucket. For example:</p> <p>arn:aws:s3:::<i>my_bucket</i>/<i>my_subfolder</i>/</p> <p>The subfolder is optional. Note that you can't use <code>AWSLogs</code> as a subfolder name.</p> </li>
    /// <li> <p>If the destination type is <code>kinesis-data-firehose</code>, specify the ARN of a Kinesis Data Firehose delivery stream. For example:</p> <p>arn:aws:firehose:<i>region</i>:<i>account_id</i>:deliverystream:<i>my_stream</i> </p> </li>
    /// </ul>
    pub fn set_log_destination(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_log_destination(input);
        self
    }
    /// <p>The fields to include in the flow log record. List the fields in the order in which they should appear. If you omit this parameter, the flow log is created using the default format. If you specify this parameter, you must include at least one field. For more information about the available fields, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/flow-logs.html#flow-log-records">Flow log records</a> in the <i>Amazon VPC User Guide</i> or <a href="https://docs.aws.amazon.com/vpc/latest/tgw/tgw-flow-logs.html#flow-log-records">Transit Gateway Flow Log records</a> in the <i>Amazon Web Services Transit Gateway Guide</i>.</p>
    /// <p>Specify the fields using the <code>${field-id}</code> format, separated by spaces. For the CLI, surround this parameter value with single quotes on Linux or double quotes on Windows.</p>
    pub fn log_format(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.log_format(input.into());
        self
    }
    /// <p>The fields to include in the flow log record. List the fields in the order in which they should appear. If you omit this parameter, the flow log is created using the default format. If you specify this parameter, you must include at least one field. For more information about the available fields, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/flow-logs.html#flow-log-records">Flow log records</a> in the <i>Amazon VPC User Guide</i> or <a href="https://docs.aws.amazon.com/vpc/latest/tgw/tgw-flow-logs.html#flow-log-records">Transit Gateway Flow Log records</a> in the <i>Amazon Web Services Transit Gateway Guide</i>.</p>
    /// <p>Specify the fields using the <code>${field-id}</code> format, separated by spaces. For the CLI, surround this parameter value with single quotes on Linux or double quotes on Windows.</p>
    pub fn set_log_format(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_log_format(input);
        self
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the flow logs.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the flow logs.</p>
    pub fn set_tag_specifications(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
    /// <p>The maximum interval of time during which a flow of packets is captured and aggregated into a flow log record. The possible values are 60 seconds (1 minute) or 600 seconds (10 minutes). This parameter must be 60 seconds for transit gateway resource types.</p>
    /// <p>When a network interface is attached to a <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#ec2-nitro-instances">Nitro-based instance</a>, the aggregation interval is always 60 seconds or less, regardless of the value that you specify.</p>
    /// <p>Default: 600</p>
    pub fn max_aggregation_interval(mut self, input: i32) -> Self {
        self.inner = self.inner.max_aggregation_interval(input);
        self
    }
    /// <p>The maximum interval of time during which a flow of packets is captured and aggregated into a flow log record. The possible values are 60 seconds (1 minute) or 600 seconds (10 minutes). This parameter must be 60 seconds for transit gateway resource types.</p>
    /// <p>When a network interface is attached to a <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#ec2-nitro-instances">Nitro-based instance</a>, the aggregation interval is always 60 seconds or less, regardless of the value that you specify.</p>
    /// <p>Default: 600</p>
    pub fn set_max_aggregation_interval(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_aggregation_interval(input);
        self
    }
    /// <p>The destination options.</p>
    pub fn destination_options(mut self, input: crate::types::DestinationOptionsRequest) -> Self {
        self.inner = self.inner.destination_options(input);
        self
    }
    /// <p>The destination options.</p>
    pub fn set_destination_options(
        mut self,
        input: std::option::Option<crate::types::DestinationOptionsRequest>,
    ) -> Self {
        self.inner = self.inner.set_destination_options(input);
        self
    }
}
