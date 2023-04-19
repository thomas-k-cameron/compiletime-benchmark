// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_vpc_endpoint_service_configurations::_delete_vpc_endpoint_service_configurations_output::DeleteVpcEndpointServiceConfigurationsOutputBuilder;

pub use crate::operation::delete_vpc_endpoint_service_configurations::_delete_vpc_endpoint_service_configurations_input::DeleteVpcEndpointServiceConfigurationsInputBuilder;

/// Fluent builder constructing a request to `DeleteVpcEndpointServiceConfigurations`.
///
/// <p>Deletes the specified VPC endpoint service configurations. Before you can delete an endpoint service configuration, you must reject any <code>Available</code> or <code>PendingAcceptance</code> interface endpoint connections that are attached to the service.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteVpcEndpointServiceConfigurationsFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::delete_vpc_endpoint_service_configurations::builders::DeleteVpcEndpointServiceConfigurationsInputBuilder
            }
impl DeleteVpcEndpointServiceConfigurationsFluentBuilder {
    /// Creates a new `DeleteVpcEndpointServiceConfigurations`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::delete_vpc_endpoint_service_configurations::DeleteVpcEndpointServiceConfigurations, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::delete_vpc_endpoint_service_configurations::DeleteVpcEndpointServiceConfigurationsError>
    >{
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
                    pub async fn send(self) -> std::result::Result<crate::operation::delete_vpc_endpoint_service_configurations::DeleteVpcEndpointServiceConfigurationsOutput, aws_smithy_http::result::SdkError<crate::operation::delete_vpc_endpoint_service_configurations::DeleteVpcEndpointServiceConfigurationsError>>
                     {
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
    ///     let deserialized_parameters: crate::operation::delete_vpc_endpoint_service_configurations::builders::DeleteVpcEndpointServiceConfigurationsInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.delete_vpc_endpoint_service_configurations().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::delete_vpc_endpoint_service_configurations::builders::DeleteVpcEndpointServiceConfigurationsInputBuilder,
    ) -> Self {
        self.inner = data;
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
    /// Appends an item to `ServiceIds`.
    ///
    /// To override the contents of this collection use [`set_service_ids`](Self::set_service_ids).
    ///
    /// <p>The IDs of the services.</p>
    pub fn service_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.service_ids(input.into());
        self
    }
    /// <p>The IDs of the services.</p>
    pub fn set_service_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_service_ids(input);
        self
    }
}