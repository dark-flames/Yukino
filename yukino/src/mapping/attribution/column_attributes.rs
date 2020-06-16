use yui::YuiAttribute;

#[derive(YuiAttribute, Clone)]
struct Id;

#[derive(YuiAttribute, Clone)]
struct Column {
    pub name: Option<String>,
    #[attribute_field(default=false)]
    pub unique: Option<bool>,
    pub precision: Option<u8>,


    pub scale: Option<u8>
}