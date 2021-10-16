pub trait SAXAttributes {
    fn get_length(&self) -> usize;
    // Need for boxing https://doc.rust-lang.org/book/trait-objects.html
    fn iter(&self) -> Box<Iterator<Item = Box<SAXAttribute>>>;
    // fn get_by_qualified_name(&mut self, index: usize) -> Option<Attribute>;
    // when using trait as object use box otherwise:
    // the trait `std::marker::Sized` is not implemented for `sax::SAXAttribute + 'static`
    fn get_by_index(&self, index: usize) -> Option<Box<SAXAttribute>>;
}

pub trait SAXAttribute {
    // fn get_index() -> usize;
    fn get_value(&self) -> &str;
    fn get_local_name(&self) -> &str;
    fn get_qualified_name(&self) -> &str;
    fn get_uri(&self) -> &str;
}

pub trait ContentHandler {
    fn start_document(&mut self);
    fn end_document(&mut self);

    fn start_element(
        &mut self,
        uri: &str,
        local_name: &str,
        qualified_name: &str,
        attributes: &SAXAttributes,
    );
    fn end_element(&mut self, uri: &str, local_name: &str, qualified_name: &str);
    fn characters(&mut self, characters: &str);

    //fn start_prefix_mapping(&mut self, prefix: &str , uri: &str);
    //fn end_prefix_mapping(&mut self, prefix: &str , uri: &str);
}

pub trait StatsHandler {
    fn offset(&mut self, offset: usize);
}

//ErrorHandler

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StartElement<'a> {
    pub name: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EndElement<'a> {
    pub name: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event<'a> {
    StartDocument,
    EndDocument,
    StartElement(StartElement<'a>),
    EndElement(EndElement<'a>),
    Characters(&'a str),
    // Comment(BytesText<'a>),
    // CData(BytesText<'a>),
    // Decl(BytesDecl<'a>),
    // PI(BytesText<'a>),
    // DocType(BytesText<'a>),
    // Eof
}
