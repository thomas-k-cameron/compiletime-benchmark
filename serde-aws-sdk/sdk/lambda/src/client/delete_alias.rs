// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteAlias`](crate::operation::delete_alias::builders::DeleteAliasFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`function_name(impl Into<String>)`](crate::operation::delete_alias::builders::DeleteAliasFluentBuilder::function_name) / [`set_function_name(Option<String>)`](crate::operation::delete_alias::builders::DeleteAliasFluentBuilder::set_function_name): <p>The name of the Lambda function.</p>  <p class="title"> <b>Name formats</b> </p>  <ul>   <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li>   <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li>   <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li>  </ul>  <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    ///   - [`name(impl Into<String>)`](crate::operation::delete_alias::builders::DeleteAliasFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::delete_alias::builders::DeleteAliasFluentBuilder::set_name): <p>The name of the alias.</p>
    /// - On success, responds with [`DeleteAliasOutput`](crate::operation::delete_alias::DeleteAliasOutput)
    /// - On failure, responds with [`SdkError<DeleteAliasError>`](crate::operation::delete_alias::DeleteAliasError)
    pub fn delete_alias(
        &self,
    ) -> crate::operation::delete_alias::builders::DeleteAliasFluentBuilder {
        crate::operation::delete_alias::builders::DeleteAliasFluentBuilder::new(self.handle.clone())
    }
}
