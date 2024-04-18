use std::collections::HashMap;

#[derive(Clone)]
pub struct Repository {
    dict: HashMap<String, String>,
}

impl Repository {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Repository {
            dict: HashMap::new(),
        })
    }

    fn get_page_path(page_name: String) -> String {
        format!("page::{}", page_name)
    }

    pub fn set_page(
        &mut self,
        page_name: String,
        content: String,
        _ttl: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.dict
            .insert(Repository::get_page_path(page_name), content);

        Ok(())
    }

    pub fn get_page(&self, page_name: String) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self.dict[&Repository::get_page_path(page_name)].clone())
    }
}
