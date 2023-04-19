// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_submit_multi_region_access_point_routes_input_input(
    input: &crate::operation::submit_multi_region_access_point_routes::SubmitMultiRegionAccessPointRoutesInput,
    writer: aws_smithy_xml::encode::ElWriter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.route_updates {
        let mut inner_writer = scope.start_el("RouteUpdates").finish();
        for list_item_2 in var_1 {
            {
                let inner_writer = inner_writer.start_el("Route");
                crate::protocol_serde::shape_multi_region_access_point_route::ser_multi_region_access_point_route(list_item_2, inner_writer)?
            }
        }
    }
    scope.finish();
    Ok(())
}
