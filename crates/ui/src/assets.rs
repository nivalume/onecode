//! The app's asset source: gpui-kit's default component icons, the extra
//! Lucide icons OneCode's views use, and OneCode's own mark.

use std::borrow::Cow;

use gpui_kit::{AssetSource, Result, SharedString};

// gpui-kit embeds only its ~100 component icons by default; every other
// `IconName` a view uses must be listed here or it renders blank.
gpui_kit::assets::icon_assets!(
    ExtraIcons,
    [
        ChartColumn,
        CircleDot,
        CirclePause,
        Clock,
        Code,
        Columns2,
        Compass,
        Download,
        FileDiff,
        GitBranch,
        GitPullRequest,
        Bookmark,
        Laptop,
        Layers,
        Maximize2,
        MessageSquare,
        Monitor,
        Paperclip,
        Plug,
        Rows2,
        Shield,
        ShieldOff,
        SquareMenu,
        SquarePen,
        Smartphone,
        SquarePlay,
    ]
);

/// Path of the OneCode mark, drawn on the agent's avatar.
pub(crate) const ONECODE_MARK: &str = "icons/onecode-mark.svg";

/// Asset source to register with `Application::with_assets`.
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path == ONECODE_MARK {
            return Ok(Some(Cow::Borrowed(include_bytes!(
                "../assets/icons/onecode-mark.svg"
            ))));
        }
        match ExtraIcons.load(path)? {
            Some(bytes) => Ok(Some(bytes)),
            None => gpui_kit::assets::Assets.load(path),
        }
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut paths = gpui_kit::assets::Assets.list(path)?;
        paths.extend(ExtraIcons.list(path)?);
        if ONECODE_MARK.starts_with(path) {
            paths.push(ONECODE_MARK.into());
        }
        paths.sort();
        paths.dedup();
        Ok(paths)
    }
}
