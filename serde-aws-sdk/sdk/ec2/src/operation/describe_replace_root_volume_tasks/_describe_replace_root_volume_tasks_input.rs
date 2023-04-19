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
pub struct DescribeReplaceRootVolumeTasksInput {
    /// <p>The ID of the root volume replacement task to view.</p>
    #[doc(hidden)]
    pub replace_root_volume_task_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>Filter to use:</p>
    /// <ul>
    /// <li> <p> <code>instance-id</code> - The ID of the instance for which the root volume replacement task was created.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub filters: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    #[doc(hidden)]
    pub max_results: std::option::Option<i32>,
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl DescribeReplaceRootVolumeTasksInput {
    /// <p>The ID of the root volume replacement task to view.</p>
    pub fn replace_root_volume_task_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.replace_root_volume_task_ids.as_deref()
    }
    /// <p>Filter to use:</p>
    /// <ul>
    /// <li> <p> <code>instance-id</code> - The ID of the instance for which the root volume replacement task was created.</p> </li>
    /// </ul>
    pub fn filters(&self) -> std::option::Option<&[crate::types::Filter]> {
        self.filters.as_deref()
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn max_results(&self) -> std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl DescribeReplaceRootVolumeTasksInput {
    /// Creates a new builder-style object to manufacture [`DescribeReplaceRootVolumeTasksInput`](crate::operation::describe_replace_root_volume_tasks::DescribeReplaceRootVolumeTasksInput).
    pub fn builder() -> crate::operation::describe_replace_root_volume_tasks::builders::DescribeReplaceRootVolumeTasksInputBuilder{
        crate::operation::describe_replace_root_volume_tasks::builders::DescribeReplaceRootVolumeTasksInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::describe_replace_root_volume_tasks::DescribeReplaceRootVolumeTasksInput;
/// A builder for [`DescribeReplaceRootVolumeTasksInput`](crate::operation::describe_replace_root_volume_tasks::DescribeReplaceRootVolumeTasksInput).
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
pub struct DescribeReplaceRootVolumeTasksInputBuilder {
    pub(crate) replace_root_volume_task_ids:
        std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) filters: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    pub(crate) max_results: std::option::Option<i32>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl DescribeReplaceRootVolumeTasksInputBuilder {
    /// Appends an item to `replace_root_volume_task_ids`.
    ///
    /// To override the contents of this collection use [`set_replace_root_volume_task_ids`](Self::set_replace_root_volume_task_ids).
    ///
    /// <p>The ID of the root volume replacement task to view.</p>
    pub fn replace_root_volume_task_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.replace_root_volume_task_ids.unwrap_or_default();
        v.push(input.into());
        self.replace_root_volume_task_ids = Some(v);
        self
    }
    /// <p>The ID of the root volume replacement task to view.</p>
    pub fn set_replace_root_volume_task_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.replace_root_volume_task_ids = input;
        self
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>Filter to use:</p>
    /// <ul>
    /// <li> <p> <code>instance-id</code> - The ID of the instance for which the root volume replacement task was created.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = Some(v);
        self
    }
    /// <p>Filter to use:</p>
    /// <ul>
    /// <li> <p> <code>instance-id</code> - The ID of the instance for which the root volume replacement task was created.</p> </li>
    /// </ul>
    pub fn set_filters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.filters = input;
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = Some(input);
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeReplaceRootVolumeTasksInput`](crate::operation::describe_replace_root_volume_tasks::DescribeReplaceRootVolumeTasksInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::describe_replace_root_volume_tasks::DescribeReplaceRootVolumeTasksInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::describe_replace_root_volume_tasks::DescribeReplaceRootVolumeTasksInput {
                replace_root_volume_task_ids: self.replace_root_volume_task_ids
                ,
                filters: self.filters
                ,
                max_results: self.max_results
                ,
                next_token: self.next_token
                ,
                dry_run: self.dry_run
                ,
            }
        )
    }
}