// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Details of the parts that were uploaded.</p>
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
pub struct CompletedPart {
    /// <p>Entity tag returned when the part was uploaded.</p>
    #[doc(hidden)]
    pub e_tag: std::option::Option<std::string::String>,
    /// <p>The base64-encoded, 32-bit CRC32 checksum of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    #[doc(hidden)]
    pub checksum_crc32: std::option::Option<std::string::String>,
    /// <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    #[doc(hidden)]
    pub checksum_crc32_c: std::option::Option<std::string::String>,
    /// <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    #[doc(hidden)]
    pub checksum_sha1: std::option::Option<std::string::String>,
    /// <p>The base64-encoded, 256-bit SHA-256 digest of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    #[doc(hidden)]
    pub checksum_sha256: std::option::Option<std::string::String>,
    /// <p>Part number that identifies the part. This is a positive integer between 1 and 10,000.</p>
    #[doc(hidden)]
    pub part_number: i32,
}
impl CompletedPart {
    /// <p>Entity tag returned when the part was uploaded.</p>
    pub fn e_tag(&self) -> std::option::Option<&str> {
        self.e_tag.as_deref()
    }
    /// <p>The base64-encoded, 32-bit CRC32 checksum of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_crc32(&self) -> std::option::Option<&str> {
        self.checksum_crc32.as_deref()
    }
    /// <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_crc32_c(&self) -> std::option::Option<&str> {
        self.checksum_crc32_c.as_deref()
    }
    /// <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_sha1(&self) -> std::option::Option<&str> {
        self.checksum_sha1.as_deref()
    }
    /// <p>The base64-encoded, 256-bit SHA-256 digest of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_sha256(&self) -> std::option::Option<&str> {
        self.checksum_sha256.as_deref()
    }
    /// <p>Part number that identifies the part. This is a positive integer between 1 and 10,000.</p>
    pub fn part_number(&self) -> i32 {
        self.part_number
    }
}
impl CompletedPart {
    /// Creates a new builder-style object to manufacture [`CompletedPart`](crate::types::CompletedPart).
    pub fn builder() -> crate::types::builders::CompletedPartBuilder {
        crate::types::builders::CompletedPartBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::CompletedPart;
/// A builder for [`CompletedPart`](crate::types::CompletedPart).
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
pub struct CompletedPartBuilder {
    pub(crate) e_tag: std::option::Option<std::string::String>,
    pub(crate) checksum_crc32: std::option::Option<std::string::String>,
    pub(crate) checksum_crc32_c: std::option::Option<std::string::String>,
    pub(crate) checksum_sha1: std::option::Option<std::string::String>,
    pub(crate) checksum_sha256: std::option::Option<std::string::String>,
    pub(crate) part_number: std::option::Option<i32>,
}
impl CompletedPartBuilder {
    /// <p>Entity tag returned when the part was uploaded.</p>
    pub fn e_tag(mut self, input: impl Into<std::string::String>) -> Self {
        self.e_tag = Some(input.into());
        self
    }
    /// <p>Entity tag returned when the part was uploaded.</p>
    pub fn set_e_tag(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.e_tag = input;
        self
    }
    /// <p>The base64-encoded, 32-bit CRC32 checksum of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_crc32(mut self, input: impl Into<std::string::String>) -> Self {
        self.checksum_crc32 = Some(input.into());
        self
    }
    /// <p>The base64-encoded, 32-bit CRC32 checksum of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_checksum_crc32(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.checksum_crc32 = input;
        self
    }
    /// <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_crc32_c(mut self, input: impl Into<std::string::String>) -> Self {
        self.checksum_crc32_c = Some(input.into());
        self
    }
    /// <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_checksum_crc32_c(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.checksum_crc32_c = input;
        self
    }
    /// <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_sha1(mut self, input: impl Into<std::string::String>) -> Self {
        self.checksum_sha1 = Some(input.into());
        self
    }
    /// <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_checksum_sha1(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.checksum_sha1 = input;
        self
    }
    /// <p>The base64-encoded, 256-bit SHA-256 digest of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_sha256(mut self, input: impl Into<std::string::String>) -> Self {
        self.checksum_sha256 = Some(input.into());
        self
    }
    /// <p>The base64-encoded, 256-bit SHA-256 digest of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_checksum_sha256(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.checksum_sha256 = input;
        self
    }
    /// <p>Part number that identifies the part. This is a positive integer between 1 and 10,000.</p>
    pub fn part_number(mut self, input: i32) -> Self {
        self.part_number = Some(input);
        self
    }
    /// <p>Part number that identifies the part. This is a positive integer between 1 and 10,000.</p>
    pub fn set_part_number(mut self, input: std::option::Option<i32>) -> Self {
        self.part_number = input;
        self
    }
    /// Consumes the builder and constructs a [`CompletedPart`](crate::types::CompletedPart).
    pub fn build(self) -> crate::types::CompletedPart {
        crate::types::CompletedPart {
            e_tag: self.e_tag,
            checksum_crc32: self.checksum_crc32,
            checksum_crc32_c: self.checksum_crc32_c,
            checksum_sha1: self.checksum_sha1,
            checksum_sha256: self.checksum_sha256,
            part_number: self.part_number.unwrap_or_default(),
        }
    }
}