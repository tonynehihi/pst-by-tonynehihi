# Photoshop by Tonynehihi — Hướng dẫn cài đặt & sử dụng

Bản build cá nhân của Tony, dựa trên PhotoCraft v0.8.2 (MIT/Apache-2.0) — bản clone Photoshop bằng Rust, chạy offline, không cần Adobe.

Đã rebrand:
- Tên app **"Photoshop by Tonynehihi"**, icon **PsT** phong cách Adobe (nền xanh dương)
- Menu bar macOS: About / Hide / Quit đều hiện "Photoshop by Tonynehihi"
- About dialog gọn: "Photoshop by Tonynehihi — by Tonynehihi" + version
- Đã gỡ hết link Discord / website / GitHub của upstream khỏi menu Help

## Có gì mới ở bản 1.1.0

- **Tên Spotlight rút gọn còn "PsT"**: trong Finder/Spotlight/Dock app tên là **PsT** (giống LrT); trong app vẫn ghi đầy đủ "Photoshop by Tonynehihi". Settings giữ nguyên, không mất gì.
- **Tự cập nhật như Lightroom by Tony**: mở app là tự kiểm tra bản mới trên GitHub (repo `tonynehihi/pst-by-tonynehihi`), có bản mới thì hiện pop-up kèm changelog; bấm Download → app tự build bản mới ngay trong app (có tiến trình) → Finish Install là xong. Kiểm tra tay: Help → Check for Updates, hoặc trong About.
- **Icon PsT vẽ lại theo đúng nhận diện Adobe** (nền #001E36, chữ #31A8FF).

**Lưu ý: bản 1.1.0 phải cài TAY một lần** (app cũ chưa có tính năng tự cập nhật): build như hướng dẫn dưới đây, xóa "Photoshop by Tony.app" cũ trong Applications, kéo **PsT.app** vào. Từ 1.1.1 trở đi app tự cập nhật.

## Gỡ bản cũ / cài lại sạch

**Nếu mày đã build bản trước:**
```bash
# Xóa bản build cũ
rm -rf target/
# Xóa app đã cài (nếu đã kéo vào Applications)
rm -rf "/Applications/Photoshop by Tony.app"
rm -rf "/Applications/PsT.app"
rm -rf "/Applications/PhotoCraft.app"
```

**Xóa sạch settings/cache (nếu muốn làm lại từ đầu):**
```bash
# CẨN THẬN: xóa là mất settings, sao lưu trước nếu cần
rm -rf ~/Library/Application\ Support/photocraft
rm -rf ~/.config/photocraft
```

## Cài đặt (làm 1 lần)

**Bước 1** — Cài Xcode Command Line Tools (nếu chưa có):
```
xcode-select --install
```

**Bước 2** — Cài Rust (nếu chưa có):
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Xong thì **đóng Terminal, mở lại**.

## Build & chạy

**Bước 3** — Giải nén `pst-by-tonynehihi-1.1.0.zip`.

**Bước 4** — Trong Terminal:
```bash
cd <kéo folder pst-by-tonynehihi vào Terminal rồi thả>
cargo build --release -p photocraft
```
(Lần đầu 15–25 phút.)

**Bước 5** — Chạy thử:
```bash
./target/release/photocraft
```

**Bước 6** — Đóng gói thành app (không cần Terminal nữa):
```bash
packaging/macos/package.sh --arch aarch64
```
(Máy Intel cũ: `--arch x86_64`.)
Xong mở file `.dmg` trong `dist/release/`, kéo **PsT.app** vào Applications.

## Verify source trước khi build (khỏi build nhầm bản cũ)

```bash
grep -r "Photoshop by Tonynehihi" crates/ui-egui/src/native_menu.rs | head -2
```
Phải thấy dòng `pub const APP_NAME: &str = "Photoshop by Tonynehihi";` — nếu không thấy thì đang ở nhầm folder.

## Gatekeeper (nếu macOS chặn mở app)

Nếu mở app lần đầu bị báo "cannot be verified" / "damaged":
1. Chuột phải vào app → **Open** → bấm **Open** trong hộp thoại.
2. Hoặc vào System Settings → Privacy & Security → bấm **Open Anyway**.

## Tính năng chính

- Layers, masks, adjustment layers, layer styles, type, vectors, brushes
- Mở/lưu file **PSD** thật (tương thích Photoshop)
- Format riêng `.pcraft`, hỗ trợ PNG, JPG, TIFF, WebP, OpenRaster, Krita, v.v.
- Chạy offline hoàn toàn, không cần tài khoản Adobe.
