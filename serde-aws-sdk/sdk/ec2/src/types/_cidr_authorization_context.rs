// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides authorization for Amazon to bring a specific IP address range to a specific Amazon Web Services account using bring your own IP addresses (BYOIP). For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-byoip.html#prepare-for-byoip">Configuring your BYOIP address range</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
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
pub struct CidrAuthorizationContext {
    /// <p>The plain-text authorization message for the prefix and account.</p>
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    /// <p>The signed authorization message for the prefix and account.</p>
    #[doc(hidden)]
    pub signature: std::option::Option<std::string::String>,
}
impl CidrAuthorizationContext {
    /// <p>The plain-text authorization message for the prefix and account.</p>
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
    /// <p>The signed authorization message for the prefix and account.</p>
    pub fn signature(&self) -> std::option::Option<&str> {
        self.signature.as_deref()
    }
}
impl CidrAuthorizationContext {
    /// Creates a new builder-style object to manufacture [`CidrAuthorizationContext`](crate::types::CidrAuthorizationContext).
    pub fn builder() -> crate::types::builders::CidrAuthorizationContextBuilder {
        crate::types::builders::CidrAuthorizationContextBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::CidrAuthorizationContext;
/// A builder for [`CidrAuthorizationContext`](crate::types::CidrAuthorizationContext).
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
pub struct CidrAuthorizationContextBuilder {
    pub(crate) message: std::option::Option<std::string::String>,
    pub(crate) signature: std::option::Option<std::string::String>,
}
impl CidrAuthorizationContextBuilder {
    /// <p>The plain-text authorization message for the prefix and account.</p>
    pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
        self.message = Some(input.into());
        self
    }
    /// <p>The plain-text authorization message for the prefix and account.</p>
    pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// <p>The signed authorization message for the prefix and account.</p>
    pub fn signature(mut self, input: impl Into<std::string::String>) -> Self {
        self.signature = Some(input.into());
        self
    }
    /// <p>The signed authorization message for the prefix and account.</p>
    pub fn set_signature(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.signature = input;
        self
    }
    /// Consumes the builder and constructs a [`CidrAuthorizationContext`](crate::types::CidrAuthorizationContext).
    pub fn build(self) -> crate::types::CidrAuthorizationContext {
        crate::types::CidrAuthorizationContext {
            message: self.message,
            signature: self.signature,
        }
    }
}