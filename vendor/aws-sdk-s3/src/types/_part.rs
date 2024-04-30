// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Container for elements related to a part.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Part {
    /// <p>Part number identifying the part. This is a positive integer between 1 and 10,000.</p>
    pub part_number: ::std::option::Option<i32>,
    /// <p>Date and time at which the part was uploaded.</p>
    pub last_modified: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>Entity tag returned when the part was uploaded.</p>
    pub e_tag: ::std::option::Option<::std::string::String>,
    /// <p>Size in bytes of the uploaded part data.</p>
    pub size: ::std::option::Option<i64>,
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 32-bit CRC32 checksum of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32: ::std::option::Option<::std::string::String>,
    /// <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded with the object. When you use an API operation on an object that was uploaded using multipart uploads, this value may not be a direct checksum value of the full object. Instead, it's a calculation based on the checksum values of each individual part. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_crc32_c: ::std::option::Option<::std::string::String>,
    /// <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded with the object. When you use the API operation on an object that was uploaded using multipart uploads, this value may not be a direct checksum value of the full object. Instead, it's a calculation based on the checksum values of each individual part. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha1: ::std::option::Option<::std::string::String>,
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 256-bit SHA-256 digest of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub checksum_sha256: ::std::option::Option<::std::string::String>,
}
impl Part {
    /// <p>Part number identifying the part. This is a positive integer between 1 and 10,000.</p>
    pub fn part_number(&self) -> ::std::option::Option<i32> {
        self.part_number
    }
    /// <p>Date and time at which the part was uploaded.</p>
    pub fn last_modified(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_modified.as_ref()
    }
    /// <p>Entity tag returned when the part was uploaded.</p>
    pub fn e_tag(&self) -> ::std::option::Option<&str> {
        self.e_tag.as_deref()
    }
    /// <p>Size in bytes of the uploaded part data.</p>
    pub fn size(&self) -> ::std::option::Option<i64> {
        self.size
    }
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 32-bit CRC32 checksum of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_crc32(&self) -> ::std::option::Option<&str> {
        self.checksum_crc32.as_deref()
    }
    /// <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded with the object. When you use an API operation on an object that was uploaded using multipart uploads, this value may not be a direct checksum value of the full object. Instead, it's a calculation based on the checksum values of each individual part. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_crc32_c(&self) -> ::std::option::Option<&str> {
        self.checksum_crc32_c.as_deref()
    }
    /// <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded with the object. When you use the API operation on an object that was uploaded using multipart uploads, this value may not be a direct checksum value of the full object. Instead, it's a calculation based on the checksum values of each individual part. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_sha1(&self) -> ::std::option::Option<&str> {
        self.checksum_sha1.as_deref()
    }
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 256-bit SHA-256 digest of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_sha256(&self) -> ::std::option::Option<&str> {
        self.checksum_sha256.as_deref()
    }
}
impl Part {
    /// Creates a new builder-style object to manufacture [`Part`](crate::types::Part).
    pub fn builder() -> crate::types::builders::PartBuilder {
        crate::types::builders::PartBuilder::default()
    }
}

/// A builder for [`Part`](crate::types::Part).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PartBuilder {
    pub(crate) part_number: ::std::option::Option<i32>,
    pub(crate) last_modified: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) e_tag: ::std::option::Option<::std::string::String>,
    pub(crate) size: ::std::option::Option<i64>,
    pub(crate) checksum_crc32: ::std::option::Option<::std::string::String>,
    pub(crate) checksum_crc32_c: ::std::option::Option<::std::string::String>,
    pub(crate) checksum_sha1: ::std::option::Option<::std::string::String>,
    pub(crate) checksum_sha256: ::std::option::Option<::std::string::String>,
}
impl PartBuilder {
    /// <p>Part number identifying the part. This is a positive integer between 1 and 10,000.</p>
    pub fn part_number(mut self, input: i32) -> Self {
        self.part_number = ::std::option::Option::Some(input);
        self
    }
    /// <p>Part number identifying the part. This is a positive integer between 1 and 10,000.</p>
    pub fn set_part_number(mut self, input: ::std::option::Option<i32>) -> Self {
        self.part_number = input;
        self
    }
    /// <p>Part number identifying the part. This is a positive integer between 1 and 10,000.</p>
    pub fn get_part_number(&self) -> &::std::option::Option<i32> {
        &self.part_number
    }
    /// <p>Date and time at which the part was uploaded.</p>
    pub fn last_modified(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_modified = ::std::option::Option::Some(input);
        self
    }
    /// <p>Date and time at which the part was uploaded.</p>
    pub fn set_last_modified(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.last_modified = input;
        self
    }
    /// <p>Date and time at which the part was uploaded.</p>
    pub fn get_last_modified(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.last_modified
    }
    /// <p>Entity tag returned when the part was uploaded.</p>
    pub fn e_tag(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.e_tag = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Entity tag returned when the part was uploaded.</p>
    pub fn set_e_tag(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.e_tag = input;
        self
    }
    /// <p>Entity tag returned when the part was uploaded.</p>
    pub fn get_e_tag(&self) -> &::std::option::Option<::std::string::String> {
        &self.e_tag
    }
    /// <p>Size in bytes of the uploaded part data.</p>
    pub fn size(mut self, input: i64) -> Self {
        self.size = ::std::option::Option::Some(input);
        self
    }
    /// <p>Size in bytes of the uploaded part data.</p>
    pub fn set_size(mut self, input: ::std::option::Option<i64>) -> Self {
        self.size = input;
        self
    }
    /// <p>Size in bytes of the uploaded part data.</p>
    pub fn get_size(&self) -> &::std::option::Option<i64> {
        &self.size
    }
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 32-bit CRC32 checksum of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_crc32(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.checksum_crc32 = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 32-bit CRC32 checksum of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_checksum_crc32(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.checksum_crc32 = input;
        self
    }
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 32-bit CRC32 checksum of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_checksum_crc32(&self) -> &::std::option::Option<::std::string::String> {
        &self.checksum_crc32
    }
    /// <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded with the object. When you use an API operation on an object that was uploaded using multipart uploads, this value may not be a direct checksum value of the full object. Instead, it's a calculation based on the checksum values of each individual part. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_crc32_c(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.checksum_crc32_c = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded with the object. When you use an API operation on an object that was uploaded using multipart uploads, this value may not be a direct checksum value of the full object. Instead, it's a calculation based on the checksum values of each individual part. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_checksum_crc32_c(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.checksum_crc32_c = input;
        self
    }
    /// <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded with the object. When you use an API operation on an object that was uploaded using multipart uploads, this value may not be a direct checksum value of the full object. Instead, it's a calculation based on the checksum values of each individual part. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_checksum_crc32_c(&self) -> &::std::option::Option<::std::string::String> {
        &self.checksum_crc32_c
    }
    /// <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded with the object. When you use the API operation on an object that was uploaded using multipart uploads, this value may not be a direct checksum value of the full object. Instead, it's a calculation based on the checksum values of each individual part. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_sha1(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.checksum_sha1 = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded with the object. When you use the API operation on an object that was uploaded using multipart uploads, this value may not be a direct checksum value of the full object. Instead, it's a calculation based on the checksum values of each individual part. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_checksum_sha1(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.checksum_sha1 = input;
        self
    }
    /// <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded with the object. When you use the API operation on an object that was uploaded using multipart uploads, this value may not be a direct checksum value of the full object. Instead, it's a calculation based on the checksum values of each individual part. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_checksum_sha1(&self) -> &::std::option::Option<::std::string::String> {
        &self.checksum_sha1
    }
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 256-bit SHA-256 digest of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_sha256(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.checksum_sha256 = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 256-bit SHA-256 digest of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_checksum_sha256(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.checksum_sha256 = input;
        self
    }
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 256-bit SHA-256 digest of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_checksum_sha256(&self) -> &::std::option::Option<::std::string::String> {
        &self.checksum_sha256
    }
    /// Consumes the builder and constructs a [`Part`](crate::types::Part).
    pub fn build(self) -> crate::types::Part {
        crate::types::Part {
            part_number: self.part_number,
            last_modified: self.last_modified,
            e_tag: self.e_tag,
            size: self.size,
            checksum_crc32: self.checksum_crc32,
            checksum_crc32_c: self.checksum_crc32_c,
            checksum_sha1: self.checksum_sha1,
            checksum_sha256: self.checksum_sha256,
        }
    }
}
