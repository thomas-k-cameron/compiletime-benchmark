// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::submit_multi_region_access_point_routes::_submit_multi_region_access_point_routes_output::SubmitMultiRegionAccessPointRoutesOutputBuilder;

pub use crate::operation::submit_multi_region_access_point_routes::_submit_multi_region_access_point_routes_input::SubmitMultiRegionAccessPointRoutesInputBuilder;

/// Fluent builder constructing a request to `SubmitMultiRegionAccessPointRoutes`.
///
/// <p>Submits an updated route configuration for a Multi-Region Access Point. This API operation updates the routing status for the specified Regions from active to passive, or from passive to active. A value of <code>0</code> indicates a passive status, which means that traffic won't be routed to the specified Region. A value of <code>100</code> indicates an active status, which means that traffic will be routed to the specified Region. At least one Region must be active at all times.</p>
/// <p>When the routing configuration is changed, any in-progress operations (uploads, copies, deletes, and so on) to formerly active Regions will continue to run to their final completion state (success or failure). The routing configurations of any Regions that aren’t specified remain unchanged.</p> <note>
/// <p>Updated routing configurations might not be immediately applied. It can take up to 2 minutes for your changes to take effect.</p>
/// </note>
/// <p>To submit routing control changes and failover requests, use the Amazon S3 failover control infrastructure endpoints in these five Amazon Web Services Regions:</p>
/// <ul>
/// <li> <p> <code>us-east-1</code> </p> </li>
/// <li> <p> <code>us-west-2</code> </p> </li>
/// <li> <p> <code>ap-southeast-2</code> </p> </li>
/// <li> <p> <code>ap-northeast-1</code> </p> </li>
/// <li> <p> <code>eu-west-1</code> </p> </li>
/// </ul> <note>
/// <p>Your Amazon S3 bucket does not need to be in these five Regions.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct SubmitMultiRegionAccessPointRoutesFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::submit_multi_region_access_point_routes::builders::SubmitMultiRegionAccessPointRoutesInputBuilder
            }
impl SubmitMultiRegionAccessPointRoutesFluentBuilder {
    /// Creates a new `SubmitMultiRegionAccessPointRoutes`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::submit_multi_region_access_point_routes::SubmitMultiRegionAccessPointRoutes, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::submit_multi_region_access_point_routes::SubmitMultiRegionAccessPointRoutesError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::submit_multi_region_access_point_routes::SubmitMultiRegionAccessPointRoutesOutput, aws_smithy_http::result::SdkError<crate::operation::submit_multi_region_access_point_routes::SubmitMultiRegionAccessPointRoutesError>>
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
    ///     let deserialized_parameters: crate::operation::submit_multi_region_access_point_routes::builders::SubmitMultiRegionAccessPointRoutesInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.submit_multi_region_access_point_routes().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::submit_multi_region_access_point_routes::builders::SubmitMultiRegionAccessPointRoutesInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>The Multi-Region Access Point ARN.</p>
    pub fn mrap(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.mrap(input.into());
        self
    }
    /// <p>The Multi-Region Access Point ARN.</p>
    pub fn set_mrap(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_mrap(input);
        self
    }
    /// Appends an item to `RouteUpdates`.
    ///
    /// To override the contents of this collection use [`set_route_updates`](Self::set_route_updates).
    ///
    /// <p>The different routes that make up the new route configuration. Active routes return a value of <code>100</code>, and passive routes return a value of <code>0</code>.</p>
    pub fn route_updates(mut self, input: crate::types::MultiRegionAccessPointRoute) -> Self {
        self.inner = self.inner.route_updates(input);
        self
    }
    /// <p>The different routes that make up the new route configuration. Active routes return a value of <code>100</code>, and passive routes return a value of <code>0</code>.</p>
    pub fn set_route_updates(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::MultiRegionAccessPointRoute>>,
    ) -> Self {
        self.inner = self.inner.set_route_updates(input);
        self
    }
}