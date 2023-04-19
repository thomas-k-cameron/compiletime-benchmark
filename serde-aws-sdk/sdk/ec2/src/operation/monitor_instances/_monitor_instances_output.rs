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
pub struct MonitorInstancesOutput {
    /// <p>The monitoring information.</p>
    #[doc(hidden)]
    pub instance_monitorings: std::option::Option<std::vec::Vec<crate::types::InstanceMonitoring>>,
    _request_id: Option<String>,
}
impl MonitorInstancesOutput {
    /// <p>The monitoring information.</p>
    pub fn instance_monitorings(&self) -> std::option::Option<&[crate::types::InstanceMonitoring]> {
        self.instance_monitorings.as_deref()
    }
}
impl aws_http::request_id::RequestId for MonitorInstancesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl MonitorInstancesOutput {
    /// Creates a new builder-style object to manufacture [`MonitorInstancesOutput`](crate::operation::monitor_instances::MonitorInstancesOutput).
    pub fn builder() -> crate::operation::monitor_instances::builders::MonitorInstancesOutputBuilder
    {
        crate::operation::monitor_instances::builders::MonitorInstancesOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::monitor_instances::MonitorInstancesOutput;
/// A builder for [`MonitorInstancesOutput`](crate::operation::monitor_instances::MonitorInstancesOutput).
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
pub struct MonitorInstancesOutputBuilder {
    pub(crate) instance_monitorings:
        std::option::Option<std::vec::Vec<crate::types::InstanceMonitoring>>,
    _request_id: Option<String>,
}
impl MonitorInstancesOutputBuilder {
    /// Appends an item to `instance_monitorings`.
    ///
    /// To override the contents of this collection use [`set_instance_monitorings`](Self::set_instance_monitorings).
    ///
    /// <p>The monitoring information.</p>
    pub fn instance_monitorings(mut self, input: crate::types::InstanceMonitoring) -> Self {
        let mut v = self.instance_monitorings.unwrap_or_default();
        v.push(input);
        self.instance_monitorings = Some(v);
        self
    }
    /// <p>The monitoring information.</p>
    pub fn set_instance_monitorings(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::InstanceMonitoring>>,
    ) -> Self {
        self.instance_monitorings = input;
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
    /// Consumes the builder and constructs a [`MonitorInstancesOutput`](crate::operation::monitor_instances::MonitorInstancesOutput).
    pub fn build(self) -> crate::operation::monitor_instances::MonitorInstancesOutput {
        crate::operation::monitor_instances::MonitorInstancesOutput {
            instance_monitorings: self.instance_monitorings,
            _request_id: self._request_id,
        }
    }
}
