// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_job_definition<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::JobDefinition>,
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
            let mut builder = crate::types::builders::JobDefinitionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "jobDefinitionName" => {
                                builder = builder.set_job_definition_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "jobDefinitionArn" => {
                                builder = builder.set_job_definition_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "revision" => {
                                builder = builder.set_revision(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "status" => {
                                builder = builder.set_status(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "type" => {
                                builder = builder.set_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "schedulingPriority" => {
                                builder = builder.set_scheduling_priority(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "parameters" => {
                                builder = builder.set_parameters(
                                    crate::protocol_serde::shape_parameters_map::de_parameters_map(
                                        tokens,
                                    )?,
                                );
                            }
                            "retryStrategy" => {
                                builder = builder.set_retry_strategy(
                                    crate::protocol_serde::shape_retry_strategy::de_retry_strategy(
                                        tokens,
                                    )?,
                                );
                            }
                            "containerProperties" => {
                                builder = builder.set_container_properties(
                                    crate::protocol_serde::shape_container_properties::de_container_properties(tokens)?
                                );
                            }
                            "timeout" => {
                                builder = builder.set_timeout(
                                    crate::protocol_serde::shape_job_timeout::de_job_timeout(
                                        tokens,
                                    )?,
                                );
                            }
                            "nodeProperties" => {
                                builder = builder.set_node_properties(
                                    crate::protocol_serde::shape_node_properties::de_node_properties(tokens)?
                                );
                            }
                            "tags" => {
                                builder = builder.set_tags(
                                    crate::protocol_serde::shape_tagris_tags_map::de_tagris_tags_map(tokens)?
                                );
                            }
                            "propagateTags" => {
                                builder = builder.set_propagate_tags(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(
                                        tokens.next(),
                                    )?,
                                );
                            }
                            "platformCapabilities" => {
                                builder = builder.set_platform_capabilities(
                                    crate::protocol_serde::shape_platform_capability_list::de_platform_capability_list(tokens)?
                                );
                            }
                            "eksProperties" => {
                                builder = builder.set_eks_properties(
                                    crate::protocol_serde::shape_eks_properties::de_eks_properties(
                                        tokens,
                                    )?,
                                );
                            }
                            "containerOrchestrationType" => {
                                builder = builder.set_container_orchestration_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::OrchestrationType::from(u.as_ref())
                                        })
                                    })
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
