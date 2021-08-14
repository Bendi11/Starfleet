use crate::component;

#[component]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Test {
    a: i32
}