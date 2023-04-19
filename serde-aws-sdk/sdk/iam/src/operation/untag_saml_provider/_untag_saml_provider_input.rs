// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
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
pub struct UntagSamlProviderInput {
    /// <p>The ARN of the SAML identity provider in IAM from which you want to remove tags.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    #[doc(hidden)]
    pub saml_provider_arn: std::option::Option<std::string::String>,
    /// <p>A list of key names as a simple array of strings. The tags with matching keys are removed from the specified SAML identity provider.</p>
    #[doc(hidden)]
    pub tag_keys: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl UntagSamlProviderInput {
    /// <p>The ARN of the SAML identity provider in IAM from which you want to remove tags.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn saml_provider_arn(&self) -> std::option::Option<&str> {
        self.saml_provider_arn.as_deref()
    }
    /// <p>A list of key names as a simple array of strings. The tags with matching keys are removed from the specified SAML identity provider.</p>
    pub fn tag_keys(&self) -> std::option::Option<&[std::string::String]> {
        self.tag_keys.as_deref()
    }
}
impl UntagSamlProviderInput {
    /// Creates a new builder-style object to manufacture [`UntagSamlProviderInput`](crate::operation::untag_saml_provider::UntagSamlProviderInput).
    pub fn builder(
    ) -> crate::operation::untag_saml_provider::builders::UntagSamlProviderInputBuilder {
        crate::operation::untag_saml_provider::builders::UntagSamlProviderInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::untag_saml_provider::UntagSamlProviderInput;
/// A builder for [`UntagSamlProviderInput`](crate::operation::untag_saml_provider::UntagSamlProviderInput).
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
pub struct UntagSamlProviderInputBuilder {
    pub(crate) saml_provider_arn: std::option::Option<std::string::String>,
    pub(crate) tag_keys: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl UntagSamlProviderInputBuilder {
    /// <p>The ARN of the SAML identity provider in IAM from which you want to remove tags.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn saml_provider_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.saml_provider_arn = Some(input.into());
        self
    }
    /// <p>The ARN of the SAML identity provider in IAM from which you want to remove tags.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_saml_provider_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.saml_provider_arn = input;
        self
    }
    /// Appends an item to `tag_keys`.
    ///
    /// To override the contents of this collection use [`set_tag_keys`](Self::set_tag_keys).
    ///
    /// <p>A list of key names as a simple array of strings. The tags with matching keys are removed from the specified SAML identity provider.</p>
    pub fn tag_keys(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.tag_keys.unwrap_or_default();
        v.push(input.into());
        self.tag_keys = Some(v);
        self
    }
    /// <p>A list of key names as a simple array of strings. The tags with matching keys are removed from the specified SAML identity provider.</p>
    pub fn set_tag_keys(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.tag_keys = input;
        self
    }
    /// Consumes the builder and constructs a [`UntagSamlProviderInput`](crate::operation::untag_saml_provider::UntagSamlProviderInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::untag_saml_provider::UntagSamlProviderInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::untag_saml_provider::UntagSamlProviderInput {
                saml_provider_arn: self.saml_provider_arn,
                tag_keys: self.tag_keys,
            },
        )
    }
}
