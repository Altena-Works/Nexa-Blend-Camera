# Nexa Camera

[日本語](#日本語) | [English](#english)

## 日本語

Nexa Cameraは、MikuMikuDance（MMD）の全バイナリのリバースエンジニアリングと、その成果に基づく改良を目指す「Nexa」計画から生まれた独立成果物です。

Blenderの`.blend`からカメラVMDを生成する、Windows x64向けMMDプラグインです。

**v0.2.0は検証版です。** 変換後にMMDへVMDを渡す処理を含みますが、自動読み込みと既存キーの保持は実MMDで未確認です。平行投影の倍率差と透視投影の位置差も残っています。

### 導入・使用（ビルド不要）

[Release v0.2.0](https://github.com/Altena-Works/Nexa-Camera/releases/tag/v0.2.0)から`Nexa-Camera-0.2.0-x64.zip`を入手します。`Source code`には実行ファイルがありません。

別途[MMD 9.32 x64](https://sites.google.com/view/vpvp/)・[MMDPlugin v0.41 x64](https://bowlroll.net/file/121761)・[Blender](https://www.blender.org/download/)が必要です。Blender 4.5.4で検証しています。

1. MMDを終了し、ZIP内の`NexaCamera`フォルダーをMMDの`Plugin`内へ配置します。更新時は既存版を別の場所へ保存し、DLLとexeを両方更新します。
2. 作業を保存し、MMDをカメラ・照明・アクセサリ操作に切り替えます。
3. 「Nexa Camera → blendを取り込み」で`.blend`を選択します。
4. Blender実行ファイル・出力VMD・出力寸法・単位倍率を指定し、「変換」を押します。MMD側の出力寸法も同じ値にします。
5. 生成VMDが読み込まれない場合は、MMDの標準モーション読み込みから保存先のVMDを開きます。

入力blendは保存変更せず、埋込みスクリプトの自動実行を無効にします。既存出力は、上書きを明示した場合だけ置き換えます。

### 制限

- カメラのみを30fpsで出力します。モデル・モデルアニメーション・ライト・物理・エフェクトの取り込み、ライブ同期には対応していません。
- 非整数視野角は既定で拒否し、許可した場合だけ近似します。レンズシフト・非剛体カメラ・パノラマ・複雑なPythonドライバーは未対応です。クリップ面やサブフレームの完全なカットは再現しません。
- 未署名のWindows x64版です。最低対応Windowsバージョンは未確定です。

### 開発版

ソース版ではインストール済みBlenderの実行ファイルを自動検出します。複数候補の選択と手動指定に対応し、Blender 4.5.4で設定画面への初期入力とGUI変換を実機確認しています。公開済みv0.2.0のZIPには含まれず、手動指定が必要です。

Rust 1.95.0・MSVC x64環境で`.\scripts\build.ps1 -Test`を実行するとビルドできます。[検証状況](VERIFICATION.md)と[変更履歴](CHANGELOG.md)も参照してください。

### ライセンス

自作部分は[Apache-2.0](LICENSE)です。[第三者の通知](THIRD_PARTY_NOTICES.md)を併せて確認してください。

### 謝辞

MikuMikuDance（MMD）の作者である樋口優氏をはじめ、MMDPlugin、MMEffect、Blender、Bullet、Saba、GLMの開発者・貢献者、モデル・モーション・エフェクト・資料の制作者、MMDコミュニティの皆様に感謝します。

## English

Nexa Camera is an independent outcome of **Nexa**, a project that aims to reverse-engineer all MikuMikuDance (MMD) binaries and make improvements based on those results.

It is a Windows x64 MMD plugin that converts cameras from Blender `.blend` files into camera VMD files.

**v0.2.0 is a verification build.** It includes a path for delivering the converted VMD to MMD, but automatic loading and preservation of existing keys have not been verified in MMD. Orthographic scale and perspective position discrepancies also remain.

### Installation and use — no build required

Download `Nexa-Camera-0.2.0-x64.zip` from [Release v0.2.0](https://github.com/Altena-Works/Nexa-Camera/releases/tag/v0.2.0). The automatically generated `Source code` archives do not contain executables.

Install [MMD 9.32 x64](https://sites.google.com/view/vpvp/), [MMDPlugin v0.41 x64](https://bowlroll.net/file/121761) and [Blender](https://www.blender.org/download/) separately. Blender 4.5.4 is the tested version.

1. Close MMD and place the ZIP's `NexaCamera` folder inside MMD's `Plugin` folder. When updating, preserve the previous version elsewhere and replace both the DLL and the exe.
2. Save your work and switch MMD to camera/light/accessory mode.
3. Select **Nexa Camera → blendを取り込み** and choose a `.blend` file.
4. Specify the Blender executable, output VMD, output dimensions and unit scale, then click **変換**. Set the same output dimensions in MMD.
5. If the generated VMD is not loaded, open the saved VMD through MMD's standard motion loader.

The input blend file is not saved or modified, and automatic execution of embedded scripts is disabled. Existing output files are replaced only when overwrite is explicitly enabled.

### Limitations

- Only cameras are exported, at 30 fps. Model, model-animation, light, physics and effect import, and live synchronization are not supported.
- Non-integer fields of view are rejected by default and approximated only when allowed. Lens shift, non-rigid camera transforms, panoramas and complex Python drivers are unsupported. Clipping planes and exact subframe camera cuts are not reproduced.
- The Windows x64 binaries are unsigned. The minimum supported Windows version has not been established.

### Development

The source version automatically detects installed Blender executables, supports selecting among multiple candidates, and retains manual selection. Initial path entry and GUI conversion have been verified with Blender 4.5.4 in MMD. The published v0.2.0 ZIP does not include detection and requires manual selection.

To build, use Rust 1.95.0 and the MSVC x64 build tools and run `.\scripts\build.ps1 -Test`. See [verification status](VERIFICATION.md) and the [changelog](CHANGELOG.md).

### License

Original code is licensed under [Apache-2.0](LICENSE). See the [third-party notices](THIRD_PARTY_NOTICES.md) for other components.

### Acknowledgements

We thank MikuMikuDance (MMD) creator 樋口優, the developers and contributors of MMDPlugin, MMEffect, Blender, Bullet, Saba and GLM, the creators of models, motions, effects and reference materials, and the MMD community.
