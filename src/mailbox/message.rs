use crate::mailbox::{
    cons::{Cons, Empty, FindItem},
    tag::{IntoRes, IntoTag, RequestTag},
    MSG_STATE_REQ,
};

#[repr(C, align(16))]
#[derive(Debug)]
pub struct Message<Tags> {
    pub size: u32,
    pub state: u32,
    tags: Tags,
    end: u32,
}

impl Message<Empty> {
    pub fn new() -> Self {
        Self {
            size: 12,
            state: MSG_STATE_REQ,
            tags: Empty,
            end: 0,
        }
    }
}

impl Default for Message<Empty> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Tags> Message<Tags> {
    pub fn with_tag<T>(self, tag: T) -> Message<Cons<Tags, T>>
    where
        T: RequestTag,
    {
        Message {
            size: self.size + T::TAG_SIZE + 12,
            state: self.state,
            tags: Cons {
                prev: self.tags,
                item: tag,
            },
            end: self.end,
        }
    }

    pub(crate) fn into_tags(self) -> Message<Tags::Output>
    where
        Tags: IntoTag,
    {
        Message {
            size: self.size,
            state: self.state,
            tags: self.tags.into_tag(),
            end: self.end,
        }
    }

    pub(crate) fn into_res(self) -> Message<Tags::Output>
    where
        Tags: IntoRes,
    {
        Message {
            size: self.size,
            state: self.state,
            tags: self.tags.into_res(),
            end: self.end,
        }
    }
}

impl<Prev, Tail> Message<Cons<Prev, Tail>> {
    pub fn get_tag<T, Pos>(&self) -> &T
    where
        Cons<Prev, Tail>: FindItem<T, Pos>,
    {
        self.tags.find()
    }
}
