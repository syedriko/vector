// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The S3 Intelligent-Tiering storage class is designed to optimize storage costs by automatically moving data to the most cost-effective storage access tier, without additional operational overhead.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Tiering {
    /// <p>The number of consecutive days of no access after which an object will be eligible to be transitioned to the corresponding tier. The minimum number of days specified for Archive Access tier must be at least 90 days and Deep Archive Access tier must be at least 180 days. The maximum can be up to 2 years (730 days).</p>
    pub days: i32,
    /// <p>S3 Intelligent-Tiering access tier. See <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html#sc-dynamic-data-access">Storage class for automatically optimizing frequently and infrequently accessed objects</a> for a list of access tiers in the S3 Intelligent-Tiering storage class.</p>
    pub access_tier: crate::types::IntelligentTieringAccessTier,
}
impl Tiering {
    /// <p>The number of consecutive days of no access after which an object will be eligible to be transitioned to the corresponding tier. The minimum number of days specified for Archive Access tier must be at least 90 days and Deep Archive Access tier must be at least 180 days. The maximum can be up to 2 years (730 days).</p>
    pub fn days(&self) -> i32 {
        self.days
    }
    /// <p>S3 Intelligent-Tiering access tier. See <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html#sc-dynamic-data-access">Storage class for automatically optimizing frequently and infrequently accessed objects</a> for a list of access tiers in the S3 Intelligent-Tiering storage class.</p>
    pub fn access_tier(&self) -> &crate::types::IntelligentTieringAccessTier {
        &self.access_tier
    }
}
impl Tiering {
    /// Creates a new builder-style object to manufacture [`Tiering`](crate::types::Tiering).
    pub fn builder() -> crate::types::builders::TieringBuilder {
        crate::types::builders::TieringBuilder::default()
    }
}

/// A builder for [`Tiering`](crate::types::Tiering).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct TieringBuilder {
    pub(crate) days: ::std::option::Option<i32>,
    pub(crate) access_tier: ::std::option::Option<crate::types::IntelligentTieringAccessTier>,
}
impl TieringBuilder {
    /// <p>The number of consecutive days of no access after which an object will be eligible to be transitioned to the corresponding tier. The minimum number of days specified for Archive Access tier must be at least 90 days and Deep Archive Access tier must be at least 180 days. The maximum can be up to 2 years (730 days).</p>
    /// This field is required.
    pub fn days(mut self, input: i32) -> Self {
        self.days = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of consecutive days of no access after which an object will be eligible to be transitioned to the corresponding tier. The minimum number of days specified for Archive Access tier must be at least 90 days and Deep Archive Access tier must be at least 180 days. The maximum can be up to 2 years (730 days).</p>
    pub fn set_days(mut self, input: ::std::option::Option<i32>) -> Self {
        self.days = input;
        self
    }
    /// <p>The number of consecutive days of no access after which an object will be eligible to be transitioned to the corresponding tier. The minimum number of days specified for Archive Access tier must be at least 90 days and Deep Archive Access tier must be at least 180 days. The maximum can be up to 2 years (730 days).</p>
    pub fn get_days(&self) -> &::std::option::Option<i32> {
        &self.days
    }
    /// <p>S3 Intelligent-Tiering access tier. See <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html#sc-dynamic-data-access">Storage class for automatically optimizing frequently and infrequently accessed objects</a> for a list of access tiers in the S3 Intelligent-Tiering storage class.</p>
    /// This field is required.
    pub fn access_tier(mut self, input: crate::types::IntelligentTieringAccessTier) -> Self {
        self.access_tier = ::std::option::Option::Some(input);
        self
    }
    /// <p>S3 Intelligent-Tiering access tier. See <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html#sc-dynamic-data-access">Storage class for automatically optimizing frequently and infrequently accessed objects</a> for a list of access tiers in the S3 Intelligent-Tiering storage class.</p>
    pub fn set_access_tier(mut self, input: ::std::option::Option<crate::types::IntelligentTieringAccessTier>) -> Self {
        self.access_tier = input;
        self
    }
    /// <p>S3 Intelligent-Tiering access tier. See <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html#sc-dynamic-data-access">Storage class for automatically optimizing frequently and infrequently accessed objects</a> for a list of access tiers in the S3 Intelligent-Tiering storage class.</p>
    pub fn get_access_tier(&self) -> &::std::option::Option<crate::types::IntelligentTieringAccessTier> {
        &self.access_tier
    }
    /// Consumes the builder and constructs a [`Tiering`](crate::types::Tiering).
    /// This method will fail if any of the following fields are not set:
    /// - [`days`](crate::types::builders::TieringBuilder::days)
    /// - [`access_tier`](crate::types::builders::TieringBuilder::access_tier)
    pub fn build(self) -> ::std::result::Result<crate::types::Tiering, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::Tiering {
            days: self.days.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "days",
                    "days was not specified but it is required when building Tiering",
                )
            })?,
            access_tier: self.access_tier.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "access_tier",
                    "access_tier was not specified but it is required when building Tiering",
                )
            })?,
        })
    }
}
