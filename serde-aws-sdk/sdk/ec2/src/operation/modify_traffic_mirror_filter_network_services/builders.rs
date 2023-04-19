// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_traffic_mirror_filter_network_services::_modify_traffic_mirror_filter_network_services_output::ModifyTrafficMirrorFilterNetworkServicesOutputBuilder;

pub use crate::operation::modify_traffic_mirror_filter_network_services::_modify_traffic_mirror_filter_network_services_input::ModifyTrafficMirrorFilterNetworkServicesInputBuilder;

/// Fluent builder constructing a request to `ModifyTrafficMirrorFilterNetworkServices`.
///
/// <p>Allows or restricts mirroring network services.</p>
/// <p> By default, Amazon DNS network services are not eligible for Traffic Mirror. Use <code>AddNetworkServices</code> to add network services to a Traffic Mirror filter. When a network service is added to the Traffic Mirror filter, all traffic related to that network service will be mirrored. When you no longer want to mirror network services, use <code>RemoveNetworkServices</code> to remove the network services from the Traffic Mirror filter. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ModifyTrafficMirrorFilterNetworkServicesFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::modify_traffic_mirror_filter_network_services::builders::ModifyTrafficMirrorFilterNetworkServicesInputBuilder
            }
impl ModifyTrafficMirrorFilterNetworkServicesFluentBuilder {
    /// Creates a new `ModifyTrafficMirrorFilterNetworkServices`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServices, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServicesError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServicesOutput, aws_smithy_http::result::SdkError<crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServicesError>>
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
    ///     let deserialized_parameters: crate::operation::modify_traffic_mirror_filter_network_services::builders::ModifyTrafficMirrorFilterNetworkServicesInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.modify_traffic_mirror_filter_network_services().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::modify_traffic_mirror_filter_network_services::builders::ModifyTrafficMirrorFilterNetworkServicesInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID of the Traffic Mirror filter.</p>
    pub fn traffic_mirror_filter_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.traffic_mirror_filter_id(input.into());
        self
    }
    /// <p>The ID of the Traffic Mirror filter.</p>
    pub fn set_traffic_mirror_filter_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_traffic_mirror_filter_id(input);
        self
    }
    /// Appends an item to `AddNetworkServices`.
    ///
    /// To override the contents of this collection use [`set_add_network_services`](Self::set_add_network_services).
    ///
    /// <p>The network service, for example Amazon DNS, that you want to mirror.</p>
    pub fn add_network_services(
        mut self,
        input: crate::types::TrafficMirrorNetworkService,
    ) -> Self {
        self.inner = self.inner.add_network_services(input);
        self
    }
    /// <p>The network service, for example Amazon DNS, that you want to mirror.</p>
    pub fn set_add_network_services(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TrafficMirrorNetworkService>>,
    ) -> Self {
        self.inner = self.inner.set_add_network_services(input);
        self
    }
    /// Appends an item to `RemoveNetworkServices`.
    ///
    /// To override the contents of this collection use [`set_remove_network_services`](Self::set_remove_network_services).
    ///
    /// <p>The network service, for example Amazon DNS, that you no longer want to mirror.</p>
    pub fn remove_network_services(
        mut self,
        input: crate::types::TrafficMirrorNetworkService,
    ) -> Self {
        self.inner = self.inner.remove_network_services(input);
        self
    }
    /// <p>The network service, for example Amazon DNS, that you no longer want to mirror.</p>
    pub fn set_remove_network_services(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TrafficMirrorNetworkService>>,
    ) -> Self {
        self.inner = self.inner.set_remove_network_services(input);
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
