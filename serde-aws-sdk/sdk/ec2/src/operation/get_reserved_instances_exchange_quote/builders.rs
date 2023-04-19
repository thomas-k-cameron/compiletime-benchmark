// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_reserved_instances_exchange_quote::_get_reserved_instances_exchange_quote_output::GetReservedInstancesExchangeQuoteOutputBuilder;

pub use crate::operation::get_reserved_instances_exchange_quote::_get_reserved_instances_exchange_quote_input::GetReservedInstancesExchangeQuoteInputBuilder;

/// Fluent builder constructing a request to `GetReservedInstancesExchangeQuote`.
///
/// <p>Returns a quote and exchange information for exchanging one or more specified Convertible Reserved Instances for a new Convertible Reserved Instance. If the exchange cannot be performed, the reason is returned in the response. Use <code>AcceptReservedInstancesExchangeQuote</code> to perform the exchange.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetReservedInstancesExchangeQuoteFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::get_reserved_instances_exchange_quote::builders::GetReservedInstancesExchangeQuoteInputBuilder
            }
impl GetReservedInstancesExchangeQuoteFluentBuilder {
    /// Creates a new `GetReservedInstancesExchangeQuote`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::get_reserved_instances_exchange_quote::GetReservedInstancesExchangeQuote, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::get_reserved_instances_exchange_quote::GetReservedInstancesExchangeQuoteError>
    >{
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
                    pub async fn send(self) -> std::result::Result<crate::operation::get_reserved_instances_exchange_quote::GetReservedInstancesExchangeQuoteOutput, aws_smithy_http::result::SdkError<crate::operation::get_reserved_instances_exchange_quote::GetReservedInstancesExchangeQuoteError>>
                     {
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
    ///     let deserialized_parameters: crate::operation::get_reserved_instances_exchange_quote::builders::GetReservedInstancesExchangeQuoteInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.get_reserved_instances_exchange_quote().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::get_reserved_instances_exchange_quote::builders::GetReservedInstancesExchangeQuoteInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// Appends an item to `ReservedInstanceIds`.
    ///
    /// To override the contents of this collection use [`set_reserved_instance_ids`](Self::set_reserved_instance_ids).
    ///
    /// <p>The IDs of the Convertible Reserved Instances to exchange.</p>
    pub fn reserved_instance_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.reserved_instance_ids(input.into());
        self
    }
    /// <p>The IDs of the Convertible Reserved Instances to exchange.</p>
    pub fn set_reserved_instance_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_reserved_instance_ids(input);
        self
    }
    /// Appends an item to `TargetConfigurations`.
    ///
    /// To override the contents of this collection use [`set_target_configurations`](Self::set_target_configurations).
    ///
    /// <p>The configuration of the target Convertible Reserved Instance to exchange for your current Convertible Reserved Instances.</p>
    pub fn target_configurations(
        mut self,
        input: crate::types::TargetConfigurationRequest,
    ) -> Self {
        self.inner = self.inner.target_configurations(input);
        self
    }
    /// <p>The configuration of the target Convertible Reserved Instance to exchange for your current Convertible Reserved Instances.</p>
    pub fn set_target_configurations(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TargetConfigurationRequest>>,
    ) -> Self {
        self.inner = self.inner.set_target_configurations(input);
        self
    }
}