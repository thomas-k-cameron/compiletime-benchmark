// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> In terms of implementation, a Bucket is a resource. An Amazon S3 bucket name is globally unique, and the namespace is shared by all Amazon Web Services accounts. </p>
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
pub struct Bucket {
    /// <p>The name of the bucket.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// <p>Date the bucket was created. This date can change when making changes to your bucket, such as editing its bucket policy.</p>
    #[doc(hidden)]
    pub creation_date: std::option::Option<aws_smithy_types::DateTime>,
}
impl Bucket {
    /// <p>The name of the bucket.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>Date the bucket was created. This date can change when making changes to your bucket, such as editing its bucket policy.</p>
    pub fn creation_date(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.creation_date.as_ref()
    }
}
impl Bucket {
    /// Creates a new builder-style object to manufacture [`Bucket`](crate::types::Bucket).
    pub fn builder() -> crate::types::builders::BucketBuilder {
        crate::types::builders::BucketBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::Bucket;
/// A builder for [`Bucket`](crate::types::Bucket).
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
pub struct BucketBuilder {
    pub(crate) name: std::option::Option<std::string::String>,
    pub(crate) creation_date: std::option::Option<aws_smithy_types::DateTime>,
}
impl BucketBuilder {
    /// <p>The name of the bucket.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.name = Some(input.into());
        self
    }
    /// <p>The name of the bucket.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>Date the bucket was created. This date can change when making changes to your bucket, such as editing its bucket policy.</p>
    pub fn creation_date(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.creation_date = Some(input);
        self
    }
    /// <p>Date the bucket was created. This date can change when making changes to your bucket, such as editing its bucket policy.</p>
    pub fn set_creation_date(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.creation_date = input;
        self
    }
    /// Consumes the builder and constructs a [`Bucket`](crate::types::Bucket).
    pub fn build(self) -> crate::types::Bucket {
        crate::types::Bucket {
            name: self.name,
            creation_date: self.creation_date,
        }
    }
}
