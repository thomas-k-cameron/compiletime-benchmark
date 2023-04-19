// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an IAM instance profile.</p>
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
pub struct LaunchTemplateIamInstanceProfileSpecification {
    /// <p>The Amazon Resource Name (ARN) of the instance profile.</p>
    #[doc(hidden)]
    pub arn: std::option::Option<std::string::String>,
    /// <p>The name of the instance profile.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
}
impl LaunchTemplateIamInstanceProfileSpecification {
    /// <p>The Amazon Resource Name (ARN) of the instance profile.</p>
    pub fn arn(&self) -> std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The name of the instance profile.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl LaunchTemplateIamInstanceProfileSpecification {
    /// Creates a new builder-style object to manufacture [`LaunchTemplateIamInstanceProfileSpecification`](crate::types::LaunchTemplateIamInstanceProfileSpecification).
    pub fn builder() -> crate::types::builders::LaunchTemplateIamInstanceProfileSpecificationBuilder
    {
        crate::types::builders::LaunchTemplateIamInstanceProfileSpecificationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::LaunchTemplateIamInstanceProfileSpecification;
/// A builder for [`LaunchTemplateIamInstanceProfileSpecification`](crate::types::LaunchTemplateIamInstanceProfileSpecification).
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
pub struct LaunchTemplateIamInstanceProfileSpecificationBuilder {
    pub(crate) arn: std::option::Option<std::string::String>,
    pub(crate) name: std::option::Option<std::string::String>,
}
impl LaunchTemplateIamInstanceProfileSpecificationBuilder {
    /// <p>The Amazon Resource Name (ARN) of the instance profile.</p>
    pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the instance profile.</p>
    pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The name of the instance profile.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.name = Some(input.into());
        self
    }
    /// <p>The name of the instance profile.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`LaunchTemplateIamInstanceProfileSpecification`](crate::types::LaunchTemplateIamInstanceProfileSpecification).
    pub fn build(self) -> crate::types::LaunchTemplateIamInstanceProfileSpecification {
        crate::types::LaunchTemplateIamInstanceProfileSpecification {
            arn: self.arn,
            name: self.name,
        }
    }
}