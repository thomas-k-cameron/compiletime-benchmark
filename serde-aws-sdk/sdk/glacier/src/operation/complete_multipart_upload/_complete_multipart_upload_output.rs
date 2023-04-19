// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the Amazon S3 Glacier response to your request.</p>
/// <p>For information about the underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-archive-post.html">Upload Archive</a>. For conceptual information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/working-with-archives.html">Working with Archives in Amazon S3 Glacier</a>.</p>
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
pub struct CompleteMultipartUploadOutput {
    /// <p>The relative URI path of the newly added archive resource.</p>
    #[doc(hidden)]
    pub location: std::option::Option<std::string::String>,
    /// <p>The checksum of the archive computed by Amazon S3 Glacier.</p>
    #[doc(hidden)]
    pub checksum: std::option::Option<std::string::String>,
    /// <p>The ID of the archive. This value is also included as part of the location.</p>
    #[doc(hidden)]
    pub archive_id: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl CompleteMultipartUploadOutput {
    /// <p>The relative URI path of the newly added archive resource.</p>
    pub fn location(&self) -> std::option::Option<&str> {
        self.location.as_deref()
    }
    /// <p>The checksum of the archive computed by Amazon S3 Glacier.</p>
    pub fn checksum(&self) -> std::option::Option<&str> {
        self.checksum.as_deref()
    }
    /// <p>The ID of the archive. This value is also included as part of the location.</p>
    pub fn archive_id(&self) -> std::option::Option<&str> {
        self.archive_id.as_deref()
    }
}
impl aws_http::request_id::RequestId for CompleteMultipartUploadOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CompleteMultipartUploadOutput {
    /// Creates a new builder-style object to manufacture [`CompleteMultipartUploadOutput`](crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput).
    pub fn builder(
    ) -> crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadOutputBuilder
    {
        crate::operation::complete_multipart_upload::builders::CompleteMultipartUploadOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput;
/// A builder for [`CompleteMultipartUploadOutput`](crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput).
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
pub struct CompleteMultipartUploadOutputBuilder {
    pub(crate) location: std::option::Option<std::string::String>,
    pub(crate) checksum: std::option::Option<std::string::String>,
    pub(crate) archive_id: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl CompleteMultipartUploadOutputBuilder {
    /// <p>The relative URI path of the newly added archive resource.</p>
    pub fn location(mut self, input: impl Into<std::string::String>) -> Self {
        self.location = Some(input.into());
        self
    }
    /// <p>The relative URI path of the newly added archive resource.</p>
    pub fn set_location(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.location = input;
        self
    }
    /// <p>The checksum of the archive computed by Amazon S3 Glacier.</p>
    pub fn checksum(mut self, input: impl Into<std::string::String>) -> Self {
        self.checksum = Some(input.into());
        self
    }
    /// <p>The checksum of the archive computed by Amazon S3 Glacier.</p>
    pub fn set_checksum(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.checksum = input;
        self
    }
    /// <p>The ID of the archive. This value is also included as part of the location.</p>
    pub fn archive_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.archive_id = Some(input.into());
        self
    }
    /// <p>The ID of the archive. This value is also included as part of the location.</p>
    pub fn set_archive_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.archive_id = input;
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
    /// Consumes the builder and constructs a [`CompleteMultipartUploadOutput`](crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput).
    pub fn build(
        self,
    ) -> crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput {
        crate::operation::complete_multipart_upload::CompleteMultipartUploadOutput {
            location: self.location,
            checksum: self.checksum,
            archive_id: self.archive_id,
            _request_id: self._request_id,
        }
    }
}