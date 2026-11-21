# Uploader

File queue with application-provided upload futures and per-file progress.
Coordinates picking files, reporting percentage upload progress, handling retries, and item removal.

Status: **draft** on web.

## Use

* Batch uploading files to a cloud or remote endpoint.
* Providing granular per-file progress and failure recovery.

## Geometry

| Property | Value |
| :--- | :--- |
| Gap | `gap_small` (`2 DU`). |
| Padding | `0 DU`. |
| Corner radius | `0 DU`. |

## Behavior

* The embedded [FileField](file-field.md) accepts chosen and dropped files.
* Clicking upload executes the async handler for each file with a progress reporter callback.
* While uploading, individual linear progress bars reflect current fractions.
* Failed items show error messages and present a retry button.
* Removing an item cancels its upload attempt and removes it from the list.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `on_upload` | `AsyncAction<UploadRequest>` | required | Upload handler receiving file and progress callback. |
| `files` | `RwSignal<Vec<PickedFile>>` | internal | Signal containing picked files. |
| `enabled` | bool | `true` | Enables upload actions. |
| `accept` | string | none | MIME type and file extension filter. |
| `label` | string | `uploader.label` | Accessible title of the uploader section. |
| `upload_label` | string | `uploader.upload` | Label for the upload action button. |
| `retry_label` | string | `uploader.retry` | Label for retry buttons on failed items. |
| `remove_label` | string | `uploader.remove` | Label for remove item buttons. |
| `empty_label` | string | `uploader.empty` | Message displayed when queue is empty. |
| `on_change` | callback `(Vec<PickedFile>)` | none | Emits when file list changes. |
| `on_complete` | callback `((u64, ActionResult))` | none | Emits on individual upload completion. |
| `row` | `ScopedSlot<(PickedFile, UploadStatus)>` | none | Custom renderer for a file queue row. |
| `header` | `Slot` | none | Header content slot above file field. |
| `actions` | `Slot` | none | Custom action buttons slot. |
| `footer` | `Slot` | none | Footer content slot below queue list. |
| `gap` | DU metric | `gap_small` | Spacing between items and sections. |
| `padding` | DU metric | none | Optional padding around uploader. |
| `class`, `style` | string | none | Root hooks. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `gap` | DU | `--op-uploader-gap` | `gap_small` (`2 DU`) |
| `padding` | DU | `--op-uploader-padding` | `0 DU` |

| Color variable | Default |
| :--- | :--- |
| `--op-uploader-background` | `surface` |
| `--op-uploader-text` | `text_primary` |

| Slot | Place |
| :--- | :--- |
| `header` | Preceding the file field. |
| `actions` | Between file field and queue list. |
| `footer` | Below the file list. |
| `row` | Scoped slot overriding each queue row item. |

`class` and `style` apply to the root section element.

### Strings

| Key | Default |
| :--- | :--- |
| `uploader.label` | `Upload files` |
| `uploader.upload` | `Upload` |
| `uploader.retry` | `Retry` |
| `uploader.remove` | `Remove` |
| `uploader.empty` | `No files selected` |

### Icons

Uploader reuses icons from FileField and LinearProgress.

## Accessibility

The root renders an accessible `section` with an `aria-label`.
Each uploading row reports progress and failure states using `role="status"` and accessible action buttons.
