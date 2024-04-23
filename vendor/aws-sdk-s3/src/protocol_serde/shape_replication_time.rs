// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_replication_time(
    input: &crate::types::ReplicationTime,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    {
        let mut inner_writer = scope.start_el("Status").finish();
        inner_writer.data(input.status.as_str());
    }
    if let Some(var_1) = &input.time {
        let inner_writer = scope.start_el("Time");
        crate::protocol_serde::shape_replication_time_value::ser_replication_time_value(var_1, inner_writer)?
    }
    scope.finish();
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_replication_time(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::ReplicationTime, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ReplicationTime::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Status") /* Status com.amazonaws.s3#ReplicationTime$Status */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::types::ReplicationTimeStatus, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ReplicationTimeStatus::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_2);
            }
            ,
            s if s.matches("Time") /* Time com.amazonaws.s3#ReplicationTime$Time */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_replication_time_value::de_replication_time_value(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_time(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(crate::serde_util::replication_time_correct_errors(builder)
        .build()
        .map_err(|_| ::aws_smithy_xml::decode::XmlDecodeError::custom("missing field"))?)
}
