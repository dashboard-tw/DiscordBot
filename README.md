# 臺灣儀表板Discord機器人

![Rust](https://img.shields.io/badge/Rust-1.99.1-000000.svg?logo=rust&logoColor=white)
![Discord](https://img.shields.io/badge/Discord-7289DA.svg?logo=discord&logoColor=white)

由Rust所編寫的Discord機器人，依照[SampleCode](https://github.com/dashboard-tw/SampleCode)進行使用者的互動開發。

設定與資料請至[dashboard.tw](https://dashboard.tw)進行取得。

## 基本設置

使用`.env.example`進行設置後將其改名為`.env`環境變數檔案：

```ini
TOKEN="yourbottoken"              // 設置機器人的TOKEN
PREFIX="yourbotprefix"            // 設置機器人的觸發前綴(預設為"!")
CLIENT_ID="yourclientid"          // 設置dashboard.tw的client id
CLIENT_SECRET="yourclientsecret"  // 設置dashboard.tw的client secret
ADMINS=[]                         // 設置管理指令如 ADMINS='["123456789012345678"]'
```

## 自動更新

查看自動[更新文檔](/generators/README.md)快速為指令與Discord Embed快速更新。

## TODO

- [x] 基本交互
  - [x] 機器人啟動
  - [x] 環境變數
  - [x] 延遲測試
  - [x] 錯誤處理

- [ ] API交互
  - [x] 檢查健康狀態
  - [x] 自動刷新token
  - [x] 強制刷新token
  - [x] 請求資料
  - [ ] MQTT訂閱
  - [ ] 更多...

- [ ] 資料處理
  - [x] 快取管理
  - [x] 資料格式
  - [ ] 資料訂閱

- [x] 自動生成
  - [x] 資料格式
  - [x] 指令

- [x] 開發者功能
  - [x] 強制刷新token
  - [x] 更多...

- [ ] 使用者體驗優化
  - [x] 斜線指令
  - [x] 嵌入訊息
  - [ ] 圖片顯示
  - [ ] 資料繪圖
  - [ ] 更多...

## 授權

範例程式以 [MIT License](/LICENSE) 授權，可自由複製、修改並用在你的專案中。API 提供的資料依各來源機關的授權條款使用，請見 dashboard.tw 的資料來源說明。