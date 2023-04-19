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
pub struct DescribeSnapshotsOutput {
    /// <p>Information about the snapshots.</p>
    #[doc(hidden)]
    pub snapshots: std::option::Option<std::vec::Vec<crate::types::Snapshot>>,
    /// <p>The token to include in another request to return the next page of snapshots. This value is <code>null</code> when there are no more snapshots to return.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribeSnapshotsOutput {
    /// <p>Information about the snapshots.</p>
    pub fn snapshots(&self) -> std::option::Option<&[crate::types::Snapshot]> {
        self.snapshots.as_deref()
    }
    /// <p>The token to include in another request to return the next page of snapshots. This value is <code>null</code> when there are no more snapshots to return.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for DescribeSnapshotsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeSnapshotsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeSnapshotsOutput`](crate::operation::describe_snapshots::DescribeSnapshotsOutput).
    pub fn builder(
    ) -> crate::operation::describe_snapshots::builders::DescribeSnapshotsOutputBuilder {
        crate::operation::describe_snapshots::builders::DescribeSnapshotsOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_snapshots::DescribeSnapshotsOutput;
/// A builder for [`DescribeSnapshotsOutput`](crate::operation::describe_snapshots::DescribeSnapshotsOutput).
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
pub struct DescribeSnapshotsOutputBuilder {
    pub(crate) snapshots: std::option::Option<std::vec::Vec<crate::types::Snapshot>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribeSnapshotsOutputBuilder {
    /// Appends an item to `snapshots`.
    ///
    /// To override the contents of this collection use [`set_snapshots`](Self::set_snapshots).
    ///
    /// <p>Information about the snapshots.</p>
    pub fn snapshots(mut self, input: crate::types::Snapshot) -> Self {
        let mut v = self.snapshots.unwrap_or_default();
        v.push(input);
        self.snapshots = Some(v);
        self
    }
    /// <p>Information about the snapshots.</p>
    pub fn set_snapshots(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Snapshot>>,
    ) -> Self {
        self.snapshots = input;
        self
    }
    /// <p>The token to include in another request to return the next page of snapshots. This value is <code>null</code> when there are no more snapshots to return.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The token to include in another request to return the next page of snapshots. This value is <code>null</code> when there are no more snapshots to return.</p>
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
    /// Consumes the builder and constructs a [`DescribeSnapshotsOutput`](crate::operation::describe_snapshots::DescribeSnapshotsOutput).
    pub fn build(self) -> crate::operation::describe_snapshots::DescribeSnapshotsOutput {
        crate::operation::describe_snapshots::DescribeSnapshotsOutput {
            snapshots: self.snapshots,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
