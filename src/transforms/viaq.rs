use serde::{Deserialize, Serialize};

use crate::{
    config::{
        DataType, GenerateConfig, Input, Output, TransformConfig, TransformContext,
        TransformDescription,
    },
    event::Event,
    schema,
    transforms::{FunctionTransform, OutputBuffer, Transform},
};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ViaqConfig {
    pub foobar: Option<String>,
}

inventory::submit! {
    TransformDescription::new::<ViaqConfig>("viaq")
}

impl GenerateConfig for ViaqConfig {
    fn generate_config() -> toml::Value {
        toml::Value::try_from(Self {
            foobar: Some("foobar".to_string()),
        })
        .unwrap()
    }
}

#[async_trait::async_trait]
#[typetag::serde(name = "viaq")]
impl TransformConfig for ViaqConfig {
    async fn build(&self, _context: &TransformContext) -> crate::Result<Transform> {
        Ok(Transform::function(Viaq::new()))
    }

    fn input(&self) -> Input {
        Input::log()
    }

    fn outputs(&self, _: &schema::Definition) -> Vec<Output> {
        vec![Output::default(DataType::Log)]
    }

    fn transform_type(&self) -> &'static str {
        "viaq"
    }
}

#[derive(Clone)]
pub struct Viaq {}

impl Viaq {
    pub const fn new() -> Self {
        Self {}
    }
}

const EXCLUSIONS: [&'static str; 7] = ["app.kubernetes.io/name", "app.kubernetes.io/instance",
 "app.kubernetes.io/version", "app.kubernetes.io/component",
 "app.kubernetes.io/part-of", "app.kubernetes.io/managed-by",
 "app.kubernetes.io/created-by"
];

impl FunctionTransform for Viaq {
    fn transform(&mut self, output: &mut OutputBuffer, mut event: Event) {
        let mut flat_labels = Vec::<String>::new();
        if let Some(labels) = event.as_mut_log().get_mut("kubernetes.labels") {
            if labels.is_object() {
                for (key, value) in labels.as_object().unwrap().iter() {
                    if value.is_bytes() {
                        flat_labels.push(key.to_string() + "=" + &value.as_str().unwrap());
                    }
                }
                labels.as_object_mut_unwrap().retain(|k, _| EXCLUSIONS.contains(&k.as_str()));
            }
        }
        if !flat_labels.is_empty() {
            event.as_mut_log().insert("kubernetes.flat_labels", flat_labels);
        }
        output.push(event);
    }
}
