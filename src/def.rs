pub trait Definition<'a> {
    fn mint(self, builder: specs::EntityBuilder<'a>) -> specs::EntityBuilder<'a>;
}