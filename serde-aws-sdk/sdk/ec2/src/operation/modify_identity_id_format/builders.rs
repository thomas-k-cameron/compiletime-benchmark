// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_identity_id_format::_modify_identity_id_format_output::ModifyIdentityIdFormatOutputBuilder;

pub use crate::operation::modify_identity_id_format::_modify_identity_id_format_input::ModifyIdentityIdFormatInputBuilder;

/// Fluent builder constructing a request to `ModifyIdentityIdFormat`.
///
/// <p>Modifies the ID format of a resource for a specified IAM user, IAM role, or the root user for an account; or all IAM users, IAM roles, and the root user for an account. You can specify that resources should receive longer IDs (17-character IDs) when they are created. </p>
/// <p>This request can only be used to modify longer ID settings for resource types that are within the opt-in period. Resources currently in their opt-in period include: <code>bundle</code> | <code>conversion-task</code> | <code>customer-gateway</code> | <code>dhcp-options</code> | <code>elastic-ip-allocation</code> | <code>elastic-ip-association</code> | <code>export-task</code> | <code>flow-log</code> | <code>image</code> | <code>import-task</code> | <code>internet-gateway</code> | <code>network-acl</code> | <code>network-acl-association</code> | <code>network-interface</code> | <code>network-interface-attachment</code> | <code>prefix-list</code> | <code>route-table</code> | <code>route-table-association</code> | <code>security-group</code> | <code>subnet</code> | <code>subnet-cidr-block-association</code> | <code>vpc</code> | <code>vpc-cidr-block-association</code> | <code>vpc-endpoint</code> | <code>vpc-peering-connection</code> | <code>vpn-connection</code> | <code>vpn-gateway</code>. </p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/resource-ids.html">Resource IDs</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>. </p>
/// <p>This setting applies to the principal specified in the request; it does not apply to the principal that makes the request. </p>
/// <p>Resources created with longer IDs are visible to all IAM roles and users, regardless of these settings and provided that they have permission to use the relevant <code>Describe</code> command for the resource type.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ModifyIdentityIdFormatFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::modify_identity_id_format::builders::ModifyIdentityIdFormatInputBuilder,
}
impl ModifyIdentityIdFormatFluentBuilder {
    /// Creates a new `ModifyIdentityIdFormat`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::modify_identity_id_format::ModifyIdentityIdFormat,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::modify_identity_id_format::ModifyIdentityIdFormatError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::modify_identity_id_format::ModifyIdentityIdFormatOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::modify_identity_id_format::ModifyIdentityIdFormatError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    #[cfg(aws_sdk_unstable)]
    /// This function replaces the parameter with new one.
    /// It is useful when you want to replace the existing data with de-serialized data.
    /// ```compile_fail
    /// let result_future = async {
    ///     let deserialized_parameters: crate::operation::modify_identity_id_format::builders::ModifyIdentityIdFormatInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.modify_identity_id_format().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::modify_identity_id_format::builders::ModifyIdentityIdFormatInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ARN of the principal, which can be an IAM user, IAM role, or the root user. Specify <code>all</code> to modify the ID format for all IAM users, IAM roles, and the root user of the account.</p>
    pub fn principal_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.principal_arn(input.into());
        self
    }
    /// <p>The ARN of the principal, which can be an IAM user, IAM role, or the root user. Specify <code>all</code> to modify the ID format for all IAM users, IAM roles, and the root user of the account.</p>
    pub fn set_principal_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_principal_arn(input);
        self
    }
    /// <p>The type of resource: <code>bundle</code> | <code>conversion-task</code> | <code>customer-gateway</code> | <code>dhcp-options</code> | <code>elastic-ip-allocation</code> | <code>elastic-ip-association</code> | <code>export-task</code> | <code>flow-log</code> | <code>image</code> | <code>import-task</code> | <code>internet-gateway</code> | <code>network-acl</code> | <code>network-acl-association</code> | <code>network-interface</code> | <code>network-interface-attachment</code> | <code>prefix-list</code> | <code>route-table</code> | <code>route-table-association</code> | <code>security-group</code> | <code>subnet</code> | <code>subnet-cidr-block-association</code> | <code>vpc</code> | <code>vpc-cidr-block-association</code> | <code>vpc-endpoint</code> | <code>vpc-peering-connection</code> | <code>vpn-connection</code> | <code>vpn-gateway</code>.</p>
    /// <p>Alternatively, use the <code>all-current</code> option to include all resource types that are currently within their opt-in period for longer IDs.</p>
    pub fn resource(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.resource(input.into());
        self
    }
    /// <p>The type of resource: <code>bundle</code> | <code>conversion-task</code> | <code>customer-gateway</code> | <code>dhcp-options</code> | <code>elastic-ip-allocation</code> | <code>elastic-ip-association</code> | <code>export-task</code> | <code>flow-log</code> | <code>image</code> | <code>import-task</code> | <code>internet-gateway</code> | <code>network-acl</code> | <code>network-acl-association</code> | <code>network-interface</code> | <code>network-interface-attachment</code> | <code>prefix-list</code> | <code>route-table</code> | <code>route-table-association</code> | <code>security-group</code> | <code>subnet</code> | <code>subnet-cidr-block-association</code> | <code>vpc</code> | <code>vpc-cidr-block-association</code> | <code>vpc-endpoint</code> | <code>vpc-peering-connection</code> | <code>vpn-connection</code> | <code>vpn-gateway</code>.</p>
    /// <p>Alternatively, use the <code>all-current</code> option to include all resource types that are currently within their opt-in period for longer IDs.</p>
    pub fn set_resource(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_resource(input);
        self
    }
    /// <p>Indicates whether the resource should use longer IDs (17-character IDs)</p>
    pub fn use_long_ids(mut self, input: bool) -> Self {
        self.inner = self.inner.use_long_ids(input);
        self
    }
    /// <p>Indicates whether the resource should use longer IDs (17-character IDs)</p>
    pub fn set_use_long_ids(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_use_long_ids(input);
        self
    }
}
