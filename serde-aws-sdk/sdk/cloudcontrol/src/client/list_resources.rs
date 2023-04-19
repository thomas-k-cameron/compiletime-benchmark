// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListResources`](crate::operation::list_resources::builders::ListResourcesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_resources::builders::ListResourcesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`type_name(impl Into<String>)`](crate::operation::list_resources::builders::ListResourcesFluentBuilder::type_name) / [`set_type_name(Option<String>)`](crate::operation::list_resources::builders::ListResourcesFluentBuilder::set_type_name): <p>The name of the resource type.</p>
    ///   - [`type_version_id(impl Into<String>)`](crate::operation::list_resources::builders::ListResourcesFluentBuilder::type_version_id) / [`set_type_version_id(Option<String>)`](crate::operation::list_resources::builders::ListResourcesFluentBuilder::set_type_version_id): <p>For private resource types, the type version to use in this resource operation. If you do not specify a resource version, CloudFormation uses the default version.</p>
    ///   - [`role_arn(impl Into<String>)`](crate::operation::list_resources::builders::ListResourcesFluentBuilder::role_arn) / [`set_role_arn(Option<String>)`](crate::operation::list_resources::builders::ListResourcesFluentBuilder::set_role_arn): <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role for Cloud Control API to use when performing this resource operation. The role specified must have the permissions required for this operation. The necessary permissions for each event handler are defined in the <code> <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-schema.html#schema-properties-handlers">handlers</a> </code> section of the <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-schema.html">resource type definition schema</a>.</p>  <p>If you do not specify a role, Cloud Control API uses a temporary session created using your Amazon Web Services user credentials.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations.html#resource-operations-permissions">Specifying credentials</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_resources::builders::ListResourcesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_resources::builders::ListResourcesFluentBuilder::set_next_token): <p>If the previous paginated request didn't return all of the remaining results, the response object's <code>NextToken</code> parameter value is set to a token. To retrieve the next set of results, call this action again and assign that token to the request object's <code>NextToken</code> parameter. If there are no remaining results, the previous response object's <code>NextToken</code> parameter is set to <code>null</code>.</p>
    ///   - [`max_results(i32)`](crate::operation::list_resources::builders::ListResourcesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_resources::builders::ListResourcesFluentBuilder::set_max_results): <p>Reserved.</p>
    ///   - [`resource_model(impl Into<String>)`](crate::operation::list_resources::builders::ListResourcesFluentBuilder::resource_model) / [`set_resource_model(Option<String>)`](crate::operation::list_resources::builders::ListResourcesFluentBuilder::set_resource_model): <p>The resource model to use to select the resources to return.</p>
    /// - On success, responds with [`ListResourcesOutput`](crate::operation::list_resources::ListResourcesOutput) with field(s):
    ///   - [`type_name(Option<String>)`](crate::operation::list_resources::ListResourcesOutput::type_name): <p>The name of the resource type.</p>
    ///   - [`resource_descriptions(Option<Vec<ResourceDescription>>)`](crate::operation::list_resources::ListResourcesOutput::resource_descriptions): <p>Information about the specified resources, including primary identifier and resource model.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_resources::ListResourcesOutput::next_token): <p>If the request doesn't return all of the remaining results, <code>NextToken</code> is set to a token. To retrieve the next set of results, call <code>ListResources</code> again and assign that token to the request object's <code>NextToken</code> parameter. If the request returns all results, <code>NextToken</code> is set to null.</p>
    /// - On failure, responds with [`SdkError<ListResourcesError>`](crate::operation::list_resources::ListResourcesError)
    pub fn list_resources(
        &self,
    ) -> crate::operation::list_resources::builders::ListResourcesFluentBuilder {
        crate::operation::list_resources::builders::ListResourcesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}