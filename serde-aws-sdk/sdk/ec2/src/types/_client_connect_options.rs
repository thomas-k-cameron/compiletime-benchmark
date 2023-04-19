// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The options for managing connection authorization for new client connections.</p>
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
pub struct ClientConnectOptions {
    /// <p>Indicates whether client connect options are enabled. The default is <code>false</code> (not enabled).</p>
    #[doc(hidden)]
    pub enabled: std::option::Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the Lambda function used for connection authorization.</p>
    #[doc(hidden)]
    pub lambda_function_arn: std::option::Option<std::string::String>,
}
impl ClientConnectOptions {
    /// <p>Indicates whether client connect options are enabled. The default is <code>false</code> (not enabled).</p>
    pub fn enabled(&self) -> std::option::Option<bool> {
        self.enabled
    }
    /// <p>The Amazon Resource Name (ARN) of the Lambda function used for connection authorization.</p>
    pub fn lambda_function_arn(&self) -> std::option::Option<&str> {
        self.lambda_function_arn.as_deref()
    }
}
impl ClientConnectOptions {
    /// Creates a new builder-style object to manufacture [`ClientConnectOptions`](crate::types::ClientConnectOptions).
    pub fn builder() -> crate::types::builders::ClientConnectOptionsBuilder {
        crate::types::builders::ClientConnectOptionsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ClientConnectOptions;
/// A builder for [`ClientConnectOptions`](crate::types::ClientConnectOptions).
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
pub struct ClientConnectOptionsBuilder {
    pub(crate) enabled: std::option::Option<bool>,
    pub(crate) lambda_function_arn: std::option::Option<std::string::String>,
}
impl ClientConnectOptionsBuilder {
    /// <p>Indicates whether client connect options are enabled. The default is <code>false</code> (not enabled).</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.enabled = Some(input);
        self
    }
    /// <p>Indicates whether client connect options are enabled. The default is <code>false</code> (not enabled).</p>
    pub fn set_enabled(mut self, input: std::option::Option<bool>) -> Self {
        self.enabled = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Lambda function used for connection authorization.</p>
    pub fn lambda_function_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.lambda_function_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Lambda function used for connection authorization.</p>
    pub fn set_lambda_function_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.lambda_function_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`ClientConnectOptions`](crate::types::ClientConnectOptions).
    pub fn build(self) -> crate::types::ClientConnectOptions {
        crate::types::ClientConnectOptions {
            enabled: self.enabled,
            lambda_function_arn: self.lambda_function_arn,
        }
    }
}
