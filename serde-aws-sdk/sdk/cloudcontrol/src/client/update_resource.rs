// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateResource`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`type_name(impl Into<String>)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::type_name) / [`set_type_name(Option<String>)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::set_type_name): <p>The name of the resource type.</p>
    ///   - [`type_version_id(impl Into<String>)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::type_version_id) / [`set_type_version_id(Option<String>)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::set_type_version_id): <p>For private resource types, the type version to use in this resource operation. If you do not specify a resource version, CloudFormation uses the default version.</p>
    ///   - [`role_arn(impl Into<String>)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::role_arn) / [`set_role_arn(Option<String>)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::set_role_arn): <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role for Cloud Control API to use when performing this resource operation. The role specified must have the permissions required for this operation. The necessary permissions for each event handler are defined in the <code> <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-schema.html#schema-properties-handlers">handlers</a> </code> section of the <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-schema.html">resource type definition schema</a>.</p>  <p>If you do not specify a role, Cloud Control API uses a temporary session created using your Amazon Web Services user credentials.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations.html#resource-operations-permissions">Specifying credentials</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    ///   - [`client_token(impl Into<String>)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::set_client_token): <p>A unique identifier to ensure the idempotency of the resource request. As a best practice, specify this token to ensure idempotency, so that Amazon Web Services Cloud Control API can accurately distinguish between request retries and new resource requests. You might retry a resource request to ensure that it was successfully received.</p>  <p>A client token is valid for 36 hours once used. After that, a resource request with the same client token is treated as a new request.</p>  <p>If you do not specify a client token, one is generated for inclusion in the request.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations.html#resource-operations-idempotency">Ensuring resource operation requests are unique</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    ///   - [`identifier(impl Into<String>)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::identifier) / [`set_identifier(Option<String>)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::set_identifier): <p>The identifier for the resource.</p>  <p>You can specify the primary identifier, or any secondary identifier defined for the resource type in its resource schema. You can only specify one identifier. Primary identifiers can be specified as a string or JSON; secondary identifiers must be specified as JSON.</p>  <p>For compound primary identifiers (that is, one that consists of multiple resource properties strung together), to specify the primary identifier as a string, list the property values <i>in the order they are specified</i> in the primary identifier definition, separated by <code>|</code>.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-identifier.html">Identifying resources</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    ///   - [`patch_document(impl Into<String>)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::patch_document) / [`set_patch_document(Option<String>)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::set_patch_document): <p>A JavaScript Object Notation (JSON) document listing the patch operations that represent the updates to apply to the current resource properties. For details, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations-update.html#resource-operations-update-patch">Composing the patch document</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    /// - On success, responds with [`UpdateResourceOutput`](crate::operation::update_resource::UpdateResourceOutput) with field(s):
    ///   - [`progress_event(Option<ProgressEvent>)`](crate::operation::update_resource::UpdateResourceOutput::progress_event): <p>Represents the current status of the resource update request.</p>  <p>Use the <code>RequestToken</code> of the <code>ProgressEvent</code> with <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/APIReference/API_GetResourceRequestStatus.html">GetResourceRequestStatus</a> to return the current status of a resource operation request.</p>
    /// - On failure, responds with [`SdkError<UpdateResourceError>`](crate::operation::update_resource::UpdateResourceError)
    pub fn update_resource(
        &self,
    ) -> crate::operation::update_resource::builders::UpdateResourceFluentBuilder {
        crate::operation::update_resource::builders::UpdateResourceFluentBuilder::new(
            self.handle.clone(),
        )
    }
}