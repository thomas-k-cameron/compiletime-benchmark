// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the state of an authorization rule.</p>
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
pub struct ClientVpnAuthorizationRuleStatus {
    /// <p>The state of the authorization rule.</p>
    #[doc(hidden)]
    pub code: std::option::Option<crate::types::ClientVpnAuthorizationRuleStatusCode>,
    /// <p>A message about the status of the authorization rule, if applicable.</p>
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl ClientVpnAuthorizationRuleStatus {
    /// <p>The state of the authorization rule.</p>
    pub fn code(&self) -> std::option::Option<&crate::types::ClientVpnAuthorizationRuleStatusCode> {
        self.code.as_ref()
    }
    /// <p>A message about the status of the authorization rule, if applicable.</p>
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl ClientVpnAuthorizationRuleStatus {
    /// Creates a new builder-style object to manufacture [`ClientVpnAuthorizationRuleStatus`](crate::types::ClientVpnAuthorizationRuleStatus).
    pub fn builder() -> crate::types::builders::ClientVpnAuthorizationRuleStatusBuilder {
        crate::types::builders::ClientVpnAuthorizationRuleStatusBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ClientVpnAuthorizationRuleStatus;
/// A builder for [`ClientVpnAuthorizationRuleStatus`](crate::types::ClientVpnAuthorizationRuleStatus).
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
pub struct ClientVpnAuthorizationRuleStatusBuilder {
    pub(crate) code: std::option::Option<crate::types::ClientVpnAuthorizationRuleStatusCode>,
    pub(crate) message: std::option::Option<std::string::String>,
}
impl ClientVpnAuthorizationRuleStatusBuilder {
    /// <p>The state of the authorization rule.</p>
    pub fn code(mut self, input: crate::types::ClientVpnAuthorizationRuleStatusCode) -> Self {
        self.code = Some(input);
        self
    }
    /// <p>The state of the authorization rule.</p>
    pub fn set_code(
        mut self,
        input: std::option::Option<crate::types::ClientVpnAuthorizationRuleStatusCode>,
    ) -> Self {
        self.code = input;
        self
    }
    /// <p>A message about the status of the authorization rule, if applicable.</p>
    pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
        self.message = Some(input.into());
        self
    }
    /// <p>A message about the status of the authorization rule, if applicable.</p>
    pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// Consumes the builder and constructs a [`ClientVpnAuthorizationRuleStatus`](crate::types::ClientVpnAuthorizationRuleStatus).
    pub fn build(self) -> crate::types::ClientVpnAuthorizationRuleStatus {
        crate::types::ClientVpnAuthorizationRuleStatus {
            code: self.code,
            message: self.message,
        }
    }
}
