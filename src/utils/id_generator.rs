use uuid::Uuid;

pub fn generate_uuid(prefix: &str) -> String {
    let s = prefix.to_string() + &Uuid::new_v4().to_string();
    s.replace("-", "")
}