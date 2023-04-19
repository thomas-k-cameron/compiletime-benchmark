// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateIndexType`](crate::operation::update_index_type::builders::UpdateIndexTypeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::operation::update_index_type::builders::UpdateIndexTypeFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::update_index_type::builders::UpdateIndexTypeFluentBuilder::set_arn): <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the index that you want to update.</p>
    ///   - [`r#type(IndexType)`](crate::operation::update_index_type::builders::UpdateIndexTypeFluentBuilder::type) / [`set_type(Option<IndexType>)`](crate::operation::update_index_type::builders::UpdateIndexTypeFluentBuilder::set_type): <p>The type of the index. To understand the difference between <code>LOCAL</code> and <code>AGGREGATOR</code>, see <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/manage-aggregator-region.html">Turning on cross-Region search</a> in the <i>Amazon Web Services Resource Explorer User Guide</i>.</p>
    /// - On success, responds with [`UpdateIndexTypeOutput`](crate::operation::update_index_type::UpdateIndexTypeOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::update_index_type::UpdateIndexTypeOutput::arn): <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the index that you updated.</p>
    ///   - [`r#type(Option<IndexType>)`](crate::operation::update_index_type::UpdateIndexTypeOutput::type): <p>Specifies the type of the specified index after the operation completes.</p>
    ///   - [`state(Option<IndexState>)`](crate::operation::update_index_type::UpdateIndexTypeOutput::state): <p>Indicates the state of the request to update the index. This operation is asynchronous. Call the <code>GetIndex</code> operation to check for changes.</p>
    ///   - [`last_updated_at(Option<DateTime>)`](crate::operation::update_index_type::UpdateIndexTypeOutput::last_updated_at): <p>The date and timestamp when the index was last updated.</p>
    /// - On failure, responds with [`SdkError<UpdateIndexTypeError>`](crate::operation::update_index_type::UpdateIndexTypeError)
    pub fn update_index_type(
        &self,
    ) -> crate::operation::update_index_type::builders::UpdateIndexTypeFluentBuilder {
        crate::operation::update_index_type::builders::UpdateIndexTypeFluentBuilder::new(
            self.handle.clone(),
        )
    }
}