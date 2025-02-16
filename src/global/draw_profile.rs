pub mod options;

use options::Options;

#[derive(Debug)]
pub struct DrawProfile {
    pub name: String,
    pub user_id: UserId,
    pub options: Options,
}
