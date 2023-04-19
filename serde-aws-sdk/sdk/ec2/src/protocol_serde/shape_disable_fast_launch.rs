// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_disable_fast_launch_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::disable_fast_launch::DisableFastLaunchOutput,
    crate::operation::disable_fast_launch::DisableFastLaunchError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::disable_fast_launch::DisableFastLaunchError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::disable_fast_launch::DisableFastLaunchError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_disable_fast_launch_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::disable_fast_launch::DisableFastLaunchOutput,
    crate::operation::disable_fast_launch::DisableFastLaunchError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::disable_fast_launch::builders::DisableFastLaunchOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_disable_fast_launch::de_disable_fast_launch(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::disable_fast_launch::DisableFastLaunchError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_disable_fast_launch(
    inp: &[u8],
    mut builder: crate::operation::disable_fast_launch::builders::DisableFastLaunchOutputBuilder,
) -> Result<
    crate::operation::disable_fast_launch::builders::DisableFastLaunchOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DisableFastLaunchResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DisableFastLaunchResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("imageId") /* ImageId com.amazonaws.ec2.synthetic#DisableFastLaunchOutput$ImageId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_image_id(var_1);
            }
            ,
            s if s.matches("resourceType") /* ResourceType com.amazonaws.ec2.synthetic#DisableFastLaunchOutput$ResourceType */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::types::FastLaunchResourceType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::FastLaunchResourceType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_resource_type(var_2);
            }
            ,
            s if s.matches("snapshotConfiguration") /* SnapshotConfiguration com.amazonaws.ec2.synthetic#DisableFastLaunchOutput$SnapshotConfiguration */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_fast_launch_snapshot_configuration_response::de_fast_launch_snapshot_configuration_response(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_snapshot_configuration(var_3);
            }
            ,
            s if s.matches("launchTemplate") /* LaunchTemplate com.amazonaws.ec2.synthetic#DisableFastLaunchOutput$LaunchTemplate */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_fast_launch_launch_template_specification_response::de_fast_launch_launch_template_specification_response(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_launch_template(var_4);
            }
            ,
            s if s.matches("maxParallelLaunches") /* MaxParallelLaunches com.amazonaws.ec2.synthetic#DisableFastLaunchOutput$MaxParallelLaunches */ =>  {
                let var_5 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_max_parallel_launches(var_5);
            }
            ,
            s if s.matches("ownerId") /* OwnerId com.amazonaws.ec2.synthetic#DisableFastLaunchOutput$OwnerId */ =>  {
                let var_6 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_owner_id(var_6);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2.synthetic#DisableFastLaunchOutput$State */ =>  {
                let var_7 =
                    Some(
                        Result::<crate::types::FastLaunchStateCode, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::FastLaunchStateCode::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_7);
            }
            ,
            s if s.matches("stateTransitionReason") /* StateTransitionReason com.amazonaws.ec2.synthetic#DisableFastLaunchOutput$StateTransitionReason */ =>  {
                let var_8 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_state_transition_reason(var_8);
            }
            ,
            s if s.matches("stateTransitionTime") /* StateTransitionTime com.amazonaws.ec2.synthetic#DisableFastLaunchOutput$StateTransitionTime */ =>  {
                let var_9 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#MillisecondDateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_state_transition_time(var_9);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
