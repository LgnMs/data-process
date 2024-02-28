<h4 align="center">
   数据处理平台
    <a href="https://github.com/LgnMs/data-process-center/actions/workflows/rust.yml" target="_blank">
       <img src="https://github.com/LgnMs/data-process-center/actions/workflows/rust.yml/badge.svg" />
    </a>
</h4>


### setup
1. 准备数据库 (下列数据库可选)
   - postgres >= 16.1
   - mysql >= 8.x
2. 创建数据库data_process_web、data_process_cache
3. 安装运行环境
   - [rust](https://www.rust-lang.org/tools/install) >= 1.75.0
   - [nodejs](https://nodejs.org/) >= v18.17.1

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
