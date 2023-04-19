// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the storage parameters for Amazon S3 and Amazon S3 buckets for an instance store-backed AMI.</p>
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
pub struct S3Storage {
    /// <p>The access key ID of the owner of the bucket. Before you specify a value for your access key ID, review and follow the guidance in <a href="https://docs.aws.amazon.com/accounts/latest/reference/best-practices.html">Best Practices for Amazon Web Services accounts</a> in the <i>Account ManagementReference Guide</i>.</p>
    #[doc(hidden)]
    pub aws_access_key_id: std::option::Option<std::string::String>,
    /// <p>The bucket in which to store the AMI. You can specify a bucket that you already own or a new bucket that Amazon EC2 creates on your behalf. If you specify a bucket that belongs to someone else, Amazon EC2 returns an error.</p>
    #[doc(hidden)]
    pub bucket: std::option::Option<std::string::String>,
    /// <p>The beginning of the file name of the AMI.</p>
    #[doc(hidden)]
    pub prefix: std::option::Option<std::string::String>,
    /// <p>An Amazon S3 upload policy that gives Amazon EC2 permission to upload items into Amazon S3 on your behalf.</p>
    #[doc(hidden)]
    pub upload_policy: std::option::Option<aws_smithy_types::Blob>,
    /// <p>The signature of the JSON document.</p>
    #[doc(hidden)]
    pub upload_policy_signature: std::option::Option<std::string::String>,
}
impl S3Storage {
    /// <p>The access key ID of the owner of the bucket. Before you specify a value for your access key ID, review and follow the guidance in <a href="https://docs.aws.amazon.com/accounts/latest/reference/best-practices.html">Best Practices for Amazon Web Services accounts</a> in the <i>Account ManagementReference Guide</i>.</p>
    pub fn aws_access_key_id(&self) -> std::option::Option<&str> {
        self.aws_access_key_id.as_deref()
    }
    /// <p>The bucket in which to store the AMI. You can specify a bucket that you already own or a new bucket that Amazon EC2 creates on your behalf. If you specify a bucket that belongs to someone else, Amazon EC2 returns an error.</p>
    pub fn bucket(&self) -> std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The beginning of the file name of the AMI.</p>
    pub fn prefix(&self) -> std::option::Option<&str> {
        self.prefix.as_deref()
    }
    /// <p>An Amazon S3 upload policy that gives Amazon EC2 permission to upload items into Amazon S3 on your behalf.</p>
    pub fn upload_policy(&self) -> std::option::Option<&aws_smithy_types::Blob> {
        self.upload_policy.as_ref()
    }
    /// <p>The signature of the JSON document.</p>
    pub fn upload_policy_signature(&self) -> std::option::Option<&str> {
        self.upload_policy_signature.as_deref()
    }
}
impl S3Storage {
    /// Creates a new builder-style object to manufacture [`S3Storage`](crate::types::S3Storage).
    pub fn builder() -> crate::types::builders::S3StorageBuilder {
        crate::types::builders::S3StorageBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::S3Storage;
/// A builder for [`S3Storage`](crate::types::S3Storage).
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
pub struct S3StorageBuilder {
    pub(crate) aws_access_key_id: std::option::Option<std::string::String>,
    pub(crate) bucket: std::option::Option<std::string::String>,
    pub(crate) prefix: std::option::Option<std::string::String>,
    pub(crate) upload_policy: std::option::Option<aws_smithy_types::Blob>,
    pub(crate) upload_policy_signature: std::option::Option<std::string::String>,
}
impl S3StorageBuilder {
    /// <p>The access key ID of the owner of the bucket. Before you specify a value for your access key ID, review and follow the guidance in <a href="https://docs.aws.amazon.com/accounts/latest/reference/best-practices.html">Best Practices for Amazon Web Services accounts</a> in the <i>Account ManagementReference Guide</i>.</p>
    pub fn aws_access_key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.aws_access_key_id = Some(input.into());
        self
    }
    /// <p>The access key ID of the owner of the bucket. Before you specify a value for your access key ID, review and follow the guidance in <a href="https://docs.aws.amazon.com/accounts/latest/reference/best-practices.html">Best Practices for Amazon Web Services accounts</a> in the <i>Account ManagementReference Guide</i>.</p>
    pub fn set_aws_access_key_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.aws_access_key_id = input;
        self
    }
    /// <p>The bucket in which to store the AMI. You can specify a bucket that you already own or a new bucket that Amazon EC2 creates on your behalf. If you specify a bucket that belongs to someone else, Amazon EC2 returns an error.</p>
    pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.bucket = Some(input.into());
        self
    }
    /// <p>The bucket in which to store the AMI. You can specify a bucket that you already own or a new bucket that Amazon EC2 creates on your behalf. If you specify a bucket that belongs to someone else, Amazon EC2 returns an error.</p>
    pub fn set_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The beginning of the file name of the AMI.</p>
    pub fn prefix(mut self, input: impl Into<std::string::String>) -> Self {
        self.prefix = Some(input.into());
        self
    }
    /// <p>The beginning of the file name of the AMI.</p>
    pub fn set_prefix(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.prefix = input;
        self
    }
    /// <p>An Amazon S3 upload policy that gives Amazon EC2 permission to upload items into Amazon S3 on your behalf.</p>
    pub fn upload_policy(mut self, input: aws_smithy_types::Blob) -> Self {
        self.upload_policy = Some(input);
        self
    }
    /// <p>An Amazon S3 upload policy that gives Amazon EC2 permission to upload items into Amazon S3 on your behalf.</p>
    pub fn set_upload_policy(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
        self.upload_policy = input;
        self
    }
    /// <p>The signature of the JSON document.</p>
    pub fn upload_policy_signature(mut self, input: impl Into<std::string::String>) -> Self {
        self.upload_policy_signature = Some(input.into());
        self
    }
    /// <p>The signature of the JSON document.</p>
    pub fn set_upload_policy_signature(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.upload_policy_signature = input;
        self
    }
    /// Consumes the builder and constructs a [`S3Storage`](crate::types::S3Storage).
    pub fn build(self) -> crate::types::S3Storage {
        crate::types::S3Storage {
            aws_access_key_id: self.aws_access_key_id,
            bucket: self.bucket,
            prefix: self.prefix,
            upload_policy: self.upload_policy,
            upload_policy_signature: self.upload_policy_signature,
        }
    }
}
