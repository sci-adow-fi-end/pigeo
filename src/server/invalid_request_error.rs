use crate::comunication::message_type::Answer;
use ::thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidRequestError {
    #[error("the inserted username is not valid")]
    BadUsername,
    #[error("the inserted password is not valid")]
    BadPassword,
    #[error("the inserted sender is not valid")]
    BadSender,
    #[error("the inserted receiver is not valid")]
    BadReceiver,
}
impl InvalidRequestError {
    pub fn into_answer(&self) -> Answer {
        return match self {
            Self::BadUsername => Answer::BadName,
            Self::BadPassword => Answer::BadPwd,
            Self::BadSender => Answer::BadSender,
            Self::BadReceiver => Answer::BadReceiver,
        };
    }
}
