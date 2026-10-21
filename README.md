# Nexa Blend Camera

[日本語](#日本語) | [English](#english)

## 日本語

Nexa Blend Cameraは、MikuMikuDance（MMD）の全バイナリのリバースエンジニアリングと、その成果に基づく改良を目指す「Nexa」計画から生まれた独立成果物です。

Blenderの`.blend`からカメラを変換し、MMDへ読み込むWindows x64向けプラグインです。Blender自動検出、カメラ切替、透視・平行投影、日英表示に対応します。

**v1.0.0** — [ダウンロード](https://github.com/Altena-Works/Nexa-Blend-Camera/releases/tag/v1.0.0)

### 導入・使用（ビルド不要）

配布ファイルは`Nexa-Blend-Camera-1.0.0-x64.zip`です。[Releases](https://github.com/Altena-Works/Nexa-Blend-Camera/releases)ではバージョンを確認してください。`Source code`には実行ファイルがありません。チェックサム・更新・トラブル時の手順は[導入ガイド](INSTALL.md)にまとめています。

別途[MMD 9.32 x64](https://sites.google.com/view/vpvp/)・[MMDPlugin v0.41 x64](https://bowlroll.net/file/121761)・[Blender](https://www.blender.org/download/)が必要です。Blender 4.5.4で検証しています。

1. MMDを終了し、ZIP内の`NexaCamera`フォルダーをMMDの`Plugin`内へ配置します。更新時は既存版を別の場所へ保存し、DLLとexeを両方更新します。
2. 作業を保存し、MMDをカメラ・照明・アクセサリ操作に切り替えます。先頭から配置する場合は現在フレームを0にします。標準読み込みは現在フレームを開始位置にします。
3. 「Nexa Blend Camera → blendを取り込み」で`.blend`を選択します。
4. 自動検出されたBlender、出力VMD、出力寸法、単位倍率を確認し、「変換」を押します。Blenderは手動指定も可能です。MMD側の出力寸法も同じ値にします。
5. 変換後、カメラVMDを起動元のMMDへ自動で渡します。読み込まれない場合は、保存済みVMDを標準のモーション読み込みから開いてください。

入力blendは保存変更せず、埋込みスクリプトの自動実行を無効にします。既存出力は上書きを明示した場合だけ置き換えます。処理中はキャンセルでき、Blender処理の制限時間は600秒です。

### 制限

- カメラのみを30fpsで出力します。モデル・モデルアニメーション・ライト・物理・エフェクトの取り込み、ライブ同期には対応していません。
- 非整数視野角は既定で拒否し、許可した場合だけ近似します。レンズシフト・非剛体カメラ・パノラマ・複雑なPythonドライバーは未対応です。クリップ面やサブフレームの完全なカットは再現しません。
- 未署名のWindows x64版です。最低対応Windowsバージョンは未確定です。

### 確認範囲と開発

日英GUIからの自動読み込みと、指定フレームの既存モデル・モーション・照明・範囲外カメラキー保持を確認しました。自作データの投影比較は12点すべて0.5px以内です。任意のblendとの互換性を保証するものではありません。600秒経過の実機試験など、確認範囲の詳細は[検証状況](VERIFICATION.md)を参照してください。

メニュー・設定画面はMMDの言語設定に従います。英語では「Import .blend...」「Convert」を使用します。Windows標準ファイル選択の共通部分はOSの表示言語に従います。単独GUIは`nexa-mmd-camera.exe --gui --language ja`または`--language en`で起動できます。

Rust 1.95.0・MSVC x64環境で`.\scripts\build.ps1 -Test`を実行するとビルドできます。[検証状況](VERIFICATION.md)と[変更履歴](CHANGELOG.md)も参照してください。

### ライセンス

自作部分は[Apache-2.0](LICENSE)です。[第三者の通知](THIRD_PARTY_NOTICES.md)を併せて確認してください。

### 謝辞

MikuMikuDance（MMD）の作者である樋口優氏をはじめ、MMDPlugin、MMEffect、Blender、Bullet、Saba、GLMの開発者・貢献者、モデル・モーション・エフェクト・資料の制作者、MMDコミュニティの皆様に感謝します。

## English

Nexa Blend Camera is an independent outcome of **Nexa**, a project that aims to reverse-engineer all MikuMikuDance (MMD) binaries and make improvements based on those results.

This Windows x64 plugin converts cameras from Blender `.blend` files and imports them into MMD. It supports Blender detection, camera cuts, perspective and orthographic cameras, and Japanese/English interfaces.

**v1.0.0** — [Download](https://github.com/Altena-Works/Nexa-Blend-Camera/releases/tag/v1.0.0)

### Installation and use — no build required

The package is `Nexa-Blend-Camera-1.0.0-x64.zip`. Check the version on [Releases](https://github.com/Altena-Works/Nexa-Blend-Camera/releases); `Source code` archives do not contain executables. See the [installation guide](INSTALL.md) for checksums, updates and troubleshooting.

Install [MMD 9.32 x64](https://sites.google.com/view/vpvp/), [MMDPlugin v0.41 x64](https://bowlroll.net/file/121761) and [Blender](https://www.blender.org/download/) separately. Blender 4.5.4 is the tested version.

1. Close MMD and place the ZIP's `NexaCamera` folder inside MMD's `Plugin` folder. When updating, preserve the previous version elsewhere and replace both the DLL and the exe.
2. Save your work and switch MMD to camera/light/accessory mode. Set the current frame to 0 to import from the beginning; the standard loader uses the current frame as the insertion point.
3. Select **Nexa Blend Camera → Import .blend...** and choose a `.blend` file.
4. Check the detected Blender executable, output VMD, output dimensions and unit scale, then click **Convert**. Manual Blender selection is available. Use the same output dimensions in MMD.
5. The converted VMD is delivered to the originating MMD window automatically. If it is not loaded, open the saved VMD through MMD's standard motion loader.

The input blend file is not saved or modified, and automatic execution of embedded scripts is disabled. Existing output is replaced only with explicit overwrite consent. Conversion can be cancelled; the Blender process has a 600-second limit.

### Limitations

- Only cameras are exported, at 30 fps. Model, model-animation, light, physics and effect import, and live synchronization are not supported.
- Non-integer fields of view are rejected by default and approximated only when allowed. Lens shift, non-rigid camera transforms, panoramas and complex Python drivers are unsupported. Clipping planes and exact subframe camera cuts are not reproduced.
- The Windows x64 binaries are unsigned. The minimum supported Windows version has not been established.

### Verification and development

Automatic loading through both language interfaces and preservation of models, motion, lighting and out-of-range camera keys at selected frames were verified with self-authored data. All twelve projection markers were within 0.5 pixels. This does not guarantee compatibility with arbitrary blend files. See [verification](VERIFICATION.md) for evidence and limits, including the unperformed native 600-second timeout run.

Plugin interfaces follow MMD's language setting. Standalone GUI launch accepts `nexa-mmd-camera.exe --gui --language en` or `--language ja`. Common parts of the Windows file picker follow the OS display language.

To build, use Rust 1.95.0 and the MSVC x64 build tools and run `.\scripts\build.ps1 -Test`. See [verification status](VERIFICATION.md) and the [changelog](CHANGELOG.md).

### License

Original code is licensed under [Apache-2.0](LICENSE). See the [third-party notices](THIRD_PARTY_NOTICES.md) for other components.

### Acknowledgements

We thank MikuMikuDance (MMD) creator 樋口優, the developers and contributors of MMDPlugin, MMEffect, Blender, Bullet, Saba and GLM, the creators of models, motions, effects and reference materials, and the MMD community.
