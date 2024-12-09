# 服务端 Response Json 格式

> 尽可能的简单，并忽略性能

## 成功

成功时直接返回全部数据，将状态放在http code中。

```json
{
    "user_id": "aabb",
    "user_name": "ccdd"
}
```

## 失败

失败时直接返回`message`字段，将状态放在http code中。

```json
{
    "message": "user already exist."
}
```