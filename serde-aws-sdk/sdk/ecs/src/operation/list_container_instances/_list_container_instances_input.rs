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
pub struct ListContainerInstancesInput {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the container instances to list. If you do not specify a cluster, the default cluster is assumed.</p>
    #[doc(hidden)]
    pub cluster: std::option::Option<std::string::String>,
    /// <p>You can filter the results of a <code>ListContainerInstances</code> operation with cluster query language statements. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html">Cluster Query Language</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[doc(hidden)]
    pub filter: std::option::Option<std::string::String>,
    /// <p>The <code>nextToken</code> value returned from a <code>ListContainerInstances</code> request indicating that more results are available to fulfill the request and further calls are needed. If <code>maxResults</code> was provided, it's possible the number of results to be fewer than <code>maxResults</code>.</p> <note>
    /// <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p>
    /// </note>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The maximum number of container instance results that <code>ListContainerInstances</code> returned in paginated output. When this parameter is used, <code>ListContainerInstances</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListContainerInstances</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListContainerInstances</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[doc(hidden)]
    pub max_results: std::option::Option<i32>,
    /// <p>Filters the container instances by status. For example, if you specify the <code>DRAINING</code> status, the results include only container instances that have been set to <code>DRAINING</code> using <code>UpdateContainerInstancesState</code>. If you don't specify this parameter, the default is to include container instances set to all states other than <code>INACTIVE</code>.</p>
    #[doc(hidden)]
    pub status: std::option::Option<crate::types::ContainerInstanceStatus>,
}
impl ListContainerInstancesInput {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the container instances to list. If you do not specify a cluster, the default cluster is assumed.</p>
    pub fn cluster(&self) -> std::option::Option<&str> {
        self.cluster.as_deref()
    }
    /// <p>You can filter the results of a <code>ListContainerInstances</code> operation with cluster query language statements. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html">Cluster Query Language</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    pub fn filter(&self) -> std::option::Option<&str> {
        self.filter.as_deref()
    }
    /// <p>The <code>nextToken</code> value returned from a <code>ListContainerInstances</code> request indicating that more results are available to fulfill the request and further calls are needed. If <code>maxResults</code> was provided, it's possible the number of results to be fewer than <code>maxResults</code>.</p> <note>
    /// <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p>
    /// </note>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of container instance results that <code>ListContainerInstances</code> returned in paginated output. When this parameter is used, <code>ListContainerInstances</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListContainerInstances</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListContainerInstances</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    pub fn max_results(&self) -> std::option::Option<i32> {
        self.max_results
    }
    /// <p>Filters the container instances by status. For example, if you specify the <code>DRAINING</code> status, the results include only container instances that have been set to <code>DRAINING</code> using <code>UpdateContainerInstancesState</code>. If you don't specify this parameter, the default is to include container instances set to all states other than <code>INACTIVE</code>.</p>
    pub fn status(&self) -> std::option::Option<&crate::types::ContainerInstanceStatus> {
        self.status.as_ref()
    }
}
impl ListContainerInstancesInput {
    /// Creates a new builder-style object to manufacture [`ListContainerInstancesInput`](crate::operation::list_container_instances::ListContainerInstancesInput).
    pub fn builder(
    ) -> crate::operation::list_container_instances::builders::ListContainerInstancesInputBuilder
    {
        crate::operation::list_container_instances::builders::ListContainerInstancesInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_container_instances::ListContainerInstancesInput;
/// A builder for [`ListContainerInstancesInput`](crate::operation::list_container_instances::ListContainerInstancesInput).
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
pub struct ListContainerInstancesInputBuilder {
    pub(crate) cluster: std::option::Option<std::string::String>,
    pub(crate) filter: std::option::Option<std::string::String>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    pub(crate) max_results: std::option::Option<i32>,
    pub(crate) status: std::option::Option<crate::types::ContainerInstanceStatus>,
}
impl ListContainerInstancesInputBuilder {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the container instances to list. If you do not specify a cluster, the default cluster is assumed.</p>
    pub fn cluster(mut self, input: impl Into<std::string::String>) -> Self {
        self.cluster = Some(input.into());
        self
    }
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the container instances to list. If you do not specify a cluster, the default cluster is assumed.</p>
    pub fn set_cluster(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.cluster = input;
        self
    }
    /// <p>You can filter the results of a <code>ListContainerInstances</code> operation with cluster query language statements. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html">Cluster Query Language</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    pub fn filter(mut self, input: impl Into<std::string::String>) -> Self {
        self.filter = Some(input.into());
        self
    }
    /// <p>You can filter the results of a <code>ListContainerInstances</code> operation with cluster query language statements. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html">Cluster Query Language</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    pub fn set_filter(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.filter = input;
        self
    }
    /// <p>The <code>nextToken</code> value returned from a <code>ListContainerInstances</code> request indicating that more results are available to fulfill the request and further calls are needed. If <code>maxResults</code> was provided, it's possible the number of results to be fewer than <code>maxResults</code>.</p> <note>
    /// <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p>
    /// </note>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The <code>nextToken</code> value returned from a <code>ListContainerInstances</code> request indicating that more results are available to fulfill the request and further calls are needed. If <code>maxResults</code> was provided, it's possible the number of results to be fewer than <code>maxResults</code>.</p> <note>
    /// <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p>
    /// </note>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of container instance results that <code>ListContainerInstances</code> returned in paginated output. When this parameter is used, <code>ListContainerInstances</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListContainerInstances</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListContainerInstances</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = Some(input);
        self
    }
    /// <p>The maximum number of container instance results that <code>ListContainerInstances</code> returned in paginated output. When this parameter is used, <code>ListContainerInstances</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListContainerInstances</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListContainerInstances</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>Filters the container instances by status. For example, if you specify the <code>DRAINING</code> status, the results include only container instances that have been set to <code>DRAINING</code> using <code>UpdateContainerInstancesState</code>. If you don't specify this parameter, the default is to include container instances set to all states other than <code>INACTIVE</code>.</p>
    pub fn status(mut self, input: crate::types::ContainerInstanceStatus) -> Self {
        self.status = Some(input);
        self
    }
    /// <p>Filters the container instances by status. For example, if you specify the <code>DRAINING</code> status, the results include only container instances that have been set to <code>DRAINING</code> using <code>UpdateContainerInstancesState</code>. If you don't specify this parameter, the default is to include container instances set to all states other than <code>INACTIVE</code>.</p>
    pub fn set_status(
        mut self,
        input: std::option::Option<crate::types::ContainerInstanceStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// Consumes the builder and constructs a [`ListContainerInstancesInput`](crate::operation::list_container_instances::ListContainerInstancesInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::list_container_instances::ListContainerInstancesInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::list_container_instances::ListContainerInstancesInput {
                cluster: self.cluster,
                filter: self.filter,
                next_token: self.next_token,
                max_results: self.max_results,
                status: self.status,
            },
        )
    }
}
