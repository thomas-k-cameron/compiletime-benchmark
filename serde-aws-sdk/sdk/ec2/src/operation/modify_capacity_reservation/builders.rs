// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_capacity_reservation::_modify_capacity_reservation_output::ModifyCapacityReservationOutputBuilder;

pub use crate::operation::modify_capacity_reservation::_modify_capacity_reservation_input::ModifyCapacityReservationInputBuilder;

/// Fluent builder constructing a request to `ModifyCapacityReservation`.
///
/// <p>Modifies a Capacity Reservation's capacity and the conditions under which it is to be released. You cannot change a Capacity Reservation's instance type, EBS optimization, instance store settings, platform, Availability Zone, or instance eligibility. If you need to modify any of these attributes, we recommend that you cancel the Capacity Reservation, and then create a new one with the required attributes.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ModifyCapacityReservationFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::modify_capacity_reservation::builders::ModifyCapacityReservationInputBuilder
            }
impl ModifyCapacityReservationFluentBuilder {
    /// Creates a new `ModifyCapacityReservation`.
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
            crate::operation::modify_capacity_reservation::ModifyCapacityReservation,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::modify_capacity_reservation::ModifyCapacityReservationError,
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
        crate::operation::modify_capacity_reservation::ModifyCapacityReservationOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::modify_capacity_reservation::ModifyCapacityReservationError,
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
    ///     let deserialized_parameters: crate::operation::modify_capacity_reservation::builders::ModifyCapacityReservationInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.modify_capacity_reservation().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::modify_capacity_reservation::builders::ModifyCapacityReservationInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID of the Capacity Reservation.</p>
    pub fn capacity_reservation_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.capacity_reservation_id(input.into());
        self
    }
    /// <p>The ID of the Capacity Reservation.</p>
    pub fn set_capacity_reservation_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_capacity_reservation_id(input);
        self
    }
    /// <p>The number of instances for which to reserve capacity. The number of instances can't be increased or decreased by more than <code>1000</code> in a single request.</p>
    pub fn instance_count(mut self, input: i32) -> Self {
        self.inner = self.inner.instance_count(input);
        self
    }
    /// <p>The number of instances for which to reserve capacity. The number of instances can't be increased or decreased by more than <code>1000</code> in a single request.</p>
    pub fn set_instance_count(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_instance_count(input);
        self
    }
    /// <p>The date and time at which the Capacity Reservation expires. When a Capacity Reservation expires, the reserved capacity is released and you can no longer launch instances into it. The Capacity Reservation's state changes to <code>expired</code> when it reaches its end date and time.</p>
    /// <p>The Capacity Reservation is cancelled within an hour from the specified time. For example, if you specify 5/31/2019, 13:30:55, the Capacity Reservation is guaranteed to end between 13:30:55 and 14:30:55 on 5/31/2019.</p>
    /// <p>You must provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>limited</code>. Omit <code>EndDate</code> if <code>EndDateType</code> is <code>unlimited</code>.</p>
    pub fn end_date(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.end_date(input);
        self
    }
    /// <p>The date and time at which the Capacity Reservation expires. When a Capacity Reservation expires, the reserved capacity is released and you can no longer launch instances into it. The Capacity Reservation's state changes to <code>expired</code> when it reaches its end date and time.</p>
    /// <p>The Capacity Reservation is cancelled within an hour from the specified time. For example, if you specify 5/31/2019, 13:30:55, the Capacity Reservation is guaranteed to end between 13:30:55 and 14:30:55 on 5/31/2019.</p>
    /// <p>You must provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>limited</code>. Omit <code>EndDate</code> if <code>EndDateType</code> is <code>unlimited</code>.</p>
    pub fn set_end_date(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_end_date(input);
        self
    }
    /// <p>Indicates the way in which the Capacity Reservation ends. A Capacity Reservation can have one of the following end types:</p>
    /// <ul>
    /// <li> <p> <code>unlimited</code> - The Capacity Reservation remains active until you explicitly cancel it. Do not provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>unlimited</code>.</p> </li>
    /// <li> <p> <code>limited</code> - The Capacity Reservation expires automatically at a specified date and time. You must provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>limited</code>.</p> </li>
    /// </ul>
    pub fn end_date_type(mut self, input: crate::types::EndDateType) -> Self {
        self.inner = self.inner.end_date_type(input);
        self
    }
    /// <p>Indicates the way in which the Capacity Reservation ends. A Capacity Reservation can have one of the following end types:</p>
    /// <ul>
    /// <li> <p> <code>unlimited</code> - The Capacity Reservation remains active until you explicitly cancel it. Do not provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>unlimited</code>.</p> </li>
    /// <li> <p> <code>limited</code> - The Capacity Reservation expires automatically at a specified date and time. You must provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>limited</code>.</p> </li>
    /// </ul>
    pub fn set_end_date_type(
        mut self,
        input: std::option::Option<crate::types::EndDateType>,
    ) -> Self {
        self.inner = self.inner.set_end_date_type(input);
        self
    }
    /// <p>Reserved. Capacity Reservations you have created are accepted by default.</p>
    pub fn accept(mut self, input: bool) -> Self {
        self.inner = self.inner.accept(input);
        self
    }
    /// <p>Reserved. Capacity Reservations you have created are accepted by default.</p>
    pub fn set_accept(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_accept(input);
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
    /// <p>Reserved for future use.</p>
    pub fn additional_info(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.additional_info(input.into());
        self
    }
    /// <p>Reserved for future use.</p>
    pub fn set_additional_info(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_additional_info(input);
        self
    }
}
