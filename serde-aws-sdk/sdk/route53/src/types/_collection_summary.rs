// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A complex type that is an entry in an <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_CidrCollection.html">CidrCollection</a> array.</p>
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
pub struct CollectionSummary {
    /// <p>The ARN of the collection summary. Can be used to reference the collection in IAM policy or cross-account.</p>
    #[doc(hidden)]
    pub arn: std::option::Option<std::string::String>,
    /// <p>Unique ID for the CIDR collection.</p>
    #[doc(hidden)]
    pub id: std::option::Option<std::string::String>,
    /// <p>The name of a CIDR collection.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// <p>A sequential counter that Route&nbsp;53 sets to 1 when you create a CIDR collection and increments by 1 each time you update settings for the CIDR collection.</p>
    #[doc(hidden)]
    pub version: std::option::Option<i64>,
}
impl CollectionSummary {
    /// <p>The ARN of the collection summary. Can be used to reference the collection in IAM policy or cross-account.</p>
    pub fn arn(&self) -> std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>Unique ID for the CIDR collection.</p>
    pub fn id(&self) -> std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The name of a CIDR collection.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>A sequential counter that Route&nbsp;53 sets to 1 when you create a CIDR collection and increments by 1 each time you update settings for the CIDR collection.</p>
    pub fn version(&self) -> std::option::Option<i64> {
        self.version
    }
}
impl CollectionSummary {
    /// Creates a new builder-style object to manufacture [`CollectionSummary`](crate::types::CollectionSummary).
    pub fn builder() -> crate::types::builders::CollectionSummaryBuilder {
        crate::types::builders::CollectionSummaryBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::CollectionSummary;
/// A builder for [`CollectionSummary`](crate::types::CollectionSummary).
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
pub struct CollectionSummaryBuilder {
    pub(crate) arn: std::option::Option<std::string::String>,
    pub(crate) id: std::option::Option<std::string::String>,
    pub(crate) name: std::option::Option<std::string::String>,
    pub(crate) version: std::option::Option<i64>,
}
impl CollectionSummaryBuilder {
    /// <p>The ARN of the collection summary. Can be used to reference the collection in IAM policy or cross-account.</p>
    pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.arn = Some(input.into());
        self
    }
    /// <p>The ARN of the collection summary. Can be used to reference the collection in IAM policy or cross-account.</p>
    pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>Unique ID for the CIDR collection.</p>
    pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
        self.id = Some(input.into());
        self
    }
    /// <p>Unique ID for the CIDR collection.</p>
    pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The name of a CIDR collection.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.name = Some(input.into());
        self
    }
    /// <p>The name of a CIDR collection.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>A sequential counter that Route&nbsp;53 sets to 1 when you create a CIDR collection and increments by 1 each time you update settings for the CIDR collection.</p>
    pub fn version(mut self, input: i64) -> Self {
        self.version = Some(input);
        self
    }
    /// <p>A sequential counter that Route&nbsp;53 sets to 1 when you create a CIDR collection and increments by 1 each time you update settings for the CIDR collection.</p>
    pub fn set_version(mut self, input: std::option::Option<i64>) -> Self {
        self.version = input;
        self
    }
    /// Consumes the builder and constructs a [`CollectionSummary`](crate::types::CollectionSummary).
    pub fn build(self) -> crate::types::CollectionSummary {
        crate::types::CollectionSummary {
            arn: self.arn,
            id: self.id,
            name: self.name,
            version: self.version,
        }
    }
}