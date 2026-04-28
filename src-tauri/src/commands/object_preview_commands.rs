use crate::services::object_preview_service;

#[tauri::command]
pub fn get_object_file_info(
    path: String,
) -> Result<object_preview_service::ObjectFileInfo, String> {
    object_preview_service::get_object_file_info(&path)
}

#[tauri::command]
pub fn list_object_directory(
    path: String,
) -> Result<Vec<object_preview_service::ObjectDirectoryEntry>, String> {
    object_preview_service::list_object_directory(&path)
}

#[tauri::command]
pub fn get_audio_preview(path: String) -> Result<object_preview_service::AudioPreviewInfo, String> {
    object_preview_service::get_audio_preview(&path)
}
