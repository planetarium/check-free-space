const { loadBinding } = require('@node-rs/helper')

if (!process) {
  require(require.resolve(`@planetarium/check-free-space-${process}.node`));
}

/**
 * __dirname means load native addon from current dir
 * 'check-free-space' is the name of native addon
 * the second arguments was decided by `napi.name` field in `package.json`
 * the third arguments was decided by `name` field in `package.json`
 * `loadBinding` helper will load `check-free-space.[PLATFORM].node` from `__dirname` first
 * If failed to load addon, it will fallback to load from `@planetarium/check-free-space-[PLATFORM]`
 */
module.exports = loadBinding(__dirname, 'check-free-space', '@planetarium/check-free-space')
