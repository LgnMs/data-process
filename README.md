<h4 style="text-align: center">
   数据处理平台
    <a href="https://github.com/LgnMs/data-process-center/actions/workflows/rust.yml" target="_blank">
       <img src="https://github.com/LgnMs/data-process-center/actions/workflows/rust.yml/badge.svg"  alt="icon"/>
    </a>
</h4>


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

- [ ] 采集Excel中的数据
- [ ] 采集日志存储方式优化？
- [ ] 首页-报表展示
- [ ] 可接入第三方登录平台认证信息
- [ ] 平台名称可配置
- [ ] 缓存表入库时去掉重复数据