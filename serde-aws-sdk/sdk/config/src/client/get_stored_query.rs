// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetStoredQuery`](crate::operation::get_stored_query::builders::GetStoredQueryFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`query_name(impl Into<String>)`](crate::operation::get_stored_query::builders::GetStoredQueryFluentBuilder::query_name) / [`set_query_name(Option<String>)`](crate::operation::get_stored_query::builders::GetStoredQueryFluentBuilder::set_query_name): <p>The name of the query.</p>
    /// - On success, responds with [`GetStoredQueryOutput`](crate::operation::get_stored_query::GetStoredQueryOutput) with field(s):
    ///   - [`stored_query(Option<StoredQuery>)`](crate::operation::get_stored_query::GetStoredQueryOutput::stored_query): <p>Returns a <code>StoredQuery</code> object.</p>
    /// - On failure, responds with [`SdkError<GetStoredQueryError>`](crate::operation::get_stored_query::GetStoredQueryError)
    pub fn get_stored_query(
        &self,
    ) -> crate::operation::get_stored_query::builders::GetStoredQueryFluentBuilder {
        crate::operation::get_stored_query::builders::GetStoredQueryFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
