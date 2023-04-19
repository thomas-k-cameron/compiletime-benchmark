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
pub struct CreateReusableDelegationSetInput {
    /// <p>A unique string that identifies the request, and that allows you to retry failed <code>CreateReusableDelegationSet</code> requests without the risk of executing the operation twice. You must use a unique <code>CallerReference</code> string every time you submit a <code>CreateReusableDelegationSet</code> request. <code>CallerReference</code> can be any unique string, for example a date/time stamp.</p>
    #[doc(hidden)]
    pub caller_reference: std::option::Option<std::string::String>,
    /// <p>If you want to mark the delegation set for an existing hosted zone as reusable, the ID for that hosted zone.</p>
    #[doc(hidden)]
    pub hosted_zone_id: std::option::Option<std::string::String>,
}
impl CreateReusableDelegationSetInput {
    /// <p>A unique string that identifies the request, and that allows you to retry failed <code>CreateReusableDelegationSet</code> requests without the risk of executing the operation twice. You must use a unique <code>CallerReference</code> string every time you submit a <code>CreateReusableDelegationSet</code> request. <code>CallerReference</code> can be any unique string, for example a date/time stamp.</p>
    pub fn caller_reference(&self) -> std::option::Option<&str> {
        self.caller_reference.as_deref()
    }
    /// <p>If you want to mark the delegation set for an existing hosted zone as reusable, the ID for that hosted zone.</p>
    pub fn hosted_zone_id(&self) -> std::option::Option<&str> {
        self.hosted_zone_id.as_deref()
    }
}
impl CreateReusableDelegationSetInput {
    /// Creates a new builder-style object to manufacture [`CreateReusableDelegationSetInput`](crate::operation::create_reusable_delegation_set::CreateReusableDelegationSetInput).
    pub fn builder() -> crate::operation::create_reusable_delegation_set::builders::CreateReusableDelegationSetInputBuilder{
        crate::operation::create_reusable_delegation_set::builders::CreateReusableDelegationSetInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::create_reusable_delegation_set::CreateReusableDelegationSetInput;
/// A builder for [`CreateReusableDelegationSetInput`](crate::operation::create_reusable_delegation_set::CreateReusableDelegationSetInput).
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
pub struct CreateReusableDelegationSetInputBuilder {
    pub(crate) caller_reference: std::option::Option<std::string::String>,
    pub(crate) hosted_zone_id: std::option::Option<std::string::String>,
}
impl CreateReusableDelegationSetInputBuilder {
    /// <p>A unique string that identifies the request, and that allows you to retry failed <code>CreateReusableDelegationSet</code> requests without the risk of executing the operation twice. You must use a unique <code>CallerReference</code> string every time you submit a <code>CreateReusableDelegationSet</code> request. <code>CallerReference</code> can be any unique string, for example a date/time stamp.</p>
    pub fn caller_reference(mut self, input: impl Into<std::string::String>) -> Self {
        self.caller_reference = Some(input.into());
        self
    }
    /// <p>A unique string that identifies the request, and that allows you to retry failed <code>CreateReusableDelegationSet</code> requests without the risk of executing the operation twice. You must use a unique <code>CallerReference</code> string every time you submit a <code>CreateReusableDelegationSet</code> request. <code>CallerReference</code> can be any unique string, for example a date/time stamp.</p>
    pub fn set_caller_reference(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.caller_reference = input;
        self
    }
    /// <p>If you want to mark the delegation set for an existing hosted zone as reusable, the ID for that hosted zone.</p>
    pub fn hosted_zone_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.hosted_zone_id = Some(input.into());
        self
    }
    /// <p>If you want to mark the delegation set for an existing hosted zone as reusable, the ID for that hosted zone.</p>
    pub fn set_hosted_zone_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.hosted_zone_id = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateReusableDelegationSetInput`](crate::operation::create_reusable_delegation_set::CreateReusableDelegationSetInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::create_reusable_delegation_set::CreateReusableDelegationSetInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::create_reusable_delegation_set::CreateReusableDelegationSetInput {
                caller_reference: self.caller_reference,
                hosted_zone_id: self.hosted_zone_id,
            },
        )
    }
}
