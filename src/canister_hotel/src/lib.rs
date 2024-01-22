#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello Bali Hotel, {}!", name)
}
