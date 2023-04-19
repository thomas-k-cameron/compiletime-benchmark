// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A container for the bucket where the Amazon S3 Storage Lens metrics export files are located.</p>
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
pub struct S3BucketDestination {
    /// <p></p>
    #[doc(hidden)]
    pub format: std::option::Option<crate::types::Format>,
    /// <p>The schema version of the export file.</p>
    #[doc(hidden)]
    pub output_schema_version: std::option::Option<crate::types::OutputSchemaVersion>,
    /// <p>The account ID of the owner of the S3 Storage Lens metrics export bucket.</p>
    #[doc(hidden)]
    pub account_id: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the bucket. This property is read-only and follows the following format: <code> arn:aws:s3:<i>us-east-1</i>:<i>example-account-id</i>:bucket/<i>your-destination-bucket-name</i> </code> </p>
    #[doc(hidden)]
    pub arn: std::option::Option<std::string::String>,
    /// <p>The prefix of the destination bucket where the metrics export will be delivered.</p>
    #[doc(hidden)]
    pub prefix: std::option::Option<std::string::String>,
    /// <p>The container for the type encryption of the metrics exports in this bucket.</p>
    #[doc(hidden)]
    pub encryption: std::option::Option<crate::types::StorageLensDataExportEncryption>,
}
impl S3BucketDestination {
    /// <p></p>
    pub fn format(&self) -> std::option::Option<&crate::types::Format> {
        self.format.as_ref()
    }
    /// <p>The schema version of the export file.</p>
    pub fn output_schema_version(&self) -> std::option::Option<&crate::types::OutputSchemaVersion> {
        self.output_schema_version.as_ref()
    }
    /// <p>The account ID of the owner of the S3 Storage Lens metrics export bucket.</p>
    pub fn account_id(&self) -> std::option::Option<&str> {
        self.account_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the bucket. This property is read-only and follows the following format: <code> arn:aws:s3:<i>us-east-1</i>:<i>example-account-id</i>:bucket/<i>your-destination-bucket-name</i> </code> </p>
    pub fn arn(&self) -> std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The prefix of the destination bucket where the metrics export will be delivered.</p>
    pub fn prefix(&self) -> std::option::Option<&str> {
        self.prefix.as_deref()
    }
    /// <p>The container for the type encryption of the metrics exports in this bucket.</p>
    pub fn encryption(
        &self,
    ) -> std::option::Option<&crate::types::StorageLensDataExportEncryption> {
        self.encryption.as_ref()
    }
}
impl S3BucketDestination {
    /// Creates a new builder-style object to manufacture [`S3BucketDestination`](crate::types::S3BucketDestination).
    pub fn builder() -> crate::types::builders::S3BucketDestinationBuilder {
        crate::types::builders::S3BucketDestinationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::S3BucketDestination;
/// A builder for [`S3BucketDestination`](crate::types::S3BucketDestination).
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
pub struct S3BucketDestinationBuilder {
    pub(crate) format: std::option::Option<crate::types::Format>,
    pub(crate) output_schema_version: std::option::Option<crate::types::OutputSchemaVersion>,
    pub(crate) account_id: std::option::Option<std::string::String>,
    pub(crate) arn: std::option::Option<std::string::String>,
    pub(crate) prefix: std::option::Option<std::string::String>,
    pub(crate) encryption: std::option::Option<crate::types::StorageLensDataExportEncryption>,
}
impl S3BucketDestinationBuilder {
    /// <p></p>
    pub fn format(mut self, input: crate::types::Format) -> Self {
        self.format = Some(input);
        self
    }
    /// <p></p>
    pub fn set_format(mut self, input: std::option::Option<crate::types::Format>) -> Self {
        self.format = input;
        self
    }
    /// <p>The schema version of the export file.</p>
    pub fn output_schema_version(mut self, input: crate::types::OutputSchemaVersion) -> Self {
        self.output_schema_version = Some(input);
        self
    }
    /// <p>The schema version of the export file.</p>
    pub fn set_output_schema_version(
        mut self,
        input: std::option::Option<crate::types::OutputSchemaVersion>,
    ) -> Self {
        self.output_schema_version = input;
        self
    }
    /// <p>The account ID of the owner of the S3 Storage Lens metrics export bucket.</p>
    pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.account_id = Some(input.into());
        self
    }
    /// <p>The account ID of the owner of the S3 Storage Lens metrics export bucket.</p>
    pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.account_id = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the bucket. This property is read-only and follows the following format: <code> arn:aws:s3:<i>us-east-1</i>:<i>example-account-id</i>:bucket/<i>your-destination-bucket-name</i> </code> </p>
    pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the bucket. This property is read-only and follows the following format: <code> arn:aws:s3:<i>us-east-1</i>:<i>example-account-id</i>:bucket/<i>your-destination-bucket-name</i> </code> </p>
    pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The prefix of the destination bucket where the metrics export will be delivered.</p>
    pub fn prefix(mut self, input: impl Into<std::string::String>) -> Self {
        self.prefix = Some(input.into());
        self
    }
    /// <p>The prefix of the destination bucket where the metrics export will be delivered.</p>
    pub fn set_prefix(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.prefix = input;
        self
    }
    /// <p>The container for the type encryption of the metrics exports in this bucket.</p>
    pub fn encryption(mut self, input: crate::types::StorageLensDataExportEncryption) -> Self {
        self.encryption = Some(input);
        self
    }
    /// <p>The container for the type encryption of the metrics exports in this bucket.</p>
    pub fn set_encryption(
        mut self,
        input: std::option::Option<crate::types::StorageLensDataExportEncryption>,
    ) -> Self {
        self.encryption = input;
        self
    }
    /// Consumes the builder and constructs a [`S3BucketDestination`](crate::types::S3BucketDestination).
    pub fn build(self) -> crate::types::S3BucketDestination {
        crate::types::S3BucketDestination {
            format: self.format,
            output_schema_version: self.output_schema_version,
            account_id: self.account_id,
            arn: self.arn,
            prefix: self.prefix,
            encryption: self.encryption,
        }
    }
}
