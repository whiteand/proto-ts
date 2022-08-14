mod ast;
mod commit_folder;
mod file_name_to_folder_name;
mod file_to_folder;
mod package_tree_to_folder;
mod render_file;
use crate::proto::error::ProtoError;
use crate::proto::package_tree::PackageTree;

use self::ast::Folder;

pub(crate) fn compile(package_tree: &PackageTree) -> Result<(), ProtoError> {
    let folder: Folder = package_tree.try_into()?;

    commit_folder::commit_folder(&folder)?;

    Ok(())
}
