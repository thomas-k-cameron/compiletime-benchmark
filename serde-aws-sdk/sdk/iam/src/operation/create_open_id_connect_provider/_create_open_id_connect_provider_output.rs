// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the response to a successful <code>CreateOpenIDConnectProvider</code> request. </p>
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
pub struct CreateOpenIdConnectProviderOutput {
    /// <p>The Amazon Resource Name (ARN) of the new IAM OpenID Connect provider that is created. For more information, see <code>OpenIDConnectProviderListEntry</code>. </p>
    #[doc(hidden)]
    pub open_id_connect_provider_arn: std::option::Option<std::string::String>,
    /// <p>A list of tags that are attached to the new IAM OIDC provider. The returned list of tags is sorted by tag key. For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the <i>IAM User Guide</i>.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    _request_id: Option<String>,
}
impl CreateOpenIdConnectProviderOutput {
    /// <p>The Amazon Resource Name (ARN) of the new IAM OpenID Connect provider that is created. For more information, see <code>OpenIDConnectProviderListEntry</code>. </p>
    pub fn open_id_connect_provider_arn(&self) -> std::option::Option<&str> {
        self.open_id_connect_provider_arn.as_deref()
    }
    /// <p>A list of tags that are attached to the new IAM OIDC provider. The returned list of tags is sorted by tag key. For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the <i>IAM User Guide</i>.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl aws_http::request_id::RequestId for CreateOpenIdConnectProviderOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateOpenIdConnectProviderOutput {
    /// Creates a new builder-style object to manufacture [`CreateOpenIdConnectProviderOutput`](crate::operation::create_open_id_connect_provider::CreateOpenIdConnectProviderOutput).
    pub fn builder() -> crate::operation::create_open_id_connect_provider::builders::CreateOpenIdConnectProviderOutputBuilder{
        crate::operation::create_open_id_connect_provider::builders::CreateOpenIdConnectProviderOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::create_open_id_connect_provider::CreateOpenIdConnectProviderOutput;
/// A builder for [`CreateOpenIdConnectProviderOutput`](crate::operation::create_open_id_connect_provider::CreateOpenIdConnectProviderOutput).
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
pub struct CreateOpenIdConnectProviderOutputBuilder {
    pub(crate) open_id_connect_provider_arn: std::option::Option<std::string::String>,
    pub(crate) tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    _request_id: Option<String>,
}
impl CreateOpenIdConnectProviderOutputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the new IAM OpenID Connect provider that is created. For more information, see <code>OpenIDConnectProviderListEntry</code>. </p>
    pub fn open_id_connect_provider_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.open_id_connect_provider_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the new IAM OpenID Connect provider that is created. For more information, see <code>OpenIDConnectProviderListEntry</code>. </p>
    pub fn set_open_id_connect_provider_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.open_id_connect_provider_arn = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of tags that are attached to the new IAM OIDC provider. The returned list of tags is sorted by tag key. For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the <i>IAM User Guide</i>.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = Some(v);
        self
    }
    /// <p>A list of tags that are attached to the new IAM OIDC provider. The returned list of tags is sorted by tag key. For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the <i>IAM User Guide</i>.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateOpenIdConnectProviderOutput`](crate::operation::create_open_id_connect_provider::CreateOpenIdConnectProviderOutput).
    pub fn build(
        self,
    ) -> crate::operation::create_open_id_connect_provider::CreateOpenIdConnectProviderOutput {
        crate::operation::create_open_id_connect_provider::CreateOpenIdConnectProviderOutput {
            open_id_connect_provider_arn: self.open_id_connect_provider_arn,
            tags: self.tags,
            _request_id: self._request_id,
        }
    }
}
