// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_organization_conformance_pack::_delete_organization_conformance_pack_output::DeleteOrganizationConformancePackOutputBuilder;

pub use crate::operation::delete_organization_conformance_pack::_delete_organization_conformance_pack_input::DeleteOrganizationConformancePackInputBuilder;

/// Fluent builder constructing a request to `DeleteOrganizationConformancePack`.
///
/// <p>Deletes the specified organization conformance pack and all of the Config rules and remediation actions from all member accounts in that organization. </p>
/// <p> Only a management account or a delegated administrator account can delete an organization conformance pack. When calling this API with a delegated administrator, you must ensure Organizations <code>ListDelegatedAdministrator</code> permissions are added.</p>
/// <p>Config sets the state of a conformance pack to DELETE_IN_PROGRESS until the deletion is complete. You cannot update a conformance pack while it is in this state. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteOrganizationConformancePackFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::delete_organization_conformance_pack::builders::DeleteOrganizationConformancePackInputBuilder
            }
impl DeleteOrganizationConformancePackFluentBuilder {
    /// Creates a new `DeleteOrganizationConformancePack`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::delete_organization_conformance_pack::DeleteOrganizationConformancePack, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::delete_organization_conformance_pack::DeleteOrganizationConformancePackError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::delete_organization_conformance_pack::DeleteOrganizationConformancePackOutput, aws_smithy_http::result::SdkError<crate::operation::delete_organization_conformance_pack::DeleteOrganizationConformancePackError>>
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
    ///     let deserialized_parameters: crate::operation::delete_organization_conformance_pack::builders::DeleteOrganizationConformancePackInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.delete_organization_conformance_pack().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::delete_organization_conformance_pack::builders::DeleteOrganizationConformancePackInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The name of organization conformance pack that you want to delete.</p>
    pub fn organization_conformance_pack_name(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.organization_conformance_pack_name(input.into());
        self
    }
    /// <p>The name of organization conformance pack that you want to delete.</p>
    pub fn set_organization_conformance_pack_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_organization_conformance_pack_name(input);
        self
    }
}
