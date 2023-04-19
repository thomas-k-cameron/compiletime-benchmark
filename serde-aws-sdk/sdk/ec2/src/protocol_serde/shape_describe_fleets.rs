// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_fleets_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::describe_fleets::DescribeFleetsOutput,
    crate::operation::describe_fleets::DescribeFleetsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::describe_fleets::DescribeFleetsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::describe_fleets::DescribeFleetsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_fleets_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::describe_fleets::DescribeFleetsOutput,
    crate::operation::describe_fleets::DescribeFleetsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::describe_fleets::builders::DescribeFleetsOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_fleets::de_describe_fleets(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::describe_fleets::DescribeFleetsError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_fleets(
    inp: &[u8],
    mut builder: crate::operation::describe_fleets::builders::DescribeFleetsOutputBuilder,
) -> Result<
    crate::operation::describe_fleets::builders::DescribeFleetsOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DescribeFleetsResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DescribeFleetsResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("nextToken") /* NextToken com.amazonaws.ec2.synthetic#DescribeFleetsOutput$NextToken */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_token(var_1);
            }
            ,
            s if s.matches("fleetSet") /* Fleets com.amazonaws.ec2.synthetic#DescribeFleetsOutput$Fleets */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_fleet_set::de_fleet_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_fleets(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
