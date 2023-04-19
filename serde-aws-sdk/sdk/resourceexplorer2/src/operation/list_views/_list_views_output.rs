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
pub struct ListViewsOutput {
    /// <p>The list of views available in the Amazon Web Services Region in which you called this operation.</p>
    #[doc(hidden)]
    pub views: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListViewsOutput {
    /// <p>The list of views available in the Amazon Web Services Region in which you called this operation.</p>
    pub fn views(&self) -> std::option::Option<&[std::string::String]> {
        self.views.as_deref()
    }
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for ListViewsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListViewsOutput {
    /// Creates a new builder-style object to manufacture [`ListViewsOutput`](crate::operation::list_views::ListViewsOutput).
    pub fn builder() -> crate::operation::list_views::builders::ListViewsOutputBuilder {
        crate::operation::list_views::builders::ListViewsOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_views::ListViewsOutput;
/// A builder for [`ListViewsOutput`](crate::operation::list_views::ListViewsOutput).
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
pub struct ListViewsOutputBuilder {
    pub(crate) views: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListViewsOutputBuilder {
    /// Appends an item to `views`.
    ///
    /// To override the contents of this collection use [`set_views`](Self::set_views).
    ///
    /// <p>The list of views available in the Amazon Web Services Region in which you called this operation.</p>
    pub fn views(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.views.unwrap_or_default();
        v.push(input.into());
        self.views = Some(v);
        self
    }
    /// <p>The list of views available in the Amazon Web Services Region in which you called this operation.</p>
    pub fn set_views(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.views = input;
        self
    }
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
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
    /// Consumes the builder and constructs a [`ListViewsOutput`](crate::operation::list_views::ListViewsOutput).
    pub fn build(self) -> crate::operation::list_views::ListViewsOutput {
        crate::operation::list_views::ListViewsOutput {
            views: self.views,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
