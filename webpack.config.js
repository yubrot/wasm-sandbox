const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  mode: 'production',
  entry: __dirname + '/src/index.ts',
  output: {
    path: __dirname + '/dist',
    publicPath: '/wasm-sandbox',
    filename: 'wasm-sandbox.js',
  },
  module: {
    rules: [{ test: /\.tsx?$/, loader: 'ts-loader' }],
  },
  resolve: {
    extensions: ['*', '.js', '.ts', '.tsx'],
  },
  devServer: {
    contentBase: __dirname + '/dist',
    contentBasePublicPath: '/wasm-sandbox',
    port: 3000,
  },
  experiments: {
    asyncWebAssembly: true,
  },
  plugins: [
    // ./crate  --[wasm-pack build]->  ./crate/pkg
    new WasmPackPlugin({
      crateDirectory: __dirname + '/crate',
      outDir: __dirname + '/crate/pkg',
    }),
  ],
};
