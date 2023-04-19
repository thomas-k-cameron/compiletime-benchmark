// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_event_source_mapping::_create_event_source_mapping_output::CreateEventSourceMappingOutputBuilder;

pub use crate::operation::create_event_source_mapping::_create_event_source_mapping_input::CreateEventSourceMappingInputBuilder;

/// Fluent builder constructing a request to `CreateEventSourceMapping`.
///
/// <p>Creates a mapping between an event source and an Lambda function. Lambda reads items from the event source and invokes the function.</p>
/// <p>For details about how to configure different event sources, see the following topics. </p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-ddb.html#services-dynamodb-eventsourcemapping"> Amazon DynamoDB Streams</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-kinesis.html#services-kinesis-eventsourcemapping"> Amazon Kinesis</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-sqs.html#events-sqs-eventsource"> Amazon SQS</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-mq.html#services-mq-eventsourcemapping"> Amazon MQ and RabbitMQ</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-msk.html"> Amazon MSK</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/kafka-smaa.html"> Apache Kafka</a> </p> </li>
/// </ul>
/// <p>The following error handling options are available only for stream sources (DynamoDB and Kinesis):</p>
/// <ul>
/// <li> <p> <code>BisectBatchOnFunctionError</code> – If the function returns an error, split the batch in two and retry.</p> </li>
/// <li> <p> <code>DestinationConfig</code> – Send discarded records to an Amazon SQS queue or Amazon SNS topic.</p> </li>
/// <li> <p> <code>MaximumRecordAgeInSeconds</code> – Discard records older than the specified age. The default value is infinite (-1). When set to infinite (-1), failed records are retried until the record expires</p> </li>
/// <li> <p> <code>MaximumRetryAttempts</code> – Discard records after the specified number of retries. The default value is infinite (-1). When set to infinite (-1), failed records are retried until the record expires.</p> </li>
/// <li> <p> <code>ParallelizationFactor</code> – Process multiple batches from each shard concurrently.</p> </li>
/// </ul>
/// <p>For information about which configuration parameters apply to each event source, see the following topics.</p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-ddb.html#services-ddb-params"> Amazon DynamoDB Streams</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-kinesis.html#services-kinesis-params"> Amazon Kinesis</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-sqs.html#services-sqs-params"> Amazon SQS</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-mq.html#services-mq-params"> Amazon MQ and RabbitMQ</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-msk.html#services-msk-parms"> Amazon MSK</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-kafka.html#services-kafka-parms"> Apache Kafka</a> </p> </li>
/// </ul>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateEventSourceMappingFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::create_event_source_mapping::builders::CreateEventSourceMappingInputBuilder
            }
impl CreateEventSourceMappingFluentBuilder {
    /// Creates a new `CreateEventSourceMapping`.
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
            crate::operation::create_event_source_mapping::CreateEventSourceMapping,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_event_source_mapping::CreateEventSourceMappingError,
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
        crate::operation::create_event_source_mapping::CreateEventSourceMappingOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_event_source_mapping::CreateEventSourceMappingError,
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
    ///     let deserialized_parameters: crate::operation::create_event_source_mapping::builders::CreateEventSourceMappingInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.create_event_source_mapping().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::create_event_source_mapping::builders::CreateEventSourceMappingInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the event source.</p>
    /// <ul>
    /// <li> <p> <b>Amazon Kinesis</b> – The ARN of the data stream or a stream consumer.</p> </li>
    /// <li> <p> <b>Amazon DynamoDB Streams</b> – The ARN of the stream.</p> </li>
    /// <li> <p> <b>Amazon Simple Queue Service</b> – The ARN of the queue.</p> </li>
    /// <li> <p> <b>Amazon Managed Streaming for Apache Kafka</b> – The ARN of the cluster.</p> </li>
    /// <li> <p> <b>Amazon MQ</b> – The ARN of the broker.</p> </li>
    /// </ul>
    pub fn event_source_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.event_source_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the event source.</p>
    /// <ul>
    /// <li> <p> <b>Amazon Kinesis</b> – The ARN of the data stream or a stream consumer.</p> </li>
    /// <li> <p> <b>Amazon DynamoDB Streams</b> – The ARN of the stream.</p> </li>
    /// <li> <p> <b>Amazon Simple Queue Service</b> – The ARN of the queue.</p> </li>
    /// <li> <p> <b>Amazon Managed Streaming for Apache Kafka</b> – The ARN of the cluster.</p> </li>
    /// <li> <p> <b>Amazon MQ</b> – The ARN of the broker.</p> </li>
    /// </ul>
    pub fn set_event_source_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_event_source_arn(input);
        self
    }
    /// <p>The name of the Lambda function.</p>
    /// <p class="title"> <b>Name formats</b> </p>
    /// <ul>
    /// <li> <p> <b>Function name</b> – <code>MyFunction</code>.</p> </li>
    /// <li> <p> <b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li>
    /// <li> <p> <b>Version or Alias ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction:PROD</code>.</p> </li>
    /// <li> <p> <b>Partial ARN</b> – <code>123456789012:function:MyFunction</code>.</p> </li>
    /// </ul>
    /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it's limited to 64 characters in length.</p>
    pub fn function_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.function_name(input.into());
        self
    }
    /// <p>The name of the Lambda function.</p>
    /// <p class="title"> <b>Name formats</b> </p>
    /// <ul>
    /// <li> <p> <b>Function name</b> – <code>MyFunction</code>.</p> </li>
    /// <li> <p> <b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li>
    /// <li> <p> <b>Version or Alias ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction:PROD</code>.</p> </li>
    /// <li> <p> <b>Partial ARN</b> – <code>123456789012:function:MyFunction</code>.</p> </li>
    /// </ul>
    /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it's limited to 64 characters in length.</p>
    pub fn set_function_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_function_name(input);
        self
    }
    /// <p>When true, the event source mapping is active. When false, Lambda pauses polling and invocation.</p>
    /// <p>Default: True</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.enabled(input);
        self
    }
    /// <p>When true, the event source mapping is active. When false, Lambda pauses polling and invocation.</p>
    /// <p>Default: True</p>
    pub fn set_enabled(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_enabled(input);
        self
    }
    /// <p>The maximum number of records in each batch that Lambda pulls from your stream or queue and sends to your function. Lambda passes all of the records in the batch to the function in a single call, up to the payload limit for synchronous invocation (6 MB).</p>
    /// <ul>
    /// <li> <p> <b>Amazon Kinesis</b> – Default 100. Max 10,000.</p> </li>
    /// <li> <p> <b>Amazon DynamoDB Streams</b> – Default 100. Max 10,000.</p> </li>
    /// <li> <p> <b>Amazon Simple Queue Service</b> – Default 10. For standard queues the max is 10,000. For FIFO queues the max is 10.</p> </li>
    /// <li> <p> <b>Amazon Managed Streaming for Apache Kafka</b> – Default 100. Max 10,000.</p> </li>
    /// <li> <p> <b>Self-managed Apache Kafka</b> – Default 100. Max 10,000.</p> </li>
    /// <li> <p> <b>Amazon MQ (ActiveMQ and RabbitMQ)</b> – Default 100. Max 10,000.</p> </li>
    /// </ul>
    pub fn batch_size(mut self, input: i32) -> Self {
        self.inner = self.inner.batch_size(input);
        self
    }
    /// <p>The maximum number of records in each batch that Lambda pulls from your stream or queue and sends to your function. Lambda passes all of the records in the batch to the function in a single call, up to the payload limit for synchronous invocation (6 MB).</p>
    /// <ul>
    /// <li> <p> <b>Amazon Kinesis</b> – Default 100. Max 10,000.</p> </li>
    /// <li> <p> <b>Amazon DynamoDB Streams</b> – Default 100. Max 10,000.</p> </li>
    /// <li> <p> <b>Amazon Simple Queue Service</b> – Default 10. For standard queues the max is 10,000. For FIFO queues the max is 10.</p> </li>
    /// <li> <p> <b>Amazon Managed Streaming for Apache Kafka</b> – Default 100. Max 10,000.</p> </li>
    /// <li> <p> <b>Self-managed Apache Kafka</b> – Default 100. Max 10,000.</p> </li>
    /// <li> <p> <b>Amazon MQ (ActiveMQ and RabbitMQ)</b> – Default 100. Max 10,000.</p> </li>
    /// </ul>
    pub fn set_batch_size(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_batch_size(input);
        self
    }
    /// <p>An object that defines the filter criteria that determine whether Lambda should process an event. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-eventfiltering.html">Lambda event filtering</a>.</p>
    pub fn filter_criteria(mut self, input: crate::types::FilterCriteria) -> Self {
        self.inner = self.inner.filter_criteria(input);
        self
    }
    /// <p>An object that defines the filter criteria that determine whether Lambda should process an event. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-eventfiltering.html">Lambda event filtering</a>.</p>
    pub fn set_filter_criteria(
        mut self,
        input: std::option::Option<crate::types::FilterCriteria>,
    ) -> Self {
        self.inner = self.inner.set_filter_criteria(input);
        self
    }
    /// <p>The maximum amount of time, in seconds, that Lambda spends gathering records before invoking the function. You can configure <code>MaximumBatchingWindowInSeconds</code> to any value from 0 seconds to 300 seconds in increments of seconds.</p>
    /// <p>For streams and Amazon SQS event sources, the default batching window is 0 seconds. For Amazon MSK, Self-managed Apache Kafka, and Amazon MQ event sources, the default batching window is 500 ms. Note that because you can only change <code>MaximumBatchingWindowInSeconds</code> in increments of seconds, you cannot revert back to the 500 ms default batching window after you have changed it. To restore the default batching window, you must create a new event source mapping.</p>
    /// <p>Related setting: For streams and Amazon SQS event sources, when you set <code>BatchSize</code> to a value greater than 10, you must set <code>MaximumBatchingWindowInSeconds</code> to at least 1.</p>
    pub fn maximum_batching_window_in_seconds(mut self, input: i32) -> Self {
        self.inner = self.inner.maximum_batching_window_in_seconds(input);
        self
    }
    /// <p>The maximum amount of time, in seconds, that Lambda spends gathering records before invoking the function. You can configure <code>MaximumBatchingWindowInSeconds</code> to any value from 0 seconds to 300 seconds in increments of seconds.</p>
    /// <p>For streams and Amazon SQS event sources, the default batching window is 0 seconds. For Amazon MSK, Self-managed Apache Kafka, and Amazon MQ event sources, the default batching window is 500 ms. Note that because you can only change <code>MaximumBatchingWindowInSeconds</code> in increments of seconds, you cannot revert back to the 500 ms default batching window after you have changed it. To restore the default batching window, you must create a new event source mapping.</p>
    /// <p>Related setting: For streams and Amazon SQS event sources, when you set <code>BatchSize</code> to a value greater than 10, you must set <code>MaximumBatchingWindowInSeconds</code> to at least 1.</p>
    pub fn set_maximum_batching_window_in_seconds(
        mut self,
        input: std::option::Option<i32>,
    ) -> Self {
        self.inner = self.inner.set_maximum_batching_window_in_seconds(input);
        self
    }
    /// <p>(Streams only) The number of batches to process from each shard concurrently.</p>
    pub fn parallelization_factor(mut self, input: i32) -> Self {
        self.inner = self.inner.parallelization_factor(input);
        self
    }
    /// <p>(Streams only) The number of batches to process from each shard concurrently.</p>
    pub fn set_parallelization_factor(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_parallelization_factor(input);
        self
    }
    /// <p>The position in a stream from which to start reading. Required for Amazon Kinesis, Amazon DynamoDB, and Amazon MSK Streams sources. <code>AT_TIMESTAMP</code> is supported only for Amazon Kinesis streams.</p>
    pub fn starting_position(mut self, input: crate::types::EventSourcePosition) -> Self {
        self.inner = self.inner.starting_position(input);
        self
    }
    /// <p>The position in a stream from which to start reading. Required for Amazon Kinesis, Amazon DynamoDB, and Amazon MSK Streams sources. <code>AT_TIMESTAMP</code> is supported only for Amazon Kinesis streams.</p>
    pub fn set_starting_position(
        mut self,
        input: std::option::Option<crate::types::EventSourcePosition>,
    ) -> Self {
        self.inner = self.inner.set_starting_position(input);
        self
    }
    /// <p>With <code>StartingPosition</code> set to <code>AT_TIMESTAMP</code>, the time from which to start reading.</p>
    pub fn starting_position_timestamp(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.starting_position_timestamp(input);
        self
    }
    /// <p>With <code>StartingPosition</code> set to <code>AT_TIMESTAMP</code>, the time from which to start reading.</p>
    pub fn set_starting_position_timestamp(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_starting_position_timestamp(input);
        self
    }
    /// <p>(Streams only) An Amazon SQS queue or Amazon SNS topic destination for discarded records.</p>
    pub fn destination_config(mut self, input: crate::types::DestinationConfig) -> Self {
        self.inner = self.inner.destination_config(input);
        self
    }
    /// <p>(Streams only) An Amazon SQS queue or Amazon SNS topic destination for discarded records.</p>
    pub fn set_destination_config(
        mut self,
        input: std::option::Option<crate::types::DestinationConfig>,
    ) -> Self {
        self.inner = self.inner.set_destination_config(input);
        self
    }
    /// <p>(Streams only) Discard records older than the specified age. The default value is infinite (-1).</p>
    pub fn maximum_record_age_in_seconds(mut self, input: i32) -> Self {
        self.inner = self.inner.maximum_record_age_in_seconds(input);
        self
    }
    /// <p>(Streams only) Discard records older than the specified age. The default value is infinite (-1).</p>
    pub fn set_maximum_record_age_in_seconds(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_maximum_record_age_in_seconds(input);
        self
    }
    /// <p>(Streams only) If the function returns an error, split the batch in two and retry.</p>
    pub fn bisect_batch_on_function_error(mut self, input: bool) -> Self {
        self.inner = self.inner.bisect_batch_on_function_error(input);
        self
    }
    /// <p>(Streams only) If the function returns an error, split the batch in two and retry.</p>
    pub fn set_bisect_batch_on_function_error(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_bisect_batch_on_function_error(input);
        self
    }
    /// <p>(Streams only) Discard records after the specified number of retries. The default value is infinite (-1). When set to infinite (-1), failed records are retried until the record expires.</p>
    pub fn maximum_retry_attempts(mut self, input: i32) -> Self {
        self.inner = self.inner.maximum_retry_attempts(input);
        self
    }
    /// <p>(Streams only) Discard records after the specified number of retries. The default value is infinite (-1). When set to infinite (-1), failed records are retried until the record expires.</p>
    pub fn set_maximum_retry_attempts(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_maximum_retry_attempts(input);
        self
    }
    /// <p>(Streams only) The duration in seconds of a processing window. The range is between 1 second and 900 seconds.</p>
    pub fn tumbling_window_in_seconds(mut self, input: i32) -> Self {
        self.inner = self.inner.tumbling_window_in_seconds(input);
        self
    }
    /// <p>(Streams only) The duration in seconds of a processing window. The range is between 1 second and 900 seconds.</p>
    pub fn set_tumbling_window_in_seconds(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_tumbling_window_in_seconds(input);
        self
    }
    /// Appends an item to `Topics`.
    ///
    /// To override the contents of this collection use [`set_topics`](Self::set_topics).
    ///
    /// <p>The name of the Kafka topic.</p>
    pub fn topics(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.topics(input.into());
        self
    }
    /// <p>The name of the Kafka topic.</p>
    pub fn set_topics(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_topics(input);
        self
    }
    /// Appends an item to `Queues`.
    ///
    /// To override the contents of this collection use [`set_queues`](Self::set_queues).
    ///
    /// <p> (MQ) The name of the Amazon MQ broker destination queue to consume. </p>
    pub fn queues(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.queues(input.into());
        self
    }
    /// <p> (MQ) The name of the Amazon MQ broker destination queue to consume. </p>
    pub fn set_queues(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_queues(input);
        self
    }
    /// Appends an item to `SourceAccessConfigurations`.
    ///
    /// To override the contents of this collection use [`set_source_access_configurations`](Self::set_source_access_configurations).
    ///
    /// <p>An array of authentication protocols or VPC components required to secure your event source.</p>
    pub fn source_access_configurations(
        mut self,
        input: crate::types::SourceAccessConfiguration,
    ) -> Self {
        self.inner = self.inner.source_access_configurations(input);
        self
    }
    /// <p>An array of authentication protocols or VPC components required to secure your event source.</p>
    pub fn set_source_access_configurations(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::SourceAccessConfiguration>>,
    ) -> Self {
        self.inner = self.inner.set_source_access_configurations(input);
        self
    }
    /// <p>The self-managed Apache Kafka cluster to receive records from.</p>
    pub fn self_managed_event_source(
        mut self,
        input: crate::types::SelfManagedEventSource,
    ) -> Self {
        self.inner = self.inner.self_managed_event_source(input);
        self
    }
    /// <p>The self-managed Apache Kafka cluster to receive records from.</p>
    pub fn set_self_managed_event_source(
        mut self,
        input: std::option::Option<crate::types::SelfManagedEventSource>,
    ) -> Self {
        self.inner = self.inner.set_self_managed_event_source(input);
        self
    }
    /// Appends an item to `FunctionResponseTypes`.
    ///
    /// To override the contents of this collection use [`set_function_response_types`](Self::set_function_response_types).
    ///
    /// <p>(Streams and Amazon SQS) A list of current response type enums applied to the event source mapping.</p>
    pub fn function_response_types(mut self, input: crate::types::FunctionResponseType) -> Self {
        self.inner = self.inner.function_response_types(input);
        self
    }
    /// <p>(Streams and Amazon SQS) A list of current response type enums applied to the event source mapping.</p>
    pub fn set_function_response_types(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::FunctionResponseType>>,
    ) -> Self {
        self.inner = self.inner.set_function_response_types(input);
        self
    }
    /// <p>Specific configuration settings for an Amazon Managed Streaming for Apache Kafka (Amazon MSK) event source.</p>
    pub fn amazon_managed_kafka_event_source_config(
        mut self,
        input: crate::types::AmazonManagedKafkaEventSourceConfig,
    ) -> Self {
        self.inner = self.inner.amazon_managed_kafka_event_source_config(input);
        self
    }
    /// <p>Specific configuration settings for an Amazon Managed Streaming for Apache Kafka (Amazon MSK) event source.</p>
    pub fn set_amazon_managed_kafka_event_source_config(
        mut self,
        input: std::option::Option<crate::types::AmazonManagedKafkaEventSourceConfig>,
    ) -> Self {
        self.inner = self
            .inner
            .set_amazon_managed_kafka_event_source_config(input);
        self
    }
    /// <p>Specific configuration settings for a self-managed Apache Kafka event source.</p>
    pub fn self_managed_kafka_event_source_config(
        mut self,
        input: crate::types::SelfManagedKafkaEventSourceConfig,
    ) -> Self {
        self.inner = self.inner.self_managed_kafka_event_source_config(input);
        self
    }
    /// <p>Specific configuration settings for a self-managed Apache Kafka event source.</p>
    pub fn set_self_managed_kafka_event_source_config(
        mut self,
        input: std::option::Option<crate::types::SelfManagedKafkaEventSourceConfig>,
    ) -> Self {
        self.inner = self.inner.set_self_managed_kafka_event_source_config(input);
        self
    }
    /// <p>(Amazon SQS only) The scaling configuration for the event source. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-sqs.html#events-sqs-max-concurrency">Configuring maximum concurrency for Amazon SQS event sources</a>.</p>
    pub fn scaling_config(mut self, input: crate::types::ScalingConfig) -> Self {
        self.inner = self.inner.scaling_config(input);
        self
    }
    /// <p>(Amazon SQS only) The scaling configuration for the event source. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-sqs.html#events-sqs-max-concurrency">Configuring maximum concurrency for Amazon SQS event sources</a>.</p>
    pub fn set_scaling_config(
        mut self,
        input: std::option::Option<crate::types::ScalingConfig>,
    ) -> Self {
        self.inner = self.inner.set_scaling_config(input);
        self
    }
    /// <p>Specific configuration settings for a DocumentDB event source.</p>
    pub fn document_db_event_source_config(
        mut self,
        input: crate::types::DocumentDbEventSourceConfig,
    ) -> Self {
        self.inner = self.inner.document_db_event_source_config(input);
        self
    }
    /// <p>Specific configuration settings for a DocumentDB event source.</p>
    pub fn set_document_db_event_source_config(
        mut self,
        input: std::option::Option<crate::types::DocumentDbEventSourceConfig>,
    ) -> Self {
        self.inner = self.inner.set_document_db_event_source_config(input);
        self
    }
}
