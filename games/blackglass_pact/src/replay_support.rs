use engine_core::Diagnostic;

use crate::{
    state::BlackglassPactState,
    visibility::{viewer_view, BlackglassViewer, ViewerView},
};

pub const PUBLIC_EXPORT_V1: &str = "blackglass_pact_public_export_v1";
pub const SEAT_PRIVATE_EXPORT_V1: &str = "blackglass_pact_seat_private_export_v1";
pub const ADR_0009_MIGRATION_NOTE: &str = "none";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewerReplayExport {
    pub format: &'static str,
    pub viewer: BlackglassViewer,
    pub migration_note: &'static str,
    pub view: ViewerView,
}

pub fn export_for_viewer(
    state: &BlackglassPactState,
    viewer: BlackglassViewer,
) -> ViewerReplayExport {
    ViewerReplayExport {
        format: match viewer {
            BlackglassViewer::Observer => PUBLIC_EXPORT_V1,
            BlackglassViewer::Seat(_) => SEAT_PRIVATE_EXPORT_V1,
        },
        viewer,
        migration_note: ADR_0009_MIGRATION_NOTE,
        view: viewer_view(state, viewer),
    }
}

pub fn import_for_viewer(
    export: &ViewerReplayExport,
    requested_viewer: BlackglassViewer,
) -> Result<ViewerView, Diagnostic> {
    if export.viewer != requested_viewer {
        return Err(viewer_scope_mismatch_diagnostic());
    }
    Ok(export.view.clone())
}

pub fn export_stable_bytes(export: &ViewerReplayExport) -> Vec<u8> {
    format!(
        "format={};viewer={:?};migration={};view={:?}",
        export.format, export.viewer, export.migration_note, export.view
    )
    .into_bytes()
}

fn viewer_scope_mismatch_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_VIEWER_SCOPE_MISMATCH".to_owned(),
        message: "viewer-scoped export cannot be imported as a different viewer".to_owned(),
    }
}
