// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_verified_access_logs(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::VerifiedAccessLogs, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::VerifiedAccessLogs::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("s3") /* S3 com.amazonaws.ec2#VerifiedAccessLogs$S3 */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_verified_access_log_s3_destination::de_verified_access_log_s3_destination(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_s3(var_1);
            }
            ,
            s if s.matches("cloudWatchLogs") /* CloudWatchLogs com.amazonaws.ec2#VerifiedAccessLogs$CloudWatchLogs */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_verified_access_log_cloud_watch_logs_destination::de_verified_access_log_cloud_watch_logs_destination(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_cloud_watch_logs(var_2);
            }
            ,
            s if s.matches("kinesisDataFirehose") /* KinesisDataFirehose com.amazonaws.ec2#VerifiedAccessLogs$KinesisDataFirehose */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_verified_access_log_kinesis_data_firehose_destination::de_verified_access_log_kinesis_data_firehose_destination(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_kinesis_data_firehose(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
