// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListFunctionEventInvokeConfigs`](crate::operation::list_function_event_invoke_configs::builders::ListFunctionEventInvokeConfigsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_function_event_invoke_configs::builders::ListFunctionEventInvokeConfigsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`function_name(impl Into<String>)`](crate::operation::list_function_event_invoke_configs::builders::ListFunctionEventInvokeConfigsFluentBuilder::function_name) / [`set_function_name(Option<String>)`](crate::operation::list_function_event_invoke_configs::builders::ListFunctionEventInvokeConfigsFluentBuilder::set_function_name): <p>The name of the Lambda function.</p>  <p class="title"> <b>Name formats</b> </p>  <ul>   <li> <p> <b>Function name</b> - <code>my-function</code>.</p> </li>   <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li>   <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li>  </ul>  <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    ///   - [`marker(impl Into<String>)`](crate::operation::list_function_event_invoke_configs::builders::ListFunctionEventInvokeConfigsFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_function_event_invoke_configs::builders::ListFunctionEventInvokeConfigsFluentBuilder::set_marker): <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    ///   - [`max_items(i32)`](crate::operation::list_function_event_invoke_configs::builders::ListFunctionEventInvokeConfigsFluentBuilder::max_items) / [`set_max_items(Option<i32>)`](crate::operation::list_function_event_invoke_configs::builders::ListFunctionEventInvokeConfigsFluentBuilder::set_max_items): <p>The maximum number of configurations to return.</p>
    /// - On success, responds with [`ListFunctionEventInvokeConfigsOutput`](crate::operation::list_function_event_invoke_configs::ListFunctionEventInvokeConfigsOutput) with field(s):
    ///   - [`function_event_invoke_configs(Option<Vec<FunctionEventInvokeConfig>>)`](crate::operation::list_function_event_invoke_configs::ListFunctionEventInvokeConfigsOutput::function_event_invoke_configs): <p>A list of configurations.</p>
    ///   - [`next_marker(Option<String>)`](crate::operation::list_function_event_invoke_configs::ListFunctionEventInvokeConfigsOutput::next_marker): <p>The pagination token that's included if more results are available.</p>
    /// - On failure, responds with [`SdkError<ListFunctionEventInvokeConfigsError>`](crate::operation::list_function_event_invoke_configs::ListFunctionEventInvokeConfigsError)
    pub fn list_function_event_invoke_configs(&self) -> crate::operation::list_function_event_invoke_configs::builders::ListFunctionEventInvokeConfigsFluentBuilder{
        crate::operation::list_function_event_invoke_configs::builders::ListFunctionEventInvokeConfigsFluentBuilder::new(self.handle.clone())
    }
}
