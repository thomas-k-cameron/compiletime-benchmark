// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::replace_network_acl_association::_replace_network_acl_association_output::ReplaceNetworkAclAssociationOutputBuilder;

pub use crate::operation::replace_network_acl_association::_replace_network_acl_association_input::ReplaceNetworkAclAssociationInputBuilder;

/// Fluent builder constructing a request to `ReplaceNetworkAclAssociation`.
///
/// <p>Changes which network ACL a subnet is associated with. By default when you create a subnet, it's automatically associated with the default network ACL. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/VPC_ACLs.html">Network ACLs</a> in the <i>Amazon Virtual Private Cloud User Guide</i>.</p>
/// <p>This is an idempotent operation.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ReplaceNetworkAclAssociationFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::replace_network_acl_association::builders::ReplaceNetworkAclAssociationInputBuilder
            }
impl ReplaceNetworkAclAssociationFluentBuilder {
    /// Creates a new `ReplaceNetworkAclAssociation`.
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
            crate::operation::replace_network_acl_association::ReplaceNetworkAclAssociation,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::replace_network_acl_association::ReplaceNetworkAclAssociationError,
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
        crate::operation::replace_network_acl_association::ReplaceNetworkAclAssociationOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::replace_network_acl_association::ReplaceNetworkAclAssociationError,
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
    ///     let deserialized_parameters: crate::operation::replace_network_acl_association::builders::ReplaceNetworkAclAssociationInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.replace_network_acl_association().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::replace_network_acl_association::builders::ReplaceNetworkAclAssociationInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID of the current association between the original network ACL and the subnet.</p>
    pub fn association_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.association_id(input.into());
        self
    }
    /// <p>The ID of the current association between the original network ACL and the subnet.</p>
    pub fn set_association_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_association_id(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>The ID of the new network ACL to associate with the subnet.</p>
    pub fn network_acl_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.network_acl_id(input.into());
        self
    }
    /// <p>The ID of the new network ACL to associate with the subnet.</p>
    pub fn set_network_acl_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_network_acl_id(input);
        self
    }
}
