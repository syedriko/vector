// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_parts_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::list_parts::ListPartsOutput, crate::operation::list_parts::ListPartsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::list_parts::ListPartsError::unhandled)?;
    generic_builder = crate::s3_request_id::apply_extended_request_id(generic_builder, _response_headers);
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::list_parts::ListPartsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_parts_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::list_parts::ListPartsOutput, crate::operation::list_parts::ListPartsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::list_parts::builders::ListPartsOutputBuilder::default();
        output = crate::protocol_serde::shape_list_parts::de_list_parts(_response_body, output)
            .map_err(crate::operation::list_parts::ListPartsError::unhandled)?;
        output = output.set_abort_date(
            crate::protocol_serde::shape_list_parts_output::de_abort_date_header(_response_headers)
                .map_err(|_| crate::operation::list_parts::ListPartsError::unhandled("Failed to parse AbortDate from header `x-amz-abort-date"))?,
        );
        output = output.set_abort_rule_id(
            crate::protocol_serde::shape_list_parts_output::de_abort_rule_id_header(_response_headers).map_err(|_| {
                crate::operation::list_parts::ListPartsError::unhandled("Failed to parse AbortRuleId from header `x-amz-abort-rule-id")
            })?,
        );
        output = output.set_request_charged(
            crate::protocol_serde::shape_list_parts_output::de_request_charged_header(_response_headers).map_err(|_| {
                crate::operation::list_parts::ListPartsError::unhandled("Failed to parse RequestCharged from header `x-amz-request-charged")
            })?,
        );
        output._set_extended_request_id(crate::s3_request_id::RequestIdExt::extended_request_id(_response_headers).map(str::to_string));
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_list_parts_headers(
    input: &crate::operation::list_parts::ListPartsInput,
    mut builder: ::http::request::Builder,
) -> std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
    if let ::std::option::Option::Some(inner_1) = &input.request_payer {
        let formatted_2 = inner_1.as_str();
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "request_payer",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-request-payer", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_3) = &input.expected_bucket_owner {
        let formatted_4 = inner_3.as_str();
        if !formatted_4.is_empty() {
            let header_value = formatted_4;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "expected_bucket_owner",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-expected-bucket-owner", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_5) = &input.sse_customer_algorithm {
        let formatted_6 = inner_5.as_str();
        if !formatted_6.is_empty() {
            let header_value = formatted_6;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "sse_customer_algorithm",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-server-side-encryption-customer-algorithm", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_7) = &input.sse_customer_key {
        let formatted_8 = inner_7.as_str();
        if !formatted_8.is_empty() {
            let header_value = formatted_8;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "sse_customer_key",
                    format!("`{}` cannot be used as a header value: {}", &"*** Sensitive Data Redacted ***", err),
                )
            })?;
            builder = builder.header("x-amz-server-side-encryption-customer-key", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_9) = &input.sse_customer_key_md5 {
        let formatted_10 = inner_9.as_str();
        if !formatted_10.is_empty() {
            let header_value = formatted_10;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "sse_customer_key_md5",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-server-side-encryption-customer-key-MD5", header_value);
        }
    }
    Ok(builder)
}

#[allow(unused_mut)]
pub fn de_list_parts(
    inp: &[u8],
    mut builder: crate::operation::list_parts::builders::ListPartsOutputBuilder,
) -> Result<crate::operation::list_parts::builders::ListPartsOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !start_el.matches("ListPartsResult") {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "encountered invalid XML root: expected ListPartsResult but got {:?}. This is likely a bug in the SDK.",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Owner") /* Owner com.amazonaws.s3.synthetic#ListPartsOutput$Owner */ =>  {
                let var_11 =
                    Some(
                        crate::protocol_serde::shape_owner::de_owner(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_owner(var_11);
            }
            ,
            s if s.matches("NextPartNumberMarker") /* NextPartNumberMarker com.amazonaws.s3.synthetic#ListPartsOutput$NextPartNumberMarker */ =>  {
                let var_12 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_part_number_marker(var_12);
            }
            ,
            s if s.matches("StorageClass") /* StorageClass com.amazonaws.s3.synthetic#ListPartsOutput$StorageClass */ =>  {
                let var_13 =
                    Some(
                        Result::<crate::types::StorageClass, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::StorageClass::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_storage_class(var_13);
            }
            ,
            s if s.matches("ChecksumAlgorithm") /* ChecksumAlgorithm com.amazonaws.s3.synthetic#ListPartsOutput$ChecksumAlgorithm */ =>  {
                let var_14 =
                    Some(
                        Result::<crate::types::ChecksumAlgorithm, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ChecksumAlgorithm::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_checksum_algorithm(var_14);
            }
            ,
            s if s.matches("IsTruncated") /* IsTruncated com.amazonaws.s3.synthetic#ListPartsOutput$IsTruncated */ =>  {
                let var_15 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.s3#IsTruncated`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_is_truncated(var_15);
            }
            ,
            s if s.matches("MaxParts") /* MaxParts com.amazonaws.s3.synthetic#ListPartsOutput$MaxParts */ =>  {
                let var_16 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.s3#MaxParts`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_max_parts(var_16);
            }
            ,
            s if s.matches("UploadId") /* UploadId com.amazonaws.s3.synthetic#ListPartsOutput$UploadId */ =>  {
                let var_17 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_upload_id(var_17);
            }
            ,
            s if s.matches("PartNumberMarker") /* PartNumberMarker com.amazonaws.s3.synthetic#ListPartsOutput$PartNumberMarker */ =>  {
                let var_18 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_part_number_marker(var_18);
            }
            ,
            s if s.matches("Bucket") /* Bucket com.amazonaws.s3.synthetic#ListPartsOutput$Bucket */ =>  {
                let var_19 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_bucket(var_19);
            }
            ,
            s if s.matches("Part") /* Parts com.amazonaws.s3.synthetic#ListPartsOutput$Parts */ =>  {
                let var_20 =
                    Some(
                        Result::<::std::vec::Vec::<crate::types::Part>, ::aws_smithy_xml::decode::XmlDecodeError>::Ok({
                            let mut list_21 = builder.parts.take().unwrap_or_default();
                            list_21.push(
                                crate::protocol_serde::shape_part::de_part(&mut tag)
                                ?
                            );
                            list_21
                        })
                        ?
                    )
                ;
                builder = builder.set_parts(var_20);
            }
            ,
            s if s.matches("Initiator") /* Initiator com.amazonaws.s3.synthetic#ListPartsOutput$Initiator */ =>  {
                let var_22 =
                    Some(
                        crate::protocol_serde::shape_initiator::de_initiator(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_initiator(var_22);
            }
            ,
            s if s.matches("Key") /* Key com.amazonaws.s3.synthetic#ListPartsOutput$Key */ =>  {
                let var_23 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_key(var_23);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
