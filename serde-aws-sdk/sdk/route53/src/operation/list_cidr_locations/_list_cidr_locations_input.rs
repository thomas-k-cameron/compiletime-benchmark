// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListCidrLocationsInput {
    /// <p>The CIDR collection ID.</p>
    #[doc(hidden)]
    pub collection_id: std::option::Option<std::string::String>,
    /// <p>An opaque pagination token to indicate where the service is to begin enumerating results.</p>
    /// <p>If no value is provided, the listing of results starts from the beginning.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The maximum number of CIDR collection locations to return in the response.</p>
    #[doc(hidden)]
    pub max_results: std::option::Option<i32>,
}
impl ListCidrLocationsInput {
    /// <p>The CIDR collection ID.</p>
    pub fn collection_id(&self) -> std::option::Option<&str> {
        self.collection_id.as_deref()
    }
    /// <p>An opaque pagination token to indicate where the service is to begin enumerating results.</p>
    /// <p>If no value is provided, the listing of results starts from the beginning.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of CIDR collection locations to return in the response.</p>
    pub fn max_results(&self) -> std::option::Option<i32> {
        self.max_results
    }
}
impl ListCidrLocationsInput {
    /// Creates a new builder-style object to manufacture [`ListCidrLocationsInput`](crate::operation::list_cidr_locations::ListCidrLocationsInput).
    pub fn builder(
    ) -> crate::operation::list_cidr_locations::builders::ListCidrLocationsInputBuilder {
        crate::operation::list_cidr_locations::builders::ListCidrLocationsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_cidr_locations::ListCidrLocationsInput;
/// A builder for [`ListCidrLocationsInput`](crate::operation::list_cidr_locations::ListCidrLocationsInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct ListCidrLocationsInputBuilder {
    pub(crate) collection_id: std::option::Option<std::string::String>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    pub(crate) max_results: std::option::Option<i32>,
}
impl ListCidrLocationsInputBuilder {
    /// <p>The CIDR collection ID.</p>
    pub fn collection_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.collection_id = Some(input.into());
        self
    }
    /// <p>The CIDR collection ID.</p>
    pub fn set_collection_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.collection_id = input;
        self
    }
    /// <p>An opaque pagination token to indicate where the service is to begin enumerating results.</p>
    /// <p>If no value is provided, the listing of results starts from the beginning.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>An opaque pagination token to indicate where the service is to begin enumerating results.</p>
    /// <p>If no value is provided, the listing of results starts from the beginning.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of CIDR collection locations to return in the response.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = Some(input);
        self
    }
    /// <p>The maximum number of CIDR collection locations to return in the response.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// Consumes the builder and constructs a [`ListCidrLocationsInput`](crate::operation::list_cidr_locations::ListCidrLocationsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::list_cidr_locations::ListCidrLocationsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::list_cidr_locations::ListCidrLocationsInput {
                collection_id: self.collection_id,
                next_token: self.next_token,
                max_results: self.max_results,
            },
        )
    }
}
