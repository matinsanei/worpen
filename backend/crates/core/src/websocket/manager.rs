use dashmap::DashMap;
use tokio::sync::mpsc;
use std::sync::Arc;

/// Message to be sent through WebSocket
#[derive(Debug, Clone)]
pub struct WsMessage {
    pub content: String,
    pub channel: Option<String>,
}

/// Type for WebSocket sender
pub type WsSender = mpsc::UnboundedSender<String>;

/// WebSocket connection manager for handling multiple active connections
#[derive(Clone)]
pub struct WebSocketManager {
    /// Map of connection_id -> sender
    connections: Arc<DashMap<String, WsSender>>,
    /// Map of channel_name -> Vec<connection_id>
    channels: Arc<DashMap<String, Vec<String>>>,
}

impl WebSocketManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(DashMap::new()),
            channels: Arc::new(DashMap::new()),
        }
    }

    /// Register a new WebSocket connection
    pub fn register(&self, connection_id: String, sender: WsSender) {
        self.connections.insert(connection_id, sender);
    }

    /// Unregister a WebSocket connection
    pub fn unregister(&self, connection_id: &str) {
        self.connections.remove(connection_id);
        
        // Remove from all channels
        for mut entry in self.channels.iter_mut() {
            entry.value_mut().retain(|id| id != connection_id);
        }
    }

    /// Subscribe a connection to a channel
    pub fn subscribe(&self, connection_id: String, channel: String) {
        self.channels
            .entry(channel)
            .or_default()
            .push(connection_id);
    }

    /// Unsubscribe a connection from a channel
    pub fn unsubscribe(&self, connection_id: &str, channel: &str) {
        if let Some(mut entry) = self.channels.get_mut(channel) {
            entry.value_mut().retain(|id| id != connection_id);
        }
    }

    /// Send message to a specific connection
    pub fn send_to(&self, connection_id: &str, message: String) -> Result<(), String> {
        if let Some(sender) = self.connections.get(connection_id) {
            sender.send(message)
                .map_err(|e| format!("Failed to send message: {}", e))?;
            Ok(())
        } else {
            Err(format!("Connection {} not found", connection_id))
        }
    }

    /// Broadcast message to all connections
    pub fn broadcast(&self, message: String) -> Result<(), String> {
        let mut failed = Vec::new();
        
        for entry in self.connections.iter() {
            if let Err(e) = entry.value().send(message.clone()) {
                failed.push(format!("Failed to send to {}: {}", entry.key(), e));
            }
        }

        if failed.is_empty() {
            Ok(())
        } else {
            Err(failed.join("; "))
        }
    }

    /// Broadcast message to all connections in a channel
    pub fn broadcast_to_channel(&self, channel: &str, message: String) -> Result<(), String> {
        if let Some(connection_ids) = self.channels.get(channel) {
            let mut failed = Vec::new();
            
            for conn_id in connection_ids.value() {
                if let Some(sender) = self.connections.get(conn_id) {
                    if let Err(e) = sender.send(message.clone()) {
                        failed.push(format!("Failed to send to {}: {}", conn_id, e));
                    }
                }
            }

            if failed.is_empty() {
                Ok(())
            } else {
                Err(failed.join("; "))
            }
        } else {
            Err(format!("Channel {} not found", channel))
        }
    }

    /// Get number of active connections
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }

    /// Get number of connections in a channel
    pub fn channel_size(&self, channel: &str) -> usize {
        self.channels
            .get(channel)
            .map(|entry| entry.value().len())
            .unwrap_or(0)
    }
}

impl Default for WebSocketManager {
    fn default() -> Self {
        Self::new()
    }
}
