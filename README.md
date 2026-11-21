# 臺灣儀表板Discord機器人

![Rust](https://img.shields.io/badge/Rust-1.99.1-000000.svg?logo=rust&logoColor=white)
![Discord](https://img.shields.io/badge/Discord-7289DA.svg?logo=discord&logoColor=white)

由Rust所編寫的Discord機器人，依照[SampleCode](https://github.com/dashboard-tw/SampleCode)進行使用者的互動開發。

## 基本設置

使用`.env.example`進行設置後將其改名為`.env`環境變數檔案：

```ini
TOKEN="yourbottoken"    // 設置機器人的TOKEN
PREFIX="yourbotprefix"  // 設置機器人的觸發前綴(預設為"!")
API_KEY="yourapikey"    // 設置dashboard.tw的API Key
```

## TODO

- [x] 基本交互
  - [x] 機器人啟動
  - [x] 環境變數
  - [x] 延遲測試
  - [x] 錯誤處理

- [ ] API交互
  - [ ] 檢查健康狀態
  - [ ] 請求資料
  - [ ] 更多...

- [ ] 使用者體驗優化
  - [ ] 更多...

## 授權

範例程式以 [MIT License](/LICENSE) 授權，可自由複製、修改並用在你的專案中。API 提供的資料依各來源機關的授權條款使用，請見 dashboard.tw 的資料來源說明。