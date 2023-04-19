// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_compliance_details_by_resource_input(
    input: &crate::operation::get_compliance_details_by_resource::GetComplianceDetailsByResourceInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_compliance_details_by_resource_input::ser_get_compliance_details_by_resource_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_compliance_details_by_resource_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_compliance_details_by_resource::GetComplianceDetailsByResourceOutput,
    crate::operation::get_compliance_details_by_resource::GetComplianceDetailsByResourceError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::get_compliance_details_by_resource::GetComplianceDetailsByResourceError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::get_compliance_details_by_resource::GetComplianceDetailsByResourceError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidParameterValueException" => crate::operation::get_compliance_details_by_resource::GetComplianceDetailsByResourceError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_compliance_details_by_resource::GetComplianceDetailsByResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::get_compliance_details_by_resource::GetComplianceDetailsByResourceError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_compliance_details_by_resource_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_compliance_details_by_resource::GetComplianceDetailsByResourceOutput,
    crate::operation::get_compliance_details_by_resource::GetComplianceDetailsByResourceError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_compliance_details_by_resource::builders::GetComplianceDetailsByResourceOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_compliance_details_by_resource::de_get_compliance_details_by_resource(response.body().as_ref(), output).map_err(crate::operation::get_compliance_details_by_resource::GetComplianceDetailsByResourceError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_get_compliance_details_by_resource(value: &[u8], mut builder: crate::operation::get_compliance_details_by_resource::builders::GetComplianceDetailsByResourceOutputBuilder) -> Result<crate::operation::get_compliance_details_by_resource::builders::GetComplianceDetailsByResourceOutputBuilder, aws_smithy_json::deserialize::error::DeserializeError>{
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "EvaluationResults" => {
                        builder = builder.set_evaluation_results(
                            crate::protocol_serde::shape_evaluation_results::de_evaluation_results(
                                tokens,
                            )?,
                        );
                    }
                    "NextToken" => {
                        builder = builder.set_next_token(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(
                    aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                        "expected object key or end object, found: {:?}",
                        other
                    )),
                )
            }
        }
    }
    if tokens.next().is_some() {
        return Err(
            aws_smithy_json::deserialize::error::DeserializeError::custom(
                "found more JSON tokens after completing parsing",
            ),
        );
    }
    Ok(builder)
}
