// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_execute_command_log_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ExecuteCommandLogConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.cloud_watch_log_group_name {
        object.key("cloudWatchLogGroupName").string(var_1.as_str());
    }
    if input.cloud_watch_encryption_enabled {
        object
            .key("cloudWatchEncryptionEnabled")
            .boolean(input.cloud_watch_encryption_enabled);
    }
    if let Some(var_2) = &input.s3_bucket_name {
        object.key("s3BucketName").string(var_2.as_str());
    }
    if input.s3_encryption_enabled {
        object
            .key("s3EncryptionEnabled")
            .boolean(input.s3_encryption_enabled);
    }
    if let Some(var_3) = &input.s3_key_prefix {
        object.key("s3KeyPrefix").string(var_3.as_str());
    }
    Ok(())
}

pub(crate) fn de_execute_command_log_configuration<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::ExecuteCommandLogConfiguration>,
    aws_smithy_json::deserialize::error::DeserializeError,
>
where
    I: Iterator<
        Item = Result<
            aws_smithy_json::deserialize::Token<'a>,
            aws_smithy_json::deserialize::error::DeserializeError,
        >,
    >,
{
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder =
                crate::types::builders::ExecuteCommandLogConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "cloudWatchLogGroupName" => {
                                builder = builder.set_cloud_watch_log_group_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "cloudWatchEncryptionEnabled" => {
                                builder = builder.set_cloud_watch_encryption_enabled(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(
                                        tokens.next(),
                                    )?,
                                );
                            }
                            "s3BucketName" => {
                                builder = builder.set_s3_bucket_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "s3EncryptionEnabled" => {
                                builder = builder.set_s3_encryption_enabled(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(
                                        tokens.next(),
                                    )?,
                                );
                            }
                            "s3KeyPrefix" => {
                                builder = builder.set_s3_key_prefix(
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
            Ok(Some(builder.build()))
        }
        _ => Err(
            aws_smithy_json::deserialize::error::DeserializeError::custom(
                "expected start object or null",
            ),
        ),
    }
}
