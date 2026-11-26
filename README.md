# 臺灣儀表板Discord機器人

![Rust](https://img.shields.io/badge/Rust-1.99.1-000000.svg?logo=rust&logoColor=white)
![Discord](https://img.shields.io/badge/Discord-7289DA.svg?logo=discord&logoColor=white)

由Rust所編寫的Discord機器人，依照[SampleCode](https://github.com/dashboard-tw/SampleCode)進行使用者的互動開發。

設定與資料請至[dashboard.tw](https://dashboard.tw)進行取得。

## 基本設置

複製`config.example.yml`改為`config.yml`填入以下設定。程式會讀取目前工作目錄下的 `config.yml`：

```yaml
bot:
  token: "yourbottoken"           # 機器人的 TOKEN
  prefix: "!"                     # 觸發前綴省略時預設為 !
  client_config:
    id: "yourclientid"            # dashboard.tw 的 client id
    secret: "yourclientsecret"    # dashboard.tw 的 client secret
  admins: []                      # 可操作者的ID，例如 ["123456789012345678"]
  webhook_url: ""                 # 留空或省略則不啟用通知

# 選填(功能資料尚未完成)
features:
  alert:                          # 功能名稱
    enable: true                  # 是否啟用功能
    channels: []                  # 通知頻道的ID，例如 ["123456789012345678"]
```

## 意圖

- [x] 訊息意圖
- [ ] 成員意圖
- [ ] 狀態意圖

## 自動更新

查看自動[更新文檔](/generators/README.md)快速為指令與Discord Embed快速更新。

## TODO

- [x] 基本交互
  - [x] 機器人啟動
  - [x] YAML 設定檔
  - [x] 延遲測試
  - [x] 錯誤處理
  - [ ] 日誌紀錄
  - [ ] 更多...

- [ ] API交互
  - [x] 檢查健康狀態
  - [x] 自動刷新token
  - [x] 強制刷新token
  - [x] 請求資料
  - [ ] MQTT訂閱(暫無)
  - [ ] 更多...

- [ ] 資料處理
  - [x] 快取管理
  - [x] 資料格式
  - [ ] 資料訂閱(暫無)

- [x] 自動生成
  - [x] 資料格式
  - [x] 指令

- [x] 開發者功能
  - [x] 強制刷新token

- [ ] 使用者體驗優化
  - [x] 斜線指令
  - [x] 嵌入訊息
  - [x] 圖片顯示
  - [ ] 資料繪圖
  - [ ] 自動通知
  - [ ] 更多...

## 授權

範例程式以 [MIT License](/LICENSE) 授權，可自由複製、修改並用在你的專案中。API 提供的資料依各來源機關的授權條款使用，請見 dashboard.tw 的資料來源說明。