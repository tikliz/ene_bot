use serde::{Serialize, Serializer, ser::SerializeStruct};

#[derive(Clone)]
pub struct Payload {
    pub url: String,
    pub map_artist: String,
    pub map_title: String,
    pub mapper: String,
    pub map_bpm: String,
    pub map_bg: String,
    pub requester: String,
    pub badge: String,

}
impl Serialize for Payload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
            let mut state = serializer.serialize_struct("Color", 3)?;
            state.serialize_field("url", &self.url)?;
            state.serialize_field("map_artist", &self.map_artist)?;
            state.serialize_field("map_title", &self.map_title)?;
            state.serialize_field("mapper", &self.mapper)?;
            state.serialize_field("map_bpm", &self.map_bpm)?;
            state.serialize_field("map_bg", &self.map_bg)?;
            state.serialize_field("requester", &self.requester)?;
            state.serialize_field("badge", &self.badge)?;
            state.end()
        
    }

}

impl IntoIterator for Payload {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            self.url,
            self.map_artist, 
            self.map_title,
            self.mapper,
            self.map_bpm,
            self.map_bg,
            self.requester,
            self.badge,
            
            ].into_iter()

    }

}