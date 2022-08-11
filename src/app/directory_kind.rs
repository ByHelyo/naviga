use crate::app::directory::Directory;

enum DirectoryKind {
    Directory(Directory),
    PermissionDenied,
}

impl DirectoryKind {
    pub fn is_permission_denied(&self) -> bool {
        if let DirectoryKind::PermissionDenied = self {
            true
        } else {
            false
        }
    }
}
