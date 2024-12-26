use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct News {
    title: String,
    description: String,
}