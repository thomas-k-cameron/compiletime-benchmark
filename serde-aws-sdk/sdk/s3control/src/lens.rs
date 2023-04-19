// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_access_points_output_next_token(
    input: &crate::operation::list_access_points::ListAccessPointsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_access_points_for_object_lambda_output_next_token(
    input: &crate::operation::list_access_points_for_object_lambda::ListAccessPointsForObjectLambdaOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_jobs_output_next_token(
    input: &crate::operation::list_jobs::ListJobsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_multi_region_access_points_output_next_token(
    input: &crate::operation::list_multi_region_access_points::ListMultiRegionAccessPointsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_regional_buckets_output_next_token(
    input: &crate::operation::list_regional_buckets::ListRegionalBucketsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_storage_lens_configurations_output_next_token(
    input: &crate::operation::list_storage_lens_configurations::ListStorageLensConfigurationsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_access_points_for_object_lambda_output_object_lambda_access_point_list(
    input: crate::operation::list_access_points_for_object_lambda::ListAccessPointsForObjectLambdaOutput,
) -> std::option::Option<std::vec::Vec<crate::types::ObjectLambdaAccessPoint>> {
    let input = match input.object_lambda_access_point_list {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}