// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Hostnames and IP address entries that are added to the <code>/etc/hosts</code> file of a container via the <code>extraHosts</code> parameter of its <code>ContainerDefinition</code>. </p>
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
pub struct HostEntry {
    /// <p>The hostname to use in the <code>/etc/hosts</code> entry.</p>
    #[doc(hidden)]
    pub hostname: std::option::Option<std::string::String>,
    /// <p>The IP address to use in the <code>/etc/hosts</code> entry.</p>
    #[doc(hidden)]
    pub ip_address: std::option::Option<std::string::String>,
}
impl HostEntry {
    /// <p>The hostname to use in the <code>/etc/hosts</code> entry.</p>
    pub fn hostname(&self) -> std::option::Option<&str> {
        self.hostname.as_deref()
    }
    /// <p>The IP address to use in the <code>/etc/hosts</code> entry.</p>
    pub fn ip_address(&self) -> std::option::Option<&str> {
        self.ip_address.as_deref()
    }
}
impl HostEntry {
    /// Creates a new builder-style object to manufacture [`HostEntry`](crate::types::HostEntry).
    pub fn builder() -> crate::types::builders::HostEntryBuilder {
        crate::types::builders::HostEntryBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::HostEntry;
/// A builder for [`HostEntry`](crate::types::HostEntry).
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
pub struct HostEntryBuilder {
    pub(crate) hostname: std::option::Option<std::string::String>,
    pub(crate) ip_address: std::option::Option<std::string::String>,
}
impl HostEntryBuilder {
    /// <p>The hostname to use in the <code>/etc/hosts</code> entry.</p>
    pub fn hostname(mut self, input: impl Into<std::string::String>) -> Self {
        self.hostname = Some(input.into());
        self
    }
    /// <p>The hostname to use in the <code>/etc/hosts</code> entry.</p>
    pub fn set_hostname(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.hostname = input;
        self
    }
    /// <p>The IP address to use in the <code>/etc/hosts</code> entry.</p>
    pub fn ip_address(mut self, input: impl Into<std::string::String>) -> Self {
        self.ip_address = Some(input.into());
        self
    }
    /// <p>The IP address to use in the <code>/etc/hosts</code> entry.</p>
    pub fn set_ip_address(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.ip_address = input;
        self
    }
    /// Consumes the builder and constructs a [`HostEntry`](crate::types::HostEntry).
    pub fn build(self) -> crate::types::HostEntry {
        crate::types::HostEntry {
            hostname: self.hostname,
            ip_address: self.ip_address,
        }
    }
}