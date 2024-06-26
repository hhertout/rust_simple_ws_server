use warp::reject::Reject;


pub(crate) mod chat_controller;
pub(crate) mod room_controller;


#[derive(Debug)]
pub struct CustomError {
    pub message: String,
}

impl Reject for CustomError {}