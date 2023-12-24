use r_base::SecurityAgent;

pub struct RaspAgent{
    pub name: String
}

impl SecurityAgent for RaspAgent {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn on_load(&self) {
        println!("load agent {}", &self.name);
    }
}

impl Drop for RaspAgent {
    fn drop(&mut self) {
        println!("agent droped: {}", self.name);
        let _ = drop(self);
    }
}


#[no_mangle]
pub extern "C" fn _load_agent() -> *mut dyn SecurityAgent {
    let rasp_agent = RaspAgent{name: "test2".to_string()};
    let boxed = Box::new(rasp_agent);
    Box::into_raw(boxed)
}