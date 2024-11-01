#![allow(dead_code)]
use std::{collections::HashMap, future::Future, pin::Pin};

/// How to store async fn in a struct? And how to call the async fn?

type CallbackFn = Box<dyn Fn(i32) -> Pin<Box<dyn Future<Output = String>>>>;

struct Register {
    callbacks: HashMap<String, CallbackFn>,
}

impl Register {
    fn new() -> Self {
        Register {
            callbacks: HashMap::new(),
        }
    }

    fn add_callback(&mut self, name: String, callback: CallbackFn) {
        self.callbacks.insert(name, callback);
    }

    async fn call_callback(&self, name: &str, param: i32) -> Option<String> {
        if let Some(callback) = self.callbacks.get(name) {
            let fut = (callback)(param);
            Some(fut.await)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn double(i: i32) -> String {
        let i = i * 2;
        i.to_string()
    }
    async fn square(i: i32) -> String {
        let i = i * i;
        i.to_string()
    }

    #[tokio::test]
    async fn test() {
        let mut register = Register::new();

        // Register a callback function
        register.add_callback("double".to_string(), Box::new(|a| Box::pin(double(a))));
        register.add_callback("square".to_string(), Box::new(|a| Box::pin(square(a))));

        // Call the registered callbacks
        if let Some(result) = register.call_callback("double", 5).await {
            println!("Double of 5 is: {}", result);
        }

        if let Some(result) = register.call_callback("square", 5).await {
            println!("Square of 5 is: {}", result);
        }

        // Attempt to call a non-existent callback
        if let Some(result) = register.call_callback("nonexistent", 5).await {
            println!("This should not be printed: {}", result);
        } else {
            println!("Callback 'nonexistent' not found.");
        }
    }
}
