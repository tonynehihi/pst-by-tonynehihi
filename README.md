<p align="center">
  <img alt="Photoshop by Tonynehihi" src="assets/app-icon/pst-icon-1024.png" width="180">
</p>

<h1 align="center">Photoshop by Tonynehihi</h1>

<p align="center">
  <b>A personal fork of <a href="https://github.com/storytold/photocraft">PhotoCraft</a> — rebranded and customized by Tony for his own use.</b><br>
  Not an official release. Personal use only.
</p>

## Origin

All of the original source code in this project is [PhotoCraft](https://github.com/storytold/photocraft) by **storytold** — an open-source, clean-room reimplementation of a Photoshop-style image editor, written in Rust and released under the **MIT / Apache-2.0** licenses (see `LICENSE-MIT` and `LICENSE-APACHE` in this repo, kept unchanged from the original).

This repository is **not** the official PhotoCraft and is not a public product release. It is a copy that Tony took, renamed to **Photoshop by Tonynehihi**, and adjusted for his own personal workflow. All credit for the original software belongs to storytold and the PhotoCraft contributors.

> "Photoshop" is a trademark of Adobe. This personal project is not affiliated with or endorsed by Adobe in any way; the name is used only for Tony's private use.

## What Tony changed

- **Name & identity**: the app appears as **PsT** in Spotlight, Finder and the Dock, with its own Adobe-style icon; inside the app it is called "Photoshop by Tonynehihi", including the About screen.
- **Built-in self-update**: *Help → Check for Updates* — the app checks the Releases of this repo for the newest version, shows the changelog in the app, then downloads, builds and installs the update on macOS by itself, with live progress. It also checks once at launch and only prompts when a newer version exists.
- **A cleaned-up Help menu**: the upstream Discord, website and GitHub links are gone — Help keeps only About, Check for Updates, System Info and Keyboard Shortcuts.
- **Settings and presets are untouched**: the existing settings folder and portable data keep their original names, so nothing is lost when switching to this build.

## Installing on macOS

The Mac build is a source build, compiled with Cargo on Tony's own machine. In short:

```bash
cargo build --release -p photocraft
packaging/macos/package.sh --arch aarch64
```

The DMG lands in `dist/release/`. Full instructions (clean install, removing the old build, Gatekeeper troubleshooting) are in `packaging/macos/HUONG-DAN.md` (in Vietnamese).

Version **1.3.0** is the current release. Already on an earlier release? It arrives by itself through **Check for Updates** — every release since 1.1.0 updates in-app.

## Tools

<h4>🧰 67 tools</h4>
Move · Artboard · Rectangular and Elliptical Marquee · Single Row Marquee · Single Column Marquee · Lasso · Polygonal Lasso · Magnetic Lasso · Object Selection · Quick Selection · Magic Wand · Crop · Perspective Crop · Slice · Slice Select · Frame · Eyedropper · Color Sampler · Ruler · Note · Count · Remove · Spot Healing Brush · Healing Brush · Patch · Content-Aware Move · Red Eye · Brush · Pencil · Mixer Brush · Color Replacement · Clone Stamp · Pattern Stamp · History Brush · Eraser · Background Eraser · Magic Eraser · Gradient · Paint Bucket · Blur · Sharpen · Smudge · Dodge · Burn · Sponge · Pen · Freeform Pen · Curvature Pen · Add Anchor Point · Delete Anchor Point · Convert Point · Horizontal and Vertical Type · Horizontal and Vertical Type Mask · Path Selection · Direct Selection · Rectangle · Ellipse · Triangle · Polygon · Line · Custom Shape · Hand · Rotate View · Zoom

## License

Kept from the original project: **MIT OR Apache-2.0**. Tony's modifications in this repo are offered under the same terms, with the attribution above.
