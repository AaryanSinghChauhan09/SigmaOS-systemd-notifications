// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-systemd-notifications: systemd notifications component model.
//!
//! Real implementation: notificationss, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notifications {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemdNotificationsError {
    EmptyNotificationsName,
    DuplicateNotifications(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemdNotificationsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNotificationsName => write!(f, "notifications name must not be empty"),
            Self::DuplicateNotifications(id) => write!(f, "duplicate notifications {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemdNotificationsError {}

pub struct SystemdNotifications {
    pub name: String,
    notificationss: BTreeMap<String, Notifications>,
    events: BTreeMap<String, Event>,
}

impl SystemdNotifications {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            notificationss: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_notifications(&mut self, item: Notifications) -> Result<(), SystemdNotificationsError> {
        if item.name.trim().is_empty() {
            return Err(SystemdNotificationsError::EmptyNotificationsName);
        }
        let id = item.id.clone();
        if self.notificationss.contains_key(&id) {
            return Err(SystemdNotificationsError::DuplicateNotifications(id.clone()));
        }
        self.notificationss.insert(id, item);
        Ok(())
    }

    pub fn notifications_count(&self) -> usize {
        self.notificationss.len()
    }

    pub fn get_notifications(&self, id: &str) -> Option<&Notifications> {
        self.notificationss.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemdNotificationsError> {
        if event.subject.trim().is_empty() {
            return Err(SystemdNotificationsError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_notifications(id: &str, name: &str) -> Notifications {
        Notifications {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemdNotifications::new("SystemdNotifications");
        assert_eq!(m.name, "SystemdNotifications");
        assert_eq!(m.notifications_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_notifications() {
        let mut m = SystemdNotifications::new("SystemdNotifications");
        m.add_notifications(sample_notifications("a1", "Alpha")).unwrap();
        assert_eq!(m.notifications_count(), 1);
        assert!(m.get_notifications("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemdNotifications::new("SystemdNotifications");
        assert!(m.add_notifications(sample_notifications("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemdNotifications::new("SystemdNotifications");
        m.add_notifications(sample_notifications("x", "One")).unwrap();
        assert!(m.add_notifications(sample_notifications("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemdNotifications::new("SystemdNotifications");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
