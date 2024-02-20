## 数据处理平台

### setup
1. 准备数据库 (下列数据库可选)
   - postgres >= 16.1
   - mysql (TODO)
2. 创建数据库data_process_web、data_process_cache
3. 安装运行环境
   - [rust](https://www.rust-lang.org/tools/install) >= 1.75.0
   - [nodejs](https://nodejs.org/) >= v18.17.1

4. 创建数据库data_process_web、data_process_cache（表会自动创建）
5. 修改.env
6. 运行
```shell
# 后端
$ cargo run
# 前端
$ cd crates/process_web/ui && npm run dev
```
