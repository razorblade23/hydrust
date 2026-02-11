pub fn generate_uuid() -> String {
    use uuid::Uuid;
    Uuid::new_v4().to_string()
}
