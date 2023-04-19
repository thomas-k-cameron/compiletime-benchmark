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
pub struct ListTagsOfResourceOutput {
    /// <p>The tags currently associated with the Amazon DynamoDB resource.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    /// <p>If this value is returned, there are additional results to be displayed. To retrieve them, call ListTagsOfResource again, with NextToken set to this value.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListTagsOfResourceOutput {
    /// <p>The tags currently associated with the Amazon DynamoDB resource.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
    /// <p>If this value is returned, there are additional results to be displayed. To retrieve them, call ListTagsOfResource again, with NextToken set to this value.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for ListTagsOfResourceOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListTagsOfResourceOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsOfResourceOutput`](crate::operation::list_tags_of_resource::ListTagsOfResourceOutput).
    pub fn builder(
    ) -> crate::operation::list_tags_of_resource::builders::ListTagsOfResourceOutputBuilder {
        crate::operation::list_tags_of_resource::builders::ListTagsOfResourceOutputBuilder::default(
        )
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_tags_of_resource::ListTagsOfResourceOutput;
/// A builder for [`ListTagsOfResourceOutput`](crate::operation::list_tags_of_resource::ListTagsOfResourceOutput).
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
pub struct ListTagsOfResourceOutputBuilder {
    pub(crate) tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListTagsOfResourceOutputBuilder {
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags currently associated with the Amazon DynamoDB resource.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = Some(v);
        self
    }
    /// <p>The tags currently associated with the Amazon DynamoDB resource.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// <p>If this value is returned, there are additional results to be displayed. To retrieve them, call ListTagsOfResource again, with NextToken set to this value.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>If this value is returned, there are additional results to be displayed. To retrieve them, call ListTagsOfResource again, with NextToken set to this value.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
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
    /// Consumes the builder and constructs a [`ListTagsOfResourceOutput`](crate::operation::list_tags_of_resource::ListTagsOfResourceOutput).
    pub fn build(self) -> crate::operation::list_tags_of_resource::ListTagsOfResourceOutput {
        crate::operation::list_tags_of_resource::ListTagsOfResourceOutput {
            tags: self.tags,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
