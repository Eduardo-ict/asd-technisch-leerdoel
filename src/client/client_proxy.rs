use crate::client::client;

pub struct Calculator {
    ip: String,
}

impl Calculator {
    pub fn new(ip: &str) -> Calculator {
        Calculator {
            ip: ip.to_string(),
        }
    }

    pub fn add(&self, a: i32, b: i32) -> i32 {
        let message = format!("add:{},{}", a, b);
        self.send_message(&message)
    }

    pub fn subtract(&self, a: i32, b: i32) -> i32 {
        let message = format!("subtract:{},{}", a, b);
        self.send_message(&message)
    }

    pub fn multiply(&self, a: i32, b: i32) -> i32 {
        let message = format!("multiply:{},{}", a, b);
        self.send_message(&message)
    }

    pub fn divide(&self, a: i32, b: i32) -> i32 {
        let message = format!("divide:{},{}", a, b);
        self.send_message(&message)
    }

    fn send_message(&self, message: &str) -> i32 {
        let response = match client::send_message(&self.ip, message) {
            Ok(response) => response,
            Err(e) => {
                panic!("Error: {}", e);
            }
        };

        response.trim().parse().unwrap()
    }
}