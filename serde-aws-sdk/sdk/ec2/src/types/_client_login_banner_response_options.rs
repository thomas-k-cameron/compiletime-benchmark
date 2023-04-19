// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Current state of options for customizable text banner that will be displayed on Amazon Web Services provided clients when a VPN session is established.</p>
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
pub struct ClientLoginBannerResponseOptions {
    /// <p>Current state of text banner feature.</p>
    /// <p>Valid values: <code>true | false</code> </p>
    #[doc(hidden)]
    pub enabled: std::option::Option<bool>,
    /// <p>Customizable text that will be displayed in a banner on Amazon Web Services provided clients when a VPN session is established. UTF-8 encoded characters only. Maximum of 1400 characters.</p>
    #[doc(hidden)]
    pub banner_text: std::option::Option<std::string::String>,
}
impl ClientLoginBannerResponseOptions {
    /// <p>Current state of text banner feature.</p>
    /// <p>Valid values: <code>true | false</code> </p>
    pub fn enabled(&self) -> std::option::Option<bool> {
        self.enabled
    }
    /// <p>Customizable text that will be displayed in a banner on Amazon Web Services provided clients when a VPN session is established. UTF-8 encoded characters only. Maximum of 1400 characters.</p>
    pub fn banner_text(&self) -> std::option::Option<&str> {
        self.banner_text.as_deref()
    }
}
impl ClientLoginBannerResponseOptions {
    /// Creates a new builder-style object to manufacture [`ClientLoginBannerResponseOptions`](crate::types::ClientLoginBannerResponseOptions).
    pub fn builder() -> crate::types::builders::ClientLoginBannerResponseOptionsBuilder {
        crate::types::builders::ClientLoginBannerResponseOptionsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ClientLoginBannerResponseOptions;
/// A builder for [`ClientLoginBannerResponseOptions`](crate::types::ClientLoginBannerResponseOptions).
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
pub struct ClientLoginBannerResponseOptionsBuilder {
    pub(crate) enabled: std::option::Option<bool>,
    pub(crate) banner_text: std::option::Option<std::string::String>,
}
impl ClientLoginBannerResponseOptionsBuilder {
    /// <p>Current state of text banner feature.</p>
    /// <p>Valid values: <code>true | false</code> </p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.enabled = Some(input);
        self
    }
    /// <p>Current state of text banner feature.</p>
    /// <p>Valid values: <code>true | false</code> </p>
    pub fn set_enabled(mut self, input: std::option::Option<bool>) -> Self {
        self.enabled = input;
        self
    }
    /// <p>Customizable text that will be displayed in a banner on Amazon Web Services provided clients when a VPN session is established. UTF-8 encoded characters only. Maximum of 1400 characters.</p>
    pub fn banner_text(mut self, input: impl Into<std::string::String>) -> Self {
        self.banner_text = Some(input.into());
        self
    }
    /// <p>Customizable text that will be displayed in a banner on Amazon Web Services provided clients when a VPN session is established. UTF-8 encoded characters only. Maximum of 1400 characters.</p>
    pub fn set_banner_text(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.banner_text = input;
        self
    }
    /// Consumes the builder and constructs a [`ClientLoginBannerResponseOptions`](crate::types::ClientLoginBannerResponseOptions).
    pub fn build(self) -> crate::types::ClientLoginBannerResponseOptions {
        crate::types::ClientLoginBannerResponseOptions {
            enabled: self.enabled,
            banner_text: self.banner_text,
        }
    }
}