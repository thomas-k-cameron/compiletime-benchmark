// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_hosted_zones_by_vpc_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::list_hosted_zones_by_vpc::ListHostedZonesByVpcOutput,
    crate::operation::list_hosted_zones_by_vpc::ListHostedZonesByVPCError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::list_hosted_zones_by_vpc::ListHostedZonesByVPCError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code =
        match generic.code() {
            Some(code) => code,
            None => return Err(
                crate::operation::list_hosted_zones_by_vpc::ListHostedZonesByVPCError::unhandled(
                    generic,
                ),
            ),
        };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidInput" => crate::operation::list_hosted_zones_by_vpc::ListHostedZonesByVPCError::InvalidInput({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidInputBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input::de_invalid_input_xml_err(response.body().as_ref(), output).map_err(crate::operation::list_hosted_zones_by_vpc::ListHostedZonesByVPCError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidPaginationToken" => crate::operation::list_hosted_zones_by_vpc::ListHostedZonesByVPCError::InvalidPaginationToken({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidPaginationTokenBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_pagination_token::de_invalid_pagination_token_xml_err(response.body().as_ref(), output).map_err(crate::operation::list_hosted_zones_by_vpc::ListHostedZonesByVPCError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::list_hosted_zones_by_vpc::ListHostedZonesByVPCError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_hosted_zones_by_vpc_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::list_hosted_zones_by_vpc::ListHostedZonesByVpcOutput,
    crate::operation::list_hosted_zones_by_vpc::ListHostedZonesByVPCError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::list_hosted_zones_by_vpc::builders::ListHostedZonesByVpcOutputBuilder::default();
        let _ = response;
        output =
            crate::protocol_serde::shape_list_hosted_zones_by_vpc::de_list_hosted_zones_by_vpc(
                response.body().as_ref(),
                output,
            )
            .map_err(
                crate::operation::list_hosted_zones_by_vpc::ListHostedZonesByVPCError::unhandled,
            )?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_list_hosted_zones_by_vpc(
    inp: &[u8],
    mut builder: crate::operation::list_hosted_zones_by_vpc::builders::ListHostedZonesByVpcOutputBuilder,
) -> Result<
    crate::operation::list_hosted_zones_by_vpc::builders::ListHostedZonesByVpcOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !start_el.matches("ListHostedZonesByVPCResponse") {
        return Err(
                                aws_smithy_xml::decode::XmlDecodeError::custom(
                                    format!("encountered invalid XML root: expected ListHostedZonesByVPCResponse but got {:?}. This is likely a bug in the SDK.", start_el)
                                )
                            );
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("NextToken") /* NextToken com.amazonaws.route53.synthetic#ListHostedZonesByVPCOutput$NextToken */ =>  {
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
            s if s.matches("HostedZoneSummaries") /* HostedZoneSummaries com.amazonaws.route53.synthetic#ListHostedZonesByVPCOutput$HostedZoneSummaries */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_hosted_zone_summaries::de_hosted_zone_summaries(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_hosted_zone_summaries(var_2);
            }
            ,
            s if s.matches("MaxItems") /* MaxItems com.amazonaws.route53.synthetic#ListHostedZonesByVPCOutput$MaxItems */ =>  {
                let var_3 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `smithy.api#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_max_items(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
