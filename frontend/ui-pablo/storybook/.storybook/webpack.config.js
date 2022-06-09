const path = require("path");

module.exports = ({ config }) => {

  // Transpile @integrations-lib module to ES5
  // config.module.rules.exclude = [];
  // config.module.rules[0].resolve = {
  //   fullySpecified: false
  // };

  config.module.rules.unshift({
    test: /\.(ts|tsx|js|jsx)$/,
    use: [
      {
        loader: require.resolve("babel-loader"),
        options: {
          presets: [
            [
              "@babel/preset-env",
              {
                targets: {
                  browsers: ["last 2 versions", "ie >= 11"]
                }
              }
            ],
            "@babel/preset-react",
            "@babel/preset-typescript"
          ],
          plugins: [
            "@babel/plugin-syntax-import-meta",
            "@babel/plugin-proposal-class-properties",
            "@babel/plugin-proposal-object-rest-spread",
            "@babel/plugin-syntax-dynamic-import",
            "@babel/plugin-transform-runtime",
          ]
        }
      },
      {
        loader: require.resolve('@open-wc/webpack-import-meta-loader'),
      },
    ],
    // Exclude the untransformed packages from the exclude rule here
    exclude: /node_modules\/(?!(@integrations-lib\/core|@polkadot\/api|@polkadot\/types-codec|@polkadot\/types|@polkadot\/rpc-provider|@polkadot\/keyring|@polkadot\/api-derive|@polkadot\/rpc-core|@polkadot\/util-crypto)\/).*/,
  });
  // Alternately, for an alias:
  config.resolve.alias = {
    "@/pages": path.resolve(__dirname, "..", "..", "app", "pages"),
    "@/components": path.resolve(__dirname, "..", "..", "app", "components"),
    "@/constants": path.resolve(__dirname, "..", "..", "app", "constants"),
    "@/styles": path.resolve(__dirname, "..", "..", "app", "styles"),
    "@/utils": path.resolve(__dirname, "..", "..", "app", "utils"),
    "@/contexts": path.resolve(__dirname, "..", "..", "app", "contexts"),
    "@/hooks": path.resolve(__dirname, "..", "..", "app", "hooks"),
    "@/stores": path.resolve(__dirname, "..", "..", "app", "stores"),
    "@/store": path.resolve(__dirname, "..", "..", "app", "store"),
    "@/defi": path.resolve(__dirname, "..", "..", "app", "defi"),
    "@/updaters": path.resolve(__dirname, "..", "..", "app", "updaters"),
    "@/subsquid": path.resolve(__dirname, "..", "..", "app", "subsquid")
  };

  return config;
};
