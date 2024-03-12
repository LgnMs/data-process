const { readFileSync } = require('fs');
const path = require('path');

// 在server环境中读取配置
export async function getConfig() {
  const filePath = path.resolve('public/config.json');
  const rawConfig = readFileSync(filePath);
  const config = JSON.parse(rawConfig.toString());

  return Promise.resolve(config)
}
