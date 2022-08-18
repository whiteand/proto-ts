mod ast;
mod commit_folder;
mod file_name_to_folder_name;
mod file_to_folder;
mod package_tree_to_folder;
mod render_file;
use crate::proto::error::ProtoError;
use crate::proto::package_tree::PackageTree;
use package_tree_to_folder::root_tree_to_folder;
mod protopath;
mod ts_path;
mod block_scope;
mod defined_id;
mod ensure_import;
mod get_relative_import;
mod message_name_to_encode_type_name;
mod types_compiler;

use self::ast::Folder;

pub(crate) fn compile(package_tree: &PackageTree) -> Result<(), ProtoError> {
    let folder: Folder = root_tree_to_folder(package_tree)?;

    commit_folder::commit_folder(&folder)?;

    Ok(())
}
