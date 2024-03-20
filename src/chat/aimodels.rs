pub fn chat() {
    println!("Hello, world!");
}

struct BaseAIModel {
    name: String,
    description: String,
    version: String,
    model: String,
    token: BaseToken,
}

struct BaseToken {
    name: String,
    description: String,
    key: String,
    url: String,
}

trait BaseAIModelTrait {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn version(&self) -> String;
    fn model(&self) -> String;
    fn token(&self) -> BaseToken;
}
