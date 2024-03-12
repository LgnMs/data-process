
// 在server环境中读取配置
export async function getConfig() {
  return await fetch(`http://localhost:${process.env.PORT}/config.json`, {cache: 'no-store'}).then(res => res.json());
}
