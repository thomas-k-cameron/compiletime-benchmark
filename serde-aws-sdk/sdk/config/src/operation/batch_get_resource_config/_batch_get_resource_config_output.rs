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
pub struct BatchGetResourceConfigOutput {
    /// <p>A list that contains the current configuration of one or more resources.</p>
    #[doc(hidden)]
    pub base_configuration_items:
        std::option::Option<std::vec::Vec<crate::types::BaseConfigurationItem>>,
    /// <p>A list of resource keys that were not processed with the current response. The unprocessesResourceKeys value is in the same form as ResourceKeys, so the value can be directly provided to a subsequent BatchGetResourceConfig operation. If there are no unprocessed resource keys, the response contains an empty unprocessedResourceKeys list. </p>
    #[doc(hidden)]
    pub unprocessed_resource_keys: std::option::Option<std::vec::Vec<crate::types::ResourceKey>>,
    _request_id: Option<String>,
}
impl BatchGetResourceConfigOutput {
    /// <p>A list that contains the current configuration of one or more resources.</p>
    pub fn base_configuration_items(
        &self,
    ) -> std::option::Option<&[crate::types::BaseConfigurationItem]> {
        self.base_configuration_items.as_deref()
    }
    /// <p>A list of resource keys that were not processed with the current response. The unprocessesResourceKeys value is in the same form as ResourceKeys, so the value can be directly provided to a subsequent BatchGetResourceConfig operation. If there are no unprocessed resource keys, the response contains an empty unprocessedResourceKeys list. </p>
    pub fn unprocessed_resource_keys(&self) -> std::option::Option<&[crate::types::ResourceKey]> {
        self.unprocessed_resource_keys.as_deref()
    }
}
impl aws_http::request_id::RequestId for BatchGetResourceConfigOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl BatchGetResourceConfigOutput {
    /// Creates a new builder-style object to manufacture [`BatchGetResourceConfigOutput`](crate::operation::batch_get_resource_config::BatchGetResourceConfigOutput).
    pub fn builder(
    ) -> crate::operation::batch_get_resource_config::builders::BatchGetResourceConfigOutputBuilder
    {
        crate::operation::batch_get_resource_config::builders::BatchGetResourceConfigOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::batch_get_resource_config::BatchGetResourceConfigOutput;
/// A builder for [`BatchGetResourceConfigOutput`](crate::operation::batch_get_resource_config::BatchGetResourceConfigOutput).
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
pub struct BatchGetResourceConfigOutputBuilder {
    pub(crate) base_configuration_items:
        std::option::Option<std::vec::Vec<crate::types::BaseConfigurationItem>>,
    pub(crate) unprocessed_resource_keys:
        std::option::Option<std::vec::Vec<crate::types::ResourceKey>>,
    _request_id: Option<String>,
}
impl BatchGetResourceConfigOutputBuilder {
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
    /// Appends an item to `unprocessed_resource_keys`.
    ///
    /// To override the contents of this collection use [`set_unprocessed_resource_keys`](Self::set_unprocessed_resource_keys).
    ///
    /// <p>A list of resource keys that were not processed with the current response. The unprocessesResourceKeys value is in the same form as ResourceKeys, so the value can be directly provided to a subsequent BatchGetResourceConfig operation. If there are no unprocessed resource keys, the response contains an empty unprocessedResourceKeys list. </p>
    pub fn unprocessed_resource_keys(mut self, input: crate::types::ResourceKey) -> Self {
        let mut v = self.unprocessed_resource_keys.unwrap_or_default();
        v.push(input);
        self.unprocessed_resource_keys = Some(v);
        self
    }
    /// <p>A list of resource keys that were not processed with the current response. The unprocessesResourceKeys value is in the same form as ResourceKeys, so the value can be directly provided to a subsequent BatchGetResourceConfig operation. If there are no unprocessed resource keys, the response contains an empty unprocessedResourceKeys list. </p>
    pub fn set_unprocessed_resource_keys(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ResourceKey>>,
    ) -> Self {
        self.unprocessed_resource_keys = input;
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
    /// Consumes the builder and constructs a [`BatchGetResourceConfigOutput`](crate::operation::batch_get_resource_config::BatchGetResourceConfigOutput).
    pub fn build(
        self,
    ) -> crate::operation::batch_get_resource_config::BatchGetResourceConfigOutput {
        crate::operation::batch_get_resource_config::BatchGetResourceConfigOutput {
            base_configuration_items: self.base_configuration_items,
            unprocessed_resource_keys: self.unprocessed_resource_keys,
            _request_id: self._request_id,
        }
    }
}