import configCommon from './common.json';

const configEnv = require(`./${process.env.NODE_ENV}.json`);

const envVarNames = [
  'PROVIDER_SOCKET',
  'DEVELOPMENT_KEYRING'
];
const envVars = envVarNames.reduce((mem, n) => {
  if (process.env[n] !== undefined) mem[n.slice(10)] = process.env[n];
  return mem;
}, {});

const config = {
  ...configCommon,
  ...configEnv,
  ...envVars
};

export default config;
