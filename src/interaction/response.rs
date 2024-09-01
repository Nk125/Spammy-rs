use serde::{Deserialize, Serialize};
use serde_repr::*;
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "type")]
    pub category: ResponseType,
    pub data: Option<serde_json::Value>
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ResponseType {
    Pong = 1,
    ChannelMessageWithSource = 4
}

/*impl Response {
    pub fn extend_data(&mut self, mut from: serde_json::Value) -> Result<&mut Self, ()> {
        let und_obj = self.data.get_or_insert(serde_json::Value::default()).as_object_mut().ok_or(())?;

        und_obj.append(from.as_object_mut().ok_or(())?);

        Ok(self)
    }
}*/
