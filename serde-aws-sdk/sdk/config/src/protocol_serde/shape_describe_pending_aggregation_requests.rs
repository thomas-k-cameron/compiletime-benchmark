// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_pending_aggregation_requests_input(
    input: &crate::operation::describe_pending_aggregation_requests::DescribePendingAggregationRequestsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_describe_pending_aggregation_requests_input::ser_describe_pending_aggregation_requests_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_pending_aggregation_requests_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::describe_pending_aggregation_requests::DescribePendingAggregationRequestsOutput, crate::operation::describe_pending_aggregation_requests::DescribePendingAggregationRequestsError>{
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::describe_pending_aggregation_requests::DescribePendingAggregationRequestsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::describe_pending_aggregation_requests::DescribePendingAggregationRequestsError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidLimitException" => crate::operation::describe_pending_aggregation_requests::DescribePendingAggregationRequestsError::InvalidLimitException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidLimitExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::describe_pending_aggregation_requests::DescribePendingAggregationRequestsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidNextTokenException" => crate::operation::describe_pending_aggregation_requests::DescribePendingAggregationRequestsError::InvalidNextTokenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidNextTokenExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::describe_pending_aggregation_requests::DescribePendingAggregationRequestsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterValueException" => crate::operation::describe_pending_aggregation_requests::DescribePendingAggregationRequestsError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::describe_pending_aggregation_requests::DescribePendingAggregationRequestsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::describe_pending_aggregation_requests::DescribePendingAggregationRequestsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_pending_aggregation_requests_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::describe_pending_aggregation_requests::DescribePendingAggregationRequestsOutput, crate::operation::describe_pending_aggregation_requests::DescribePendingAggregationRequestsError>{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_pending_aggregation_requests::builders::DescribePendingAggregationRequestsOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_pending_aggregation_requests::de_describe_pending_aggregation_requests(response.body().as_ref(), output).map_err(crate::operation::describe_pending_aggregation_requests::DescribePendingAggregationRequestsError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_describe_pending_aggregation_requests(value: &[u8], mut builder: crate::operation::describe_pending_aggregation_requests::builders::DescribePendingAggregationRequestsOutputBuilder) -> Result<crate::operation::describe_pending_aggregation_requests::builders::DescribePendingAggregationRequestsOutputBuilder, aws_smithy_json::deserialize::error::DeserializeError>{
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
                    "PendingAggregationRequests" => {
                        builder = builder.set_pending_aggregation_requests(
                            crate::protocol_serde::shape_pending_aggregation_request_list::de_pending_aggregation_request_list(tokens)?
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
