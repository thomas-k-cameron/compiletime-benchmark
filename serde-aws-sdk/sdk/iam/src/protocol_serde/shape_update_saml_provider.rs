// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_update_saml_provider_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::update_saml_provider::UpdateSamlProviderOutput,
    crate::operation::update_saml_provider::UpdateSAMLProviderError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::update_saml_provider::UpdateSAMLProviderError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::update_saml_provider::UpdateSAMLProviderError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidInput" => {
            crate::operation::update_saml_provider::UpdateSAMLProviderError::InvalidInputException(
                {
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::types::error::builders::InvalidInputExceptionBuilder::default();
                        let _ = response;
                        output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::update_saml_provider::UpdateSAMLProviderError::unhandled)?;
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
        "LimitExceeded" => {
            crate::operation::update_saml_provider::UpdateSAMLProviderError::LimitExceededException(
                {
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::types::error::builders::LimitExceededExceptionBuilder::default();
                        let _ = response;
                        output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::update_saml_provider::UpdateSAMLProviderError::unhandled)?;
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
        "NoSuchEntity" => {
            crate::operation::update_saml_provider::UpdateSAMLProviderError::NoSuchEntityException(
                {
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::types::error::builders::NoSuchEntityExceptionBuilder::default();
                        let _ = response;
                        output = crate::protocol_serde::shape_no_such_entity_exception::de_no_such_entity_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::update_saml_provider::UpdateSAMLProviderError::unhandled)?;
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
        "ServiceFailure" => {
            crate::operation::update_saml_provider::UpdateSAMLProviderError::ServiceFailureException(
                {
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::types::error::builders::ServiceFailureExceptionBuilder::default(
                            );
                        let _ = response;
                        output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::update_saml_provider::UpdateSAMLProviderError::unhandled)?;
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
        _ => crate::operation::update_saml_provider::UpdateSAMLProviderError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_saml_provider_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::update_saml_provider::UpdateSamlProviderOutput,
    crate::operation::update_saml_provider::UpdateSAMLProviderError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::update_saml_provider::builders::UpdateSamlProviderOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_update_saml_provider::de_update_saml_provider(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::update_saml_provider::UpdateSAMLProviderError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_update_saml_provider(
    inp: &[u8],
    mut builder: crate::operation::update_saml_provider::builders::UpdateSamlProviderOutputBuilder,
) -> Result<
    crate::operation::update_saml_provider::builders::UpdateSamlProviderOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("UpdateSAMLProviderResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected UpdateSAMLProviderResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("UpdateSAMLProviderResult")) {
            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected UpdateSAMLProviderResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("SAMLProviderArn") /* SAMLProviderArn com.amazonaws.iam.synthetic#UpdateSAMLProviderOutput$SAMLProviderArn */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_saml_provider_arn(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected UpdateSAMLProviderResult tag",
        ));
    };
    Ok(builder)
}
