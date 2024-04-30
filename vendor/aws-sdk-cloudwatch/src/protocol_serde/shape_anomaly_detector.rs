// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_anomaly_detector(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::AnomalyDetector, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::AnomalyDetector::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Namespace") /* Namespace com.amazonaws.cloudwatch#AnomalyDetector$Namespace */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_namespace(var_1);
            }
            ,
            s if s.matches("MetricName") /* MetricName com.amazonaws.cloudwatch#AnomalyDetector$MetricName */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_metric_name(var_2);
            }
            ,
            s if s.matches("Dimensions") /* Dimensions com.amazonaws.cloudwatch#AnomalyDetector$Dimensions */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_dimensions::de_dimensions(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_dimensions(var_3);
            }
            ,
            s if s.matches("Stat") /* Stat com.amazonaws.cloudwatch#AnomalyDetector$Stat */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_stat(var_4);
            }
            ,
            s if s.matches("Configuration") /* Configuration com.amazonaws.cloudwatch#AnomalyDetector$Configuration */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_anomaly_detector_configuration::de_anomaly_detector_configuration(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_configuration(var_5);
            }
            ,
            s if s.matches("StateValue") /* StateValue com.amazonaws.cloudwatch#AnomalyDetector$StateValue */ =>  {
                let var_6 =
                    Some(
                        Result::<crate::types::AnomalyDetectorStateValue, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::AnomalyDetectorStateValue::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state_value(var_6);
            }
            ,
            s if s.matches("SingleMetricAnomalyDetector") /* SingleMetricAnomalyDetector com.amazonaws.cloudwatch#AnomalyDetector$SingleMetricAnomalyDetector */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_single_metric_anomaly_detector::de_single_metric_anomaly_detector(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_single_metric_anomaly_detector(var_7);
            }
            ,
            s if s.matches("MetricMathAnomalyDetector") /* MetricMathAnomalyDetector com.amazonaws.cloudwatch#AnomalyDetector$MetricMathAnomalyDetector */ =>  {
                let var_8 =
                    Some(
                        crate::protocol_serde::shape_metric_math_anomaly_detector::de_metric_math_anomaly_detector(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_metric_math_anomaly_detector(var_8);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
