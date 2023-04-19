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
pub struct ListIndexesOutput {
    /// <p>A structure that contains the details and status of each index.</p>
    #[doc(hidden)]
    pub indexes: std::option::Option<std::vec::Vec<crate::types::Index>>,
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListIndexesOutput {
    /// <p>A structure that contains the details and status of each index.</p>
    pub fn indexes(&self) -> std::option::Option<&[crate::types::Index]> {
        self.indexes.as_deref()
    }
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for ListIndexesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListIndexesOutput {
    /// Creates a new builder-style object to manufacture [`ListIndexesOutput`](crate::operation::list_indexes::ListIndexesOutput).
    pub fn builder() -> crate::operation::list_indexes::builders::ListIndexesOutputBuilder {
        crate::operation::list_indexes::builders::ListIndexesOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_indexes::ListIndexesOutput;
/// A builder for [`ListIndexesOutput`](crate::operation::list_indexes::ListIndexesOutput).
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
pub struct ListIndexesOutputBuilder {
    pub(crate) indexes: std::option::Option<std::vec::Vec<crate::types::Index>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListIndexesOutputBuilder {
    /// Appends an item to `indexes`.
    ///
    /// To override the contents of this collection use [`set_indexes`](Self::set_indexes).
    ///
    /// <p>A structure that contains the details and status of each index.</p>
    pub fn indexes(mut self, input: crate::types::Index) -> Self {
        let mut v = self.indexes.unwrap_or_default();
        v.push(input);
        self.indexes = Some(v);
        self
    }
    /// <p>A structure that contains the details and status of each index.</p>
    pub fn set_indexes(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Index>>,
    ) -> Self {
        self.indexes = input;
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
    /// Consumes the builder and constructs a [`ListIndexesOutput`](crate::operation::list_indexes::ListIndexesOutput).
    pub fn build(self) -> crate::operation::list_indexes::ListIndexesOutput {
        crate::operation::list_indexes::ListIndexesOutput {
            indexes: self.indexes,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
