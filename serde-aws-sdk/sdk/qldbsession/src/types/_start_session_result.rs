// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the details of the started session.</p>
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
pub struct StartSessionResult {
    /// <p>Session token of the started session. This <code>SessionToken</code> is required for every subsequent command that is issued during the current session.</p>
    #[doc(hidden)]
    pub session_token: std::option::Option<std::string::String>,
    /// <p>Contains server-side performance information for the command.</p>
    #[doc(hidden)]
    pub timing_information: std::option::Option<crate::types::TimingInformation>,
}
impl StartSessionResult {
    /// <p>Session token of the started session. This <code>SessionToken</code> is required for every subsequent command that is issued during the current session.</p>
    pub fn session_token(&self) -> std::option::Option<&str> {
        self.session_token.as_deref()
    }
    /// <p>Contains server-side performance information for the command.</p>
    pub fn timing_information(&self) -> std::option::Option<&crate::types::TimingInformation> {
        self.timing_information.as_ref()
    }
}
impl StartSessionResult {
    /// Creates a new builder-style object to manufacture [`StartSessionResult`](crate::types::StartSessionResult).
    pub fn builder() -> crate::types::builders::StartSessionResultBuilder {
        crate::types::builders::StartSessionResultBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::StartSessionResult;
/// A builder for [`StartSessionResult`](crate::types::StartSessionResult).
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
pub struct StartSessionResultBuilder {
    pub(crate) session_token: std::option::Option<std::string::String>,
    pub(crate) timing_information: std::option::Option<crate::types::TimingInformation>,
}
impl StartSessionResultBuilder {
    /// <p>Session token of the started session. This <code>SessionToken</code> is required for every subsequent command that is issued during the current session.</p>
    pub fn session_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.session_token = Some(input.into());
        self
    }
    /// <p>Session token of the started session. This <code>SessionToken</code> is required for every subsequent command that is issued during the current session.</p>
    pub fn set_session_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.session_token = input;
        self
    }
    /// <p>Contains server-side performance information for the command.</p>
    pub fn timing_information(mut self, input: crate::types::TimingInformation) -> Self {
        self.timing_information = Some(input);
        self
    }
    /// <p>Contains server-side performance information for the command.</p>
    pub fn set_timing_information(
        mut self,
        input: std::option::Option<crate::types::TimingInformation>,
    ) -> Self {
        self.timing_information = input;
        self
    }
    /// Consumes the builder and constructs a [`StartSessionResult`](crate::types::StartSessionResult).
    pub fn build(self) -> crate::types::StartSessionResult {
        crate::types::StartSessionResult {
            session_token: self.session_token,
            timing_information: self.timing_information,
        }
    }
}
