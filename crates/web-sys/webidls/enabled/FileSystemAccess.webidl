enum FileSystemPermissionMode {
  "read",
  "readwrite"
};

dictionary FileSystemPermissionDescriptor : PermissionDescriptor {
  required FileSystemHandle handle;
  FileSystemPermissionMode mode = "read";
};

dictionary FileSystemHandlePermissionDescriptor {
  FileSystemPermissionMode mode = "read";
};

enum FileSystemHandleKind {
  "file",
  "directory",
};

[Exposed=(Window,Worker), SecureContext, Serializable]
interface FileSystemHandle {
  readonly attribute FileSystemHandleKind kind;
  readonly attribute USVString name;

  Promise<boolean> isSameEntry(FileSystemHandle other);

  Promise<PermissionState> queryPermission(optional FileSystemHandlePermissionDescriptor descriptor = {});
  Promise<PermissionState> requestPermission(optional FileSystemHandlePermissionDescriptor descriptor = {});
};

dictionary FileSystemCreateWritableOptions {
  boolean keepExistingData = false;
};

[Exposed=(Window,Worker), SecureContext, Serializable]
interface FileSystemFileHandle : FileSystemHandle {
  Promise<File> getFile();
  Promise<FileSystemWritableFileStream> createWritable(optional FileSystemCreateWritableOptions options = {});
};

dictionary FileSystemGetFileOptions {
  boolean create = false;
};

dictionary FileSystemGetDirectoryOptions {
  boolean create = false;
};

dictionary FileSystemRemoveOptions {
  boolean recursive = false;
};

[Exposed=(Window,Worker), SecureContext, Serializable]
interface FileSystemDirectoryHandle : FileSystemHandle {
  async iterable<USVString, FileSystemHandle>;

  Promise<FileSystemFileHandle> getFileHandle(USVString name, optional FileSystemGetFileOptions options = {});
  Promise<FileSystemDirectoryHandle> getDirectoryHandle(USVString name, optional FileSystemGetDirectoryOptions options = {});

  Promise<undefined> removeEntry(USVString name, optional FileSystemRemoveOptions options = {});

  Promise<sequence<USVString>?> resolve(FileSystemHandle possibleDescendant);
};

enum WriteCommandType {
  "write",
  "seek",
  "truncate",
};

dictionary WriteParams {
  required WriteCommandType type;
  unsigned long long? size;
  unsigned long long? position;
  (BufferSource or Blob or USVString)? data;
};

typedef (BufferSource or Blob or USVString or WriteParams) FileSystemWriteChunkType;

[Exposed=(Window,Worker), SecureContext]
interface FileSystemWritableFileStream : WritableStream {
  Promise<undefined> write(FileSystemWriteChunkType data);
  Promise<undefined> seek(unsigned long long position);
  Promise<undefined> truncate(unsigned long long size);
};

enum WellKnownDirectory {
  "desktop",
  "documents",
  "downloads",
  "music",
  "pictures",
  "videos",
};

typedef (WellKnownDirectory or FileSystemHandle) StartInDirectory;

dictionary FilePickerAcceptType {
    USVString description;
    record<USVString, (USVString or sequence<USVString>)> accept;
};

dictionary FilePickerOptions {
    sequence<FilePickerAcceptType> types;
    boolean excludeAcceptAllOption = false;
    DOMString id;
    StartInDirectory startIn;
};

dictionary OpenFilePickerOptions : FilePickerOptions {
    boolean multiple = false;
};

dictionary SaveFilePickerOptions : FilePickerOptions {
    USVString? suggestedName;
};

dictionary DirectoryPickerOptions {
    DOMString id;
    StartInDirectory startIn;
};

[SecureContext]
partial interface Window {
    Promise<sequence<FileSystemFileHandle>> showOpenFilePicker(optional OpenFilePickerOptions options = {});
    Promise<FileSystemFileHandle> showSaveFilePicker(optional SaveFilePickerOptions options = {});
    Promise<FileSystemDirectoryHandle> showDirectoryPicker(optional DirectoryPickerOptions options = {});
};

partial interface DataTransferItem {
    Promise<FileSystemHandle?> getAsFileSystemHandle();
};

[SecureContext]
partial interface StorageManager {
  Promise<FileSystemDirectoryHandle> getDirectory();
};
