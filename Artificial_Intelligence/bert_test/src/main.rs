use rust_bert::gpt_neo::GptNeoConfigResources;
use rust_bert::gpt_neo::GptNeoModelResources;
use rust_bert::resources::RemoteResource;

fn main() {
    let model_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoModelResource::GPT_NEO_2_7B,
    ));
    let config_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoConfigResource::GPT_NEO_2_7B,
    ));
}
