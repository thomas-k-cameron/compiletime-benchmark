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
pub struct DescribeRemediationConfigurationsOutput {
    /// <p>Returns a remediation configuration object.</p>
    #[doc(hidden)]
    pub remediation_configurations:
        std::option::Option<std::vec::Vec<crate::types::RemediationConfiguration>>,
    _request_id: Option<String>,
}
impl DescribeRemediationConfigurationsOutput {
    /// <p>Returns a remediation configuration object.</p>
    pub fn remediation_configurations(
        &self,
    ) -> std::option::Option<&[crate::types::RemediationConfiguration]> {
        self.remediation_configurations.as_deref()
    }
}
impl aws_http::request_id::RequestId for DescribeRemediationConfigurationsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeRemediationConfigurationsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeRemediationConfigurationsOutput`](crate::operation::describe_remediation_configurations::DescribeRemediationConfigurationsOutput).
    pub fn builder() -> crate::operation::describe_remediation_configurations::builders::DescribeRemediationConfigurationsOutputBuilder{
        crate::operation::describe_remediation_configurations::builders::DescribeRemediationConfigurationsOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::describe_remediation_configurations::DescribeRemediationConfigurationsOutput;
/// A builder for [`DescribeRemediationConfigurationsOutput`](crate::operation::describe_remediation_configurations::DescribeRemediationConfigurationsOutput).
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
pub struct DescribeRemediationConfigurationsOutputBuilder {
    pub(crate) remediation_configurations:
        std::option::Option<std::vec::Vec<crate::types::RemediationConfiguration>>,
    _request_id: Option<String>,
}
impl DescribeRemediationConfigurationsOutputBuilder {
    /// Appends an item to `remediation_configurations`.
    ///
    /// To override the contents of this collection use [`set_remediation_configurations`](Self::set_remediation_configurations).
    ///
    /// <p>Returns a remediation configuration object.</p>
    pub fn remediation_configurations(
        mut self,
        input: crate::types::RemediationConfiguration,
    ) -> Self {
        let mut v = self.remediation_configurations.unwrap_or_default();
        v.push(input);
        self.remediation_configurations = Some(v);
        self
    }
    /// <p>Returns a remediation configuration object.</p>
    pub fn set_remediation_configurations(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::RemediationConfiguration>>,
    ) -> Self {
        self.remediation_configurations = input;
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
    /// Consumes the builder and constructs a [`DescribeRemediationConfigurationsOutput`](crate::operation::describe_remediation_configurations::DescribeRemediationConfigurationsOutput).
    pub fn build(self) -> crate::operation::describe_remediation_configurations::DescribeRemediationConfigurationsOutput{
        crate::operation::describe_remediation_configurations::DescribeRemediationConfigurationsOutput {
            remediation_configurations: self.remediation_configurations
            ,
            _request_id: self._request_id,
        }
    }
}
