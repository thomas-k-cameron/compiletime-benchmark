// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the configuration parameters for a job-completion report.</p>
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
pub struct JobReport {
    /// <p>The Amazon Resource Name (ARN) for the bucket where specified job-completion report will be stored.</p>
    #[doc(hidden)]
    pub bucket: std::option::Option<std::string::String>,
    /// <p>The format of the specified job-completion report.</p>
    #[doc(hidden)]
    pub format: std::option::Option<crate::types::JobReportFormat>,
    /// <p>Indicates whether the specified job will generate a job-completion report.</p>
    #[doc(hidden)]
    pub enabled: bool,
    /// <p>An optional prefix to describe where in the specified bucket the job-completion report will be stored. Amazon S3 stores the job-completion report at <code>
    /// <prefix>
    /// /job-
    /// <job-id>
    /// /report.json
    /// </job-id>
    /// </prefix></code>.</p>
    #[doc(hidden)]
    pub prefix: std::option::Option<std::string::String>,
    /// <p>Indicates whether the job-completion report will include details of all tasks or only failed tasks.</p>
    #[doc(hidden)]
    pub report_scope: std::option::Option<crate::types::JobReportScope>,
}
impl JobReport {
    /// <p>The Amazon Resource Name (ARN) for the bucket where specified job-completion report will be stored.</p>
    pub fn bucket(&self) -> std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The format of the specified job-completion report.</p>
    pub fn format(&self) -> std::option::Option<&crate::types::JobReportFormat> {
        self.format.as_ref()
    }
    /// <p>Indicates whether the specified job will generate a job-completion report.</p>
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    /// <p>An optional prefix to describe where in the specified bucket the job-completion report will be stored. Amazon S3 stores the job-completion report at <code>
    /// <prefix>
    /// /job-
    /// <job-id>
    /// /report.json
    /// </job-id>
    /// </prefix></code>.</p>
    pub fn prefix(&self) -> std::option::Option<&str> {
        self.prefix.as_deref()
    }
    /// <p>Indicates whether the job-completion report will include details of all tasks or only failed tasks.</p>
    pub fn report_scope(&self) -> std::option::Option<&crate::types::JobReportScope> {
        self.report_scope.as_ref()
    }
}
impl JobReport {
    /// Creates a new builder-style object to manufacture [`JobReport`](crate::types::JobReport).
    pub fn builder() -> crate::types::builders::JobReportBuilder {
        crate::types::builders::JobReportBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::JobReport;
/// A builder for [`JobReport`](crate::types::JobReport).
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
pub struct JobReportBuilder {
    pub(crate) bucket: std::option::Option<std::string::String>,
    pub(crate) format: std::option::Option<crate::types::JobReportFormat>,
    pub(crate) enabled: std::option::Option<bool>,
    pub(crate) prefix: std::option::Option<std::string::String>,
    pub(crate) report_scope: std::option::Option<crate::types::JobReportScope>,
}
impl JobReportBuilder {
    /// <p>The Amazon Resource Name (ARN) for the bucket where specified job-completion report will be stored.</p>
    pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.bucket = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the bucket where specified job-completion report will be stored.</p>
    pub fn set_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The format of the specified job-completion report.</p>
    pub fn format(mut self, input: crate::types::JobReportFormat) -> Self {
        self.format = Some(input);
        self
    }
    /// <p>The format of the specified job-completion report.</p>
    pub fn set_format(mut self, input: std::option::Option<crate::types::JobReportFormat>) -> Self {
        self.format = input;
        self
    }
    /// <p>Indicates whether the specified job will generate a job-completion report.</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.enabled = Some(input);
        self
    }
    /// <p>Indicates whether the specified job will generate a job-completion report.</p>
    pub fn set_enabled(mut self, input: std::option::Option<bool>) -> Self {
        self.enabled = input;
        self
    }
    /// <p>An optional prefix to describe where in the specified bucket the job-completion report will be stored. Amazon S3 stores the job-completion report at <code>
    /// <prefix>
    /// /job-
    /// <job-id>
    /// /report.json
    /// </job-id>
    /// </prefix></code>.</p>
    pub fn prefix(mut self, input: impl Into<std::string::String>) -> Self {
        self.prefix = Some(input.into());
        self
    }
    /// <p>An optional prefix to describe where in the specified bucket the job-completion report will be stored. Amazon S3 stores the job-completion report at <code>
    /// <prefix>
    /// /job-
    /// <job-id>
    /// /report.json
    /// </job-id>
    /// </prefix></code>.</p>
    pub fn set_prefix(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.prefix = input;
        self
    }
    /// <p>Indicates whether the job-completion report will include details of all tasks or only failed tasks.</p>
    pub fn report_scope(mut self, input: crate::types::JobReportScope) -> Self {
        self.report_scope = Some(input);
        self
    }
    /// <p>Indicates whether the job-completion report will include details of all tasks or only failed tasks.</p>
    pub fn set_report_scope(
        mut self,
        input: std::option::Option<crate::types::JobReportScope>,
    ) -> Self {
        self.report_scope = input;
        self
    }
    /// Consumes the builder and constructs a [`JobReport`](crate::types::JobReport).
    pub fn build(self) -> crate::types::JobReport {
        crate::types::JobReport {
            bucket: self.bucket,
            format: self.format,
            enabled: self.enabled.unwrap_or_default(),
            prefix: self.prefix,
            report_scope: self.report_scope,
        }
    }
}
