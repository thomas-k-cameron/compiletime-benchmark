// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_storage_lens_data_export(
    input: &crate::types::StorageLensDataExport,
    writer: aws_smithy_xml::encode::ElWriter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.s3_bucket_destination {
        let inner_writer = scope.start_el("S3BucketDestination");
        crate::protocol_serde::shape_s3_bucket_destination::ser_s3_bucket_destination(
            var_1,
            inner_writer,
        )?
    }
    if let Some(var_2) = &input.cloud_watch_metrics {
        let inner_writer = scope.start_el("CloudWatchMetrics");
        crate::protocol_serde::shape_cloud_watch_metrics::ser_cloud_watch_metrics(
            var_2,
            inner_writer,
        )?
    }
    scope.finish();
    Ok(())
}

pub fn de_storage_lens_data_export(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::StorageLensDataExport, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::StorageLensDataExport::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("S3BucketDestination") /* S3BucketDestination com.amazonaws.s3control#StorageLensDataExport$S3BucketDestination */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_s3_bucket_destination::de_s3_bucket_destination(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_s3_bucket_destination(var_3);
            }
            ,
            s if s.matches("CloudWatchMetrics") /* CloudWatchMetrics com.amazonaws.s3control#StorageLensDataExport$CloudWatchMetrics */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_cloud_watch_metrics::de_cloud_watch_metrics(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_cloud_watch_metrics(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
