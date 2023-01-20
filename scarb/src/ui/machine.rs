use serde::{Serialize, Serializer};

use crate::ui::Message;

#[derive(Serialize)]
pub struct MachineMessage<T>(pub T);

impl<T> Message for MachineMessage<T>
where
    T: Serialize,
{
    fn text(&self) -> String
    where
        Self: Sized,
    {
        serde_json::to_string_pretty(&self.0).expect("MachineData must serialize without panics")
    }

    fn structured<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        Self: Sized,
    {
        self.0.serialize(ser)
    }
}