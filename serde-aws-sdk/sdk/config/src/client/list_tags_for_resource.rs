// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTagsForResource`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::set_resource_arn): <p>The Amazon Resource Name (ARN) that identifies the resource for which to list the tags. Currently, the supported resources are <code>ConfigRule</code>, <code>ConfigurationAggregator</code> and <code>AggregatorAuthorization</code>.</p>
    ///   - [`limit(i32)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::limit) / [`set_limit(i32)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::set_limit): <p>The maximum number of tags returned on each page. The limit maximum is 50. You cannot specify a number greater than 50. If you specify 0, Config uses the default. </p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::set_next_token): <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    /// - On success, responds with [`ListTagsForResourceOutput`](crate::operation::list_tags_for_resource::ListTagsForResourceOutput) with field(s):
    ///   - [`tags(Option<Vec<Tag>>)`](crate::operation::list_tags_for_resource::ListTagsForResourceOutput::tags): <p>The tags for the resource.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_tags_for_resource::ListTagsForResourceOutput::next_token): <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    /// - On failure, responds with [`SdkError<ListTagsForResourceError>`](crate::operation::list_tags_for_resource::ListTagsForResourceError)
    pub fn list_tags_for_resource(
        &self,
    ) -> crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder {
        crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::new(
            self.handle.clone(),
        )
    }
}