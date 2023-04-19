// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListEventSourceMappings`](crate::operation::list_event_source_mappings::builders::ListEventSourceMappingsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_event_source_mappings::builders::ListEventSourceMappingsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`event_source_arn(impl Into<String>)`](crate::operation::list_event_source_mappings::builders::ListEventSourceMappingsFluentBuilder::event_source_arn) / [`set_event_source_arn(Option<String>)`](crate::operation::list_event_source_mappings::builders::ListEventSourceMappingsFluentBuilder::set_event_source_arn): <p>The Amazon Resource Name (ARN) of the event source.</p>  <ul>   <li> <p> <b>Amazon Kinesis</b> – The ARN of the data stream or a stream consumer.</p> </li>   <li> <p> <b>Amazon DynamoDB Streams</b> – The ARN of the stream.</p> </li>   <li> <p> <b>Amazon Simple Queue Service</b> – The ARN of the queue.</p> </li>   <li> <p> <b>Amazon Managed Streaming for Apache Kafka</b> – The ARN of the cluster.</p> </li>   <li> <p> <b>Amazon MQ</b> – The ARN of the broker.</p> </li>  </ul>
    ///   - [`function_name(impl Into<String>)`](crate::operation::list_event_source_mappings::builders::ListEventSourceMappingsFluentBuilder::function_name) / [`set_function_name(Option<String>)`](crate::operation::list_event_source_mappings::builders::ListEventSourceMappingsFluentBuilder::set_function_name): <p>The name of the Lambda function.</p>  <p class="title"> <b>Name formats</b> </p>  <ul>   <li> <p> <b>Function name</b> – <code>MyFunction</code>.</p> </li>   <li> <p> <b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li>   <li> <p> <b>Version or Alias ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction:PROD</code>.</p> </li>   <li> <p> <b>Partial ARN</b> – <code>123456789012:function:MyFunction</code>.</p> </li>  </ul>  <p>The length constraint applies only to the full ARN. If you specify only the function name, it's limited to 64 characters in length.</p>
    ///   - [`marker(impl Into<String>)`](crate::operation::list_event_source_mappings::builders::ListEventSourceMappingsFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_event_source_mappings::builders::ListEventSourceMappingsFluentBuilder::set_marker): <p>A pagination token returned by a previous call.</p>
    ///   - [`max_items(i32)`](crate::operation::list_event_source_mappings::builders::ListEventSourceMappingsFluentBuilder::max_items) / [`set_max_items(Option<i32>)`](crate::operation::list_event_source_mappings::builders::ListEventSourceMappingsFluentBuilder::set_max_items): <p>The maximum number of event source mappings to return. Note that ListEventSourceMappings returns a maximum of 100 items in each response, even if you set the number higher.</p>
    /// - On success, responds with [`ListEventSourceMappingsOutput`](crate::operation::list_event_source_mappings::ListEventSourceMappingsOutput) with field(s):
    ///   - [`next_marker(Option<String>)`](crate::operation::list_event_source_mappings::ListEventSourceMappingsOutput::next_marker): <p>A pagination token that's returned when the response doesn't contain all event source mappings.</p>
    ///   - [`event_source_mappings(Option<Vec<EventSourceMappingConfiguration>>)`](crate::operation::list_event_source_mappings::ListEventSourceMappingsOutput::event_source_mappings): <p>A list of event source mappings.</p>
    /// - On failure, responds with [`SdkError<ListEventSourceMappingsError>`](crate::operation::list_event_source_mappings::ListEventSourceMappingsError)
    pub fn list_event_source_mappings(
        &self,
    ) -> crate::operation::list_event_source_mappings::builders::ListEventSourceMappingsFluentBuilder
    {
        crate::operation::list_event_source_mappings::builders::ListEventSourceMappingsFluentBuilder::new(self.handle.clone())
    }
}
