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
pub struct GetSpotPlacementScoresInput {
    /// <p>The instance types. We recommend that you specify at least three instance types. If you specify one or two instance types, or specify variations of a single instance type (for example, an <code>m3.xlarge</code> with and without instance storage), the returned placement score will always be low. </p>
    /// <p>If you specify <code>InstanceTypes</code>, you can't specify <code>InstanceRequirementsWithMetadata</code>.</p>
    #[doc(hidden)]
    pub instance_types: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The target capacity.</p>
    #[doc(hidden)]
    pub target_capacity: std::option::Option<i32>,
    /// <p>The unit for the target capacity.</p>
    /// <p>Default: <code>units</code> (translates to number of instances)</p>
    #[doc(hidden)]
    pub target_capacity_unit_type: std::option::Option<crate::types::TargetCapacityUnitType>,
    /// <p>Specify <code>true</code> so that the response returns a list of scored Availability Zones. Otherwise, the response returns a list of scored Regions.</p>
    /// <p>A list of scored Availability Zones is useful if you want to launch all of your Spot capacity into a single Availability Zone.</p>
    #[doc(hidden)]
    pub single_availability_zone: std::option::Option<bool>,
    /// <p>The Regions used to narrow down the list of Regions to be scored. Enter the Region code, for example, <code>us-east-1</code>.</p>
    #[doc(hidden)]
    pub region_names: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The attributes for the instance types. When you specify instance attributes, Amazon EC2 will identify instance types with those attributes.</p>
    /// <p>If you specify <code>InstanceRequirementsWithMetadata</code>, you can't specify <code>InstanceTypes</code>.</p>
    #[doc(hidden)]
    pub instance_requirements_with_metadata:
        std::option::Option<crate::types::InstanceRequirementsWithMetadataRequest>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    #[doc(hidden)]
    pub max_results: std::option::Option<i32>,
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl GetSpotPlacementScoresInput {
    /// <p>The instance types. We recommend that you specify at least three instance types. If you specify one or two instance types, or specify variations of a single instance type (for example, an <code>m3.xlarge</code> with and without instance storage), the returned placement score will always be low. </p>
    /// <p>If you specify <code>InstanceTypes</code>, you can't specify <code>InstanceRequirementsWithMetadata</code>.</p>
    pub fn instance_types(&self) -> std::option::Option<&[std::string::String]> {
        self.instance_types.as_deref()
    }
    /// <p>The target capacity.</p>
    pub fn target_capacity(&self) -> std::option::Option<i32> {
        self.target_capacity
    }
    /// <p>The unit for the target capacity.</p>
    /// <p>Default: <code>units</code> (translates to number of instances)</p>
    pub fn target_capacity_unit_type(
        &self,
    ) -> std::option::Option<&crate::types::TargetCapacityUnitType> {
        self.target_capacity_unit_type.as_ref()
    }
    /// <p>Specify <code>true</code> so that the response returns a list of scored Availability Zones. Otherwise, the response returns a list of scored Regions.</p>
    /// <p>A list of scored Availability Zones is useful if you want to launch all of your Spot capacity into a single Availability Zone.</p>
    pub fn single_availability_zone(&self) -> std::option::Option<bool> {
        self.single_availability_zone
    }
    /// <p>The Regions used to narrow down the list of Regions to be scored. Enter the Region code, for example, <code>us-east-1</code>.</p>
    pub fn region_names(&self) -> std::option::Option<&[std::string::String]> {
        self.region_names.as_deref()
    }
    /// <p>The attributes for the instance types. When you specify instance attributes, Amazon EC2 will identify instance types with those attributes.</p>
    /// <p>If you specify <code>InstanceRequirementsWithMetadata</code>, you can't specify <code>InstanceTypes</code>.</p>
    pub fn instance_requirements_with_metadata(
        &self,
    ) -> std::option::Option<&crate::types::InstanceRequirementsWithMetadataRequest> {
        self.instance_requirements_with_metadata.as_ref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn max_results(&self) -> std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl GetSpotPlacementScoresInput {
    /// Creates a new builder-style object to manufacture [`GetSpotPlacementScoresInput`](crate::operation::get_spot_placement_scores::GetSpotPlacementScoresInput).
    pub fn builder(
    ) -> crate::operation::get_spot_placement_scores::builders::GetSpotPlacementScoresInputBuilder
    {
        crate::operation::get_spot_placement_scores::builders::GetSpotPlacementScoresInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_spot_placement_scores::GetSpotPlacementScoresInput;
/// A builder for [`GetSpotPlacementScoresInput`](crate::operation::get_spot_placement_scores::GetSpotPlacementScoresInput).
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
pub struct GetSpotPlacementScoresInputBuilder {
    pub(crate) instance_types: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) target_capacity: std::option::Option<i32>,
    pub(crate) target_capacity_unit_type: std::option::Option<crate::types::TargetCapacityUnitType>,
    pub(crate) single_availability_zone: std::option::Option<bool>,
    pub(crate) region_names: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) instance_requirements_with_metadata:
        std::option::Option<crate::types::InstanceRequirementsWithMetadataRequest>,
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) max_results: std::option::Option<i32>,
    pub(crate) next_token: std::option::Option<std::string::String>,
}
impl GetSpotPlacementScoresInputBuilder {
    /// Appends an item to `instance_types`.
    ///
    /// To override the contents of this collection use [`set_instance_types`](Self::set_instance_types).
    ///
    /// <p>The instance types. We recommend that you specify at least three instance types. If you specify one or two instance types, or specify variations of a single instance type (for example, an <code>m3.xlarge</code> with and without instance storage), the returned placement score will always be low. </p>
    /// <p>If you specify <code>InstanceTypes</code>, you can't specify <code>InstanceRequirementsWithMetadata</code>.</p>
    pub fn instance_types(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.instance_types.unwrap_or_default();
        v.push(input.into());
        self.instance_types = Some(v);
        self
    }
    /// <p>The instance types. We recommend that you specify at least three instance types. If you specify one or two instance types, or specify variations of a single instance type (for example, an <code>m3.xlarge</code> with and without instance storage), the returned placement score will always be low. </p>
    /// <p>If you specify <code>InstanceTypes</code>, you can't specify <code>InstanceRequirementsWithMetadata</code>.</p>
    pub fn set_instance_types(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.instance_types = input;
        self
    }
    /// <p>The target capacity.</p>
    pub fn target_capacity(mut self, input: i32) -> Self {
        self.target_capacity = Some(input);
        self
    }
    /// <p>The target capacity.</p>
    pub fn set_target_capacity(mut self, input: std::option::Option<i32>) -> Self {
        self.target_capacity = input;
        self
    }
    /// <p>The unit for the target capacity.</p>
    /// <p>Default: <code>units</code> (translates to number of instances)</p>
    pub fn target_capacity_unit_type(
        mut self,
        input: crate::types::TargetCapacityUnitType,
    ) -> Self {
        self.target_capacity_unit_type = Some(input);
        self
    }
    /// <p>The unit for the target capacity.</p>
    /// <p>Default: <code>units</code> (translates to number of instances)</p>
    pub fn set_target_capacity_unit_type(
        mut self,
        input: std::option::Option<crate::types::TargetCapacityUnitType>,
    ) -> Self {
        self.target_capacity_unit_type = input;
        self
    }
    /// <p>Specify <code>true</code> so that the response returns a list of scored Availability Zones. Otherwise, the response returns a list of scored Regions.</p>
    /// <p>A list of scored Availability Zones is useful if you want to launch all of your Spot capacity into a single Availability Zone.</p>
    pub fn single_availability_zone(mut self, input: bool) -> Self {
        self.single_availability_zone = Some(input);
        self
    }
    /// <p>Specify <code>true</code> so that the response returns a list of scored Availability Zones. Otherwise, the response returns a list of scored Regions.</p>
    /// <p>A list of scored Availability Zones is useful if you want to launch all of your Spot capacity into a single Availability Zone.</p>
    pub fn set_single_availability_zone(mut self, input: std::option::Option<bool>) -> Self {
        self.single_availability_zone = input;
        self
    }
    /// Appends an item to `region_names`.
    ///
    /// To override the contents of this collection use [`set_region_names`](Self::set_region_names).
    ///
    /// <p>The Regions used to narrow down the list of Regions to be scored. Enter the Region code, for example, <code>us-east-1</code>.</p>
    pub fn region_names(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.region_names.unwrap_or_default();
        v.push(input.into());
        self.region_names = Some(v);
        self
    }
    /// <p>The Regions used to narrow down the list of Regions to be scored. Enter the Region code, for example, <code>us-east-1</code>.</p>
    pub fn set_region_names(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.region_names = input;
        self
    }
    /// <p>The attributes for the instance types. When you specify instance attributes, Amazon EC2 will identify instance types with those attributes.</p>
    /// <p>If you specify <code>InstanceRequirementsWithMetadata</code>, you can't specify <code>InstanceTypes</code>.</p>
    pub fn instance_requirements_with_metadata(
        mut self,
        input: crate::types::InstanceRequirementsWithMetadataRequest,
    ) -> Self {
        self.instance_requirements_with_metadata = Some(input);
        self
    }
    /// <p>The attributes for the instance types. When you specify instance attributes, Amazon EC2 will identify instance types with those attributes.</p>
    /// <p>If you specify <code>InstanceRequirementsWithMetadata</code>, you can't specify <code>InstanceTypes</code>.</p>
    pub fn set_instance_requirements_with_metadata(
        mut self,
        input: std::option::Option<crate::types::InstanceRequirementsWithMetadataRequest>,
    ) -> Self {
        self.instance_requirements_with_metadata = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = Some(input);
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`GetSpotPlacementScoresInput`](crate::operation::get_spot_placement_scores::GetSpotPlacementScoresInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::get_spot_placement_scores::GetSpotPlacementScoresInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::get_spot_placement_scores::GetSpotPlacementScoresInput {
                instance_types: self.instance_types,
                target_capacity: self.target_capacity,
                target_capacity_unit_type: self.target_capacity_unit_type,
                single_availability_zone: self.single_availability_zone,
                region_names: self.region_names,
                instance_requirements_with_metadata: self.instance_requirements_with_metadata,
                dry_run: self.dry_run,
                max_results: self.max_results,
                next_token: self.next_token,
            },
        )
    }
}
