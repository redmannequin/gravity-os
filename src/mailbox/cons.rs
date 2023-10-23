use core::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub struct Empty;

pub struct Found;
pub struct NotFound<Pos>(PhantomData<Pos>);

#[derive(Debug, Clone, Copy)]
pub struct Cons<Prev, Item> {
    pub prev: Prev,
    pub item: Item,
}

impl<Prev, Curr> Cons<Prev, Curr> {
    pub fn push<Next>(self, item: Next) -> Cons<Cons<Prev, Curr>, Next> {
        Cons { prev: self, item }
    }
}

pub trait FindItem<Item, Pos> {
    fn find(&self) -> &Item;
}

impl<Prev, Item> FindItem<Item, Found> for Cons<Prev, Item> {
    fn find(&self) -> &Item {
        &self.item
    }
}

impl<Prev, Item, Next, Pos> FindItem<Item, NotFound<Pos>> for Cons<Prev, Next>
where
    Prev: FindItem<Item, Pos>,
{
    fn find(&self) -> &Item {
        self.prev.find()
    }
}
