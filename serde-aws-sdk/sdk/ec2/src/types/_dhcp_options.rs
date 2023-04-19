// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a set of DHCP options.</p>
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
pub struct DhcpOptions {
    /// <p>One or more DHCP options in the set.</p>
    #[doc(hidden)]
    pub dhcp_configurations: std::option::Option<std::vec::Vec<crate::types::DhcpConfiguration>>,
    /// <p>The ID of the set of DHCP options.</p>
    #[doc(hidden)]
    pub dhcp_options_id: std::option::Option<std::string::String>,
    /// <p>The ID of the Amazon Web Services account that owns the DHCP options set.</p>
    #[doc(hidden)]
    pub owner_id: std::option::Option<std::string::String>,
    /// <p>Any tags assigned to the DHCP options set.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
}
impl DhcpOptions {
    /// <p>One or more DHCP options in the set.</p>
    pub fn dhcp_configurations(&self) -> std::option::Option<&[crate::types::DhcpConfiguration]> {
        self.dhcp_configurations.as_deref()
    }
    /// <p>The ID of the set of DHCP options.</p>
    pub fn dhcp_options_id(&self) -> std::option::Option<&str> {
        self.dhcp_options_id.as_deref()
    }
    /// <p>The ID of the Amazon Web Services account that owns the DHCP options set.</p>
    pub fn owner_id(&self) -> std::option::Option<&str> {
        self.owner_id.as_deref()
    }
    /// <p>Any tags assigned to the DHCP options set.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl DhcpOptions {
    /// Creates a new builder-style object to manufacture [`DhcpOptions`](crate::types::DhcpOptions).
    pub fn builder() -> crate::types::builders::DhcpOptionsBuilder {
        crate::types::builders::DhcpOptionsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::DhcpOptions;
/// A builder for [`DhcpOptions`](crate::types::DhcpOptions).
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
pub struct DhcpOptionsBuilder {
    pub(crate) dhcp_configurations:
        std::option::Option<std::vec::Vec<crate::types::DhcpConfiguration>>,
    pub(crate) dhcp_options_id: std::option::Option<std::string::String>,
    pub(crate) owner_id: std::option::Option<std::string::String>,
    pub(crate) tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
}
impl DhcpOptionsBuilder {
    /// Appends an item to `dhcp_configurations`.
    ///
    /// To override the contents of this collection use [`set_dhcp_configurations`](Self::set_dhcp_configurations).
    ///
    /// <p>One or more DHCP options in the set.</p>
    pub fn dhcp_configurations(mut self, input: crate::types::DhcpConfiguration) -> Self {
        let mut v = self.dhcp_configurations.unwrap_or_default();
        v.push(input);
        self.dhcp_configurations = Some(v);
        self
    }
    /// <p>One or more DHCP options in the set.</p>
    pub fn set_dhcp_configurations(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::DhcpConfiguration>>,
    ) -> Self {
        self.dhcp_configurations = input;
        self
    }
    /// <p>The ID of the set of DHCP options.</p>
    pub fn dhcp_options_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.dhcp_options_id = Some(input.into());
        self
    }
    /// <p>The ID of the set of DHCP options.</p>
    pub fn set_dhcp_options_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.dhcp_options_id = input;
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the DHCP options set.</p>
    pub fn owner_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.owner_id = Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the DHCP options set.</p>
    pub fn set_owner_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.owner_id = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Any tags assigned to the DHCP options set.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = Some(v);
        self
    }
    /// <p>Any tags assigned to the DHCP options set.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`DhcpOptions`](crate::types::DhcpOptions).
    pub fn build(self) -> crate::types::DhcpOptions {
        crate::types::DhcpOptions {
            dhcp_configurations: self.dhcp_configurations,
            dhcp_options_id: self.dhcp_options_id,
            owner_id: self.owner_id,
            tags: self.tags,
        }
    }
}
