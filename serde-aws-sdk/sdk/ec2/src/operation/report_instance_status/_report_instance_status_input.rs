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
pub struct ReportInstanceStatusInput {
    /// <p>Descriptive text about the health state of your instance.</p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>The time at which the reported instance health state ended.</p>
    #[doc(hidden)]
    pub end_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The instances.</p>
    #[doc(hidden)]
    pub instances: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The reason codes that describe the health state of your instance.</p>
    /// <ul>
    /// <li> <p> <code>instance-stuck-in-state</code>: My instance is stuck in a state.</p> </li>
    /// <li> <p> <code>unresponsive</code>: My instance is unresponsive.</p> </li>
    /// <li> <p> <code>not-accepting-credentials</code>: My instance is not accepting my credentials.</p> </li>
    /// <li> <p> <code>password-not-available</code>: A password is not available for my instance.</p> </li>
    /// <li> <p> <code>performance-network</code>: My instance is experiencing performance problems that I believe are network related.</p> </li>
    /// <li> <p> <code>performance-instance-store</code>: My instance is experiencing performance problems that I believe are related to the instance stores.</p> </li>
    /// <li> <p> <code>performance-ebs-volume</code>: My instance is experiencing performance problems that I believe are related to an EBS volume.</p> </li>
    /// <li> <p> <code>performance-other</code>: My instance is experiencing performance problems.</p> </li>
    /// <li> <p> <code>other</code>: [explain using the description parameter]</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub reason_codes: std::option::Option<std::vec::Vec<crate::types::ReportInstanceReasonCodes>>,
    /// <p>The time at which the reported instance health state began.</p>
    #[doc(hidden)]
    pub start_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The status of all instances listed.</p>
    #[doc(hidden)]
    pub status: std::option::Option<crate::types::ReportStatusType>,
}
impl ReportInstanceStatusInput {
    /// <p>Descriptive text about the health state of your instance.</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The time at which the reported instance health state ended.</p>
    pub fn end_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.end_time.as_ref()
    }
    /// <p>The instances.</p>
    pub fn instances(&self) -> std::option::Option<&[std::string::String]> {
        self.instances.as_deref()
    }
    /// <p>The reason codes that describe the health state of your instance.</p>
    /// <ul>
    /// <li> <p> <code>instance-stuck-in-state</code>: My instance is stuck in a state.</p> </li>
    /// <li> <p> <code>unresponsive</code>: My instance is unresponsive.</p> </li>
    /// <li> <p> <code>not-accepting-credentials</code>: My instance is not accepting my credentials.</p> </li>
    /// <li> <p> <code>password-not-available</code>: A password is not available for my instance.</p> </li>
    /// <li> <p> <code>performance-network</code>: My instance is experiencing performance problems that I believe are network related.</p> </li>
    /// <li> <p> <code>performance-instance-store</code>: My instance is experiencing performance problems that I believe are related to the instance stores.</p> </li>
    /// <li> <p> <code>performance-ebs-volume</code>: My instance is experiencing performance problems that I believe are related to an EBS volume.</p> </li>
    /// <li> <p> <code>performance-other</code>: My instance is experiencing performance problems.</p> </li>
    /// <li> <p> <code>other</code>: [explain using the description parameter]</p> </li>
    /// </ul>
    pub fn reason_codes(&self) -> std::option::Option<&[crate::types::ReportInstanceReasonCodes]> {
        self.reason_codes.as_deref()
    }
    /// <p>The time at which the reported instance health state began.</p>
    pub fn start_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.start_time.as_ref()
    }
    /// <p>The status of all instances listed.</p>
    pub fn status(&self) -> std::option::Option<&crate::types::ReportStatusType> {
        self.status.as_ref()
    }
}
impl ReportInstanceStatusInput {
    /// Creates a new builder-style object to manufacture [`ReportInstanceStatusInput`](crate::operation::report_instance_status::ReportInstanceStatusInput).
    pub fn builder(
    ) -> crate::operation::report_instance_status::builders::ReportInstanceStatusInputBuilder {
        crate::operation::report_instance_status::builders::ReportInstanceStatusInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::report_instance_status::ReportInstanceStatusInput;
/// A builder for [`ReportInstanceStatusInput`](crate::operation::report_instance_status::ReportInstanceStatusInput).
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
pub struct ReportInstanceStatusInputBuilder {
    pub(crate) description: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) end_time: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) instances: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) reason_codes:
        std::option::Option<std::vec::Vec<crate::types::ReportInstanceReasonCodes>>,
    pub(crate) start_time: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) status: std::option::Option<crate::types::ReportStatusType>,
}
impl ReportInstanceStatusInputBuilder {
    /// <p>Descriptive text about the health state of your instance.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.description = Some(input.into());
        self
    }
    /// <p>Descriptive text about the health state of your instance.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>The time at which the reported instance health state ended.</p>
    pub fn end_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.end_time = Some(input);
        self
    }
    /// <p>The time at which the reported instance health state ended.</p>
    pub fn set_end_time(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
        self.end_time = input;
        self
    }
    /// Appends an item to `instances`.
    ///
    /// To override the contents of this collection use [`set_instances`](Self::set_instances).
    ///
    /// <p>The instances.</p>
    pub fn instances(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.instances.unwrap_or_default();
        v.push(input.into());
        self.instances = Some(v);
        self
    }
    /// <p>The instances.</p>
    pub fn set_instances(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.instances = input;
        self
    }
    /// Appends an item to `reason_codes`.
    ///
    /// To override the contents of this collection use [`set_reason_codes`](Self::set_reason_codes).
    ///
    /// <p>The reason codes that describe the health state of your instance.</p>
    /// <ul>
    /// <li> <p> <code>instance-stuck-in-state</code>: My instance is stuck in a state.</p> </li>
    /// <li> <p> <code>unresponsive</code>: My instance is unresponsive.</p> </li>
    /// <li> <p> <code>not-accepting-credentials</code>: My instance is not accepting my credentials.</p> </li>
    /// <li> <p> <code>password-not-available</code>: A password is not available for my instance.</p> </li>
    /// <li> <p> <code>performance-network</code>: My instance is experiencing performance problems that I believe are network related.</p> </li>
    /// <li> <p> <code>performance-instance-store</code>: My instance is experiencing performance problems that I believe are related to the instance stores.</p> </li>
    /// <li> <p> <code>performance-ebs-volume</code>: My instance is experiencing performance problems that I believe are related to an EBS volume.</p> </li>
    /// <li> <p> <code>performance-other</code>: My instance is experiencing performance problems.</p> </li>
    /// <li> <p> <code>other</code>: [explain using the description parameter]</p> </li>
    /// </ul>
    pub fn reason_codes(mut self, input: crate::types::ReportInstanceReasonCodes) -> Self {
        let mut v = self.reason_codes.unwrap_or_default();
        v.push(input);
        self.reason_codes = Some(v);
        self
    }
    /// <p>The reason codes that describe the health state of your instance.</p>
    /// <ul>
    /// <li> <p> <code>instance-stuck-in-state</code>: My instance is stuck in a state.</p> </li>
    /// <li> <p> <code>unresponsive</code>: My instance is unresponsive.</p> </li>
    /// <li> <p> <code>not-accepting-credentials</code>: My instance is not accepting my credentials.</p> </li>
    /// <li> <p> <code>password-not-available</code>: A password is not available for my instance.</p> </li>
    /// <li> <p> <code>performance-network</code>: My instance is experiencing performance problems that I believe are network related.</p> </li>
    /// <li> <p> <code>performance-instance-store</code>: My instance is experiencing performance problems that I believe are related to the instance stores.</p> </li>
    /// <li> <p> <code>performance-ebs-volume</code>: My instance is experiencing performance problems that I believe are related to an EBS volume.</p> </li>
    /// <li> <p> <code>performance-other</code>: My instance is experiencing performance problems.</p> </li>
    /// <li> <p> <code>other</code>: [explain using the description parameter]</p> </li>
    /// </ul>
    pub fn set_reason_codes(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ReportInstanceReasonCodes>>,
    ) -> Self {
        self.reason_codes = input;
        self
    }
    /// <p>The time at which the reported instance health state began.</p>
    pub fn start_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.start_time = Some(input);
        self
    }
    /// <p>The time at which the reported instance health state began.</p>
    pub fn set_start_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.start_time = input;
        self
    }
    /// <p>The status of all instances listed.</p>
    pub fn status(mut self, input: crate::types::ReportStatusType) -> Self {
        self.status = Some(input);
        self
    }
    /// <p>The status of all instances listed.</p>
    pub fn set_status(
        mut self,
        input: std::option::Option<crate::types::ReportStatusType>,
    ) -> Self {
        self.status = input;
        self
    }
    /// Consumes the builder and constructs a [`ReportInstanceStatusInput`](crate::operation::report_instance_status::ReportInstanceStatusInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::report_instance_status::ReportInstanceStatusInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::report_instance_status::ReportInstanceStatusInput {
                description: self.description,
                dry_run: self.dry_run,
                end_time: self.end_time,
                instances: self.instances,
                reason_codes: self.reason_codes,
                start_time: self.start_time,
                status: self.status,
            },
        )
    }
}
