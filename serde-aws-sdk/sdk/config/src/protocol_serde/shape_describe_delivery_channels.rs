// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_delivery_channels_input(
    input: &crate::operation::describe_delivery_channels::DescribeDeliveryChannelsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_describe_delivery_channels_input::ser_describe_delivery_channels_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_delivery_channels_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::describe_delivery_channels::DescribeDeliveryChannelsOutput,
    crate::operation::describe_delivery_channels::DescribeDeliveryChannelsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::describe_delivery_channels::DescribeDeliveryChannelsError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(
            crate::operation::describe_delivery_channels::DescribeDeliveryChannelsError::unhandled(
                generic,
            ),
        ),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "NoSuchDeliveryChannelException" => crate::operation::describe_delivery_channels::DescribeDeliveryChannelsError::NoSuchDeliveryChannelException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchDeliveryChannelExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_delivery_channel_exception::de_no_such_delivery_channel_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::describe_delivery_channels::DescribeDeliveryChannelsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::describe_delivery_channels::DescribeDeliveryChannelsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_delivery_channels_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::describe_delivery_channels::DescribeDeliveryChannelsOutput,
    crate::operation::describe_delivery_channels::DescribeDeliveryChannelsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_delivery_channels::builders::DescribeDeliveryChannelsOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_delivery_channels::de_describe_delivery_channels(response.body().as_ref(), output).map_err(crate::operation::describe_delivery_channels::DescribeDeliveryChannelsError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_describe_delivery_channels(
    value: &[u8],
    mut builder: crate::operation::describe_delivery_channels::builders::DescribeDeliveryChannelsOutputBuilder,
) -> Result<
    crate::operation::describe_delivery_channels::builders::DescribeDeliveryChannelsOutputBuilder,
    aws_smithy_json::deserialize::error::DeserializeError,
> {
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
                    "DeliveryChannels" => {
                        builder = builder.set_delivery_channels(
                            crate::protocol_serde::shape_delivery_channel_list::de_delivery_channel_list(tokens)?
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