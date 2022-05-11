use super::*;
use std::ops::Range;

pub struct MessageEntityRef<'a> {
    message: &'a str,
    range: Range<usize>,
    entity: &'a MessageEntity,
}

fn convert_offsets(text: &str, ranges16: &[Range<usize>]) -> Vec<Range<usize>> {
    let mut offset16 = 0;
    let mut offset8 = 0;
    let mut mapping = vec![usize::MAX; text.as_bytes().len() + 1];
    mapping[0] = 0;
    for c in text.chars() {
        offset16 += c.len_utf16();
        offset8 += c.len_utf8();
        mapping[offset16] = offset8;
    }
    ranges16
        .iter()
        .map(|&Range { start, end }| Range {
            start: mapping.get(start).copied().unwrap_or_default(),
            end: mapping.get(end).copied().unwrap_or_default(),
        })
        .collect()
}

fn make_entities_refs<'a>(
    message: &'a str,
    entities: &'a [MessageEntity],
) -> Vec<MessageEntityRef<'a>> {
    let ranges16: Vec<_> = entities
        .iter()
        .map(|e| Range {
            start: e.offset,
            end: e.offset + e.length,
        })
        .collect();
    let ranges8 = convert_offsets(message, ranges16.as_slice());
    ranges8
        .into_iter()
        .zip(entities.iter())
        .map(|(range, entity)| MessageEntityRef {
            message,
            range,
            entity,
        })
        .collect()
}

impl MessageEntityRef<'_> {
    pub fn range_utf8(&self) -> Range<usize> {
        self.range.clone()
    }
    pub fn text(&self) -> Option<&str> {
        self.message.get(self.range.clone())
    }
    pub fn entity(&self) -> &MessageEntity {
        &self.entity
    }
}

pub trait WithEntities {
    fn entities_refs(&self) -> Vec<MessageEntityRef>;
}

impl WithEntities for MediaAnimation {
    fn entities_refs(&self) -> Vec<MessageEntityRef> {
        self.caption
            .as_ref()
            .map(|text| make_entities_refs(text.as_str(), self.caption_entities.as_slice()))
            .unwrap_or_default()
    }
}

impl WithEntities for MediaAudio {
    fn entities_refs(&self) -> Vec<MessageEntityRef> {
        self.caption
            .as_ref()
            .map(|text| make_entities_refs(text.as_str(), self.caption_entities.as_slice()))
            .unwrap_or_default()
    }
}

impl WithEntities for MediaDocument {
    fn entities_refs(&self) -> Vec<MessageEntityRef> {
        self.caption
            .as_ref()
            .map(|text| make_entities_refs(text.as_str(), self.caption_entities.as_slice()))
            .unwrap_or_default()
    }
}

impl WithEntities for MediaPhoto {
    fn entities_refs(&self) -> Vec<MessageEntityRef> {
        self.caption
            .as_ref()
            .map(|text| make_entities_refs(text.as_str(), self.caption_entities.as_slice()))
            .unwrap_or_default()
    }
}

impl WithEntities for MediaText {
    fn entities_refs(&self) -> Vec<MessageEntityRef> {
        make_entities_refs(self.text.as_str(), self.entities.as_slice())
    }
}

impl WithEntities for MediaVideo {
    fn entities_refs(&self) -> Vec<MessageEntityRef> {
        self.caption
            .as_ref()
            .map(|text| make_entities_refs(text.as_str(), self.caption_entities.as_slice()))
            .unwrap_or_default()
    }
}

impl WithEntities for MediaVoice {
    fn entities_refs(&self) -> Vec<MessageEntityRef> {
        self.caption
            .as_ref()
            .map(|text| make_entities_refs(text.as_str(), self.caption_entities.as_slice()))
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use message_entity::MessageEntityKind::*;
    use message_entity::MessageEntity;

    #[test]
    fn byba() {
        let msg = MediaText{
            text: "быба".into(),
            entities: vec![
            MessageEntity { kind: Strikethrough, offset: 0, length: 1 },
            MessageEntity { kind: Bold, offset: 1, length: 1 },
            MessageEntity { kind: Italic, offset: 2, length: 1 },
            MessageEntity { kind: Code, offset: 3, length: 1 },
            ]
        };
        let refs = msg.entities_refs();
        assert_eq!( refs[0].text(), Some("б") );
        assert_eq!( refs[1].text(), Some("ы") );
        assert_eq!( refs[2].text(), Some("б") );
        assert_eq!( refs[3].text(), Some("а") );
    }
    #[test]
    fn symbol_24bit() {
        let msg = MediaText { 
            text: "xx আ #tt".into(), 
            entities: vec![MessageEntity { kind: Hashtag, offset: 5, length: 3 }] 
        };
        let refs = msg.entities_refs();
        assert_eq!( refs[0].text(), Some("#tt") );
    }

    #[test]
    fn tag_with_smile() {
        let msg = MediaText {
            text: "smile 😁 перед тэгом #bugoga".into(),
            entities: vec![MessageEntity { kind: Hashtag, offset: 21, length: 7 }],
        };
        let refs = msg.entities_refs();
        assert_eq!(refs[0].text(), Some("#bugoga"))
    }

}