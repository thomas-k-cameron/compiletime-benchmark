// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteCapacityProvider`](crate::operation::delete_capacity_provider::builders::DeleteCapacityProviderFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`capacity_provider(impl Into<String>)`](crate::operation::delete_capacity_provider::builders::DeleteCapacityProviderFluentBuilder::capacity_provider) / [`set_capacity_provider(Option<String>)`](crate::operation::delete_capacity_provider::builders::DeleteCapacityProviderFluentBuilder::set_capacity_provider): <p>The short name or full Amazon Resource Name (ARN) of the capacity provider to delete.</p>
    /// - On success, responds with [`DeleteCapacityProviderOutput`](crate::operation::delete_capacity_provider::DeleteCapacityProviderOutput) with field(s):
    ///   - [`capacity_provider(Option<CapacityProvider>)`](crate::operation::delete_capacity_provider::DeleteCapacityProviderOutput::capacity_provider): <p>The details of the capacity provider.</p>
    /// - On failure, responds with [`SdkError<DeleteCapacityProviderError>`](crate::operation::delete_capacity_provider::DeleteCapacityProviderError)
    pub fn delete_capacity_provider(
        &self,
    ) -> crate::operation::delete_capacity_provider::builders::DeleteCapacityProviderFluentBuilder
    {
        crate::operation::delete_capacity_provider::builders::DeleteCapacityProviderFluentBuilder::new(self.handle.clone())
    }
}
