# Changelog — Photoshop by Tonynehihi

Personal build of PhotoCraft for Tony. Versions follow semver: patch = small
fixes, minor = a batch of features, major = breaking changes.

## 1.3.1 — 2026-10-24

**SYNCS UPSTREAM PHOTOCRAFT v0.15.1** into Photoshop by Tonynehihi, keeping the personal-build changes from 1.3.0: no upstream Discord / website / GitHub links anywhere in the UI, the Adobe-style PsT icon (including the Dock icon fix), and the in-app updater pointed at this repo.

- **MAJOR SECURITY HARDENING WAVE** — hostile geometry and raster work, JPEG/WebP/PSD/ICC parsing, plugin host routing, brush work, and document working sets are now bounded and charged against budgets, so crafted files cannot exhaust memory or stall the app.
- **SELECT AND MASK EDITS NOW FLOW THROUGH refineEdge PARAMS** — Brush, Lasso and Refine Edge Brush paint tile by tile, and a refine too large to allocate is refused up front instead of failing mid-edit.
- **SAFER STORED PATHS AND LINKED FILES** — Windows drive and share paths are refused outside Windows, and linked files resolve through absolute paths.
- **MACOS PACKAGING RETRY** — DMG stapling retries when Apple's ticket lookup drops, so releases are less likely to fail at the last step.
- PSD pattern budget charges now survive parse failures, ICC profile identity is confirmed by bytes (not just a hash), duplicate EXIF text is no longer copied repeatedly, and the droplet shim runs with restricted permissions.
- Many smaller fixes across paths, text, paint and the UI.

— tonynehihi

## 1.3.0 — 2026-10-23

**SYNCS UPSTREAM PHOTOCRAFT v0.14.0** (through v0.13.0) into Photoshop by Tonynehihi, keeping the personal-build changes from 1.2.0: no upstream Discord / website / GitHub links anywhere in the UI, the Adobe-style PsT icon, and the in-app updater pointed at this repo.

- **CAMERA RAW DEVELOP PREVIEW NOW RENDERS OFF THE UI THREAD** — sliders stay responsive while the preview catches up.
- **INDEXED COLOR IS UP TO 8× FASTER**, and Remove Background now finds the subject inside the layer.
- **HEX COLOR INPUT FOR SHAPE FILL AND STROKE**, plus a new 3×3 reference widget for Free Transform.
- **SECURITY HARDENING WHEN OPENING FILES**: compressed SVG is no longer decompressed twice, temporary and recovery files are created safely, and PCX/XPM outputs are preflight-checked before allocating.
- **FIXED: DOCK ICON** — PsT now keeps its own icon in the Dock instead of switching to a generic one after launch.
- View › Extras (Cmd+H) now hides the Move tool's transform controls; the Actions automation gate and paused playbacks are more robust.
- Many smaller fixes across paths, the pen tool, transforms and the UI.

— tonynehihi

## 1.2.0 — 2026-10-22

- **SYNCED WITH PHOTOCRAFT v0.12.0**: Photoshop by Tonynehihi now sits on upstream PhotoCraft v0.12.0, including everything in that feature wave.
- **41 SMART FILTERS**: Hue/Saturation, Perspective Warp, the Filter Gallery, Camera Raw and more, all as non-destructive smart filters.
- **DDS TEXTURE EXPORT**: save textures as DDS (BC1–BC3) with mipmaps.
- **INDEXED PSD KEEPS TRANSPARENCY AND ICC**: indexed-colour PSD saves no longer drop the alpha channel or the colour profile.
- **CAMERA RAW PREVIEWS OFF THE UI THREAD**: previews render in the background so the interface stays responsive; the Open/Save dialogs also gained per-format filters.
- **NEW PREFERENCE — "SHOW FILE NAME IN TITLE"**: optionally show the current file name in the window title bar.
- **ALL UPSTREAM COMMUNITY LINKS REMOVED**: the Discord button, the start-screen link row and the About/links surfaces are gone — the Help menu is now exactly About / Check for Updates / System Info / Keyboard Shortcuts, and nothing in the UI points at upstream any more.

— tonynehihi

## 1.1.0 — 2026-10-21

- **NEW NAME — "PsT"**: the app is called **PsT** in Spotlight, Finder and the Dock; inside the app it keeps the full name **Photoshop by Tonynehihi**. Settings, presets and data folders are unchanged.
- **SELF-UPDATE**: the app now checks GitHub (`tonynehihi/pst-by-tonynehihi`) at launch and under Help → Check for Updates. A launch prompt shows the changelog; on macOS the new version downloads, builds inside the app with live progress, and installs with a short quit-and-swap at the end.
- **NEW ICON**: the PsT icon redrawn in the Adobe identity style (dark blue panel, bright blue letters), matching the rest of the "by Tonynehihi" suite.
- **CLEAN HELP MENU**: upstream community links (Discord, websites, upstream GitHub) removed from the Help menu — it now holds Check for Updates, System Info and About.
- First release published on GitHub; releases from here on are installed by the in-app updater (this one is installed manually once).

— tonynehihi

## 1.0 — 2026-10-14

- First personal build: PhotoCraft 0.8.2 rebranded as "Photoshop by Tony" (name, PsT icon, macOS menus, upstream menu links removed from the About surface).

— tonynehihi
