// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_instance_export_task::_create_instance_export_task_output::CreateInstanceExportTaskOutputBuilder;

pub use crate::operation::create_instance_export_task::_create_instance_export_task_input::CreateInstanceExportTaskInputBuilder;

/// Fluent builder constructing a request to `CreateInstanceExportTask`.
///
/// <p>Exports a running or stopped instance to an Amazon S3 bucket.</p>
/// <p>For information about the supported operating systems, image formats, and known limitations for the types of instances you can export, see <a href="https://docs.aws.amazon.com/vm-import/latest/userguide/vmexport.html">Exporting an instance as a VM Using VM Import/Export</a> in the <i>VM Import/Export User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateInstanceExportTaskFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::create_instance_export_task::builders::CreateInstanceExportTaskInputBuilder
            }
impl CreateInstanceExportTaskFluentBuilder {
    /// Creates a new `CreateInstanceExportTask`.
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
            crate::operation::create_instance_export_task::CreateInstanceExportTask,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_instance_export_task::CreateInstanceExportTaskError,
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
        crate::operation::create_instance_export_task::CreateInstanceExportTaskOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_instance_export_task::CreateInstanceExportTaskError,
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
    ///     let deserialized_parameters: crate::operation::create_instance_export_task::builders::CreateInstanceExportTaskInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.create_instance_export_task().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::create_instance_export_task::builders::CreateInstanceExportTaskInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>A description for the conversion task or the resource being exported. The maximum length is 255 characters.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description for the conversion task or the resource being exported. The maximum length is 255 characters.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The format and location for an export instance task.</p>
    pub fn export_to_s3_task(mut self, input: crate::types::ExportToS3TaskSpecification) -> Self {
        self.inner = self.inner.export_to_s3_task(input);
        self
    }
    /// <p>The format and location for an export instance task.</p>
    pub fn set_export_to_s3_task(
        mut self,
        input: std::option::Option<crate::types::ExportToS3TaskSpecification>,
    ) -> Self {
        self.inner = self.inner.set_export_to_s3_task(input);
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The target virtualization environment.</p>
    pub fn target_environment(mut self, input: crate::types::ExportEnvironment) -> Self {
        self.inner = self.inner.target_environment(input);
        self
    }
    /// <p>The target virtualization environment.</p>
    pub fn set_target_environment(
        mut self,
        input: std::option::Option<crate::types::ExportEnvironment>,
    ) -> Self {
        self.inner = self.inner.set_target_environment(input);
        self
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the export instance task during creation.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the export instance task during creation.</p>
    pub fn set_tag_specifications(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
}
