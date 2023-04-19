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
pub struct ListExportsOutput {
    /// <p>A list of <code>ExportSummary</code> objects.</p>
    #[doc(hidden)]
    pub export_summaries: std::option::Option<std::vec::Vec<crate::types::ExportSummary>>,
    /// <p>If this value is returned, there are additional results to be displayed. To retrieve them, call <code>ListExports</code> again, with <code>NextToken</code> set to this value.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListExportsOutput {
    /// <p>A list of <code>ExportSummary</code> objects.</p>
    pub fn export_summaries(&self) -> std::option::Option<&[crate::types::ExportSummary]> {
        self.export_summaries.as_deref()
    }
    /// <p>If this value is returned, there are additional results to be displayed. To retrieve them, call <code>ListExports</code> again, with <code>NextToken</code> set to this value.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for ListExportsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListExportsOutput {
    /// Creates a new builder-style object to manufacture [`ListExportsOutput`](crate::operation::list_exports::ListExportsOutput).
    pub fn builder() -> crate::operation::list_exports::builders::ListExportsOutputBuilder {
        crate::operation::list_exports::builders::ListExportsOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_exports::ListExportsOutput;
/// A builder for [`ListExportsOutput`](crate::operation::list_exports::ListExportsOutput).
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
pub struct ListExportsOutputBuilder {
    pub(crate) export_summaries: std::option::Option<std::vec::Vec<crate::types::ExportSummary>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListExportsOutputBuilder {
    /// Appends an item to `export_summaries`.
    ///
    /// To override the contents of this collection use [`set_export_summaries`](Self::set_export_summaries).
    ///
    /// <p>A list of <code>ExportSummary</code> objects.</p>
    pub fn export_summaries(mut self, input: crate::types::ExportSummary) -> Self {
        let mut v = self.export_summaries.unwrap_or_default();
        v.push(input);
        self.export_summaries = Some(v);
        self
    }
    /// <p>A list of <code>ExportSummary</code> objects.</p>
    pub fn set_export_summaries(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ExportSummary>>,
    ) -> Self {
        self.export_summaries = input;
        self
    }
    /// <p>If this value is returned, there are additional results to be displayed. To retrieve them, call <code>ListExports</code> again, with <code>NextToken</code> set to this value.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>If this value is returned, there are additional results to be displayed. To retrieve them, call <code>ListExports</code> again, with <code>NextToken</code> set to this value.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
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
    /// Consumes the builder and constructs a [`ListExportsOutput`](crate::operation::list_exports::ListExportsOutput).
    pub fn build(self) -> crate::operation::list_exports::ListExportsOutput {
        crate::operation::list_exports::ListExportsOutput {
            export_summaries: self.export_summaries,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
