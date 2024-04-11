<h4 style="text-align: center">
   数据处理平台
    <a href="https://github.com/LgnMs/data-process-center/actions/workflows/rust.yml" target="_blank">
       <img src="https://github.com/LgnMs/data-process-center/actions/workflows/rust.yml/badge.svg"  alt="icon"/>
    </a>
</h4>

## 简介
提供数据的采集服务，数据同步服务，数据共享服务，以及日志记录与查看功能。

### setup
1. 准备数据库 (下列数据库可选)
   - postgres >= 16.1
   - mysql >= 8.x
2. 创建数据库data_process_web、data_process_cache
3. 安装运行环境
   - [rust](https://www.rust-lang.org/tools/install) >= 1.75.0
   - [nodejs](https://nodejs.org/) >= v16.20.2
   - （如果要操作MSSQL server，Oracle，Kingbase）java >= 11

4. 修改.env
5. 运行
```shell
# 后端
$ cargo run
# 前端
$ cd crates/process_web/ui && npm run dev
```

### TODO

- [ ] 采集Excel、csv、JSON中的数据
- [ ] 数据清洗
- [ ] 采集任务可配置要默认携带安全认证信息
- [ ] 共享接口调用添加权限认证