// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A complex type that contains the response to a <code>GetHealthCheck</code> request.</p>
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
pub struct GetHealthCheckOutput {
    /// <p>A complex type that contains information about one health check that is associated with the current Amazon Web Services account.</p>
    #[doc(hidden)]
    pub health_check: std::option::Option<crate::types::HealthCheck>,
    _request_id: Option<String>,
}
impl GetHealthCheckOutput {
    /// <p>A complex type that contains information about one health check that is associated with the current Amazon Web Services account.</p>
    pub fn health_check(&self) -> std::option::Option<&crate::types::HealthCheck> {
        self.health_check.as_ref()
    }
}
impl aws_http::request_id::RequestId for GetHealthCheckOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetHealthCheckOutput {
    /// Creates a new builder-style object to manufacture [`GetHealthCheckOutput`](crate::operation::get_health_check::GetHealthCheckOutput).
    pub fn builder() -> crate::operation::get_health_check::builders::GetHealthCheckOutputBuilder {
        crate::operation::get_health_check::builders::GetHealthCheckOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_health_check::GetHealthCheckOutput;
/// A builder for [`GetHealthCheckOutput`](crate::operation::get_health_check::GetHealthCheckOutput).
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
pub struct GetHealthCheckOutputBuilder {
    pub(crate) health_check: std::option::Option<crate::types::HealthCheck>,
    _request_id: Option<String>,
}
impl GetHealthCheckOutputBuilder {
    /// <p>A complex type that contains information about one health check that is associated with the current Amazon Web Services account.</p>
    pub fn health_check(mut self, input: crate::types::HealthCheck) -> Self {
        self.health_check = Some(input);
        self
    }
    /// <p>A complex type that contains information about one health check that is associated with the current Amazon Web Services account.</p>
    pub fn set_health_check(
        mut self,
        input: std::option::Option<crate::types::HealthCheck>,
    ) -> Self {
        self.health_check = input;
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
    /// Consumes the builder and constructs a [`GetHealthCheckOutput`](crate::operation::get_health_check::GetHealthCheckOutput).
    pub fn build(self) -> crate::operation::get_health_check::GetHealthCheckOutput {
        crate::operation::get_health_check::GetHealthCheckOutput {
            health_check: self.health_check,
            _request_id: self._request_id,
        }
    }
}
