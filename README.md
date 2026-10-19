# Nexa Camera v0.1.0

Blenderの`.blend`から評価済みカメラを取り出し、MikuMikuDance用のカメラVMDを生成するWindows x64向けプラグインです。Nexa本体やsolverに依存しません。

**v0.1.0は検証版です。** MMDのメニューからファイル選択が開くことは確認済みですが、現行GUI経由の変換完了と、実MMDでの投影一致は未確認です。平行投影には既知の変換不一致があります。

## 導入と使用

MMD 9.32 x64、MMDPlugin v0.41 x64、Blenderが必要です。Blender 4.5.4を検証に使用しています。MMDPluginは配布元の手順で専用のMMD環境へ導入してください。これらのソフトウェアは同梱しません。

1. `NexaCamera`フォルダーをMMDの`Plugin`内へ置きます。DLLと`nexa-mmd-camera.exe`は同じフォルダーに置いてください。
2. MMDで「Nexa Camera → blendを取り込み」を選び、Windows標準のファイル選択で`.blend`を指定します。
3. 設定画面でBlender実行ファイル、出力VMD、出力寸法、単位倍率を指定し、「変換」を押します。シーン名・カメラ名の空欄は現在のシーンと各時刻のアクティブカメラを使います。
4. 変換に成功したら、MMDのカメラ・照明モードで標準のモーション読込みからVMDを読み込みます。MMDの出力寸法も変換時とそろえます。

既存出力は、設定で上書きを明示した場合だけ置き換えます。入力blendは保存変更せず、埋込みスクリプトの自動実行を無効にして評価します。

## 対応範囲と制限

- カメラの位置・向き・ロール・整数視野角を30fpsで出力します。親・制約とアクティブカメラの切替を評価します。
- 非整数視野角は既定で拒否します。近似を明示的に許可した場合は、計算上の誤差を結果に表示します。これは実MMDでの描画誤差の保証ではありません。
- 平行投影の変換不一致と、透視投影の描画位置差が残っています。Blenderと同じ画を得る用途では検証が必要です。
- レンズシフト、非剛体カメラ、パノラマ、複雑なPythonドライバーは未対応です。クリップ面やサブフレームの完全なカットはVMDで再現しません。
- モデル・モデルアニメーション・ライト・物理・エフェクトは取り込みません。自動VMD読込みやライブ同期も含みません。
- 未署名のWindows x64版です。最低対応Windowsバージョンは未確定です。

## ビルド

Rust 1.95.0とMSVC x64のビルド環境が必要です。外部Rustクレートへの依存はありません。

```powershell
.\scripts\build.ps1 -Test
```

生成先は`target/x86_64-pc-windows-msvc/release/`です。プラグインには`nexa_mmd_camera_plugin.dll`と`nexa-mmd-camera.exe`を使います。worker単体でファイル選択を開く場合は`nexa-mmd-camera.exe --gui`を実行します。

Blenderを必要とする2つのAPI統合テストは通常のテスト実行では除外します。実行する場合は`target/blender-tests`を作成し、`NEXA_TEST_BLENDER`に実行ファイル、`NEXA_TEST_DIR`にその直下の新規試験フォルダーを指定して、`cargo test -p nexa-blend-camera -- --ignored --test-threads=1`を使います。

確認範囲は[VERIFICATION.md](VERIFICATION.md)、変更点は[CHANGELOG.md](CHANGELOG.md)を参照してください。プロジェクトのライセンスは未選択です。第三者の通知は[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)に保持しています。
