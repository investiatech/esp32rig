use crate::model::Message;
use crate::tool::Tool;

pub struct ChatRequest<'a> {
    pub messages: &'a [Message<'a>],
    pub tools: &'a [&'a dyn Tool]
}