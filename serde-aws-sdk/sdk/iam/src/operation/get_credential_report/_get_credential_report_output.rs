// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the response to a successful <code>GetCredentialReport</code> request. </p>
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
pub struct GetCredentialReportOutput {
    /// <p>Contains the credential report. The report is Base64-encoded.</p>
    #[doc(hidden)]
    pub content: std::option::Option<aws_smithy_types::Blob>,
    /// <p>The format (MIME type) of the credential report.</p>
    #[doc(hidden)]
    pub report_format: std::option::Option<crate::types::ReportFormatType>,
    /// <p> The date and time when the credential report was created, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a>.</p>
    #[doc(hidden)]
    pub generated_time: std::option::Option<aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl GetCredentialReportOutput {
    /// <p>Contains the credential report. The report is Base64-encoded.</p>
    pub fn content(&self) -> std::option::Option<&aws_smithy_types::Blob> {
        self.content.as_ref()
    }
    /// <p>The format (MIME type) of the credential report.</p>
    pub fn report_format(&self) -> std::option::Option<&crate::types::ReportFormatType> {
        self.report_format.as_ref()
    }
    /// <p> The date and time when the credential report was created, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a>.</p>
    pub fn generated_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.generated_time.as_ref()
    }
}
impl aws_http::request_id::RequestId for GetCredentialReportOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetCredentialReportOutput {
    /// Creates a new builder-style object to manufacture [`GetCredentialReportOutput`](crate::operation::get_credential_report::GetCredentialReportOutput).
    pub fn builder(
    ) -> crate::operation::get_credential_report::builders::GetCredentialReportOutputBuilder {
        crate::operation::get_credential_report::builders::GetCredentialReportOutputBuilder::default(
        )
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_credential_report::GetCredentialReportOutput;
/// A builder for [`GetCredentialReportOutput`](crate::operation::get_credential_report::GetCredentialReportOutput).
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
pub struct GetCredentialReportOutputBuilder {
    pub(crate) content: std::option::Option<aws_smithy_types::Blob>,
    pub(crate) report_format: std::option::Option<crate::types::ReportFormatType>,
    pub(crate) generated_time: std::option::Option<aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl GetCredentialReportOutputBuilder {
    /// <p>Contains the credential report. The report is Base64-encoded.</p>
    pub fn content(mut self, input: aws_smithy_types::Blob) -> Self {
        self.content = Some(input);
        self
    }
    /// <p>Contains the credential report. The report is Base64-encoded.</p>
    pub fn set_content(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
        self.content = input;
        self
    }
    /// <p>The format (MIME type) of the credential report.</p>
    pub fn report_format(mut self, input: crate::types::ReportFormatType) -> Self {
        self.report_format = Some(input);
        self
    }
    /// <p>The format (MIME type) of the credential report.</p>
    pub fn set_report_format(
        mut self,
        input: std::option::Option<crate::types::ReportFormatType>,
    ) -> Self {
        self.report_format = input;
        self
    }
    /// <p> The date and time when the credential report was created, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a>.</p>
    pub fn generated_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.generated_time = Some(input);
        self
    }
    /// <p> The date and time when the credential report was created, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a>.</p>
    pub fn set_generated_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.generated_time = input;
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
    /// Consumes the builder and constructs a [`GetCredentialReportOutput`](crate::operation::get_credential_report::GetCredentialReportOutput).
    pub fn build(self) -> crate::operation::get_credential_report::GetCredentialReportOutput {
        crate::operation::get_credential_report::GetCredentialReportOutput {
            content: self.content,
            report_format: self.report_format,
            generated_time: self.generated_time,
            _request_id: self._request_id,
        }
    }
}
