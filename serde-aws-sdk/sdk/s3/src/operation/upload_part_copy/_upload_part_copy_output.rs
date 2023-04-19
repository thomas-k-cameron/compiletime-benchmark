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
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UploadPartCopyOutput {
    /// <p>The version of the source object that was copied, if you have enabled versioning on the source bucket.</p>
    #[doc(hidden)]
    pub copy_source_version_id: std::option::Option<std::string::String>,
    /// <p>Container for all response elements.</p>
    #[doc(hidden)]
    pub copy_part_result: std::option::Option<crate::types::CopyPartResult>,
    /// <p>The server-side encryption algorithm used when storing this object in Amazon S3 (for example, AES256, aws:kms).</p>
    #[doc(hidden)]
    pub server_side_encryption: std::option::Option<crate::types::ServerSideEncryption>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    #[doc(hidden)]
    pub sse_customer_algorithm: std::option::Option<std::string::String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round-trip message integrity verification of the customer-provided encryption key.</p>
    #[doc(hidden)]
    pub sse_customer_key_md5: std::option::Option<std::string::String>,
    /// <p>If present, specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric customer managed key that was used for the object.</p>
    #[doc(hidden)]
    pub ssekms_key_id: std::option::Option<std::string::String>,
    /// <p>Indicates whether the multipart upload uses an S3 Bucket Key for server-side encryption with Amazon Web Services KMS (SSE-KMS).</p>
    #[doc(hidden)]
    pub bucket_key_enabled: bool,
    /// <p>If present, indicates that the requester was successfully charged for the request.</p>
    #[doc(hidden)]
    pub request_charged: std::option::Option<crate::types::RequestCharged>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl UploadPartCopyOutput {
    /// <p>The version of the source object that was copied, if you have enabled versioning on the source bucket.</p>
    pub fn copy_source_version_id(&self) -> std::option::Option<&str> {
        self.copy_source_version_id.as_deref()
    }
    /// <p>Container for all response elements.</p>
    pub fn copy_part_result(&self) -> std::option::Option<&crate::types::CopyPartResult> {
        self.copy_part_result.as_ref()
    }
    /// <p>The server-side encryption algorithm used when storing this object in Amazon S3 (for example, AES256, aws:kms).</p>
    pub fn server_side_encryption(
        &self,
    ) -> std::option::Option<&crate::types::ServerSideEncryption> {
        self.server_side_encryption.as_ref()
    }
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    pub fn sse_customer_algorithm(&self) -> std::option::Option<&str> {
        self.sse_customer_algorithm.as_deref()
    }
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round-trip message integrity verification of the customer-provided encryption key.</p>
    pub fn sse_customer_key_md5(&self) -> std::option::Option<&str> {
        self.sse_customer_key_md5.as_deref()
    }
    /// <p>If present, specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric customer managed key that was used for the object.</p>
    pub fn ssekms_key_id(&self) -> std::option::Option<&str> {
        self.ssekms_key_id.as_deref()
    }
    /// <p>Indicates whether the multipart upload uses an S3 Bucket Key for server-side encryption with Amazon Web Services KMS (SSE-KMS).</p>
    pub fn bucket_key_enabled(&self) -> bool {
        self.bucket_key_enabled
    }
    /// <p>If present, indicates that the requester was successfully charged for the request.</p>
    pub fn request_charged(&self) -> std::option::Option<&crate::types::RequestCharged> {
        self.request_charged.as_ref()
    }
}
impl std::fmt::Debug for UploadPartCopyOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UploadPartCopyOutput");
        formatter.field("copy_source_version_id", &self.copy_source_version_id);
        formatter.field("copy_part_result", &self.copy_part_result);
        formatter.field("server_side_encryption", &self.server_side_encryption);
        formatter.field("sse_customer_algorithm", &self.sse_customer_algorithm);
        formatter.field("sse_customer_key_md5", &self.sse_customer_key_md5);
        formatter.field("ssekms_key_id", &"*** Sensitive Data Redacted ***");
        formatter.field("bucket_key_enabled", &self.bucket_key_enabled);
        formatter.field("request_charged", &self.request_charged);
        formatter.field("_extended_request_id", &self._extended_request_id);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
impl crate::s3_request_id::RequestIdExt for UploadPartCopyOutput {
    fn extended_request_id(&self) -> Option<&str> {
        self._extended_request_id.as_deref()
    }
}
impl aws_http::request_id::RequestId for UploadPartCopyOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UploadPartCopyOutput {
    /// Creates a new builder-style object to manufacture [`UploadPartCopyOutput`](crate::operation::upload_part_copy::UploadPartCopyOutput).
    pub fn builder() -> crate::operation::upload_part_copy::builders::UploadPartCopyOutputBuilder {
        crate::operation::upload_part_copy::builders::UploadPartCopyOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::upload_part_copy::UploadPartCopyOutput;
/// A builder for [`UploadPartCopyOutput`](crate::operation::upload_part_copy::UploadPartCopyOutput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct UploadPartCopyOutputBuilder {
    pub(crate) copy_source_version_id: std::option::Option<std::string::String>,
    pub(crate) copy_part_result: std::option::Option<crate::types::CopyPartResult>,
    pub(crate) server_side_encryption: std::option::Option<crate::types::ServerSideEncryption>,
    pub(crate) sse_customer_algorithm: std::option::Option<std::string::String>,
    pub(crate) sse_customer_key_md5: std::option::Option<std::string::String>,
    pub(crate) ssekms_key_id: std::option::Option<std::string::String>,
    pub(crate) bucket_key_enabled: std::option::Option<bool>,
    pub(crate) request_charged: std::option::Option<crate::types::RequestCharged>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl UploadPartCopyOutputBuilder {
    /// <p>The version of the source object that was copied, if you have enabled versioning on the source bucket.</p>
    pub fn copy_source_version_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.copy_source_version_id = Some(input.into());
        self
    }
    /// <p>The version of the source object that was copied, if you have enabled versioning on the source bucket.</p>
    pub fn set_copy_source_version_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.copy_source_version_id = input;
        self
    }
    /// <p>Container for all response elements.</p>
    pub fn copy_part_result(mut self, input: crate::types::CopyPartResult) -> Self {
        self.copy_part_result = Some(input);
        self
    }
    /// <p>Container for all response elements.</p>
    pub fn set_copy_part_result(
        mut self,
        input: std::option::Option<crate::types::CopyPartResult>,
    ) -> Self {
        self.copy_part_result = input;
        self
    }
    /// <p>The server-side encryption algorithm used when storing this object in Amazon S3 (for example, AES256, aws:kms).</p>
    pub fn server_side_encryption(mut self, input: crate::types::ServerSideEncryption) -> Self {
        self.server_side_encryption = Some(input);
        self
    }
    /// <p>The server-side encryption algorithm used when storing this object in Amazon S3 (for example, AES256, aws:kms).</p>
    pub fn set_server_side_encryption(
        mut self,
        input: std::option::Option<crate::types::ServerSideEncryption>,
    ) -> Self {
        self.server_side_encryption = input;
        self
    }
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    pub fn sse_customer_algorithm(mut self, input: impl Into<std::string::String>) -> Self {
        self.sse_customer_algorithm = Some(input.into());
        self
    }
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    pub fn set_sse_customer_algorithm(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.sse_customer_algorithm = input;
        self
    }
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round-trip message integrity verification of the customer-provided encryption key.</p>
    pub fn sse_customer_key_md5(mut self, input: impl Into<std::string::String>) -> Self {
        self.sse_customer_key_md5 = Some(input.into());
        self
    }
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round-trip message integrity verification of the customer-provided encryption key.</p>
    pub fn set_sse_customer_key_md5(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.sse_customer_key_md5 = input;
        self
    }
    /// <p>If present, specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric customer managed key that was used for the object.</p>
    pub fn ssekms_key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.ssekms_key_id = Some(input.into());
        self
    }
    /// <p>If present, specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric customer managed key that was used for the object.</p>
    pub fn set_ssekms_key_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.ssekms_key_id = input;
        self
    }
    /// <p>Indicates whether the multipart upload uses an S3 Bucket Key for server-side encryption with Amazon Web Services KMS (SSE-KMS).</p>
    pub fn bucket_key_enabled(mut self, input: bool) -> Self {
        self.bucket_key_enabled = Some(input);
        self
    }
    /// <p>Indicates whether the multipart upload uses an S3 Bucket Key for server-side encryption with Amazon Web Services KMS (SSE-KMS).</p>
    pub fn set_bucket_key_enabled(mut self, input: std::option::Option<bool>) -> Self {
        self.bucket_key_enabled = input;
        self
    }
    /// <p>If present, indicates that the requester was successfully charged for the request.</p>
    pub fn request_charged(mut self, input: crate::types::RequestCharged) -> Self {
        self.request_charged = Some(input);
        self
    }
    /// <p>If present, indicates that the requester was successfully charged for the request.</p>
    pub fn set_request_charged(
        mut self,
        input: std::option::Option<crate::types::RequestCharged>,
    ) -> Self {
        self.request_charged = input;
        self
    }
    pub(crate) fn _extended_request_id(mut self, extended_request_id: impl Into<String>) -> Self {
        self._extended_request_id = Some(extended_request_id.into());
        self
    }

    pub(crate) fn _set_extended_request_id(
        &mut self,
        extended_request_id: Option<String>,
    ) -> &mut Self {
        self._extended_request_id = extended_request_id;
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
    /// Consumes the builder and constructs a [`UploadPartCopyOutput`](crate::operation::upload_part_copy::UploadPartCopyOutput).
    pub fn build(self) -> crate::operation::upload_part_copy::UploadPartCopyOutput {
        crate::operation::upload_part_copy::UploadPartCopyOutput {
            copy_source_version_id: self.copy_source_version_id,
            copy_part_result: self.copy_part_result,
            server_side_encryption: self.server_side_encryption,
            sse_customer_algorithm: self.sse_customer_algorithm,
            sse_customer_key_md5: self.sse_customer_key_md5,
            ssekms_key_id: self.ssekms_key_id,
            bucket_key_enabled: self.bucket_key_enabled.unwrap_or_default(),
            request_charged: self.request_charged,
            _extended_request_id: self._extended_request_id,
            _request_id: self._request_id,
        }
    }
}
impl std::fmt::Debug for UploadPartCopyOutputBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UploadPartCopyOutputBuilder");
        formatter.field("copy_source_version_id", &self.copy_source_version_id);
        formatter.field("copy_part_result", &self.copy_part_result);
        formatter.field("server_side_encryption", &self.server_side_encryption);
        formatter.field("sse_customer_algorithm", &self.sse_customer_algorithm);
        formatter.field("sse_customer_key_md5", &self.sse_customer_key_md5);
        formatter.field("ssekms_key_id", &"*** Sensitive Data Redacted ***");
        formatter.field("bucket_key_enabled", &self.bucket_key_enabled);
        formatter.field("request_charged", &self.request_charged);
        formatter.field("_extended_request_id", &self._extended_request_id);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}