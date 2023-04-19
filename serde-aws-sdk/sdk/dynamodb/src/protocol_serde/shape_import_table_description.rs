// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_import_table_description<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::ImportTableDescription>,
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
            let mut builder = crate::types::builders::ImportTableDescriptionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ImportArn" => {
                                builder = builder.set_import_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "ImportStatus" => {
                                builder = builder.set_import_status(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::ImportStatus::from(u.as_ref()))
                                    })
                                    .transpose()?,
                                );
                            }
                            "TableArn" => {
                                builder = builder.set_table_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "TableId" => {
                                builder = builder.set_table_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "ClientToken" => {
                                builder = builder.set_client_token(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "S3BucketSource" => {
                                builder = builder.set_s3_bucket_source(
                                    crate::protocol_serde::shape_s3_bucket_source::de_s3_bucket_source(tokens)?
                                );
                            }
                            "ErrorCount" => {
                                builder = builder.set_error_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i64::try_from)
                                    .transpose()?,
                                );
                            }
                            "CloudWatchLogGroupArn" => {
                                builder = builder.set_cloud_watch_log_group_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "InputFormat" => {
                                builder = builder.set_input_format(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::InputFormat::from(u.as_ref()))
                                    })
                                    .transpose()?,
                                );
                            }
                            "InputFormatOptions" => {
                                builder = builder.set_input_format_options(
                                    crate::protocol_serde::shape_input_format_options::de_input_format_options(tokens)?
                                );
                            }
                            "InputCompressionType" => {
                                builder = builder.set_input_compression_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::InputCompressionType::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "TableCreationParameters" => {
                                builder = builder.set_table_creation_parameters(
                                    crate::protocol_serde::shape_table_creation_parameters::de_table_creation_parameters(tokens)?
                                );
                            }
                            "StartTime" => {
                                builder = builder.set_start_time(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                        tokens.next(),
                                        aws_smithy_types::date_time::Format::EpochSeconds,
                                    )?,
                                );
                            }
                            "EndTime" => {
                                builder = builder.set_end_time(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                        tokens.next(),
                                        aws_smithy_types::date_time::Format::EpochSeconds,
                                    )?,
                                );
                            }
                            "ProcessedSizeBytes" => {
                                builder = builder.set_processed_size_bytes(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i64::try_from)
                                    .transpose()?,
                                );
                            }
                            "ProcessedItemCount" => {
                                builder = builder.set_processed_item_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i64::try_from)
                                    .transpose()?,
                                );
                            }
                            "ImportedItemCount" => {
                                builder = builder.set_imported_item_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i64::try_from)
                                    .transpose()?,
                                );
                            }
                            "FailureCode" => {
                                builder = builder.set_failure_code(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "FailureMessage" => {
                                builder = builder.set_failure_message(
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