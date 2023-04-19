// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_continuous_backups::_update_continuous_backups_output::UpdateContinuousBackupsOutputBuilder;

pub use crate::operation::update_continuous_backups::_update_continuous_backups_input::UpdateContinuousBackupsInputBuilder;

/// Fluent builder constructing a request to `UpdateContinuousBackups`.
///
/// <p> <code>UpdateContinuousBackups</code> enables or disables point in time recovery for the specified table. A successful <code>UpdateContinuousBackups</code> call returns the current <code>ContinuousBackupsDescription</code>. Continuous backups are <code>ENABLED</code> on all tables at table creation. If point in time recovery is enabled, <code>PointInTimeRecoveryStatus</code> will be set to ENABLED.</p>
/// <p> Once continuous backups and point in time recovery are enabled, you can restore to any point in time within <code>EarliestRestorableDateTime</code> and <code>LatestRestorableDateTime</code>. </p>
/// <p> <code>LatestRestorableDateTime</code> is typically 5 minutes before the current time. You can restore your table to any point in time during the last 35 days. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateContinuousBackupsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::update_continuous_backups::builders::UpdateContinuousBackupsInputBuilder,
}
impl UpdateContinuousBackupsFluentBuilder {
    /// Creates a new `UpdateContinuousBackups`.
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
            crate::operation::update_continuous_backups::UpdateContinuousBackups,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_continuous_backups::UpdateContinuousBackupsError,
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
        crate::operation::update_continuous_backups::UpdateContinuousBackupsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_continuous_backups::UpdateContinuousBackupsError,
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
    ///     let deserialized_parameters: crate::operation::update_continuous_backups::builders::UpdateContinuousBackupsInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.update_continuous_backups().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::update_continuous_backups::builders::UpdateContinuousBackupsInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The name of the table.</p>
    pub fn table_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>The name of the table.</p>
    pub fn set_table_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
    /// <p>Represents the settings used to enable point in time recovery.</p>
    pub fn point_in_time_recovery_specification(
        mut self,
        input: crate::types::PointInTimeRecoverySpecification,
    ) -> Self {
        self.inner = self.inner.point_in_time_recovery_specification(input);
        self
    }
    /// <p>Represents the settings used to enable point in time recovery.</p>
    pub fn set_point_in_time_recovery_specification(
        mut self,
        input: std::option::Option<crate::types::PointInTimeRecoverySpecification>,
    ) -> Self {
        self.inner = self.inner.set_point_in_time_recovery_specification(input);
        self
    }
}
