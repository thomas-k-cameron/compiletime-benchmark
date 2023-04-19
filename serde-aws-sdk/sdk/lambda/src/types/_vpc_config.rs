// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The VPC security groups and subnets that are attached to a Lambda function. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-vpc.html">Configuring a Lambda function to access resources in a VPC</a>.</p>
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
pub struct VpcConfig {
    /// <p>A list of VPC subnet IDs.</p>
    #[doc(hidden)]
    pub subnet_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>A list of VPC security group IDs.</p>
    #[doc(hidden)]
    pub security_group_ids: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl VpcConfig {
    /// <p>A list of VPC subnet IDs.</p>
    pub fn subnet_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.subnet_ids.as_deref()
    }
    /// <p>A list of VPC security group IDs.</p>
    pub fn security_group_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.security_group_ids.as_deref()
    }
}
impl VpcConfig {
    /// Creates a new builder-style object to manufacture [`VpcConfig`](crate::types::VpcConfig).
    pub fn builder() -> crate::types::builders::VpcConfigBuilder {
        crate::types::builders::VpcConfigBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::VpcConfig;
/// A builder for [`VpcConfig`](crate::types::VpcConfig).
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
pub struct VpcConfigBuilder {
    pub(crate) subnet_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) security_group_ids: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl VpcConfigBuilder {
    /// Appends an item to `subnet_ids`.
    ///
    /// To override the contents of this collection use [`set_subnet_ids`](Self::set_subnet_ids).
    ///
    /// <p>A list of VPC subnet IDs.</p>
    pub fn subnet_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.subnet_ids.unwrap_or_default();
        v.push(input.into());
        self.subnet_ids = Some(v);
        self
    }
    /// <p>A list of VPC subnet IDs.</p>
    pub fn set_subnet_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.subnet_ids = input;
        self
    }
    /// Appends an item to `security_group_ids`.
    ///
    /// To override the contents of this collection use [`set_security_group_ids`](Self::set_security_group_ids).
    ///
    /// <p>A list of VPC security group IDs.</p>
    pub fn security_group_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.security_group_ids.unwrap_or_default();
        v.push(input.into());
        self.security_group_ids = Some(v);
        self
    }
    /// <p>A list of VPC security group IDs.</p>
    pub fn set_security_group_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.security_group_ids = input;
        self
    }
    /// Consumes the builder and constructs a [`VpcConfig`](crate::types::VpcConfig).
    pub fn build(self) -> crate::types::VpcConfig {
        crate::types::VpcConfig {
            subnet_ids: self.subnet_ids,
            security_group_ids: self.security_group_ids,
        }
    }
}
