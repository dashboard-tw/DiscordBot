# 程式碼生成器

Python 3.12

預設讀取專案內的 `target/dashboard-catalog.json` 與 `target/dashboard-openapi.json`；這些檔案不在版本控制內，請自行準備 catalog 與 OpenAPI JSON。

## 執行
```sh
python generators/generate_models.py
python generators/generate_embed_commands.py
```
