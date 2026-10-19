v0.1.0: Nexaから独立した、ビルド済みWindows x64向けのカメラ取込み検証版。

## ダウンロード（ビルド不要）

| ファイル | 内容 |
|---|---|
| `Nexa-Camera-0.1.0-x64.zip` | Windows x64 plugin and worker; unsigned |
| `SHA256SUMS.txt` | SHA-256 checksum for the ZIP |

ZIPを展開し、`NexaCamera`フォルダーをMMDの`Plugin`へ配置してください。Rust・MSVCによるビルドは不要です。MMD x64、MMDPlugin v0.41 x64、Blenderは別途必要です。`Source code`の自動生成ZIPには実行ファイルが入っていません。

ZIP SHA-256: `6dcb63a72534fbd28fb1a71e43c696f489a80d15e251a0e68245d2eea0a53c28`

## 追加 / Added

- Nexaからカメラ変換とMMDアダプターを分離。
- 「Nexa Camera → blendを取り込み」から`.blend`専用の標準ファイル選択を表示。
- カメラVMD生成用の設定画面とCLI。

## 変更 / Changed

- Nexaと同じApache-2.0を適用し、共通の謝辞を掲載。
- GitHub Releasesからビルド済みZIPを入手する手順を追加。

## 修正 / Fixed

- MMDの標準メニュー通知を直接受け、workerを起動する経路を追加。
- 終了時の通知接続解除と、既存出力の保護。

## 確認範囲と既知の制限

MMDメニューから`.blend`の標準ファイル選択が開くことは利用者が確認済みです。Rustの独立構成では16テストとreleaseビルドが合格し、Blenderを起動する2テストは未実施です。ZIPはメニュー操作を確認した試験版と同じDLL・workerを収録しています。

現行GUIからの変換完了は未確認です。平行投影の倍率差と透視投影の描画位置差が残っており、Blenderと実MMDの投影一致は未解決です。モデル・モデルアニメーション・ライト・物理・エフェクトの取込みは含みません。カメラVMDの読込みはMMDの標準操作で行います。

## ライセンスと謝辞

Nexaと同じApache-2.0を自作部分に適用し、第三者の通知を保持しています。MMDの作者である樋口優氏と関連開発者・クリエイターへの共通の謝辞は、同梱READMEとACKNOWLEDGEMENTS.mdの末尾に掲載しています。
