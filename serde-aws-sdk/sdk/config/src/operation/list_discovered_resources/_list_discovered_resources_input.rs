// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p></p>
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
pub struct ListDiscoveredResourcesInput {
    /// <p>The type of resources that you want Config to list in the response.</p>
    #[doc(hidden)]
    pub resource_type: std::option::Option<crate::types::ResourceType>,
    /// <p>The IDs of only those resources that you want Config to list in the response. If you do not specify this parameter, Config lists all resources of the specified type that it has discovered.</p>
    #[doc(hidden)]
    pub resource_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The custom name of only those resources that you want Config to list in the response. If you do not specify this parameter, Config lists all resources of the specified type that it has discovered.</p>
    #[doc(hidden)]
    pub resource_name: std::option::Option<std::string::String>,
    /// <p>The maximum number of resource identifiers returned on each page. The default is 100. You cannot specify a number greater than 100. If you specify 0, Config uses the default.</p>
    #[doc(hidden)]
    pub limit: i32,
    /// <p>Specifies whether Config includes deleted resources in the results. By default, deleted resources are not included.</p>
    #[doc(hidden)]
    pub include_deleted_resources: bool,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListDiscoveredResourcesInput {
    /// <p>The type of resources that you want Config to list in the response.</p>
    pub fn resource_type(&self) -> std::option::Option<&crate::types::ResourceType> {
        self.resource_type.as_ref()
    }
    /// <p>The IDs of only those resources that you want Config to list in the response. If you do not specify this parameter, Config lists all resources of the specified type that it has discovered.</p>
    pub fn resource_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.resource_ids.as_deref()
    }
    /// <p>The custom name of only those resources that you want Config to list in the response. If you do not specify this parameter, Config lists all resources of the specified type that it has discovered.</p>
    pub fn resource_name(&self) -> std::option::Option<&str> {
        self.resource_name.as_deref()
    }
    /// <p>The maximum number of resource identifiers returned on each page. The default is 100. You cannot specify a number greater than 100. If you specify 0, Config uses the default.</p>
    pub fn limit(&self) -> i32 {
        self.limit
    }
    /// <p>Specifies whether Config includes deleted resources in the results. By default, deleted resources are not included.</p>
    pub fn include_deleted_resources(&self) -> bool {
        self.include_deleted_resources
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListDiscoveredResourcesInput {
    /// Creates a new builder-style object to manufacture [`ListDiscoveredResourcesInput`](crate::operation::list_discovered_resources::ListDiscoveredResourcesInput).
    pub fn builder(
    ) -> crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesInputBuilder
    {
        crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_discovered_resources::ListDiscoveredResourcesInput;
/// A builder for [`ListDiscoveredResourcesInput`](crate::operation::list_discovered_resources::ListDiscoveredResourcesInput).
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
pub struct ListDiscoveredResourcesInputBuilder {
    pub(crate) resource_type: std::option::Option<crate::types::ResourceType>,
    pub(crate) resource_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) resource_name: std::option::Option<std::string::String>,
    pub(crate) limit: std::option::Option<i32>,
    pub(crate) include_deleted_resources: std::option::Option<bool>,
    pub(crate) next_token: std::option::Option<std::string::String>,
}
impl ListDiscoveredResourcesInputBuilder {
    /// <p>The type of resources that you want Config to list in the response.</p>
    pub fn resource_type(mut self, input: crate::types::ResourceType) -> Self {
        self.resource_type = Some(input);
        self
    }
    /// <p>The type of resources that you want Config to list in the response.</p>
    pub fn set_resource_type(
        mut self,
        input: std::option::Option<crate::types::ResourceType>,
    ) -> Self {
        self.resource_type = input;
        self
    }
    /// Appends an item to `resource_ids`.
    ///
    /// To override the contents of this collection use [`set_resource_ids`](Self::set_resource_ids).
    ///
    /// <p>The IDs of only those resources that you want Config to list in the response. If you do not specify this parameter, Config lists all resources of the specified type that it has discovered.</p>
    pub fn resource_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.resource_ids.unwrap_or_default();
        v.push(input.into());
        self.resource_ids = Some(v);
        self
    }
    /// <p>The IDs of only those resources that you want Config to list in the response. If you do not specify this parameter, Config lists all resources of the specified type that it has discovered.</p>
    pub fn set_resource_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.resource_ids = input;
        self
    }
    /// <p>The custom name of only those resources that you want Config to list in the response. If you do not specify this parameter, Config lists all resources of the specified type that it has discovered.</p>
    pub fn resource_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.resource_name = Some(input.into());
        self
    }
    /// <p>The custom name of only those resources that you want Config to list in the response. If you do not specify this parameter, Config lists all resources of the specified type that it has discovered.</p>
    pub fn set_resource_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.resource_name = input;
        self
    }
    /// <p>The maximum number of resource identifiers returned on each page. The default is 100. You cannot specify a number greater than 100. If you specify 0, Config uses the default.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.limit = Some(input);
        self
    }
    /// <p>The maximum number of resource identifiers returned on each page. The default is 100. You cannot specify a number greater than 100. If you specify 0, Config uses the default.</p>
    pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.limit = input;
        self
    }
    /// <p>Specifies whether Config includes deleted resources in the results. By default, deleted resources are not included.</p>
    pub fn include_deleted_resources(mut self, input: bool) -> Self {
        self.include_deleted_resources = Some(input);
        self
    }
    /// <p>Specifies whether Config includes deleted resources in the results. By default, deleted resources are not included.</p>
    pub fn set_include_deleted_resources(mut self, input: std::option::Option<bool>) -> Self {
        self.include_deleted_resources = input;
        self
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ListDiscoveredResourcesInput`](crate::operation::list_discovered_resources::ListDiscoveredResourcesInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::list_discovered_resources::ListDiscoveredResourcesInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::list_discovered_resources::ListDiscoveredResourcesInput {
                resource_type: self.resource_type,
                resource_ids: self.resource_ids,
                resource_name: self.resource_name,
                limit: self.limit.unwrap_or_default(),
                include_deleted_resources: self.include_deleted_resources.unwrap_or_default(),
                next_token: self.next_token,
            },
        )
    }
}
