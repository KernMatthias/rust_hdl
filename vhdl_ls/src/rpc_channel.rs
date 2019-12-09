// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) 2018, Olof Kraigher olof.kraigher@gmail.com

//! Contains the RpcChannel Traid and associated convenience functions

use lsp_types::*;
use serde;
use vhdl_parser::Message;

pub trait RpcChannel {
    fn send_notification(
        &self,
        method: impl Into<String>,
        notification: impl serde::ser::Serialize,
    );

    fn window_show_message_struct(&self, message: &Message) {
        self.window_show_message(
            to_lsp_message_type(&message.message_type),
            message.message.clone(),
        );
    }

    fn window_show_message(&self, typ: MessageType, message: impl Into<String>) {
        self.send_notification(
            "window/showMessage",
            ShowMessageParams {
                typ,
                message: message.into(),
            },
        );
    }

    fn window_log_message(&self, typ: MessageType, message: impl Into<String>) {
        self.send_notification(
            "window/logMessage",
            LogMessageParams {
                typ,
                message: message.into(),
            },
        );
    }
}
fn to_lsp_message_type(message_type: &vhdl_parser::MessageType) -> MessageType {
    match message_type {
        vhdl_parser::MessageType::Error => MessageType::Error,
        vhdl_parser::MessageType::Warning => MessageType::Warning,
        vhdl_parser::MessageType::Info => MessageType::Info,
        vhdl_parser::MessageType::Log => MessageType::Log,
    }
}

#[cfg(test)]
pub mod test_support {

    use pretty_assertions::assert_eq;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    #[derive(Debug)]
    pub enum RpcExpected {
        Notification {
            method: String,
            notification: serde_json::Value,
        },
        /// Check that the string representation of the notification contains a string
        NotificationContainsString { method: String, contains: String },
    }

    #[derive(Clone)]
    pub struct RpcMock {
        expected: Rc<RefCell<VecDeque<RpcExpected>>>,
    }

    impl RpcMock {
        pub fn new() -> RpcMock {
            RpcMock {
                expected: Rc::new(RefCell::new(VecDeque::new())),
            }
        }

        pub fn expect_notification(
            &self,
            method: impl Into<String>,
            notification: impl serde::ser::Serialize,
        ) {
            self.expected
                .borrow_mut()
                .push_back(RpcExpected::Notification {
                    method: method.into(),
                    notification: serde_json::to_value(notification).unwrap(),
                });
        }

        pub fn expect_notification_contains(
            &self,
            method: impl Into<String>,
            contains: impl Into<String>,
        ) {
            self.expected
                .borrow_mut()
                .push_back(RpcExpected::NotificationContainsString {
                    method: method.into(),
                    contains: contains.into(),
                });
        }
    }

    impl Drop for RpcMock {
        fn drop(&mut self) {
            if !std::thread::panicking() {
                let expected = self.expected.replace(VecDeque::new());
                if expected.len() > 0 {
                    panic!("Not all expected data was consumed\n{:#?}", expected);
                }
            }
        }
    }

    /// True if any string field of the value has string as a substring
    fn contains_string(value: &serde_json::Value, string: &str) -> bool {
        match value {
            serde_json::Value::Array(values) => {
                values.iter().any(|value| contains_string(value, string))
            }
            serde_json::Value::Object(map) => {
                map.values().any(|value| contains_string(value, string))
            }
            serde_json::Value::String(got_string) => got_string.contains(string),
            serde_json::Value::Null => false,
            serde_json::Value::Bool(..) => false,
            serde_json::Value::Number(..) => false,
        }
    }

    impl super::RpcChannel for RpcMock {
        fn send_notification(
            &self,
            method: impl Into<String>,
            notification: impl serde::ser::Serialize,
        ) {
            let method = method.into();
            let notification = serde_json::to_value(notification).unwrap();
            let expected = self
                .expected
                .borrow_mut()
                .pop_front()
                .ok_or_else(|| {
                    panic!(
                        "No expected value, got method={} {:?}",
                        method, notification
                    )
                })
                .unwrap();

            match expected {
                RpcExpected::Notification {
                    method: exp_method,
                    notification: exp_notification,
                } => {
                    assert_eq!(method, exp_method);
                    assert_eq!(notification, exp_notification);
                }
                RpcExpected::NotificationContainsString {
                    method: exp_method,
                    contains,
                } => {
                    assert_eq!(method, exp_method);
                    if !contains_string(&notification, &contains) {
                        panic!(
                            "{:?} does not contain sub-string {:?}",
                            notification, contains
                        );
                    }
                }
            }
        }
    }
}
