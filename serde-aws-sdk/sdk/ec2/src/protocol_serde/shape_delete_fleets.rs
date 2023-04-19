// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_fleets_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_fleets::DeleteFleetsOutput,
    crate::operation::delete_fleets::DeleteFleetsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::delete_fleets::DeleteFleetsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::delete_fleets::DeleteFleetsError::generic(
        generic,
    ))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_fleets_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_fleets::DeleteFleetsOutput,
    crate::operation::delete_fleets::DeleteFleetsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::delete_fleets::builders::DeleteFleetsOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_delete_fleets::de_delete_fleets(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::delete_fleets::DeleteFleetsError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_delete_fleets(
    inp: &[u8],
    mut builder: crate::operation::delete_fleets::builders::DeleteFleetsOutputBuilder,
) -> Result<
    crate::operation::delete_fleets::builders::DeleteFleetsOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DeleteFleetsResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DeleteFleetsResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("successfulFleetDeletionSet") /* SuccessfulFleetDeletions com.amazonaws.ec2.synthetic#DeleteFleetsOutput$SuccessfulFleetDeletions */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_delete_fleet_success_set::de_delete_fleet_success_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_successful_fleet_deletions(var_1);
            }
            ,
            s if s.matches("unsuccessfulFleetDeletionSet") /* UnsuccessfulFleetDeletions com.amazonaws.ec2.synthetic#DeleteFleetsOutput$UnsuccessfulFleetDeletions */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_delete_fleet_error_set::de_delete_fleet_error_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_unsuccessful_fleet_deletions(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
