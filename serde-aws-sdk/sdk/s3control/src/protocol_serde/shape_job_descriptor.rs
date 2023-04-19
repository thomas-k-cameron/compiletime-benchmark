// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_job_descriptor(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::JobDescriptor, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::JobDescriptor::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("JobId") /* JobId com.amazonaws.s3control#JobDescriptor$JobId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_job_id(var_1);
            }
            ,
            s if s.matches("ConfirmationRequired") /* ConfirmationRequired com.amazonaws.s3control#JobDescriptor$ConfirmationRequired */ =>  {
                let var_2 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.s3control#ConfirmationRequired`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_confirmation_required(var_2);
            }
            ,
            s if s.matches("Description") /* Description com.amazonaws.s3control#JobDescriptor$Description */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_3);
            }
            ,
            s if s.matches("JobArn") /* JobArn com.amazonaws.s3control#JobDescriptor$JobArn */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_job_arn(var_4);
            }
            ,
            s if s.matches("Status") /* Status com.amazonaws.s3control#JobDescriptor$Status */ =>  {
                let var_5 =
                    Some(
                        Result::<crate::types::JobStatus, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::JobStatus::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_5);
            }
            ,
            s if s.matches("Manifest") /* Manifest com.amazonaws.s3control#JobDescriptor$Manifest */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_job_manifest::de_job_manifest(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_manifest(var_6);
            }
            ,
            s if s.matches("Operation") /* Operation com.amazonaws.s3control#JobDescriptor$Operation */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_job_operation::de_job_operation(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_operation(var_7);
            }
            ,
            s if s.matches("Priority") /* Priority com.amazonaws.s3control#JobDescriptor$Priority */ =>  {
                let var_8 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.s3control#JobPriority`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_priority(var_8);
            }
            ,
            s if s.matches("ProgressSummary") /* ProgressSummary com.amazonaws.s3control#JobDescriptor$ProgressSummary */ =>  {
                let var_9 =
                    Some(
                        crate::protocol_serde::shape_job_progress_summary::de_job_progress_summary(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_progress_summary(var_9);
            }
            ,
            s if s.matches("StatusUpdateReason") /* StatusUpdateReason com.amazonaws.s3control#JobDescriptor$StatusUpdateReason */ =>  {
                let var_10 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_status_update_reason(var_10);
            }
            ,
            s if s.matches("FailureReasons") /* FailureReasons com.amazonaws.s3control#JobDescriptor$FailureReasons */ =>  {
                let var_11 =
                    Some(
                        crate::protocol_serde::shape_job_failure_list::de_job_failure_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_failure_reasons(var_11);
            }
            ,
            s if s.matches("Report") /* Report com.amazonaws.s3control#JobDescriptor$Report */ =>  {
                let var_12 =
                    Some(
                        crate::protocol_serde::shape_job_report::de_job_report(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_report(var_12);
            }
            ,
            s if s.matches("CreationTime") /* CreationTime com.amazonaws.s3control#JobDescriptor$CreationTime */ =>  {
                let var_13 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.s3control#JobCreationTime`)"))
                        ?
                    )
                ;
                builder = builder.set_creation_time(var_13);
            }
            ,
            s if s.matches("TerminationDate") /* TerminationDate com.amazonaws.s3control#JobDescriptor$TerminationDate */ =>  {
                let var_14 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.s3control#JobTerminationDate`)"))
                        ?
                    )
                ;
                builder = builder.set_termination_date(var_14);
            }
            ,
            s if s.matches("RoleArn") /* RoleArn com.amazonaws.s3control#JobDescriptor$RoleArn */ =>  {
                let var_15 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_role_arn(var_15);
            }
            ,
            s if s.matches("SuspendedDate") /* SuspendedDate com.amazonaws.s3control#JobDescriptor$SuspendedDate */ =>  {
                let var_16 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.s3control#SuspendedDate`)"))
                        ?
                    )
                ;
                builder = builder.set_suspended_date(var_16);
            }
            ,
            s if s.matches("SuspendedCause") /* SuspendedCause com.amazonaws.s3control#JobDescriptor$SuspendedCause */ =>  {
                let var_17 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_suspended_cause(var_17);
            }
            ,
            s if s.matches("ManifestGenerator") /* ManifestGenerator com.amazonaws.s3control#JobDescriptor$ManifestGenerator */ =>  {
                let var_18 =
                    Some(
                        crate::protocol_serde::shape_job_manifest_generator::de_job_manifest_generator(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_manifest_generator(var_18);
            }
            ,
            s if s.matches("GeneratedManifestDescriptor") /* GeneratedManifestDescriptor com.amazonaws.s3control#JobDescriptor$GeneratedManifestDescriptor */ =>  {
                let var_19 =
                    Some(
                        crate::protocol_serde::shape_s3_generated_manifest_descriptor::de_s3_generated_manifest_descriptor(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_generated_manifest_descriptor(var_19);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}