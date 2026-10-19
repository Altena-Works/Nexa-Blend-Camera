# Nexa Camera v0.2.0

Nexa Cameraは、MikuMikuDance（MMD）の全バイナリのリバースエンジニアリングと、その成果に基づく改良を目指す「Nexa」計画から生まれた独立成果物です。

Blenderの`.blend`からカメラVMDを生成する、Windows x64向けMMDプラグインです。

**v0.1.0は検証版です。** MMDメニューからファイル選択が開くことは確認済みです。現行GUIでの変換完了は未確認で、平行投影の倍率差・透視投影の位置差が残っています。

v0.2.0のソースには、変換成功後に起動元MMDへカメラVMDを渡す処理を追加しています。MMDをカメラ・照明・アクセサリ操作にして使用します。Rustテストとビルドは合格していますが、実MMDでの自動読み込み・既存キー保持は確認待ちです。公開済みv0.1.0には含まれません。

## 公開済みv0.1.0の導入・使用（ビルド不要）

[Releases](https://github.com/Altena-Works/Nexa-Camera/releases)のAssetsから`Nexa-Camera-0.1.0-x64.zip`を入手します。`Source code`には実行ファイルがありません。

別途[MMD 9.32 x64](https://sites.google.com/view/vpvp/)・[MMDPlugin v0.41 x64](https://bowlroll.net/file/121761)・[Blender](https://www.blender.org/download/)が必要です。Blender 4.5.4で検証しています。

1. ZIPを展開し、`NexaCamera`フォルダーをMMDの`Plugin`内へ配置します。DLLとexeは同じフォルダーに置きます。
2. 「Nexa Camera → blendを取り込み」で`.blend`を選択します。
3. Blender実行ファイル・出力VMD・出力寸法・単位倍率を指定し、「変換」を押します。
4. MMDをカメラ・照明モードにして、生成VMDを標準のモーション読込みで開きます。出力寸法は変換時とそろえます。

入力blendは保存変更せず、埋込みスクリプトの自動実行を無効にします。既存出力は、上書きを明示した場合だけ置き換えます。

## 制限

- カメラのみを30fpsで出力します。モデル・ライト・物理・エフェクト、ライブ同期は対象外です。公開済みv0.1.0は自動VMD読込みを含みません。
- 非整数視野角は既定で拒否し、明示的に許可した場合だけ近似します。レンズシフト・非剛体カメラ・パノラマ・複雑なPythonドライバーは未対応です。クリップ面やサブフレームの完全なカットは再現しません。
- 未署名のWindows x64版です。最低対応Windowsバージョンは未確定です。

確認範囲は[VERIFICATION.md](VERIFICATION.md)、変更点は[CHANGELOG.md](CHANGELOG.md)を参照してください。開発用ビルドはRust 1.95.0・MSVC x64環境で`.\scripts\build.ps1 -Test`を実行します。

## ライセンス

Nexaと同じ[Apache-2.0](LICENSE)を自作部分に適用します。[第三者の通知](THIRD_PARTY_NOTICES.md)は別途保持しています。

## 謝辞

MikuMikuDance（MMD）を制作・公開された樋口優氏に、深く感謝します。また、MMDPlugin、MMEffect、Blender、Bullet、Saba、GLMをはじめとする関連ソフトウェア・ライブラリの開発者と貢献者、モデル・モーション・エフェクト・資料を制作・公開してきた皆様、そしてMMDコミュニティを支えてきたすべての皆様に感謝します。Nexaとその関連成果物は、皆様が築いてきた技術と創作文化に支えられています。
