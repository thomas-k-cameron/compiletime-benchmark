// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides options to upload a part of an archive in a multipart upload operation.</p>
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct UploadMultipartPartInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    #[doc(hidden)]
    pub account_id: std::option::Option<std::string::String>,
    /// <p>The name of the vault.</p>
    #[doc(hidden)]
    pub vault_name: std::option::Option<std::string::String>,
    /// <p>The upload ID of the multipart upload.</p>
    #[doc(hidden)]
    pub upload_id: std::option::Option<std::string::String>,
    /// <p>The SHA256 tree hash of the data being uploaded.</p>
    #[doc(hidden)]
    pub checksum: std::option::Option<std::string::String>,
    /// <p>Identifies the range of bytes in the assembled archive that will be uploaded in this part. Amazon S3 Glacier uses this information to assemble the archive in the proper sequence. The format of this header follows RFC 2616. An example header is Content-Range:bytes 0-4194303/*.</p>
    #[doc(hidden)]
    pub range: std::option::Option<std::string::String>,
    /// <p>The data to upload.</p>
    pub body: aws_smithy_http::byte_stream::ByteStream,
}
impl UploadMultipartPartInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    pub fn account_id(&self) -> std::option::Option<&str> {
        self.account_id.as_deref()
    }
    /// <p>The name of the vault.</p>
    pub fn vault_name(&self) -> std::option::Option<&str> {
        self.vault_name.as_deref()
    }
    /// <p>The upload ID of the multipart upload.</p>
    pub fn upload_id(&self) -> std::option::Option<&str> {
        self.upload_id.as_deref()
    }
    /// <p>The SHA256 tree hash of the data being uploaded.</p>
    pub fn checksum(&self) -> std::option::Option<&str> {
        self.checksum.as_deref()
    }
    /// <p>Identifies the range of bytes in the assembled archive that will be uploaded in this part. Amazon S3 Glacier uses this information to assemble the archive in the proper sequence. The format of this header follows RFC 2616. An example header is Content-Range:bytes 0-4194303/*.</p>
    pub fn range(&self) -> std::option::Option<&str> {
        self.range.as_deref()
    }
    /// <p>The data to upload.</p>
    pub fn body(&self) -> &aws_smithy_http::byte_stream::ByteStream {
        &self.body
    }
}
impl UploadMultipartPartInput {
    /// Creates a new builder-style object to manufacture [`UploadMultipartPartInput`](crate::operation::upload_multipart_part::UploadMultipartPartInput).
    pub fn builder(
    ) -> crate::operation::upload_multipart_part::builders::UploadMultipartPartInputBuilder {
        crate::operation::upload_multipart_part::builders::UploadMultipartPartInputBuilder::default(
        )
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::upload_multipart_part::UploadMultipartPartInput;
/// A builder for [`UploadMultipartPartInput`](crate::operation::upload_multipart_part::UploadMultipartPartInput).
#[non_exhaustive]
#[derive(std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct UploadMultipartPartInputBuilder {
    pub(crate) account_id: std::option::Option<std::string::String>,
    pub(crate) vault_name: std::option::Option<std::string::String>,
    pub(crate) upload_id: std::option::Option<std::string::String>,
    pub(crate) checksum: std::option::Option<std::string::String>,
    pub(crate) range: std::option::Option<std::string::String>,
    pub(crate) body: std::option::Option<aws_smithy_http::byte_stream::ByteStream>,
}
impl UploadMultipartPartInputBuilder {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.account_id = Some(input.into());
        self
    }
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.account_id = input;
        self
    }
    /// <p>The name of the vault.</p>
    pub fn vault_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.vault_name = Some(input.into());
        self
    }
    /// <p>The name of the vault.</p>
    pub fn set_vault_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.vault_name = input;
        self
    }
    /// <p>The upload ID of the multipart upload.</p>
    pub fn upload_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.upload_id = Some(input.into());
        self
    }
    /// <p>The upload ID of the multipart upload.</p>
    pub fn set_upload_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.upload_id = input;
        self
    }
    /// <p>The SHA256 tree hash of the data being uploaded.</p>
    pub fn checksum(mut self, input: impl Into<std::string::String>) -> Self {
        self.checksum = Some(input.into());
        self
    }
    /// <p>The SHA256 tree hash of the data being uploaded.</p>
    pub fn set_checksum(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.checksum = input;
        self
    }
    /// <p>Identifies the range of bytes in the assembled archive that will be uploaded in this part. Amazon S3 Glacier uses this information to assemble the archive in the proper sequence. The format of this header follows RFC 2616. An example header is Content-Range:bytes 0-4194303/*.</p>
    pub fn range(mut self, input: impl Into<std::string::String>) -> Self {
        self.range = Some(input.into());
        self
    }
    /// <p>Identifies the range of bytes in the assembled archive that will be uploaded in this part. Amazon S3 Glacier uses this information to assemble the archive in the proper sequence. The format of this header follows RFC 2616. An example header is Content-Range:bytes 0-4194303/*.</p>
    pub fn set_range(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.range = input;
        self
    }
    /// <p>The data to upload.</p>
    pub fn body(mut self, input: aws_smithy_http::byte_stream::ByteStream) -> Self {
        self.body = Some(input);
        self
    }
    /// <p>The data to upload.</p>
    pub fn set_body(
        mut self,
        input: std::option::Option<aws_smithy_http::byte_stream::ByteStream>,
    ) -> Self {
        self.body = input;
        self
    }
    /// Consumes the builder and constructs a [`UploadMultipartPartInput`](crate::operation::upload_multipart_part::UploadMultipartPartInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::upload_multipart_part::UploadMultipartPartInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::upload_multipart_part::UploadMultipartPartInput {
                account_id: self.account_id,
                vault_name: self.vault_name,
                upload_id: self.upload_id,
                checksum: self.checksum,
                range: self.range,
                body: self.body.unwrap_or_default(),
            },
        )
    }
}