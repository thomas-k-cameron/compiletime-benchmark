// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_hosted_zone_limit_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_hosted_zone_limit::GetHostedZoneLimitOutput,
    crate::operation::get_hosted_zone_limit::GetHostedZoneLimitError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::get_hosted_zone_limit::GetHostedZoneLimitError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::get_hosted_zone_limit::GetHostedZoneLimitError::unhandled(
                    generic,
                ),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "HostedZoneNotPrivate" => {
            crate::operation::get_hosted_zone_limit::GetHostedZoneLimitError::HostedZoneNotPrivate(
                {
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::types::error::builders::HostedZoneNotPrivateBuilder::default();
                        let _ = response;
                        output = crate::protocol_serde::shape_hosted_zone_not_private::de_hosted_zone_not_private_xml_err(response.body().as_ref(), output).map_err(crate::operation::get_hosted_zone_limit::GetHostedZoneLimitError::unhandled)?;
                        let output = output.meta(generic);
                        output.build()
                    };
                    if tmp.message.is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                },
            )
        }
        "InvalidInput" => {
            crate::operation::get_hosted_zone_limit::GetHostedZoneLimitError::InvalidInput({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidInputBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input::de_invalid_input_xml_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(
                        crate::operation::get_hosted_zone_limit::GetHostedZoneLimitError::unhandled,
                    )?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "NoSuchHostedZone" => {
            crate::operation::get_hosted_zone_limit::GetHostedZoneLimitError::NoSuchHostedZone({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::NoSuchHostedZoneBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_hosted_zone::de_no_such_hosted_zone_xml_err(response.body().as_ref(), output).map_err(crate::operation::get_hosted_zone_limit::GetHostedZoneLimitError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::get_hosted_zone_limit::GetHostedZoneLimitError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_hosted_zone_limit_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_hosted_zone_limit::GetHostedZoneLimitOutput,
    crate::operation::get_hosted_zone_limit::GetHostedZoneLimitError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_hosted_zone_limit::builders::GetHostedZoneLimitOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_hosted_zone_limit::de_get_hosted_zone_limit(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::get_hosted_zone_limit::GetHostedZoneLimitError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_hosted_zone_limit(
    inp: &[u8],
    mut builder: crate::operation::get_hosted_zone_limit::builders::GetHostedZoneLimitOutputBuilder,
) -> Result<
    crate::operation::get_hosted_zone_limit::builders::GetHostedZoneLimitOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !start_el.matches("GetHostedZoneLimitResponse") {
        return Err(
                                aws_smithy_xml::decode::XmlDecodeError::custom(
                                    format!("encountered invalid XML root: expected GetHostedZoneLimitResponse but got {:?}. This is likely a bug in the SDK.", start_el)
                                )
                            );
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Limit") /* Limit com.amazonaws.route53.synthetic#GetHostedZoneLimitOutput$Limit */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_hosted_zone_limit::de_hosted_zone_limit(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_limit(var_1);
            }
            ,
            s if s.matches("Count") /* Count com.amazonaws.route53.synthetic#GetHostedZoneLimitOutput$Count */ =>  {
                let var_2 =
                    Some(
                         {
                            <i64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (long: `com.amazonaws.route53#UsageCount`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_count(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
