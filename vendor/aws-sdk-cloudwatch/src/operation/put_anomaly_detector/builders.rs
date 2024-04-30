// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_anomaly_detector::_put_anomaly_detector_output::PutAnomalyDetectorOutputBuilder;

pub use crate::operation::put_anomaly_detector::_put_anomaly_detector_input::PutAnomalyDetectorInputBuilder;

impl PutAnomalyDetectorInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_anomaly_detector::PutAnomalyDetectorOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_anomaly_detector::PutAnomalyDetectorError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_anomaly_detector();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutAnomalyDetector`.
///
/// <p>Creates an anomaly detection model for a CloudWatch metric. You can use the model to display a band of expected normal values when the metric is graphed.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Anomaly_Detection.html">CloudWatch Anomaly Detection</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutAnomalyDetectorFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_anomaly_detector::builders::PutAnomalyDetectorInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_anomaly_detector::PutAnomalyDetectorOutput,
        crate::operation::put_anomaly_detector::PutAnomalyDetectorError,
    > for PutAnomalyDetectorFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_anomaly_detector::PutAnomalyDetectorOutput,
            crate::operation::put_anomaly_detector::PutAnomalyDetectorError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutAnomalyDetectorFluentBuilder {
    /// Creates a new `PutAnomalyDetector`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutAnomalyDetector as a reference.
    pub fn as_input(&self) -> &crate::operation::put_anomaly_detector::builders::PutAnomalyDetectorInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::put_anomaly_detector::PutAnomalyDetectorOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_anomaly_detector::PutAnomalyDetectorError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_anomaly_detector::PutAnomalyDetector::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_anomaly_detector::PutAnomalyDetector::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_anomaly_detector::PutAnomalyDetectorOutput,
        crate::operation::put_anomaly_detector::PutAnomalyDetectorError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The namespace of the metric to create the anomaly detection model for.</p>
    #[deprecated(note = "Use SingleMetricAnomalyDetector.")]
    pub fn namespace(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.namespace(input.into());
        self
    }
    /// <p>The namespace of the metric to create the anomaly detection model for.</p>
    #[deprecated(note = "Use SingleMetricAnomalyDetector.")]
    pub fn set_namespace(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_namespace(input);
        self
    }
    /// <p>The namespace of the metric to create the anomaly detection model for.</p>
    #[deprecated(note = "Use SingleMetricAnomalyDetector.")]
    pub fn get_namespace(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_namespace()
    }
    /// <p>The name of the metric to create the anomaly detection model for.</p>
    #[deprecated(note = "Use SingleMetricAnomalyDetector.")]
    pub fn metric_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.metric_name(input.into());
        self
    }
    /// <p>The name of the metric to create the anomaly detection model for.</p>
    #[deprecated(note = "Use SingleMetricAnomalyDetector.")]
    pub fn set_metric_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_metric_name(input);
        self
    }
    /// <p>The name of the metric to create the anomaly detection model for.</p>
    #[deprecated(note = "Use SingleMetricAnomalyDetector.")]
    pub fn get_metric_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_metric_name()
    }
    /// Appends an item to `Dimensions`.
    ///
    /// To override the contents of this collection use [`set_dimensions`](Self::set_dimensions).
    ///
    /// <p>The metric dimensions to create the anomaly detection model for.</p>
    #[deprecated(note = "Use SingleMetricAnomalyDetector.")]
    pub fn dimensions(mut self, input: crate::types::Dimension) -> Self {
        self.inner = self.inner.dimensions(input);
        self
    }
    /// <p>The metric dimensions to create the anomaly detection model for.</p>
    #[deprecated(note = "Use SingleMetricAnomalyDetector.")]
    pub fn set_dimensions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Dimension>>) -> Self {
        self.inner = self.inner.set_dimensions(input);
        self
    }
    /// <p>The metric dimensions to create the anomaly detection model for.</p>
    #[deprecated(note = "Use SingleMetricAnomalyDetector.")]
    pub fn get_dimensions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Dimension>> {
        self.inner.get_dimensions()
    }
    /// <p>The statistic to use for the metric and the anomaly detection model.</p>
    #[deprecated(note = "Use SingleMetricAnomalyDetector.")]
    pub fn stat(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stat(input.into());
        self
    }
    /// <p>The statistic to use for the metric and the anomaly detection model.</p>
    #[deprecated(note = "Use SingleMetricAnomalyDetector.")]
    pub fn set_stat(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stat(input);
        self
    }
    /// <p>The statistic to use for the metric and the anomaly detection model.</p>
    #[deprecated(note = "Use SingleMetricAnomalyDetector.")]
    pub fn get_stat(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_stat()
    }
    /// <p>The configuration specifies details about how the anomaly detection model is to be trained, including time ranges to exclude when training and updating the model. You can specify as many as 10 time ranges.</p>
    /// <p>The configuration can also include the time zone to use for the metric.</p>
    pub fn configuration(mut self, input: crate::types::AnomalyDetectorConfiguration) -> Self {
        self.inner = self.inner.configuration(input);
        self
    }
    /// <p>The configuration specifies details about how the anomaly detection model is to be trained, including time ranges to exclude when training and updating the model. You can specify as many as 10 time ranges.</p>
    /// <p>The configuration can also include the time zone to use for the metric.</p>
    pub fn set_configuration(mut self, input: ::std::option::Option<crate::types::AnomalyDetectorConfiguration>) -> Self {
        self.inner = self.inner.set_configuration(input);
        self
    }
    /// <p>The configuration specifies details about how the anomaly detection model is to be trained, including time ranges to exclude when training and updating the model. You can specify as many as 10 time ranges.</p>
    /// <p>The configuration can also include the time zone to use for the metric.</p>
    pub fn get_configuration(&self) -> &::std::option::Option<crate::types::AnomalyDetectorConfiguration> {
        self.inner.get_configuration()
    }
    /// <p>A single metric anomaly detector to be created.</p>
    /// <p>When using <code>SingleMetricAnomalyDetector</code>, you cannot include the following parameters in the same operation:</p>
    /// <ul>
    /// <li> <p> <code>Dimensions</code> </p> </li>
    /// <li> <p> <code>MetricName</code> </p> </li>
    /// <li> <p> <code>Namespace</code> </p> </li>
    /// <li> <p> <code>Stat</code> </p> </li>
    /// <li> <p>the <code>MetricMatchAnomalyDetector</code> parameters of <code>PutAnomalyDetectorInput</code> </p> </li>
    /// </ul>
    /// <p>Instead, specify the single metric anomaly detector attributes as part of the property <code>SingleMetricAnomalyDetector</code>.</p>
    pub fn single_metric_anomaly_detector(mut self, input: crate::types::SingleMetricAnomalyDetector) -> Self {
        self.inner = self.inner.single_metric_anomaly_detector(input);
        self
    }
    /// <p>A single metric anomaly detector to be created.</p>
    /// <p>When using <code>SingleMetricAnomalyDetector</code>, you cannot include the following parameters in the same operation:</p>
    /// <ul>
    /// <li> <p> <code>Dimensions</code> </p> </li>
    /// <li> <p> <code>MetricName</code> </p> </li>
    /// <li> <p> <code>Namespace</code> </p> </li>
    /// <li> <p> <code>Stat</code> </p> </li>
    /// <li> <p>the <code>MetricMatchAnomalyDetector</code> parameters of <code>PutAnomalyDetectorInput</code> </p> </li>
    /// </ul>
    /// <p>Instead, specify the single metric anomaly detector attributes as part of the property <code>SingleMetricAnomalyDetector</code>.</p>
    pub fn set_single_metric_anomaly_detector(mut self, input: ::std::option::Option<crate::types::SingleMetricAnomalyDetector>) -> Self {
        self.inner = self.inner.set_single_metric_anomaly_detector(input);
        self
    }
    /// <p>A single metric anomaly detector to be created.</p>
    /// <p>When using <code>SingleMetricAnomalyDetector</code>, you cannot include the following parameters in the same operation:</p>
    /// <ul>
    /// <li> <p> <code>Dimensions</code> </p> </li>
    /// <li> <p> <code>MetricName</code> </p> </li>
    /// <li> <p> <code>Namespace</code> </p> </li>
    /// <li> <p> <code>Stat</code> </p> </li>
    /// <li> <p>the <code>MetricMatchAnomalyDetector</code> parameters of <code>PutAnomalyDetectorInput</code> </p> </li>
    /// </ul>
    /// <p>Instead, specify the single metric anomaly detector attributes as part of the property <code>SingleMetricAnomalyDetector</code>.</p>
    pub fn get_single_metric_anomaly_detector(&self) -> &::std::option::Option<crate::types::SingleMetricAnomalyDetector> {
        self.inner.get_single_metric_anomaly_detector()
    }
    /// <p>The metric math anomaly detector to be created.</p>
    /// <p>When using <code>MetricMathAnomalyDetector</code>, you cannot include the following parameters in the same operation:</p>
    /// <ul>
    /// <li> <p> <code>Dimensions</code> </p> </li>
    /// <li> <p> <code>MetricName</code> </p> </li>
    /// <li> <p> <code>Namespace</code> </p> </li>
    /// <li> <p> <code>Stat</code> </p> </li>
    /// <li> <p>the <code>SingleMetricAnomalyDetector</code> parameters of <code>PutAnomalyDetectorInput</code> </p> </li>
    /// </ul>
    /// <p>Instead, specify the metric math anomaly detector attributes as part of the property <code>MetricMathAnomalyDetector</code>.</p>
    pub fn metric_math_anomaly_detector(mut self, input: crate::types::MetricMathAnomalyDetector) -> Self {
        self.inner = self.inner.metric_math_anomaly_detector(input);
        self
    }
    /// <p>The metric math anomaly detector to be created.</p>
    /// <p>When using <code>MetricMathAnomalyDetector</code>, you cannot include the following parameters in the same operation:</p>
    /// <ul>
    /// <li> <p> <code>Dimensions</code> </p> </li>
    /// <li> <p> <code>MetricName</code> </p> </li>
    /// <li> <p> <code>Namespace</code> </p> </li>
    /// <li> <p> <code>Stat</code> </p> </li>
    /// <li> <p>the <code>SingleMetricAnomalyDetector</code> parameters of <code>PutAnomalyDetectorInput</code> </p> </li>
    /// </ul>
    /// <p>Instead, specify the metric math anomaly detector attributes as part of the property <code>MetricMathAnomalyDetector</code>.</p>
    pub fn set_metric_math_anomaly_detector(mut self, input: ::std::option::Option<crate::types::MetricMathAnomalyDetector>) -> Self {
        self.inner = self.inner.set_metric_math_anomaly_detector(input);
        self
    }
    /// <p>The metric math anomaly detector to be created.</p>
    /// <p>When using <code>MetricMathAnomalyDetector</code>, you cannot include the following parameters in the same operation:</p>
    /// <ul>
    /// <li> <p> <code>Dimensions</code> </p> </li>
    /// <li> <p> <code>MetricName</code> </p> </li>
    /// <li> <p> <code>Namespace</code> </p> </li>
    /// <li> <p> <code>Stat</code> </p> </li>
    /// <li> <p>the <code>SingleMetricAnomalyDetector</code> parameters of <code>PutAnomalyDetectorInput</code> </p> </li>
    /// </ul>
    /// <p>Instead, specify the metric math anomaly detector attributes as part of the property <code>MetricMathAnomalyDetector</code>.</p>
    pub fn get_metric_math_anomaly_detector(&self) -> &::std::option::Option<crate::types::MetricMathAnomalyDetector> {
        self.inner.get_metric_math_anomaly_detector()
    }
}
