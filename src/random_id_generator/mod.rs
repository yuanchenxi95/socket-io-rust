use uuid::Uuid;

pub struct RandomIdGenerator;

impl RandomIdGenerator {
    pub fn get_random_uuid() -> String {
        Uuid::new_v4().to_simple().to_string()
    }
}
