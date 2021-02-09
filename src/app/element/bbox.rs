use serde_json::Value;

struct BBox {
    id: String,
    name: String,
    class: String,

    x: usize,
    y: usize,
    width: usize,
    height: usize,

    style: Value,
}
