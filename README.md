# 资产管理平台

## 平台架构

### 服务端

在服务端，我准备使用Rust来编写接口，并使用postgreSQL来组织数据

### 网页端

> ⚠ 在前端，我暂时准备使用solidjs来编写UI界面, 由于生态原因，我不得不放弃使用solid.js

在前端，我将使用react来开发前端界面。

#### front end log

1. framework: select `React`
2. build tool: select `Vite`
3. UI components libary:
    + most components from `shadcn/ui`
    + other intresting components from `magicUI`
4. for request: select `fetch`

#### 未来计划

+ 抽出一个管理系统框架
+ 抽出一个流程管理系统(AKA `OA`)