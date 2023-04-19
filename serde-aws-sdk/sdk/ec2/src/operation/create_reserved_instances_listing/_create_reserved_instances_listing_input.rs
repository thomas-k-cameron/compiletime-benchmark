// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for CreateReservedInstancesListing.</p>
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
pub struct CreateReservedInstancesListingInput {
    /// <p>Unique, case-sensitive identifier you provide to ensure idempotency of your listings. This helps avoid duplicate listings. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    #[doc(hidden)]
    pub client_token: std::option::Option<std::string::String>,
    /// <p>The number of instances that are a part of a Reserved Instance account to be listed in the Reserved Instance Marketplace. This number should be less than or equal to the instance count associated with the Reserved Instance ID specified in this call.</p>
    #[doc(hidden)]
    pub instance_count: std::option::Option<i32>,
    /// <p>A list specifying the price of the Standard Reserved Instance for each month remaining in the Reserved Instance term.</p>
    #[doc(hidden)]
    pub price_schedules:
        std::option::Option<std::vec::Vec<crate::types::PriceScheduleSpecification>>,
    /// <p>The ID of the active Standard Reserved Instance.</p>
    #[doc(hidden)]
    pub reserved_instances_id: std::option::Option<std::string::String>,
}
impl CreateReservedInstancesListingInput {
    /// <p>Unique, case-sensitive identifier you provide to ensure idempotency of your listings. This helps avoid duplicate listings. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(&self) -> std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>The number of instances that are a part of a Reserved Instance account to be listed in the Reserved Instance Marketplace. This number should be less than or equal to the instance count associated with the Reserved Instance ID specified in this call.</p>
    pub fn instance_count(&self) -> std::option::Option<i32> {
        self.instance_count
    }
    /// <p>A list specifying the price of the Standard Reserved Instance for each month remaining in the Reserved Instance term.</p>
    pub fn price_schedules(
        &self,
    ) -> std::option::Option<&[crate::types::PriceScheduleSpecification]> {
        self.price_schedules.as_deref()
    }
    /// <p>The ID of the active Standard Reserved Instance.</p>
    pub fn reserved_instances_id(&self) -> std::option::Option<&str> {
        self.reserved_instances_id.as_deref()
    }
}
impl CreateReservedInstancesListingInput {
    /// Creates a new builder-style object to manufacture [`CreateReservedInstancesListingInput`](crate::operation::create_reserved_instances_listing::CreateReservedInstancesListingInput).
    pub fn builder() -> crate::operation::create_reserved_instances_listing::builders::CreateReservedInstancesListingInputBuilder{
        crate::operation::create_reserved_instances_listing::builders::CreateReservedInstancesListingInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::create_reserved_instances_listing::CreateReservedInstancesListingInput;
/// A builder for [`CreateReservedInstancesListingInput`](crate::operation::create_reserved_instances_listing::CreateReservedInstancesListingInput).
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
pub struct CreateReservedInstancesListingInputBuilder {
    pub(crate) client_token: std::option::Option<std::string::String>,
    pub(crate) instance_count: std::option::Option<i32>,
    pub(crate) price_schedules:
        std::option::Option<std::vec::Vec<crate::types::PriceScheduleSpecification>>,
    pub(crate) reserved_instances_id: std::option::Option<std::string::String>,
}
impl CreateReservedInstancesListingInputBuilder {
    /// <p>Unique, case-sensitive identifier you provide to ensure idempotency of your listings. This helps avoid duplicate listings. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.client_token = Some(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier you provide to ensure idempotency of your listings. This helps avoid duplicate listings. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>The number of instances that are a part of a Reserved Instance account to be listed in the Reserved Instance Marketplace. This number should be less than or equal to the instance count associated with the Reserved Instance ID specified in this call.</p>
    pub fn instance_count(mut self, input: i32) -> Self {
        self.instance_count = Some(input);
        self
    }
    /// <p>The number of instances that are a part of a Reserved Instance account to be listed in the Reserved Instance Marketplace. This number should be less than or equal to the instance count associated with the Reserved Instance ID specified in this call.</p>
    pub fn set_instance_count(mut self, input: std::option::Option<i32>) -> Self {
        self.instance_count = input;
        self
    }
    /// Appends an item to `price_schedules`.
    ///
    /// To override the contents of this collection use [`set_price_schedules`](Self::set_price_schedules).
    ///
    /// <p>A list specifying the price of the Standard Reserved Instance for each month remaining in the Reserved Instance term.</p>
    pub fn price_schedules(mut self, input: crate::types::PriceScheduleSpecification) -> Self {
        let mut v = self.price_schedules.unwrap_or_default();
        v.push(input);
        self.price_schedules = Some(v);
        self
    }
    /// <p>A list specifying the price of the Standard Reserved Instance for each month remaining in the Reserved Instance term.</p>
    pub fn set_price_schedules(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::PriceScheduleSpecification>>,
    ) -> Self {
        self.price_schedules = input;
        self
    }
    /// <p>The ID of the active Standard Reserved Instance.</p>
    pub fn reserved_instances_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.reserved_instances_id = Some(input.into());
        self
    }
    /// <p>The ID of the active Standard Reserved Instance.</p>
    pub fn set_reserved_instances_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.reserved_instances_id = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateReservedInstancesListingInput`](crate::operation::create_reserved_instances_listing::CreateReservedInstancesListingInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::create_reserved_instances_listing::CreateReservedInstancesListingInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::create_reserved_instances_listing::CreateReservedInstancesListingInput {
                client_token: self.client_token
                ,
                instance_count: self.instance_count
                ,
                price_schedules: self.price_schedules
                ,
                reserved_instances_id: self.reserved_instances_id
                ,
            }
        )
    }
}
