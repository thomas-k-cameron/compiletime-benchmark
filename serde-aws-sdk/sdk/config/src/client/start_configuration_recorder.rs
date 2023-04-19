// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartConfigurationRecorder`](crate::operation::start_configuration_recorder::builders::StartConfigurationRecorderFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`configuration_recorder_name(impl Into<String>)`](crate::operation::start_configuration_recorder::builders::StartConfigurationRecorderFluentBuilder::configuration_recorder_name) / [`set_configuration_recorder_name(Option<String>)`](crate::operation::start_configuration_recorder::builders::StartConfigurationRecorderFluentBuilder::set_configuration_recorder_name): <p>The name of the recorder object that records each configuration change made to the resources.</p>
    /// - On success, responds with [`StartConfigurationRecorderOutput`](crate::operation::start_configuration_recorder::StartConfigurationRecorderOutput)
    /// - On failure, responds with [`SdkError<StartConfigurationRecorderError>`](crate::operation::start_configuration_recorder::StartConfigurationRecorderError)
    pub fn start_configuration_recorder(&self) -> crate::operation::start_configuration_recorder::builders::StartConfigurationRecorderFluentBuilder{
        crate::operation::start_configuration_recorder::builders::StartConfigurationRecorderFluentBuilder::new(self.handle.clone())
    }
}
