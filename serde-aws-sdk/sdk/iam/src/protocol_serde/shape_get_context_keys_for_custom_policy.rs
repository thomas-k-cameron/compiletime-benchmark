// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_context_keys_for_custom_policy_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_context_keys_for_custom_policy::GetContextKeysForCustomPolicyOutput,
    crate::operation::get_context_keys_for_custom_policy::GetContextKeysForCustomPolicyError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::get_context_keys_for_custom_policy::GetContextKeysForCustomPolicyError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::get_context_keys_for_custom_policy::GetContextKeysForCustomPolicyError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidInput" => crate::operation::get_context_keys_for_custom_policy::GetContextKeysForCustomPolicyError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidInputExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::get_context_keys_for_custom_policy::GetContextKeysForCustomPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::get_context_keys_for_custom_policy::GetContextKeysForCustomPolicyError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_context_keys_for_custom_policy_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_context_keys_for_custom_policy::GetContextKeysForCustomPolicyOutput,
    crate::operation::get_context_keys_for_custom_policy::GetContextKeysForCustomPolicyError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_context_keys_for_custom_policy::builders::GetContextKeysForCustomPolicyOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_context_keys_for_custom_policy::de_get_context_keys_for_custom_policy(response.body().as_ref(), output).map_err(crate::operation::get_context_keys_for_custom_policy::GetContextKeysForCustomPolicyError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_context_keys_for_custom_policy(inp: &[u8], mut builder: crate::operation::get_context_keys_for_custom_policy::builders::GetContextKeysForCustomPolicyOutputBuilder) -> Result<crate::operation::get_context_keys_for_custom_policy::builders::GetContextKeysForCustomPolicyOutputBuilder, aws_smithy_xml::decode::XmlDecodeError>{
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("GetContextKeysForCustomPolicyResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected GetContextKeysForCustomPolicyResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("GetContextKeysForCustomPolicyResult")) {
            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected GetContextKeysForCustomPolicyResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("ContextKeyNames") /* ContextKeyNames com.amazonaws.iam.synthetic#GetContextKeysForCustomPolicyOutput$ContextKeyNames */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_context_key_names_result_list_type::de_context_key_names_result_list_type(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_context_key_names(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected GetContextKeysForCustomPolicyResult tag",
        ));
    };
    Ok(builder)
}
