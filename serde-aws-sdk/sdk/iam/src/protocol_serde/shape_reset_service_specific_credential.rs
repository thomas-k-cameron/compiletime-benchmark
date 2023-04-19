// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_reset_service_specific_credential_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::reset_service_specific_credential::ResetServiceSpecificCredentialOutput,
    crate::operation::reset_service_specific_credential::ResetServiceSpecificCredentialError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::reset_service_specific_credential::ResetServiceSpecificCredentialError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::reset_service_specific_credential::ResetServiceSpecificCredentialError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "NoSuchEntity" => crate::operation::reset_service_specific_credential::ResetServiceSpecificCredentialError::NoSuchEntityException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchEntityExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_entity_exception::de_no_such_entity_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::reset_service_specific_credential::ResetServiceSpecificCredentialError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::reset_service_specific_credential::ResetServiceSpecificCredentialError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_reset_service_specific_credential_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::reset_service_specific_credential::ResetServiceSpecificCredentialOutput,
    crate::operation::reset_service_specific_credential::ResetServiceSpecificCredentialError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::reset_service_specific_credential::builders::ResetServiceSpecificCredentialOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_reset_service_specific_credential::de_reset_service_specific_credential(response.body().as_ref(), output).map_err(crate::operation::reset_service_specific_credential::ResetServiceSpecificCredentialError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_reset_service_specific_credential(inp: &[u8], mut builder: crate::operation::reset_service_specific_credential::builders::ResetServiceSpecificCredentialOutputBuilder) -> Result<crate::operation::reset_service_specific_credential::builders::ResetServiceSpecificCredentialOutputBuilder, aws_smithy_xml::decode::XmlDecodeError>{
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ResetServiceSpecificCredentialResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ResetServiceSpecificCredentialResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("ResetServiceSpecificCredentialResult")) {
            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected ResetServiceSpecificCredentialResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("ServiceSpecificCredential") /* ServiceSpecificCredential com.amazonaws.iam.synthetic#ResetServiceSpecificCredentialOutput$ServiceSpecificCredential */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_service_specific_credential::de_service_specific_credential(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_service_specific_credential(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected ResetServiceSpecificCredentialResult tag",
        ));
    };
    Ok(builder)
}
