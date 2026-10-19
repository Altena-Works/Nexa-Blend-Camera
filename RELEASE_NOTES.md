v0.2.0: 変換したカメラVMDを起動元MMDへ渡す処理を追加した、Windows x64検証版です。

## ダウンロード（ビルド不要）

| ファイル | 内容 |
|---|---|
| `Nexa-Camera-0.2.0-x64.zip` | Windows x64 plugin and worker; unsigned |
| `SHA256SUMS.txt` | SHA-256 checksum for the ZIP |

ZIPを展開し、`NexaCamera`をMMDの`Plugin`へ配置します。MMDを終了してからDLLとexeを両方更新してください。MMD 9.32 x64・MMDPlugin v0.41 x64・Blenderは別途必要です。同梱`INSTALL.md`を参照してください。GitHubの`Source code`には実行ファイルがありません。

ZIP SHA-256: `d0d809d1663c5f3b5aa60bac4f0d7f4acf0761cad19f7a262829b54487421212`

## 追加

- 変換したカメラVMDを起動元MMDの標準ファイル読み込み経路へ渡す処理。
- カメラ・照明操作モードとcamera-only形式の確認、完了通知の重複・取消・終了時の処理。

## 確認範囲と制限

Rustテスト19件が合格し、修正後の対象12件も合格しています。Blenderを必要とする既存2テストは未実行です。本ZIPはmainのd21b256からRust 1.95.0で新規ビルドしたv0.2.0で、PEのx64形式、ZIP内全ファイルの一致、workerのヘルプ起動を確認しました。

**実MMDでの自動読み込みと既存キー・モデル・ライトの保持は確認待ちです。** 自動配送の通知は、MMDが読み込みに成功した証明ではありません。MMDをカメラ・照明・アクセサリ操作にして使用し、既存の作業は先に保存してください。読み込まれない場合は保存済みVMDをMMDの標準モーション読み込みで開けます。

平行投影の倍率差と透視投影の位置差は未解決です。モデル・モデルアニメーション・ライトは取り込みません。未署名のWindows x64版で、最低対応Windowsバージョンは未確定です。

## ライセンス

自作部分はApache-2.0です。第三者通知を同梱しています。

## 謝辞

MikuMikuDance（MMD）の作者である樋口優氏をはじめ、MMDPlugin、MMEffect、Blender、Bullet、Saba、GLMの開発者・貢献者、モデル・モーション・エフェクト・資料の制作者、MMDコミュニティの皆様に感謝します。
