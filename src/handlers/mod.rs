pub mod auth;
pub mod bookmark;
pub mod category;
pub mod health;
pub mod note;
pub mod tag;

pub use auth::__path_login;
pub use auth::__path_me;
pub use auth::__path_register;
pub use auth::{login, me, register};

pub use bookmark::__path_create_bookmark;
pub use bookmark::__path_delete_bookmark;
pub use bookmark::__path_get_bookmark;
pub use bookmark::__path_list_bookmarks;
pub use bookmark::__path_update_bookmark;
pub use bookmark::{
    create_bookmark, delete_bookmark, get_bookmark, list_bookmarks, update_bookmark,
};

pub use category::__path_create_category;
pub use category::__path_delete_category;
pub use category::__path_get_category;
pub use category::__path_list_categories;
pub use category::__path_update_category;
pub use category::{
    create_category, delete_category, get_category, list_categories, update_category,
};

pub use note::__path_create_note;
pub use note::__path_delete_note;
pub use note::__path_get_note;
pub use note::__path_list_notes;
pub use note::__path_update_note;
pub use note::{create_note, delete_note, get_note, list_notes, update_note};

pub use tag::__path_create_tag;
pub use tag::__path_delete_tag;
pub use tag::__path_get_tag;
pub use tag::__path_list_tags;
pub use tag::__path_update_tag;
pub use tag::{create_tag, delete_tag, get_tag, list_tags, update_tag};

pub use health::__path_liveness;
pub use health::__path_readiness;
pub use health::{liveness, readiness};
