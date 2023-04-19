// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the response to a successful <code>ListServerCertificates</code> request. </p>
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
pub struct ListServerCertificatesOutput {
    /// <p>A list of server certificates.</p>
    #[doc(hidden)]
    pub server_certificate_metadata_list:
        std::option::Option<std::vec::Vec<crate::types::ServerCertificateMetadata>>,
    /// <p>A flag that indicates whether there are more items to return. If your results were truncated, you can make a subsequent pagination request using the <code>Marker</code> request parameter to retrieve more items. Note that IAM might return fewer than the <code>MaxItems</code> number of results even when there are more results available. We recommend that you check <code>IsTruncated</code> after every call to ensure that you receive all your results.</p>
    #[doc(hidden)]
    pub is_truncated: bool,
    /// <p>When <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent pagination request.</p>
    #[doc(hidden)]
    pub marker: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListServerCertificatesOutput {
    /// <p>A list of server certificates.</p>
    pub fn server_certificate_metadata_list(
        &self,
    ) -> std::option::Option<&[crate::types::ServerCertificateMetadata]> {
        self.server_certificate_metadata_list.as_deref()
    }
    /// <p>A flag that indicates whether there are more items to return. If your results were truncated, you can make a subsequent pagination request using the <code>Marker</code> request parameter to retrieve more items. Note that IAM might return fewer than the <code>MaxItems</code> number of results even when there are more results available. We recommend that you check <code>IsTruncated</code> after every call to ensure that you receive all your results.</p>
    pub fn is_truncated(&self) -> bool {
        self.is_truncated
    }
    /// <p>When <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent pagination request.</p>
    pub fn marker(&self) -> std::option::Option<&str> {
        self.marker.as_deref()
    }
}
impl aws_http::request_id::RequestId for ListServerCertificatesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListServerCertificatesOutput {
    /// Creates a new builder-style object to manufacture [`ListServerCertificatesOutput`](crate::operation::list_server_certificates::ListServerCertificatesOutput).
    pub fn builder(
    ) -> crate::operation::list_server_certificates::builders::ListServerCertificatesOutputBuilder
    {
        crate::operation::list_server_certificates::builders::ListServerCertificatesOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_server_certificates::ListServerCertificatesOutput;
/// A builder for [`ListServerCertificatesOutput`](crate::operation::list_server_certificates::ListServerCertificatesOutput).
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
pub struct ListServerCertificatesOutputBuilder {
    pub(crate) server_certificate_metadata_list:
        std::option::Option<std::vec::Vec<crate::types::ServerCertificateMetadata>>,
    pub(crate) is_truncated: std::option::Option<bool>,
    pub(crate) marker: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListServerCertificatesOutputBuilder {
    /// Appends an item to `server_certificate_metadata_list`.
    ///
    /// To override the contents of this collection use [`set_server_certificate_metadata_list`](Self::set_server_certificate_metadata_list).
    ///
    /// <p>A list of server certificates.</p>
    pub fn server_certificate_metadata_list(
        mut self,
        input: crate::types::ServerCertificateMetadata,
    ) -> Self {
        let mut v = self.server_certificate_metadata_list.unwrap_or_default();
        v.push(input);
        self.server_certificate_metadata_list = Some(v);
        self
    }
    /// <p>A list of server certificates.</p>
    pub fn set_server_certificate_metadata_list(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ServerCertificateMetadata>>,
    ) -> Self {
        self.server_certificate_metadata_list = input;
        self
    }
    /// <p>A flag that indicates whether there are more items to return. If your results were truncated, you can make a subsequent pagination request using the <code>Marker</code> request parameter to retrieve more items. Note that IAM might return fewer than the <code>MaxItems</code> number of results even when there are more results available. We recommend that you check <code>IsTruncated</code> after every call to ensure that you receive all your results.</p>
    pub fn is_truncated(mut self, input: bool) -> Self {
        self.is_truncated = Some(input);
        self
    }
    /// <p>A flag that indicates whether there are more items to return. If your results were truncated, you can make a subsequent pagination request using the <code>Marker</code> request parameter to retrieve more items. Note that IAM might return fewer than the <code>MaxItems</code> number of results even when there are more results available. We recommend that you check <code>IsTruncated</code> after every call to ensure that you receive all your results.</p>
    pub fn set_is_truncated(mut self, input: std::option::Option<bool>) -> Self {
        self.is_truncated = input;
        self
    }
    /// <p>When <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent pagination request.</p>
    pub fn marker(mut self, input: impl Into<std::string::String>) -> Self {
        self.marker = Some(input.into());
        self
    }
    /// <p>When <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent pagination request.</p>
    pub fn set_marker(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.marker = input;
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
    /// Consumes the builder and constructs a [`ListServerCertificatesOutput`](crate::operation::list_server_certificates::ListServerCertificatesOutput).
    pub fn build(self) -> crate::operation::list_server_certificates::ListServerCertificatesOutput {
        crate::operation::list_server_certificates::ListServerCertificatesOutput {
            server_certificate_metadata_list: self.server_certificate_metadata_list,
            is_truncated: self.is_truncated.unwrap_or_default(),
            marker: self.marker,
            _request_id: self._request_id,
        }
    }
}