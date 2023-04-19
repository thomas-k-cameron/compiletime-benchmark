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
pub struct GetSpotPlacementScoresOutput {
    /// <p>The Spot placement score for the top 10 Regions or Availability Zones, scored on a scale from 1 to 10. Each score  reflects how likely it is that each Region or Availability Zone will succeed at fulfilling the specified target capacity  <i>at the time of the Spot placement score request</i>. A score of <code>10</code> means that your Spot capacity request is highly likely to succeed in that Region or Availability Zone. </p>
    /// <p>If you request a Spot placement score for Regions, a high score assumes that your fleet request will be configured to use all Availability Zones and the <code>capacity-optimized</code> allocation strategy. If you request a Spot placement score for Availability Zones, a high score assumes that your fleet request will be configured to use a single Availability Zone and the <code>capacity-optimized</code> allocation strategy.</p>
    /// <p>Different  Regions or Availability Zones might return the same score.</p> <note>
    /// <p>The Spot placement score serves as a recommendation only. No score guarantees that your Spot request will be fully or partially fulfilled.</p>
    /// </note>
    #[doc(hidden)]
    pub spot_placement_scores: std::option::Option<std::vec::Vec<crate::types::SpotPlacementScore>>,
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl GetSpotPlacementScoresOutput {
    /// <p>The Spot placement score for the top 10 Regions or Availability Zones, scored on a scale from 1 to 10. Each score  reflects how likely it is that each Region or Availability Zone will succeed at fulfilling the specified target capacity  <i>at the time of the Spot placement score request</i>. A score of <code>10</code> means that your Spot capacity request is highly likely to succeed in that Region or Availability Zone. </p>
    /// <p>If you request a Spot placement score for Regions, a high score assumes that your fleet request will be configured to use all Availability Zones and the <code>capacity-optimized</code> allocation strategy. If you request a Spot placement score for Availability Zones, a high score assumes that your fleet request will be configured to use a single Availability Zone and the <code>capacity-optimized</code> allocation strategy.</p>
    /// <p>Different  Regions or Availability Zones might return the same score.</p> <note>
    /// <p>The Spot placement score serves as a recommendation only. No score guarantees that your Spot request will be fully or partially fulfilled.</p>
    /// </note>
    pub fn spot_placement_scores(
        &self,
    ) -> std::option::Option<&[crate::types::SpotPlacementScore]> {
        self.spot_placement_scores.as_deref()
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for GetSpotPlacementScoresOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetSpotPlacementScoresOutput {
    /// Creates a new builder-style object to manufacture [`GetSpotPlacementScoresOutput`](crate::operation::get_spot_placement_scores::GetSpotPlacementScoresOutput).
    pub fn builder(
    ) -> crate::operation::get_spot_placement_scores::builders::GetSpotPlacementScoresOutputBuilder
    {
        crate::operation::get_spot_placement_scores::builders::GetSpotPlacementScoresOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_spot_placement_scores::GetSpotPlacementScoresOutput;
/// A builder for [`GetSpotPlacementScoresOutput`](crate::operation::get_spot_placement_scores::GetSpotPlacementScoresOutput).
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
pub struct GetSpotPlacementScoresOutputBuilder {
    pub(crate) spot_placement_scores:
        std::option::Option<std::vec::Vec<crate::types::SpotPlacementScore>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl GetSpotPlacementScoresOutputBuilder {
    /// Appends an item to `spot_placement_scores`.
    ///
    /// To override the contents of this collection use [`set_spot_placement_scores`](Self::set_spot_placement_scores).
    ///
    /// <p>The Spot placement score for the top 10 Regions or Availability Zones, scored on a scale from 1 to 10. Each score  reflects how likely it is that each Region or Availability Zone will succeed at fulfilling the specified target capacity  <i>at the time of the Spot placement score request</i>. A score of <code>10</code> means that your Spot capacity request is highly likely to succeed in that Region or Availability Zone. </p>
    /// <p>If you request a Spot placement score for Regions, a high score assumes that your fleet request will be configured to use all Availability Zones and the <code>capacity-optimized</code> allocation strategy. If you request a Spot placement score for Availability Zones, a high score assumes that your fleet request will be configured to use a single Availability Zone and the <code>capacity-optimized</code> allocation strategy.</p>
    /// <p>Different  Regions or Availability Zones might return the same score.</p> <note>
    /// <p>The Spot placement score serves as a recommendation only. No score guarantees that your Spot request will be fully or partially fulfilled.</p>
    /// </note>
    pub fn spot_placement_scores(mut self, input: crate::types::SpotPlacementScore) -> Self {
        let mut v = self.spot_placement_scores.unwrap_or_default();
        v.push(input);
        self.spot_placement_scores = Some(v);
        self
    }
    /// <p>The Spot placement score for the top 10 Regions or Availability Zones, scored on a scale from 1 to 10. Each score  reflects how likely it is that each Region or Availability Zone will succeed at fulfilling the specified target capacity  <i>at the time of the Spot placement score request</i>. A score of <code>10</code> means that your Spot capacity request is highly likely to succeed in that Region or Availability Zone. </p>
    /// <p>If you request a Spot placement score for Regions, a high score assumes that your fleet request will be configured to use all Availability Zones and the <code>capacity-optimized</code> allocation strategy. If you request a Spot placement score for Availability Zones, a high score assumes that your fleet request will be configured to use a single Availability Zone and the <code>capacity-optimized</code> allocation strategy.</p>
    /// <p>Different  Regions or Availability Zones might return the same score.</p> <note>
    /// <p>The Spot placement score serves as a recommendation only. No score guarantees that your Spot request will be fully or partially fulfilled.</p>
    /// </note>
    pub fn set_spot_placement_scores(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::SpotPlacementScore>>,
    ) -> Self {
        self.spot_placement_scores = input;
        self
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
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
    /// Consumes the builder and constructs a [`GetSpotPlacementScoresOutput`](crate::operation::get_spot_placement_scores::GetSpotPlacementScoresOutput).
    pub fn build(
        self,
    ) -> crate::operation::get_spot_placement_scores::GetSpotPlacementScoresOutput {
        crate::operation::get_spot_placement_scores::GetSpotPlacementScoresOutput {
            spot_placement_scores: self.spot_placement_scores,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}