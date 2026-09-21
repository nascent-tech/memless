use crate::status::{MemlessHandle, MemlessStatus};

pub(crate) struct Outcome {
    pub status: MemlessStatus,
    pub handle: Option<MemlessHandle>,
    pub message: Option<String>,
}

impl Outcome {
    pub(crate) fn accepted(handle: MemlessHandle) -> Outcome {
        Outcome { status: MemlessStatus::Ok, handle: Some(handle), message: None }
    }

    pub(crate) fn refused(message: String) -> Outcome {
        Outcome { status: MemlessStatus::Refused, handle: None, message: Some(message) }
    }

    pub(crate) fn invalid(message: &str) -> Outcome {
        Outcome { status: MemlessStatus::InvalidArgument, handle: None, message: Some(message.to_string()) }
    }

    pub(crate) fn internal() -> Outcome {
        Outcome { status: MemlessStatus::Internal, handle: None, message: Some("internal error".to_string()) }
    }
}
