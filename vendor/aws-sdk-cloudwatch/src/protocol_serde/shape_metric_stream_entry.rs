// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_metric_stream_entry(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::MetricStreamEntry, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::MetricStreamEntry::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Arn") /* Arn com.amazonaws.cloudwatch#MetricStreamEntry$Arn */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_arn(var_1);
            }
            ,
            s if s.matches("CreationDate") /* CreationDate com.amazonaws.cloudwatch#MetricStreamEntry$CreationDate */ =>  {
                let var_2 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.cloudwatch#Timestamp`)"))
                        ?
                    )
                ;
                builder = builder.set_creation_date(var_2);
            }
            ,
            s if s.matches("LastUpdateDate") /* LastUpdateDate com.amazonaws.cloudwatch#MetricStreamEntry$LastUpdateDate */ =>  {
                let var_3 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.cloudwatch#Timestamp`)"))
                        ?
                    )
                ;
                builder = builder.set_last_update_date(var_3);
            }
            ,
            s if s.matches("Name") /* Name com.amazonaws.cloudwatch#MetricStreamEntry$Name */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_name(var_4);
            }
            ,
            s if s.matches("FirehoseArn") /* FirehoseArn com.amazonaws.cloudwatch#MetricStreamEntry$FirehoseArn */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_firehose_arn(var_5);
            }
            ,
            s if s.matches("State") /* State com.amazonaws.cloudwatch#MetricStreamEntry$State */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_6);
            }
            ,
            s if s.matches("OutputFormat") /* OutputFormat com.amazonaws.cloudwatch#MetricStreamEntry$OutputFormat */ =>  {
                let var_7 =
                    Some(
                        Result::<crate::types::MetricStreamOutputFormat, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::MetricStreamOutputFormat::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_output_format(var_7);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
