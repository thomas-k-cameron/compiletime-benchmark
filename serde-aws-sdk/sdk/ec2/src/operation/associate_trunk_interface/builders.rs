// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_trunk_interface::_associate_trunk_interface_output::AssociateTrunkInterfaceOutputBuilder;

pub use crate::operation::associate_trunk_interface::_associate_trunk_interface_input::AssociateTrunkInterfaceInputBuilder;

/// Fluent builder constructing a request to `AssociateTrunkInterface`.
///
/// <note>
/// <p>This API action is currently in <b>limited preview only</b>. If you are interested in using this feature, contact your account manager.</p>
/// </note>
/// <p>Associates a branch network interface with a trunk network interface.</p>
/// <p>Before you create the association, run the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateNetworkInterface.html">create-network-interface</a> command and set <code>--interface-type</code> to <code>trunk</code>. You must also create a network interface for each branch network interface that you want to associate with the trunk network interface.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct AssociateTrunkInterfaceFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::associate_trunk_interface::builders::AssociateTrunkInterfaceInputBuilder,
}
impl AssociateTrunkInterfaceFluentBuilder {
    /// Creates a new `AssociateTrunkInterface`.
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
            crate::operation::associate_trunk_interface::AssociateTrunkInterface,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::associate_trunk_interface::AssociateTrunkInterfaceError,
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
        crate::operation::associate_trunk_interface::AssociateTrunkInterfaceOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::associate_trunk_interface::AssociateTrunkInterfaceError,
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
    ///     let deserialized_parameters: crate::operation::associate_trunk_interface::builders::AssociateTrunkInterfaceInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.associate_trunk_interface().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::associate_trunk_interface::builders::AssociateTrunkInterfaceInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID of the branch network interface.</p>
    pub fn branch_interface_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.branch_interface_id(input.into());
        self
    }
    /// <p>The ID of the branch network interface.</p>
    pub fn set_branch_interface_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_branch_interface_id(input);
        self
    }
    /// <p>The ID of the trunk network interface.</p>
    pub fn trunk_interface_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.trunk_interface_id(input.into());
        self
    }
    /// <p>The ID of the trunk network interface.</p>
    pub fn set_trunk_interface_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_trunk_interface_id(input);
        self
    }
    /// <p>The ID of the VLAN. This applies to the VLAN protocol.</p>
    pub fn vlan_id(mut self, input: i32) -> Self {
        self.inner = self.inner.vlan_id(input);
        self
    }
    /// <p>The ID of the VLAN. This applies to the VLAN protocol.</p>
    pub fn set_vlan_id(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_vlan_id(input);
        self
    }
    /// <p>The application key. This applies to the GRE protocol.</p>
    pub fn gre_key(mut self, input: i32) -> Self {
        self.inner = self.inner.gre_key(input);
        self
    }
    /// <p>The application key. This applies to the GRE protocol.</p>
    pub fn set_gre_key(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_gre_key(input);
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to Ensure Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to Ensure Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
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
}
