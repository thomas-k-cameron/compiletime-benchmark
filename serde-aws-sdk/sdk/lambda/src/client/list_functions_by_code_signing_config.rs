// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListFunctionsByCodeSigningConfig`](crate::operation::list_functions_by_code_signing_config::builders::ListFunctionsByCodeSigningConfigFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_functions_by_code_signing_config::builders::ListFunctionsByCodeSigningConfigFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`code_signing_config_arn(impl Into<String>)`](crate::operation::list_functions_by_code_signing_config::builders::ListFunctionsByCodeSigningConfigFluentBuilder::code_signing_config_arn) / [`set_code_signing_config_arn(Option<String>)`](crate::operation::list_functions_by_code_signing_config::builders::ListFunctionsByCodeSigningConfigFluentBuilder::set_code_signing_config_arn): <p>The The Amazon Resource Name (ARN) of the code signing configuration.</p>
    ///   - [`marker(impl Into<String>)`](crate::operation::list_functions_by_code_signing_config::builders::ListFunctionsByCodeSigningConfigFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_functions_by_code_signing_config::builders::ListFunctionsByCodeSigningConfigFluentBuilder::set_marker): <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    ///   - [`max_items(i32)`](crate::operation::list_functions_by_code_signing_config::builders::ListFunctionsByCodeSigningConfigFluentBuilder::max_items) / [`set_max_items(Option<i32>)`](crate::operation::list_functions_by_code_signing_config::builders::ListFunctionsByCodeSigningConfigFluentBuilder::set_max_items): <p>Maximum number of items to return.</p>
    /// - On success, responds with [`ListFunctionsByCodeSigningConfigOutput`](crate::operation::list_functions_by_code_signing_config::ListFunctionsByCodeSigningConfigOutput) with field(s):
    ///   - [`next_marker(Option<String>)`](crate::operation::list_functions_by_code_signing_config::ListFunctionsByCodeSigningConfigOutput::next_marker): <p>The pagination token that's included if more results are available.</p>
    ///   - [`function_arns(Option<Vec<String>>)`](crate::operation::list_functions_by_code_signing_config::ListFunctionsByCodeSigningConfigOutput::function_arns): <p>The function ARNs. </p>
    /// - On failure, responds with [`SdkError<ListFunctionsByCodeSigningConfigError>`](crate::operation::list_functions_by_code_signing_config::ListFunctionsByCodeSigningConfigError)
    pub fn list_functions_by_code_signing_config(&self) -> crate::operation::list_functions_by_code_signing_config::builders::ListFunctionsByCodeSigningConfigFluentBuilder{
        crate::operation::list_functions_by_code_signing_config::builders::ListFunctionsByCodeSigningConfigFluentBuilder::new(self.handle.clone())
    }
}
