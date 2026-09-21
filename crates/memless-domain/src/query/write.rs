use super::delete::Delete;
use super::insert::Insert;
use super::update::Update;

#[derive(Debug, Clone, PartialEq)]
pub enum Write {
    Insert(Insert),
    Update(Update),
    Delete(Delete),
}
