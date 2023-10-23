use core::mem;

use crate::mailbox::cons::{Cons, Empty};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tag<R> {
    pub tag_id: u32,
    pub size: u32,
    pub state: u32,
    pub data: R,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union TagDataUnion<Req, Res>
where
    Req: Copy,
    Res: Copy,
{
    request: Req,
    response: Res,
}

pub trait RequestTag: Copy {
    const TAG_ID: u32;
    const TAG_SIZE: u32 = mem::size_of::<TagDataUnion<Self, Self::ReponseType>>() as _;
    type ReponseType: Copy;

    fn into_tag(self) -> Tag<TagDataUnion<Self, Self::ReponseType>> {
        Tag {
            tag_id: Self::TAG_ID,
            size: Self::TAG_SIZE,
            state: 0x0,
            data: TagDataUnion { request: self },
        }
    }
}

pub trait IntoTag {
    type Output;
    fn into_tag(self) -> Self::Output;
}

impl<T> IntoTag for T
where
    T: RequestTag,
{
    type Output = Tag<TagDataUnion<T, T::ReponseType>>;

    fn into_tag(self) -> Self::Output {
        self.into_tag()
    }
}

impl<Item> IntoTag for Cons<Empty, Item>
where
    Item: IntoTag,
{
    type Output = Cons<Empty, Item::Output>;

    fn into_tag(self) -> Self::Output {
        Cons {
            prev: Empty,
            item: self.item.into_tag(),
        }
    }
}

impl<Prev, Item> IntoTag for Cons<Prev, Item>
where
    Prev: IntoTag,
    Item: IntoTag,
{
    type Output = Cons<Prev::Output, Item::Output>;

    fn into_tag(self) -> Self::Output {
        Cons {
            prev: self.prev.into_tag(),
            item: self.item.into_tag(),
        }
    }
}

pub trait IntoRes {
    type Output;
    fn into_res(self) -> Self::Output;
}

impl<Req, Res> IntoRes for Tag<TagDataUnion<Req, Res>>
where
    Req: Copy,
    Res: Copy,
{
    type Output = Tag<Res>;
    fn into_res(self) -> Self::Output {
        Tag {
            tag_id: self.tag_id,
            size: self.size,
            state: self.state,
            data: unsafe { self.data.response },
        }
    }
}

impl<Item> IntoRes for Cons<Empty, Item>
where
    Item: IntoRes,
{
    type Output = Cons<Empty, Item::Output>;
    fn into_res(self) -> Self::Output {
        Cons {
            prev: Empty,
            item: self.item.into_res(),
        }
    }
}

impl<Prev, Item> IntoRes for Cons<Prev, Item>
where
    Prev: IntoRes,
    Item: IntoRes,
{
    type Output = Cons<Prev::Output, Item::Output>;
    fn into_res(self) -> Self::Output {
        Cons {
            prev: self.prev.into_res(),
            item: self.item.into_res(),
        }
    }
}
