// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Configures retry behavior in case Kinesis Data Firehose is unable to deliver documents to the Serverless offering for Amazon OpenSearch Service.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AmazonOpenSearchServerlessRetryOptions {
    /// <p>After an initial failure to deliver to the Serverless offering for Amazon OpenSearch Service, the total amount of time during which Kinesis Data Firehose retries delivery (including the first attempt). After this time has elapsed, the failed documents are written to Amazon S3. Default value is 300 seconds (5 minutes). A value of 0 (zero) results in no retries.</p>
    pub duration_in_seconds: ::std::option::Option<i32>,
}
impl AmazonOpenSearchServerlessRetryOptions {
    /// <p>After an initial failure to deliver to the Serverless offering for Amazon OpenSearch Service, the total amount of time during which Kinesis Data Firehose retries delivery (including the first attempt). After this time has elapsed, the failed documents are written to Amazon S3. Default value is 300 seconds (5 minutes). A value of 0 (zero) results in no retries.</p>
    pub fn duration_in_seconds(&self) -> ::std::option::Option<i32> {
        self.duration_in_seconds
    }
}
impl AmazonOpenSearchServerlessRetryOptions {
    /// Creates a new builder-style object to manufacture [`AmazonOpenSearchServerlessRetryOptions`](crate::types::AmazonOpenSearchServerlessRetryOptions).
    pub fn builder() -> crate::types::builders::AmazonOpenSearchServerlessRetryOptionsBuilder {
        crate::types::builders::AmazonOpenSearchServerlessRetryOptionsBuilder::default()
    }
}

/// A builder for [`AmazonOpenSearchServerlessRetryOptions`](crate::types::AmazonOpenSearchServerlessRetryOptions).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AmazonOpenSearchServerlessRetryOptionsBuilder {
    pub(crate) duration_in_seconds: ::std::option::Option<i32>,
}
impl AmazonOpenSearchServerlessRetryOptionsBuilder {
    /// <p>After an initial failure to deliver to the Serverless offering for Amazon OpenSearch Service, the total amount of time during which Kinesis Data Firehose retries delivery (including the first attempt). After this time has elapsed, the failed documents are written to Amazon S3. Default value is 300 seconds (5 minutes). A value of 0 (zero) results in no retries.</p>
    pub fn duration_in_seconds(mut self, input: i32) -> Self {
        self.duration_in_seconds = ::std::option::Option::Some(input);
        self
    }
    /// <p>After an initial failure to deliver to the Serverless offering for Amazon OpenSearch Service, the total amount of time during which Kinesis Data Firehose retries delivery (including the first attempt). After this time has elapsed, the failed documents are written to Amazon S3. Default value is 300 seconds (5 minutes). A value of 0 (zero) results in no retries.</p>
    pub fn set_duration_in_seconds(mut self, input: ::std::option::Option<i32>) -> Self {
        self.duration_in_seconds = input;
        self
    }
    /// <p>After an initial failure to deliver to the Serverless offering for Amazon OpenSearch Service, the total amount of time during which Kinesis Data Firehose retries delivery (including the first attempt). After this time has elapsed, the failed documents are written to Amazon S3. Default value is 300 seconds (5 minutes). A value of 0 (zero) results in no retries.</p>
    pub fn get_duration_in_seconds(&self) -> &::std::option::Option<i32> {
        &self.duration_in_seconds
    }
    /// Consumes the builder and constructs a [`AmazonOpenSearchServerlessRetryOptions`](crate::types::AmazonOpenSearchServerlessRetryOptions).
    pub fn build(self) -> crate::types::AmazonOpenSearchServerlessRetryOptions {
        crate::types::AmazonOpenSearchServerlessRetryOptions {
            duration_in_seconds: self.duration_in_seconds,
        }
    }
}
