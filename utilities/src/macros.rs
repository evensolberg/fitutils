//! Shared macros

#[macro_export]
macro_rules! set_string_field {
    ($source:ident, $field:ident, $dest:ident) => {
        if let Some($field) = &$source.$field {
            $dest.$field = Some($field.to_string());
        }
    };
    ($source:ident, $src_field:ident, $dest:ident, $dest_field:ident) => {
        if let Some($src_field) = &$source.$src_field {
            $dest.$dest_field = Some($src_field.to_string());
        }
    };
}
