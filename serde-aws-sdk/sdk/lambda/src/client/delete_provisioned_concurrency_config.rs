// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteProvisionedConcurrencyConfig`](crate::operation::delete_provisioned_concurrency_config::builders::DeleteProvisionedConcurrencyConfigFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`function_name(impl Into<String>)`](crate::operation::delete_provisioned_concurrency_config::builders::DeleteProvisionedConcurrencyConfigFluentBuilder::function_name) / [`set_function_name(Option<String>)`](crate::operation::delete_provisioned_concurrency_config::builders::DeleteProvisionedConcurrencyConfigFluentBuilder::set_function_name): <p>The name of the Lambda function.</p>  <p class="title"> <b>Name formats</b> </p>  <ul>   <li> <p> <b>Function name</b> – <code>my-function</code>.</p> </li>   <li> <p> <b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li>   <li> <p> <b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p> </li>  </ul>  <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    ///   - [`qualifier(impl Into<String>)`](crate::operation::delete_provisioned_concurrency_config::builders::DeleteProvisionedConcurrencyConfigFluentBuilder::qualifier) / [`set_qualifier(Option<String>)`](crate::operation::delete_provisioned_concurrency_config::builders::DeleteProvisionedConcurrencyConfigFluentBuilder::set_qualifier): <p>The version number or alias name.</p>
    /// - On success, responds with [`DeleteProvisionedConcurrencyConfigOutput`](crate::operation::delete_provisioned_concurrency_config::DeleteProvisionedConcurrencyConfigOutput)
    /// - On failure, responds with [`SdkError<DeleteProvisionedConcurrencyConfigError>`](crate::operation::delete_provisioned_concurrency_config::DeleteProvisionedConcurrencyConfigError)
    pub fn delete_provisioned_concurrency_config(&self) -> crate::operation::delete_provisioned_concurrency_config::builders::DeleteProvisionedConcurrencyConfigFluentBuilder{
        crate::operation::delete_provisioned_concurrency_config::builders::DeleteProvisionedConcurrencyConfigFluentBuilder::new(self.handle.clone())
    }
}
