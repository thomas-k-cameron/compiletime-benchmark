// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_traffic_mirror_target::_create_traffic_mirror_target_output::CreateTrafficMirrorTargetOutputBuilder;

pub use crate::operation::create_traffic_mirror_target::_create_traffic_mirror_target_input::CreateTrafficMirrorTargetInputBuilder;

/// Fluent builder constructing a request to `CreateTrafficMirrorTarget`.
///
/// <p>Creates a target for your Traffic Mirror session.</p>
/// <p>A Traffic Mirror target is the destination for mirrored traffic. The Traffic Mirror source and the Traffic Mirror target (monitoring appliances) can be in the same VPC, or in different VPCs connected via VPC peering or a transit gateway.</p>
/// <p>A Traffic Mirror target can be a network interface, a Network Load Balancer, or a Gateway Load Balancer endpoint.</p>
/// <p>To use the target in a Traffic Mirror session, use <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateTrafficMirrorSession.htm">CreateTrafficMirrorSession</a>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateTrafficMirrorTargetFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::create_traffic_mirror_target::builders::CreateTrafficMirrorTargetInputBuilder
            }
impl CreateTrafficMirrorTargetFluentBuilder {
    /// Creates a new `CreateTrafficMirrorTarget`.
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
            crate::operation::create_traffic_mirror_target::CreateTrafficMirrorTarget,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_traffic_mirror_target::CreateTrafficMirrorTargetError,
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
        crate::operation::create_traffic_mirror_target::CreateTrafficMirrorTargetOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_traffic_mirror_target::CreateTrafficMirrorTargetError,
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
    ///     let deserialized_parameters: crate::operation::create_traffic_mirror_target::builders::CreateTrafficMirrorTargetInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.create_traffic_mirror_target().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::create_traffic_mirror_target::builders::CreateTrafficMirrorTargetInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The network interface ID that is associated with the target.</p>
    pub fn network_interface_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.network_interface_id(input.into());
        self
    }
    /// <p>The network interface ID that is associated with the target.</p>
    pub fn set_network_interface_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_network_interface_id(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Network Load Balancer that is associated with the target.</p>
    pub fn network_load_balancer_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.network_load_balancer_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Network Load Balancer that is associated with the target.</p>
    pub fn set_network_load_balancer_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_network_load_balancer_arn(input);
        self
    }
    /// <p>The description of the Traffic Mirror target.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the Traffic Mirror target.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to assign to the Traffic Mirror target.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to assign to the Traffic Mirror target.</p>
    pub fn set_tag_specifications(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
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
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The ID of the Gateway Load Balancer endpoint.</p>
    pub fn gateway_load_balancer_endpoint_id(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.gateway_load_balancer_endpoint_id(input.into());
        self
    }
    /// <p>The ID of the Gateway Load Balancer endpoint.</p>
    pub fn set_gateway_load_balancer_endpoint_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_gateway_load_balancer_endpoint_id(input);
        self
    }
}
