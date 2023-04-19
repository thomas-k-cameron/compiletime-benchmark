// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_traffic_policy_instance_op_input(
    input: &crate::operation::update_traffic_policy_instance::UpdateTrafficPolicyInstanceInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("UpdateTrafficPolicyInstanceRequest")
            .write_ns("https://route53.amazonaws.com/doc/2013-04-01/", None);
        crate::protocol_serde::shape_update_traffic_policy_instance_input::ser_update_traffic_policy_instance_input_input(input, root)?
    }
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_traffic_policy_instance_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::update_traffic_policy_instance::UpdateTrafficPolicyInstanceOutput,
    crate::operation::update_traffic_policy_instance::UpdateTrafficPolicyInstanceError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::update_traffic_policy_instance::UpdateTrafficPolicyInstanceError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::update_traffic_policy_instance::UpdateTrafficPolicyInstanceError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConflictingTypes" => crate::operation::update_traffic_policy_instance::UpdateTrafficPolicyInstanceError::ConflictingTypes({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ConflictingTypesBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_conflicting_types::de_conflicting_types_xml_err(response.body().as_ref(), output).map_err(crate::operation::update_traffic_policy_instance::UpdateTrafficPolicyInstanceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInput" => crate::operation::update_traffic_policy_instance::UpdateTrafficPolicyInstanceError::InvalidInput({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidInputBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input::de_invalid_input_xml_err(response.body().as_ref(), output).map_err(crate::operation::update_traffic_policy_instance::UpdateTrafficPolicyInstanceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchTrafficPolicy" => crate::operation::update_traffic_policy_instance::UpdateTrafficPolicyInstanceError::NoSuchTrafficPolicy({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchTrafficPolicyBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_traffic_policy::de_no_such_traffic_policy_xml_err(response.body().as_ref(), output).map_err(crate::operation::update_traffic_policy_instance::UpdateTrafficPolicyInstanceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchTrafficPolicyInstance" => crate::operation::update_traffic_policy_instance::UpdateTrafficPolicyInstanceError::NoSuchTrafficPolicyInstance({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchTrafficPolicyInstanceBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_traffic_policy_instance::de_no_such_traffic_policy_instance_xml_err(response.body().as_ref(), output).map_err(crate::operation::update_traffic_policy_instance::UpdateTrafficPolicyInstanceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "PriorRequestNotComplete" => crate::operation::update_traffic_policy_instance::UpdateTrafficPolicyInstanceError::PriorRequestNotComplete({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::PriorRequestNotCompleteBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_prior_request_not_complete::de_prior_request_not_complete_xml_err(response.body().as_ref(), output).map_err(crate::operation::update_traffic_policy_instance::UpdateTrafficPolicyInstanceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::update_traffic_policy_instance::UpdateTrafficPolicyInstanceError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_traffic_policy_instance_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::update_traffic_policy_instance::UpdateTrafficPolicyInstanceOutput,
    crate::operation::update_traffic_policy_instance::UpdateTrafficPolicyInstanceError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::update_traffic_policy_instance::builders::UpdateTrafficPolicyInstanceOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_update_traffic_policy_instance::de_update_traffic_policy_instance(response.body().as_ref(), output).map_err(crate::operation::update_traffic_policy_instance::UpdateTrafficPolicyInstanceError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_update_traffic_policy_instance(inp: &[u8], mut builder: crate::operation::update_traffic_policy_instance::builders::UpdateTrafficPolicyInstanceOutputBuilder) -> Result<crate::operation::update_traffic_policy_instance::builders::UpdateTrafficPolicyInstanceOutputBuilder, aws_smithy_xml::decode::XmlDecodeError>{
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !start_el.matches("UpdateTrafficPolicyInstanceResponse") {
        return Err(
                                aws_smithy_xml::decode::XmlDecodeError::custom(
                                    format!("encountered invalid XML root: expected UpdateTrafficPolicyInstanceResponse but got {:?}. This is likely a bug in the SDK.", start_el)
                                )
                            );
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("TrafficPolicyInstance") /* TrafficPolicyInstance com.amazonaws.route53.synthetic#UpdateTrafficPolicyInstanceOutput$TrafficPolicyInstance */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_traffic_policy_instance::de_traffic_policy_instance(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_traffic_policy_instance(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
