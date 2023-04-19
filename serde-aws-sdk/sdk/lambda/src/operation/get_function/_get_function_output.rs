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
pub struct GetFunctionOutput {
    /// <p>The configuration of the function or version.</p>
    #[doc(hidden)]
    pub configuration: std::option::Option<crate::types::FunctionConfiguration>,
    /// <p>The deployment package of the function or version.</p>
    #[doc(hidden)]
    pub code: std::option::Option<crate::types::FunctionCodeLocation>,
    /// <p>The function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/tagging.html">tags</a>.</p>
    #[doc(hidden)]
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    /// <p>The function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/concurrent-executions.html">reserved concurrency</a>.</p>
    #[doc(hidden)]
    pub concurrency: std::option::Option<crate::types::Concurrency>,
    _request_id: Option<String>,
}
impl GetFunctionOutput {
    /// <p>The configuration of the function or version.</p>
    pub fn configuration(&self) -> std::option::Option<&crate::types::FunctionConfiguration> {
        self.configuration.as_ref()
    }
    /// <p>The deployment package of the function or version.</p>
    pub fn code(&self) -> std::option::Option<&crate::types::FunctionCodeLocation> {
        self.code.as_ref()
    }
    /// <p>The function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/tagging.html">tags</a>.</p>
    pub fn tags(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.tags.as_ref()
    }
    /// <p>The function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/concurrent-executions.html">reserved concurrency</a>.</p>
    pub fn concurrency(&self) -> std::option::Option<&crate::types::Concurrency> {
        self.concurrency.as_ref()
    }
}
impl aws_http::request_id::RequestId for GetFunctionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetFunctionOutput {
    /// Creates a new builder-style object to manufacture [`GetFunctionOutput`](crate::operation::get_function::GetFunctionOutput).
    pub fn builder() -> crate::operation::get_function::builders::GetFunctionOutputBuilder {
        crate::operation::get_function::builders::GetFunctionOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_function::GetFunctionOutput;
/// A builder for [`GetFunctionOutput`](crate::operation::get_function::GetFunctionOutput).
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
pub struct GetFunctionOutputBuilder {
    pub(crate) configuration: std::option::Option<crate::types::FunctionConfiguration>,
    pub(crate) code: std::option::Option<crate::types::FunctionCodeLocation>,
    pub(crate) tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    pub(crate) concurrency: std::option::Option<crate::types::Concurrency>,
    _request_id: Option<String>,
}
impl GetFunctionOutputBuilder {
    /// <p>The configuration of the function or version.</p>
    pub fn configuration(mut self, input: crate::types::FunctionConfiguration) -> Self {
        self.configuration = Some(input);
        self
    }
    /// <p>The configuration of the function or version.</p>
    pub fn set_configuration(
        mut self,
        input: std::option::Option<crate::types::FunctionConfiguration>,
    ) -> Self {
        self.configuration = input;
        self
    }
    /// <p>The deployment package of the function or version.</p>
    pub fn code(mut self, input: crate::types::FunctionCodeLocation) -> Self {
        self.code = Some(input);
        self
    }
    /// <p>The deployment package of the function or version.</p>
    pub fn set_code(
        mut self,
        input: std::option::Option<crate::types::FunctionCodeLocation>,
    ) -> Self {
        self.code = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/tagging.html">tags</a>.</p>
    pub fn tags(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = Some(hash_map);
        self
    }
    /// <p>The function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/tagging.html">tags</a>.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// <p>The function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/concurrent-executions.html">reserved concurrency</a>.</p>
    pub fn concurrency(mut self, input: crate::types::Concurrency) -> Self {
        self.concurrency = Some(input);
        self
    }
    /// <p>The function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/concurrent-executions.html">reserved concurrency</a>.</p>
    pub fn set_concurrency(
        mut self,
        input: std::option::Option<crate::types::Concurrency>,
    ) -> Self {
        self.concurrency = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetFunctionOutput`](crate::operation::get_function::GetFunctionOutput).
    pub fn build(self) -> crate::operation::get_function::GetFunctionOutput {
        crate::operation::get_function::GetFunctionOutput {
            configuration: self.configuration,
            code: self.code,
            tags: self.tags,
            concurrency: self.concurrency,
            _request_id: self._request_id,
        }
    }
}
