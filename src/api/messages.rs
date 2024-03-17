pub enum MessageType {
    Start = 0,
    Exit = 1,
    Interrupt = 2,
    Ok = 3,
    Log = 4,
}

/// Expects OkMessage
pub struct StartMessage {
    content: String,
}

/// Expects OkMessage
pub struct ExitMessage {
    code: i32,
    content: String,
}

/// Expects OkMessage
pub struct InterruptMessage {
    signal: i32,
}

/// Mostly used to answer other messages
pub struct OkMessage {}

/// Expects OkMessage
pub struct LogMessage {
    kind: String,
    content: String,
}
