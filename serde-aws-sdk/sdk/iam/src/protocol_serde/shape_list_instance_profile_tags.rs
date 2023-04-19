// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_instance_profile_tags_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::list_instance_profile_tags::ListInstanceProfileTagsOutput,
    crate::operation::list_instance_profile_tags::ListInstanceProfileTagsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::list_instance_profile_tags::ListInstanceProfileTagsError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(
            crate::operation::list_instance_profile_tags::ListInstanceProfileTagsError::unhandled(
                generic,
            ),
        ),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "NoSuchEntity" => crate::operation::list_instance_profile_tags::ListInstanceProfileTagsError::NoSuchEntityException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchEntityExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_entity_exception::de_no_such_entity_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::list_instance_profile_tags::ListInstanceProfileTagsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceFailure" => crate::operation::list_instance_profile_tags::ListInstanceProfileTagsError::ServiceFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceFailureExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::list_instance_profile_tags::ListInstanceProfileTagsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::list_instance_profile_tags::ListInstanceProfileTagsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_instance_profile_tags_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::list_instance_profile_tags::ListInstanceProfileTagsOutput,
    crate::operation::list_instance_profile_tags::ListInstanceProfileTagsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::list_instance_profile_tags::builders::ListInstanceProfileTagsOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_list_instance_profile_tags::de_list_instance_profile_tags(response.body().as_ref(), output).map_err(crate::operation::list_instance_profile_tags::ListInstanceProfileTagsError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_list_instance_profile_tags(
    inp: &[u8],
    mut builder: crate::operation::list_instance_profile_tags::builders::ListInstanceProfileTagsOutputBuilder,
) -> Result<
    crate::operation::list_instance_profile_tags::builders::ListInstanceProfileTagsOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ListInstanceProfileTagsResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ListInstanceProfileTagsResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("ListInstanceProfileTagsResult")) {
            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected ListInstanceProfileTagsResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("Tags") /* Tags com.amazonaws.iam.synthetic#ListInstanceProfileTagsOutput$Tags */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_tag_list_type::de_tag_list_type(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_1);
            }
            ,
            s if s.matches("IsTruncated") /* IsTruncated com.amazonaws.iam.synthetic#ListInstanceProfileTagsOutput$IsTruncated */ =>  {
                let var_2 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.iam#booleanType`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_is_truncated(var_2);
            }
            ,
            s if s.matches("Marker") /* Marker com.amazonaws.iam.synthetic#ListInstanceProfileTagsOutput$Marker */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_marker(var_3);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected ListInstanceProfileTagsResult tag",
        ));
    };
    Ok(builder)
}
