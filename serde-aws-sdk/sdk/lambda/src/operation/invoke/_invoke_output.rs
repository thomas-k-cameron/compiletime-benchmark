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
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InvokeOutput {
    /// <p>The HTTP status code is in the 200 range for a successful request. For the <code>RequestResponse</code> invocation type, this status code is 200. For the <code>Event</code> invocation type, this status code is 202. For the <code>DryRun</code> invocation type, the status code is 204.</p>
    #[doc(hidden)]
    pub status_code: i32,
    /// <p>If present, indicates that an error occurred during function execution. Details about the error are included in the response payload.</p>
    #[doc(hidden)]
    pub function_error: std::option::Option<std::string::String>,
    /// <p>The last 4 KB of the execution log, which is base64-encoded.</p>
    #[doc(hidden)]
    pub log_result: std::option::Option<std::string::String>,
    /// <p>The response from the function, or an error object.</p>
    #[doc(hidden)]
    pub payload: std::option::Option<aws_smithy_types::Blob>,
    /// <p>The version of the function that executed. When you invoke a function with an alias, this indicates which version the alias resolved to.</p>
    #[doc(hidden)]
    pub executed_version: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl InvokeOutput {
    /// <p>The HTTP status code is in the 200 range for a successful request. For the <code>RequestResponse</code> invocation type, this status code is 200. For the <code>Event</code> invocation type, this status code is 202. For the <code>DryRun</code> invocation type, the status code is 204.</p>
    pub fn status_code(&self) -> i32 {
        self.status_code
    }
    /// <p>If present, indicates that an error occurred during function execution. Details about the error are included in the response payload.</p>
    pub fn function_error(&self) -> std::option::Option<&str> {
        self.function_error.as_deref()
    }
    /// <p>The last 4 KB of the execution log, which is base64-encoded.</p>
    pub fn log_result(&self) -> std::option::Option<&str> {
        self.log_result.as_deref()
    }
    /// <p>The response from the function, or an error object.</p>
    pub fn payload(&self) -> std::option::Option<&aws_smithy_types::Blob> {
        self.payload.as_ref()
    }
    /// <p>The version of the function that executed. When you invoke a function with an alias, this indicates which version the alias resolved to.</p>
    pub fn executed_version(&self) -> std::option::Option<&str> {
        self.executed_version.as_deref()
    }
}
impl std::fmt::Debug for InvokeOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InvokeOutput");
        formatter.field("status_code", &self.status_code);
        formatter.field("function_error", &self.function_error);
        formatter.field("log_result", &self.log_result);
        formatter.field("payload", &"*** Sensitive Data Redacted ***");
        formatter.field("executed_version", &self.executed_version);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
impl aws_http::request_id::RequestId for InvokeOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl InvokeOutput {
    /// Creates a new builder-style object to manufacture [`InvokeOutput`](crate::operation::invoke::InvokeOutput).
    pub fn builder() -> crate::operation::invoke::builders::InvokeOutputBuilder {
        crate::operation::invoke::builders::InvokeOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::invoke::InvokeOutput;
/// A builder for [`InvokeOutput`](crate::operation::invoke::InvokeOutput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct InvokeOutputBuilder {
    pub(crate) status_code: std::option::Option<i32>,
    pub(crate) function_error: std::option::Option<std::string::String>,
    pub(crate) log_result: std::option::Option<std::string::String>,
    pub(crate) payload: std::option::Option<aws_smithy_types::Blob>,
    pub(crate) executed_version: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl InvokeOutputBuilder {
    /// <p>The HTTP status code is in the 200 range for a successful request. For the <code>RequestResponse</code> invocation type, this status code is 200. For the <code>Event</code> invocation type, this status code is 202. For the <code>DryRun</code> invocation type, the status code is 204.</p>
    pub fn status_code(mut self, input: i32) -> Self {
        self.status_code = Some(input);
        self
    }
    /// <p>The HTTP status code is in the 200 range for a successful request. For the <code>RequestResponse</code> invocation type, this status code is 200. For the <code>Event</code> invocation type, this status code is 202. For the <code>DryRun</code> invocation type, the status code is 204.</p>
    pub fn set_status_code(mut self, input: std::option::Option<i32>) -> Self {
        self.status_code = input;
        self
    }
    /// <p>If present, indicates that an error occurred during function execution. Details about the error are included in the response payload.</p>
    pub fn function_error(mut self, input: impl Into<std::string::String>) -> Self {
        self.function_error = Some(input.into());
        self
    }
    /// <p>If present, indicates that an error occurred during function execution. Details about the error are included in the response payload.</p>
    pub fn set_function_error(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.function_error = input;
        self
    }
    /// <p>The last 4 KB of the execution log, which is base64-encoded.</p>
    pub fn log_result(mut self, input: impl Into<std::string::String>) -> Self {
        self.log_result = Some(input.into());
        self
    }
    /// <p>The last 4 KB of the execution log, which is base64-encoded.</p>
    pub fn set_log_result(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.log_result = input;
        self
    }
    /// <p>The response from the function, or an error object.</p>
    pub fn payload(mut self, input: aws_smithy_types::Blob) -> Self {
        self.payload = Some(input);
        self
    }
    /// <p>The response from the function, or an error object.</p>
    pub fn set_payload(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
        self.payload = input;
        self
    }
    /// <p>The version of the function that executed. When you invoke a function with an alias, this indicates which version the alias resolved to.</p>
    pub fn executed_version(mut self, input: impl Into<std::string::String>) -> Self {
        self.executed_version = Some(input.into());
        self
    }
    /// <p>The version of the function that executed. When you invoke a function with an alias, this indicates which version the alias resolved to.</p>
    pub fn set_executed_version(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.executed_version = input;
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
    /// Consumes the builder and constructs a [`InvokeOutput`](crate::operation::invoke::InvokeOutput).
    pub fn build(self) -> crate::operation::invoke::InvokeOutput {
        crate::operation::invoke::InvokeOutput {
            status_code: self.status_code.unwrap_or_default(),
            function_error: self.function_error,
            log_result: self.log_result,
            payload: self.payload,
            executed_version: self.executed_version,
            _request_id: self._request_id,
        }
    }
}
impl std::fmt::Debug for InvokeOutputBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InvokeOutputBuilder");
        formatter.field("status_code", &self.status_code);
        formatter.field("function_error", &self.function_error);
        formatter.field("log_result", &self.log_result);
        formatter.field("payload", &"*** Sensitive Data Redacted ***");
        formatter.field("executed_version", &self.executed_version);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
