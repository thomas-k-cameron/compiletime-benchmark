// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_destination(
    input: &crate::types::Destination,
    writer: aws_smithy_xml::encode::ElWriter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.account {
        let mut inner_writer = scope.start_el("Account").finish();
        inner_writer.data(var_1.as_str());
    }
    if let Some(var_2) = &input.bucket {
        let mut inner_writer = scope.start_el("Bucket").finish();
        inner_writer.data(var_2.as_str());
    }
    if let Some(var_3) = &input.replication_time {
        let inner_writer = scope.start_el("ReplicationTime");
        crate::protocol_serde::shape_replication_time::ser_replication_time(var_3, inner_writer)?
    }
    if let Some(var_4) = &input.access_control_translation {
        let inner_writer = scope.start_el("AccessControlTranslation");
        crate::protocol_serde::shape_access_control_translation::ser_access_control_translation(
            var_4,
            inner_writer,
        )?
    }
    if let Some(var_5) = &input.encryption_configuration {
        let inner_writer = scope.start_el("EncryptionConfiguration");
        crate::protocol_serde::shape_encryption_configuration::ser_encryption_configuration(
            var_5,
            inner_writer,
        )?
    }
    if let Some(var_6) = &input.metrics {
        let inner_writer = scope.start_el("Metrics");
        crate::protocol_serde::shape_metrics::ser_metrics(var_6, inner_writer)?
    }
    if let Some(var_7) = &input.storage_class {
        let mut inner_writer = scope.start_el("StorageClass").finish();
        inner_writer.data(var_7.as_str());
    }
    scope.finish();
    Ok(())
}

pub fn de_destination(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::Destination, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::Destination::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Account") /* Account com.amazonaws.s3control#Destination$Account */ =>  {
                let var_8 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_account(var_8);
            }
            ,
            s if s.matches("Bucket") /* Bucket com.amazonaws.s3control#Destination$Bucket */ =>  {
                let var_9 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_bucket(var_9);
            }
            ,
            s if s.matches("ReplicationTime") /* ReplicationTime com.amazonaws.s3control#Destination$ReplicationTime */ =>  {
                let var_10 =
                    Some(
                        crate::protocol_serde::shape_replication_time::de_replication_time(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_replication_time(var_10);
            }
            ,
            s if s.matches("AccessControlTranslation") /* AccessControlTranslation com.amazonaws.s3control#Destination$AccessControlTranslation */ =>  {
                let var_11 =
                    Some(
                        crate::protocol_serde::shape_access_control_translation::de_access_control_translation(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_access_control_translation(var_11);
            }
            ,
            s if s.matches("EncryptionConfiguration") /* EncryptionConfiguration com.amazonaws.s3control#Destination$EncryptionConfiguration */ =>  {
                let var_12 =
                    Some(
                        crate::protocol_serde::shape_encryption_configuration::de_encryption_configuration(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_encryption_configuration(var_12);
            }
            ,
            s if s.matches("Metrics") /* Metrics com.amazonaws.s3control#Destination$Metrics */ =>  {
                let var_13 =
                    Some(
                        crate::protocol_serde::shape_metrics::de_metrics(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_metrics(var_13);
            }
            ,
            s if s.matches("StorageClass") /* StorageClass com.amazonaws.s3control#Destination$StorageClass */ =>  {
                let var_14 =
                    Some(
                        Result::<crate::types::ReplicationStorageClass, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ReplicationStorageClass::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_storage_class(var_14);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
