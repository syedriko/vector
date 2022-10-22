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

/*
inventory::submit! {
    TransformDescription::new::<ViaqConfig>("viaq")
}
*/

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
        Ok(Transform::function(Viaq::new(
            self.foobar.clone(),
        )))
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

// Add a compatibility alias to avoid breaking existing configs
#[derive(Deserialize, Serialize, Debug, Clone)]
struct ViaqCompatConfig(ViaqConfig);

#[async_trait::async_trait]
#[typetag::serde(name = "viaq")]
impl TransformConfig for ViaqCompatConfig {
    async fn build(&self, context: &TransformContext) -> crate::Result<Transform> {
        self.0.build(context).await
    }

    fn input(&self) -> Input {
        self.0.input()
    }

    fn outputs(&self, merged_definition: &schema::Definition) -> Vec<Output> {
        self.0.outputs(merged_definition)
    }

    fn transform_type(&self) -> &'static str {
        self.0.transform_type()
    }
}

#[derive(Clone)]
pub struct Viaq {
    foobar: Option<String>,
}

impl Viaq {
    pub const fn new(foobar: Option<String>) -> Self {
        Self {
            foobar,
        }
    }
}

impl FunctionTransform for Viaq {
    fn transform(&mut self, output: &mut OutputBuffer, event: Event) {
        self.foobar.as_ref();
        output.push(event);
    }
}