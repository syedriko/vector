// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_upload_part_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::upload_part::UploadPartOutput, crate::operation::upload_part::UploadPartError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::upload_part::UploadPartError::unhandled)?;
    generic_builder = crate::s3_request_id::apply_extended_request_id(generic_builder, _response_headers);
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::upload_part::UploadPartError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_upload_part_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::upload_part::UploadPartOutput, crate::operation::upload_part::UploadPartError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::upload_part::builders::UploadPartOutputBuilder::default();
        output = output.set_bucket_key_enabled(
            crate::protocol_serde::shape_upload_part_output::de_bucket_key_enabled_header(_response_headers).map_err(|_| {
                crate::operation::upload_part::UploadPartError::unhandled(
                    "Failed to parse BucketKeyEnabled from header `x-amz-server-side-encryption-bucket-key-enabled",
                )
            })?,
        );
        output = output.set_checksum_crc32(
            crate::protocol_serde::shape_upload_part_output::de_checksum_crc32_header(_response_headers).map_err(|_| {
                crate::operation::upload_part::UploadPartError::unhandled("Failed to parse ChecksumCRC32 from header `x-amz-checksum-crc32")
            })?,
        );
        output = output.set_checksum_crc32_c(
            crate::protocol_serde::shape_upload_part_output::de_checksum_crc32_c_header(_response_headers).map_err(|_| {
                crate::operation::upload_part::UploadPartError::unhandled("Failed to parse ChecksumCRC32C from header `x-amz-checksum-crc32c")
            })?,
        );
        output = output.set_checksum_sha1(
            crate::protocol_serde::shape_upload_part_output::de_checksum_sha1_header(_response_headers).map_err(|_| {
                crate::operation::upload_part::UploadPartError::unhandled("Failed to parse ChecksumSHA1 from header `x-amz-checksum-sha1")
            })?,
        );
        output = output.set_checksum_sha256(
            crate::protocol_serde::shape_upload_part_output::de_checksum_sha256_header(_response_headers).map_err(|_| {
                crate::operation::upload_part::UploadPartError::unhandled("Failed to parse ChecksumSHA256 from header `x-amz-checksum-sha256")
            })?,
        );
        output = output.set_e_tag(
            crate::protocol_serde::shape_upload_part_output::de_e_tag_header(_response_headers)
                .map_err(|_| crate::operation::upload_part::UploadPartError::unhandled("Failed to parse ETag from header `ETag"))?,
        );
        output = output.set_request_charged(
            crate::protocol_serde::shape_upload_part_output::de_request_charged_header(_response_headers).map_err(|_| {
                crate::operation::upload_part::UploadPartError::unhandled("Failed to parse RequestCharged from header `x-amz-request-charged")
            })?,
        );
        output = output.set_sse_customer_algorithm(
            crate::protocol_serde::shape_upload_part_output::de_sse_customer_algorithm_header(_response_headers).map_err(|_| {
                crate::operation::upload_part::UploadPartError::unhandled(
                    "Failed to parse SSECustomerAlgorithm from header `x-amz-server-side-encryption-customer-algorithm",
                )
            })?,
        );
        output = output.set_sse_customer_key_md5(
            crate::protocol_serde::shape_upload_part_output::de_sse_customer_key_md5_header(_response_headers).map_err(|_| {
                crate::operation::upload_part::UploadPartError::unhandled(
                    "Failed to parse SSECustomerKeyMD5 from header `x-amz-server-side-encryption-customer-key-MD5",
                )
            })?,
        );
        output = output.set_ssekms_key_id(
            crate::protocol_serde::shape_upload_part_output::de_ssekms_key_id_header(_response_headers).map_err(|_| {
                crate::operation::upload_part::UploadPartError::unhandled(
                    "Failed to parse SSEKMSKeyId from header `x-amz-server-side-encryption-aws-kms-key-id",
                )
            })?,
        );
        output = output.set_server_side_encryption(
            crate::protocol_serde::shape_upload_part_output::de_server_side_encryption_header(_response_headers).map_err(|_| {
                crate::operation::upload_part::UploadPartError::unhandled(
                    "Failed to parse ServerSideEncryption from header `x-amz-server-side-encryption",
                )
            })?,
        );
        output._set_extended_request_id(crate::s3_request_id::RequestIdExt::extended_request_id(_response_headers).map(str::to_string));
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_upload_part_headers(
    input: &crate::operation::upload_part::UploadPartInput,
    mut builder: ::http::request::Builder,
) -> std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
    if let ::std::option::Option::Some(inner_1) = &input.content_length {
        let mut encoder = ::aws_smithy_types::primitive::Encoder::from(*inner_1);
        let formatted_2 = encoder.encode();
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "content_length",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("Content-Length", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_3) = &input.content_md5 {
        let formatted_4 = inner_3.as_str();
        if !formatted_4.is_empty() {
            let header_value = formatted_4;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "content_md5",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("Content-MD5", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_5) = &input.checksum_algorithm {
        let formatted_6 = inner_5.as_str();
        if !formatted_6.is_empty() {
            let header_value = formatted_6;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "checksum_algorithm",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-sdk-checksum-algorithm", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_7) = &input.checksum_crc32 {
        let formatted_8 = inner_7.as_str();
        if !formatted_8.is_empty() {
            let header_value = formatted_8;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "checksum_crc32",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-checksum-crc32", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_9) = &input.checksum_crc32_c {
        let formatted_10 = inner_9.as_str();
        if !formatted_10.is_empty() {
            let header_value = formatted_10;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "checksum_crc32_c",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-checksum-crc32c", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_11) = &input.checksum_sha1 {
        let formatted_12 = inner_11.as_str();
        if !formatted_12.is_empty() {
            let header_value = formatted_12;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "checksum_sha1",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-checksum-sha1", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_13) = &input.checksum_sha256 {
        let formatted_14 = inner_13.as_str();
        if !formatted_14.is_empty() {
            let header_value = formatted_14;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "checksum_sha256",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-checksum-sha256", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_15) = &input.sse_customer_algorithm {
        let formatted_16 = inner_15.as_str();
        if !formatted_16.is_empty() {
            let header_value = formatted_16;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "sse_customer_algorithm",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-server-side-encryption-customer-algorithm", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_17) = &input.sse_customer_key {
        let formatted_18 = inner_17.as_str();
        if !formatted_18.is_empty() {
            let header_value = formatted_18;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "sse_customer_key",
                    format!("`{}` cannot be used as a header value: {}", &"*** Sensitive Data Redacted ***", err),
                )
            })?;
            builder = builder.header("x-amz-server-side-encryption-customer-key", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_19) = &input.sse_customer_key_md5 {
        let formatted_20 = inner_19.as_str();
        if !formatted_20.is_empty() {
            let header_value = formatted_20;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "sse_customer_key_md5",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-server-side-encryption-customer-key-MD5", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_21) = &input.request_payer {
        let formatted_22 = inner_21.as_str();
        if !formatted_22.is_empty() {
            let header_value = formatted_22;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "request_payer",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-request-payer", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_23) = &input.expected_bucket_owner {
        let formatted_24 = inner_23.as_str();
        if !formatted_24.is_empty() {
            let header_value = formatted_24;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "expected_bucket_owner",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-expected-bucket-owner", header_value);
        }
    }
    Ok(builder)
}
