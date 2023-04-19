// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListCidrBlocks`](crate::operation::list_cidr_blocks::builders::ListCidrBlocksFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_cidr_blocks::builders::ListCidrBlocksFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`collection_id(impl Into<String>)`](crate::operation::list_cidr_blocks::builders::ListCidrBlocksFluentBuilder::collection_id) / [`set_collection_id(Option<String>)`](crate::operation::list_cidr_blocks::builders::ListCidrBlocksFluentBuilder::set_collection_id): <p>The UUID of the CIDR collection.</p>
    ///   - [`location_name(impl Into<String>)`](crate::operation::list_cidr_blocks::builders::ListCidrBlocksFluentBuilder::location_name) / [`set_location_name(Option<String>)`](crate::operation::list_cidr_blocks::builders::ListCidrBlocksFluentBuilder::set_location_name): <p>The name of the CIDR collection location.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_cidr_blocks::builders::ListCidrBlocksFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_cidr_blocks::builders::ListCidrBlocksFluentBuilder::set_next_token): <p>An opaque pagination token to indicate where the service is to begin enumerating results.</p>
    ///   - [`max_results(i32)`](crate::operation::list_cidr_blocks::builders::ListCidrBlocksFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_cidr_blocks::builders::ListCidrBlocksFluentBuilder::set_max_results): <p>Maximum number of results you want returned.</p>
    /// - On success, responds with [`ListCidrBlocksOutput`](crate::operation::list_cidr_blocks::ListCidrBlocksOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_cidr_blocks::ListCidrBlocksOutput::next_token): <p>An opaque pagination token to indicate where the service is to begin enumerating results. </p>  <p>If no value is provided, the listing of results starts from the beginning.</p>
    ///   - [`cidr_blocks(Option<Vec<CidrBlockSummary>>)`](crate::operation::list_cidr_blocks::ListCidrBlocksOutput::cidr_blocks): <p>A complex type that contains information about the CIDR blocks.</p>
    /// - On failure, responds with [`SdkError<ListCidrBlocksError>`](crate::operation::list_cidr_blocks::ListCidrBlocksError)
    pub fn list_cidr_blocks(
        &self,
    ) -> crate::operation::list_cidr_blocks::builders::ListCidrBlocksFluentBuilder {
        crate::operation::list_cidr_blocks::builders::ListCidrBlocksFluentBuilder::new(
            self.handle.clone(),
        )
    }
}