// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <note>
/// <p>We are retiring EC2-Classic. We recommend that you migrate from EC2-Classic to a VPC. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-migrate.html">Migrate from EC2-Classic to a VPC</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
/// </note>
/// <p>Describes whether a VPC is enabled for ClassicLink.</p>
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
pub struct VpcClassicLink {
    /// <p>Indicates whether the VPC is enabled for ClassicLink.</p>
    #[doc(hidden)]
    pub classic_link_enabled: std::option::Option<bool>,
    /// <p>Any tags assigned to the VPC.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    /// <p>The ID of the VPC.</p>
    #[doc(hidden)]
    pub vpc_id: std::option::Option<std::string::String>,
}
impl VpcClassicLink {
    /// <p>Indicates whether the VPC is enabled for ClassicLink.</p>
    pub fn classic_link_enabled(&self) -> std::option::Option<bool> {
        self.classic_link_enabled
    }
    /// <p>Any tags assigned to the VPC.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(&self) -> std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
}
impl VpcClassicLink {
    /// Creates a new builder-style object to manufacture [`VpcClassicLink`](crate::types::VpcClassicLink).
    pub fn builder() -> crate::types::builders::VpcClassicLinkBuilder {
        crate::types::builders::VpcClassicLinkBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::VpcClassicLink;
/// A builder for [`VpcClassicLink`](crate::types::VpcClassicLink).
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
pub struct VpcClassicLinkBuilder {
    pub(crate) classic_link_enabled: std::option::Option<bool>,
    pub(crate) tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    pub(crate) vpc_id: std::option::Option<std::string::String>,
}
impl VpcClassicLinkBuilder {
    /// <p>Indicates whether the VPC is enabled for ClassicLink.</p>
    pub fn classic_link_enabled(mut self, input: bool) -> Self {
        self.classic_link_enabled = Some(input);
        self
    }
    /// <p>Indicates whether the VPC is enabled for ClassicLink.</p>
    pub fn set_classic_link_enabled(mut self, input: std::option::Option<bool>) -> Self {
        self.classic_link_enabled = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Any tags assigned to the VPC.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = Some(v);
        self
    }
    /// <p>Any tags assigned to the VPC.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.vpc_id = Some(input.into());
        self
    }
    /// <p>The ID of the VPC.</p>
    pub fn set_vpc_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.vpc_id = input;
        self
    }
    /// Consumes the builder and constructs a [`VpcClassicLink`](crate::types::VpcClassicLink).
    pub fn build(self) -> crate::types::VpcClassicLink {
        crate::types::VpcClassicLink {
            classic_link_enabled: self.classic_link_enabled,
            tags: self.tags,
            vpc_id: self.vpc_id,
        }
    }
}
