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
pub struct DescribeImportSnapshotTasksOutput {
    /// <p>A list of zero or more import snapshot tasks that are currently active or were completed or canceled in the previous 7 days.</p>
    #[doc(hidden)]
    pub import_snapshot_tasks: std::option::Option<std::vec::Vec<crate::types::ImportSnapshotTask>>,
    /// <p>The token to use to get the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribeImportSnapshotTasksOutput {
    /// <p>A list of zero or more import snapshot tasks that are currently active or were completed or canceled in the previous 7 days.</p>
    pub fn import_snapshot_tasks(
        &self,
    ) -> std::option::Option<&[crate::types::ImportSnapshotTask]> {
        self.import_snapshot_tasks.as_deref()
    }
    /// <p>The token to use to get the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for DescribeImportSnapshotTasksOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeImportSnapshotTasksOutput {
    /// Creates a new builder-style object to manufacture [`DescribeImportSnapshotTasksOutput`](crate::operation::describe_import_snapshot_tasks::DescribeImportSnapshotTasksOutput).
    pub fn builder() -> crate::operation::describe_import_snapshot_tasks::builders::DescribeImportSnapshotTasksOutputBuilder{
        crate::operation::describe_import_snapshot_tasks::builders::DescribeImportSnapshotTasksOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::describe_import_snapshot_tasks::DescribeImportSnapshotTasksOutput;
/// A builder for [`DescribeImportSnapshotTasksOutput`](crate::operation::describe_import_snapshot_tasks::DescribeImportSnapshotTasksOutput).
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
pub struct DescribeImportSnapshotTasksOutputBuilder {
    pub(crate) import_snapshot_tasks:
        std::option::Option<std::vec::Vec<crate::types::ImportSnapshotTask>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribeImportSnapshotTasksOutputBuilder {
    /// Appends an item to `import_snapshot_tasks`.
    ///
    /// To override the contents of this collection use [`set_import_snapshot_tasks`](Self::set_import_snapshot_tasks).
    ///
    /// <p>A list of zero or more import snapshot tasks that are currently active or were completed or canceled in the previous 7 days.</p>
    pub fn import_snapshot_tasks(mut self, input: crate::types::ImportSnapshotTask) -> Self {
        let mut v = self.import_snapshot_tasks.unwrap_or_default();
        v.push(input);
        self.import_snapshot_tasks = Some(v);
        self
    }
    /// <p>A list of zero or more import snapshot tasks that are currently active or were completed or canceled in the previous 7 days.</p>
    pub fn set_import_snapshot_tasks(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ImportSnapshotTask>>,
    ) -> Self {
        self.import_snapshot_tasks = input;
        self
    }
    /// <p>The token to use to get the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The token to use to get the next page of results. This value is <code>null</code> when there are no more results to return.</p>
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
    /// Consumes the builder and constructs a [`DescribeImportSnapshotTasksOutput`](crate::operation::describe_import_snapshot_tasks::DescribeImportSnapshotTasksOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_import_snapshot_tasks::DescribeImportSnapshotTasksOutput {
        crate::operation::describe_import_snapshot_tasks::DescribeImportSnapshotTasksOutput {
            import_snapshot_tasks: self.import_snapshot_tasks,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
