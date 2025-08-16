## やりたいこと

- 複数テーブルの編集を共通のレイアウト（表形式）で行える
- テーブルごとにさらに画面を分けて、見せたい項目を制御できるようにする
- 項目ごとにクラスを作って拡張しやすくする

## 構成

```text
app/
  Http/
    Controllers/
      DataEditorController.php
  Services/
    DataEditor/
      Interfaces/
        EditableTableServiceInterface.php
      UsersTableService.php
      ProductsTableService.php
      TableServiceFactory.php
  Models/
    User.php
    Product.php
```
