// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Container for logging status information.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BucketLoggingStatus {
    /// <p>Describes where logs are stored and the prefix that Amazon S3 assigns to all log object keys for a bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTlogging.html">PUT Bucket logging</a> in the <i>Amazon S3 API Reference</i>.</p>
    pub logging_enabled: ::std::option::Option<crate::types::LoggingEnabled>,
}
impl BucketLoggingStatus {
    /// <p>Describes where logs are stored and the prefix that Amazon S3 assigns to all log object keys for a bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTlogging.html">PUT Bucket logging</a> in the <i>Amazon S3 API Reference</i>.</p>
    pub fn logging_enabled(&self) -> ::std::option::Option<&crate::types::LoggingEnabled> {
        self.logging_enabled.as_ref()
    }
}
impl BucketLoggingStatus {
    /// Creates a new builder-style object to manufacture [`BucketLoggingStatus`](crate::types::BucketLoggingStatus).
    pub fn builder() -> crate::types::builders::BucketLoggingStatusBuilder {
        crate::types::builders::BucketLoggingStatusBuilder::default()
    }
}

/// A builder for [`BucketLoggingStatus`](crate::types::BucketLoggingStatus).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct BucketLoggingStatusBuilder {
    pub(crate) logging_enabled: ::std::option::Option<crate::types::LoggingEnabled>,
}
impl BucketLoggingStatusBuilder {
    /// <p>Describes where logs are stored and the prefix that Amazon S3 assigns to all log object keys for a bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTlogging.html">PUT Bucket logging</a> in the <i>Amazon S3 API Reference</i>.</p>
    pub fn logging_enabled(mut self, input: crate::types::LoggingEnabled) -> Self {
        self.logging_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes where logs are stored and the prefix that Amazon S3 assigns to all log object keys for a bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTlogging.html">PUT Bucket logging</a> in the <i>Amazon S3 API Reference</i>.</p>
    pub fn set_logging_enabled(mut self, input: ::std::option::Option<crate::types::LoggingEnabled>) -> Self {
        self.logging_enabled = input;
        self
    }
    /// <p>Describes where logs are stored and the prefix that Amazon S3 assigns to all log object keys for a bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTlogging.html">PUT Bucket logging</a> in the <i>Amazon S3 API Reference</i>.</p>
    pub fn get_logging_enabled(&self) -> &::std::option::Option<crate::types::LoggingEnabled> {
        &self.logging_enabled
    }
    /// Consumes the builder and constructs a [`BucketLoggingStatus`](crate::types::BucketLoggingStatus).
    pub fn build(self) -> crate::types::BucketLoggingStatus {
        crate::types::BucketLoggingStatus {
            logging_enabled: self.logging_enabled,
        }
    }
}
