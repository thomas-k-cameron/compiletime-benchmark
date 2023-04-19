// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_ipam_resource_discovery::_modify_ipam_resource_discovery_output::ModifyIpamResourceDiscoveryOutputBuilder;

pub use crate::operation::modify_ipam_resource_discovery::_modify_ipam_resource_discovery_input::ModifyIpamResourceDiscoveryInputBuilder;

/// Fluent builder constructing a request to `ModifyIpamResourceDiscovery`.
///
/// <p>Modifies a resource discovery. A resource discovery is an IPAM component that enables IPAM to manage and monitor resources that belong to the owning account.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ModifyIpamResourceDiscoveryFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::modify_ipam_resource_discovery::builders::ModifyIpamResourceDiscoveryInputBuilder
            }
impl ModifyIpamResourceDiscoveryFluentBuilder {
    /// Creates a new `ModifyIpamResourceDiscovery`.
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
            crate::operation::modify_ipam_resource_discovery::ModifyIpamResourceDiscovery,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::modify_ipam_resource_discovery::ModifyIpamResourceDiscoveryError,
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
        crate::operation::modify_ipam_resource_discovery::ModifyIpamResourceDiscoveryOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::modify_ipam_resource_discovery::ModifyIpamResourceDiscoveryError,
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
    ///     let deserialized_parameters: crate::operation::modify_ipam_resource_discovery::builders::ModifyIpamResourceDiscoveryInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.modify_ipam_resource_discovery().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::modify_ipam_resource_discovery::builders::ModifyIpamResourceDiscoveryInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>A resource discovery ID.</p>
    pub fn ipam_resource_discovery_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.ipam_resource_discovery_id(input.into());
        self
    }
    /// <p>A resource discovery ID.</p>
    pub fn set_ipam_resource_discovery_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_ipam_resource_discovery_id(input);
        self
    }
    /// <p>A resource discovery description.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A resource discovery description.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// Appends an item to `AddOperatingRegions`.
    ///
    /// To override the contents of this collection use [`set_add_operating_regions`](Self::set_add_operating_regions).
    ///
    /// <p>Add operating Regions to the resource discovery. Operating Regions are Amazon Web Services Regions where the IPAM is allowed to manage IP address CIDRs. IPAM only discovers and monitors resources in the Amazon Web Services Regions you select as operating Regions.</p>
    pub fn add_operating_regions(mut self, input: crate::types::AddIpamOperatingRegion) -> Self {
        self.inner = self.inner.add_operating_regions(input);
        self
    }
    /// <p>Add operating Regions to the resource discovery. Operating Regions are Amazon Web Services Regions where the IPAM is allowed to manage IP address CIDRs. IPAM only discovers and monitors resources in the Amazon Web Services Regions you select as operating Regions.</p>
    pub fn set_add_operating_regions(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::AddIpamOperatingRegion>>,
    ) -> Self {
        self.inner = self.inner.set_add_operating_regions(input);
        self
    }
    /// Appends an item to `RemoveOperatingRegions`.
    ///
    /// To override the contents of this collection use [`set_remove_operating_regions`](Self::set_remove_operating_regions).
    ///
    /// <p>Remove operating Regions.</p>
    pub fn remove_operating_regions(
        mut self,
        input: crate::types::RemoveIpamOperatingRegion,
    ) -> Self {
        self.inner = self.inner.remove_operating_regions(input);
        self
    }
    /// <p>Remove operating Regions.</p>
    pub fn set_remove_operating_regions(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::RemoveIpamOperatingRegion>>,
    ) -> Self {
        self.inner = self.inner.set_remove_operating_regions(input);
        self
    }
}
