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
pub struct BatchGetAggregateResourceConfigOutput {
    /// <p>A list that contains the current configuration of one or more resources.</p>
    #[doc(hidden)]
    pub base_configuration_items:
        std::option::Option<std::vec::Vec<crate::types::BaseConfigurationItem>>,
    /// <p>A list of resource identifiers that were not processed with current scope. The list is empty if all the resources are processed.</p>
    #[doc(hidden)]
    pub unprocessed_resource_identifiers:
        std::option::Option<std::vec::Vec<crate::types::AggregateResourceIdentifier>>,
    _request_id: Option<String>,
}
impl BatchGetAggregateResourceConfigOutput {
    /// <p>A list that contains the current configuration of one or more resources.</p>
    pub fn base_configuration_items(
        &self,
    ) -> std::option::Option<&[crate::types::BaseConfigurationItem]> {
        self.base_configuration_items.as_deref()
    }
    /// <p>A list of resource identifiers that were not processed with current scope. The list is empty if all the resources are processed.</p>
    pub fn unprocessed_resource_identifiers(
        &self,
    ) -> std::option::Option<&[crate::types::AggregateResourceIdentifier]> {
        self.unprocessed_resource_identifiers.as_deref()
    }
}
impl aws_http::request_id::RequestId for BatchGetAggregateResourceConfigOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl BatchGetAggregateResourceConfigOutput {
    /// Creates a new builder-style object to manufacture [`BatchGetAggregateResourceConfigOutput`](crate::operation::batch_get_aggregate_resource_config::BatchGetAggregateResourceConfigOutput).
    pub fn builder() -> crate::operation::batch_get_aggregate_resource_config::builders::BatchGetAggregateResourceConfigOutputBuilder{
        crate::operation::batch_get_aggregate_resource_config::builders::BatchGetAggregateResourceConfigOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::batch_get_aggregate_resource_config::BatchGetAggregateResourceConfigOutput;
/// A builder for [`BatchGetAggregateResourceConfigOutput`](crate::operation::batch_get_aggregate_resource_config::BatchGetAggregateResourceConfigOutput).
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
pub struct BatchGetAggregateResourceConfigOutputBuilder {
    pub(crate) base_configuration_items:
        std::option::Option<std::vec::Vec<crate::types::BaseConfigurationItem>>,
    pub(crate) unprocessed_resource_identifiers:
        std::option::Option<std::vec::Vec<crate::types::AggregateResourceIdentifier>>,
    _request_id: Option<String>,
}
impl BatchGetAggregateResourceConfigOutputBuilder {
    /// Appends an item to `base_configuration_items`.
    ///
    /// To override the contents of this collection use [`set_base_configuration_items`](Self::set_base_configuration_items).
    ///
    /// <p>A list that contains the current configuration of one or more resources.</p>
    pub fn base_configuration_items(mut self, input: crate::types::BaseConfigurationItem) -> Self {
        let mut v = self.base_configuration_items.unwrap_or_default();
        v.push(input);
        self.base_configuration_items = Some(v);
        self
    }
    /// <p>A list that contains the current configuration of one or more resources.</p>
    pub fn set_base_configuration_items(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::BaseConfigurationItem>>,
    ) -> Self {
        self.base_configuration_items = input;
        self
    }
    /// Appends an item to `unprocessed_resource_identifiers`.
    ///
    /// To override the contents of this collection use [`set_unprocessed_resource_identifiers`](Self::set_unprocessed_resource_identifiers).
    ///
    /// <p>A list of resource identifiers that were not processed with current scope. The list is empty if all the resources are processed.</p>
    pub fn unprocessed_resource_identifiers(
        mut self,
        input: crate::types::AggregateResourceIdentifier,
    ) -> Self {
        let mut v = self.unprocessed_resource_identifiers.unwrap_or_default();
        v.push(input);
        self.unprocessed_resource_identifiers = Some(v);
        self
    }
    /// <p>A list of resource identifiers that were not processed with current scope. The list is empty if all the resources are processed.</p>
    pub fn set_unprocessed_resource_identifiers(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::AggregateResourceIdentifier>>,
    ) -> Self {
        self.unprocessed_resource_identifiers = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`BatchGetAggregateResourceConfigOutput`](crate::operation::batch_get_aggregate_resource_config::BatchGetAggregateResourceConfigOutput).
    pub fn build(
        self,
    ) -> crate::operation::batch_get_aggregate_resource_config::BatchGetAggregateResourceConfigOutput
    {
        crate::operation::batch_get_aggregate_resource_config::BatchGetAggregateResourceConfigOutput {
            base_configuration_items: self.base_configuration_items
            ,
            unprocessed_resource_identifiers: self.unprocessed_resource_identifiers
            ,
            _request_id: self._request_id,
        }
    }
}
