# Nexa Blend Camera

[日本語](#日本語) | [English](#english)

## 日本語

Nexa Blend Cameraは、MikuMikuDance（MMD）の全バイナリのリバースエンジニアリングと、その成果に基づく改良を目指す「Nexa」計画から生まれた独立成果物です。

Blenderの`.blend`からカメラVMDを生成する、Windows x64向けMMDプラグインです。

**v1.0.0はローカル配布準備版です（未公開）。** Blender自動検出、日英表示、変換後の自動読み込みに対応します。自作データで投影と既存データの保持を確認しています。

### 導入・使用（ビルド不要）

配布ファイルは`Nexa-Blend-Camera-1.0.0-x64.zip`です。公開済みの[Releases](https://github.com/Altena-Works/Nexa-Blend-Camera/releases)は旧版のため、本版の公開までは準備済みZIPを使用してください。`Source code`には実行ファイルがありません。

別途[MMD 9.32 x64](https://sites.google.com/view/vpvp/)・[MMDPlugin v0.41 x64](https://bowlroll.net/file/121761)・[Blender](https://www.blender.org/download/)が必要です。Blender 4.5.4で検証しています。

1. MMDを終了し、ZIP内の`NexaCamera`フォルダーをMMDの`Plugin`内へ配置します。更新時は既存版を別の場所へ保存し、DLLとexeを両方更新します。
2. 作業を保存し、MMDをカメラ・照明・アクセサリ操作に切り替えます。先頭から配置する場合は現在フレームを0にします。標準読み込みは現在フレームを開始位置にします。
3. 「Nexa Blend Camera → blendを取り込み」で`.blend`を選択します。
4. Blender実行ファイル・出力VMD・出力寸法・単位倍率を指定し、「変換」を押します。MMD側の出力寸法も同じ値にします。
5. 生成VMDが読み込まれない場合は、MMDの標準モーション読み込みから保存先のVMDを開きます。

入力blendは保存変更せず、埋込みスクリプトの自動実行を無効にします。既存出力は、上書きを明示した場合だけ置き換えます。

### 制限

- カメラのみを30fpsで出力します。モデル・モデルアニメーション・ライト・物理・エフェクトの取り込み、ライブ同期には対応していません。
- 非整数視野角は既定で拒否し、許可した場合だけ近似します。レンズシフト・非剛体カメラ・パノラマ・複雑なPythonドライバーは未対応です。クリップ面やサブフレームの完全なカットは再現しません。
- 未署名のWindows x64版です。最低対応Windowsバージョンは未確定です。

### 確認範囲と開発

インストール済みBlenderの実行ファイルを自動検出します。複数候補の選択と手動指定に対応し、Blender 4.5.4で設定画面への初期入力とGUI変換を実機確認しています。旧v0.2.0のZIPには含まれません。

MMDの日本語・英語設定に合わせてメニュー、設定画面、結果とエラーを切り替えます。英語では「Nexa Blend Camera → Import .blend...」から選択し、「Convert」で変換します。単独起動は `nexa-mmd-camera.exe --gui --language ja` または `--language en` を指定できます。Windows標準ファイル選択の共通部分はOSの表示言語に従います。

GUIから起動したBlender処理には600秒の制限を設けています。日英GUIからの変換と121キーの自動読込みを実機確認しました。日本語版では既存のモデル姿勢・モーション・照明と、範囲外カメラキーの保持も指定フレームで確認しています。自作データの投影比較は12点すべて0.5px以内（最大0.077px）で、実変換中の取消ではVMDが作成されないことを確認しました。タイムアウトによる子プロセス終了は短い試験時間で確認し、実機で600秒を経過させる試験は行っていません。

Rust 1.95.0・MSVC x64環境で`.\scripts\build.ps1 -Test`を実行するとビルドできます。[検証状況](VERIFICATION.md)と[変更履歴](CHANGELOG.md)も参照してください。

### ライセンス

自作部分は[Apache-2.0](LICENSE)です。[第三者の通知](THIRD_PARTY_NOTICES.md)を併せて確認してください。

### 謝辞

MikuMikuDance（MMD）の作者である樋口優氏をはじめ、MMDPlugin、MMEffect、Blender、Bullet、Saba、GLMの開発者・貢献者、モデル・モーション・エフェクト・資料の制作者、MMDコミュニティの皆様に感謝します。

## English

Nexa Blend Camera is an independent outcome of **Nexa**, a project that aims to reverse-engineer all MikuMikuDance (MMD) binaries and make improvements based on those results.

It is a Windows x64 MMD plugin that converts cameras from Blender `.blend` files into camera VMD files.

**v1.0.0 is prepared locally and is not published yet.** It supports Blender detection, Japanese/English interfaces and automatic camera loading. Projection and preservation have been verified with self-authored fixtures.

### Installation and use — no build required

The package is `Nexa-Blend-Camera-1.0.0-x64.zip`. Published [releases](https://github.com/Altena-Works/Nexa-Blend-Camera/releases) are older versions; use the prepared ZIP until this version is published. Automatically generated `Source code` archives do not contain executables.

Install [MMD 9.32 x64](https://sites.google.com/view/vpvp/), [MMDPlugin v0.41 x64](https://bowlroll.net/file/121761) and [Blender](https://www.blender.org/download/) separately. Blender 4.5.4 is the tested version.

1. Close MMD and place the ZIP's `NexaCamera` folder inside MMD's `Plugin` folder. When updating, preserve the previous version elsewhere and replace both the DLL and the exe.
2. Save your work and switch MMD to camera/light/accessory mode. Set the current frame to 0 to import from the beginning; the standard loader uses the current frame as the insertion point.
3. Select **Nexa Blend Camera → Import .blend...** and choose a `.blend` file.
4. Specify the Blender executable, output VMD, output dimensions and unit scale, then click **Convert**. Set the same output dimensions in MMD.
5. If the generated VMD is not loaded, open the saved VMD through MMD's standard motion loader.

The input blend file is not saved or modified, and automatic execution of embedded scripts is disabled. Existing output files are replaced only when overwrite is explicitly enabled.

### Limitations

- Only cameras are exported, at 30 fps. Model, model-animation, light, physics and effect import, and live synchronization are not supported.
- Non-integer fields of view are rejected by default and approximated only when allowed. Lens shift, non-rigid camera transforms, panoramas and complex Python drivers are unsupported. Clipping planes and exact subframe camera cuts are not reproduced.
- The Windows x64 binaries are unsigned. The minimum supported Windows version has not been established.

### Verification and development

Installed Blender executables are detected automatically, with multiple-candidate selection and manual entry available. Initial path entry and GUI conversion have been verified with Blender 4.5.4 in MMD. The old v0.2.0 ZIP does not include detection.

Plugin menus, settings, results and errors follow MMD's Japanese or English setting. Standalone GUI launch accepts `nexa-mmd-camera.exe --gui --language en` or `--language ja`. Common parts of the Windows file picker follow the OS display language.

The GUI has a 600-second limit for its Blender child process. Conversion and automatic loading of 121 camera keys have been verified through both Japanese and English GUIs. In Japanese MMD, existing model poses, motion, lighting and camera keys outside the imported range remained at the tested frames. All twelve markers in the self-authored projection fixture are within 0.5 pixels (maximum 0.077 pixels). Cancelling a real conversion produced no VMD. Child termination on timeout was tested using a short deadline; a native 600-second timeout run was not performed.

To build, use Rust 1.95.0 and the MSVC x64 build tools and run `.\scripts\build.ps1 -Test`. See [verification status](VERIFICATION.md) and the [changelog](CHANGELOG.md).

### License

Original code is licensed under [Apache-2.0](LICENSE). See the [third-party notices](THIRD_PARTY_NOTICES.md) for other components.

### Acknowledgements

We thank MikuMikuDance (MMD) creator 樋口優, the developers and contributors of MMDPlugin, MMEffect, Blender, Bullet, Saba and GLM, the creators of models, motions, effects and reference materials, and the MMD community.
