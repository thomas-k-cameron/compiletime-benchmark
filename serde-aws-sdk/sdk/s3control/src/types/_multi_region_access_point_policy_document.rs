// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The Multi-Region Access Point access control policy.</p>
/// <p>When you update the policy, the update is first listed as the proposed policy. After the update is finished and all Regions have been updated, the proposed policy is listed as the established policy. If both policies have the same version number, the proposed policy is the established policy.</p>
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
pub struct MultiRegionAccessPointPolicyDocument {
    /// <p>The last established policy for the Multi-Region Access Point.</p>
    #[doc(hidden)]
    pub established: std::option::Option<crate::types::EstablishedMultiRegionAccessPointPolicy>,
    /// <p>The proposed policy for the Multi-Region Access Point.</p>
    #[doc(hidden)]
    pub proposed: std::option::Option<crate::types::ProposedMultiRegionAccessPointPolicy>,
}
impl MultiRegionAccessPointPolicyDocument {
    /// <p>The last established policy for the Multi-Region Access Point.</p>
    pub fn established(
        &self,
    ) -> std::option::Option<&crate::types::EstablishedMultiRegionAccessPointPolicy> {
        self.established.as_ref()
    }
    /// <p>The proposed policy for the Multi-Region Access Point.</p>
    pub fn proposed(
        &self,
    ) -> std::option::Option<&crate::types::ProposedMultiRegionAccessPointPolicy> {
        self.proposed.as_ref()
    }
}
impl MultiRegionAccessPointPolicyDocument {
    /// Creates a new builder-style object to manufacture [`MultiRegionAccessPointPolicyDocument`](crate::types::MultiRegionAccessPointPolicyDocument).
    pub fn builder() -> crate::types::builders::MultiRegionAccessPointPolicyDocumentBuilder {
        crate::types::builders::MultiRegionAccessPointPolicyDocumentBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::MultiRegionAccessPointPolicyDocument;
/// A builder for [`MultiRegionAccessPointPolicyDocument`](crate::types::MultiRegionAccessPointPolicyDocument).
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
pub struct MultiRegionAccessPointPolicyDocumentBuilder {
    pub(crate) established:
        std::option::Option<crate::types::EstablishedMultiRegionAccessPointPolicy>,
    pub(crate) proposed: std::option::Option<crate::types::ProposedMultiRegionAccessPointPolicy>,
}
impl MultiRegionAccessPointPolicyDocumentBuilder {
    /// <p>The last established policy for the Multi-Region Access Point.</p>
    pub fn established(
        mut self,
        input: crate::types::EstablishedMultiRegionAccessPointPolicy,
    ) -> Self {
        self.established = Some(input);
        self
    }
    /// <p>The last established policy for the Multi-Region Access Point.</p>
    pub fn set_established(
        mut self,
        input: std::option::Option<crate::types::EstablishedMultiRegionAccessPointPolicy>,
    ) -> Self {
        self.established = input;
        self
    }
    /// <p>The proposed policy for the Multi-Region Access Point.</p>
    pub fn proposed(mut self, input: crate::types::ProposedMultiRegionAccessPointPolicy) -> Self {
        self.proposed = Some(input);
        self
    }
    /// <p>The proposed policy for the Multi-Region Access Point.</p>
    pub fn set_proposed(
        mut self,
        input: std::option::Option<crate::types::ProposedMultiRegionAccessPointPolicy>,
    ) -> Self {
        self.proposed = input;
        self
    }
    /// Consumes the builder and constructs a [`MultiRegionAccessPointPolicyDocument`](crate::types::MultiRegionAccessPointPolicyDocument).
    pub fn build(self) -> crate::types::MultiRegionAccessPointPolicyDocument {
        crate::types::MultiRegionAccessPointPolicyDocument {
            established: self.established,
            proposed: self.proposed,
        }
    }
}