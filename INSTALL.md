# Installation / 導入

## 日本語

Windows x64向け、ビルド不要です。MMD 9.32 x64、MMDPlugin v0.41 x64、Blenderを別途用意してください。Blender 4.5.4で検証しています。配布バイナリは未署名で、最低対応Windowsバージョンは未確定です。

1. ZIPと`SHA256SUMS.txt`を同じ場所へ保存します。PowerShellで`Get-FileHash .\Nexa-Blend-Camera-1.0.0-x64.zip -Algorithm SHA256`を実行し、値が一致することを確認します。
2. MMDを終了します。更新の場合は既存の`Plugin\NexaCamera`を別の場所へ保存します。
3. ZIPを展開し、`NexaCamera`フォルダーをMMDの`Plugin`へ配置します。
4. `Plugin\NexaCamera`直下に`nexa_mmd_camera_plugin.dll`と`nexa-mmd-camera.exe`の両方があることを確認してMMDを起動します。
5. 作業を保存し、カメラ操作へ切り替えて「Nexa Blend Camera → blendを取り込み」を選びます。先頭から配置する場合は現在フレームを0にします。

### 読み込めないとき

- **メニューがない：** MMD・MMDPluginがx64版であること、フォルダーが二重になっていないこと、DLLとexeを一緒に配置したことを確認します。
- **Blenderが検出されない：** 「参照...」からインストール済みの`blender.exe`を指定します。ポータブル版・独自配置は自動検出されない場合があります。
- **出力が存在する：** 別名を指定するか、置き換えてよい場合だけ「既存の出力ファイルを上書き」を有効にします。
- **変換後に読み込まれない：** MMDの別ダイアログを閉じ、カメラ・照明・アクセサリ操作へ切り替えます。保存されたVMDは標準のモーション読み込みから開けます。
- **構図や大きさが合わない：** 変換時とMMD側の出力寸法を揃え、単位倍率を確認します。未対応のカメラ設定は[README](README.md)を参照してください。

## English

This Windows x64 package needs no build. Install MMD 9.32 x64, MMDPlugin v0.41 x64 and Blender separately. Blender 4.5.4 was tested. The binaries are unsigned; the minimum Windows version is not established.

1. Save the ZIP and `SHA256SUMS.txt` together. Run `Get-FileHash .\Nexa-Blend-Camera-1.0.0-x64.zip -Algorithm SHA256` in PowerShell and compare the result.
2. Close MMD. For an update, preserve the existing `Plugin\NexaCamera` folder elsewhere.
3. Extract the ZIP and place `NexaCamera` inside MMD's `Plugin` folder.
4. Confirm that both `nexa_mmd_camera_plugin.dll` and `nexa-mmd-camera.exe` are directly inside `Plugin\NexaCamera`, then start MMD.
5. Save your work, switch to camera mode and select **Nexa Blend Camera → Import .blend...**. Set the current frame to 0 to import from the beginning.

### Troubleshooting

- **No menu:** Check the x64 versions of MMD/MMDPlugin, the folder nesting and the presence of both the DLL and exe.
- **Blender not detected:** Use **Browse...** to select your installed `blender.exe`. Portable or custom installations may need manual selection.
- **Output exists:** Choose another filename, or enable **Overwrite the existing output file** only if replacement is intended.
- **Converted VMD not loaded:** Close other MMD dialogs and switch to camera/light/accessory mode. The saved VMD can also be opened through the standard motion loader.
- **Framing or scale differs:** Match the converter and MMD output dimensions, and check the unit scale. See [README](README.md) for unsupported camera settings.
