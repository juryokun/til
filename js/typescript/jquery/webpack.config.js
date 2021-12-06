const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
    mode: 'development',

    entry: './src/index.ts',

    output: {
        filename: 'bundle.js',
        path: path.resolve(__dirname, 'dist'),
        publicPath: '/dist/'
    },
    // devtool: 'inline-source-map',
    devtool: 'source-map',
    module: {
        rules: [{
            test: /\.ts$/,
            use: 'ts-loader',
            include: path.resolve(__dirname, 'src'),
            exclude: /node_modules/
        }]
    },
    devServer: {
        static: {
            directory: path.join(__dirname, 'dist')
        },
    },
    resolve: {
        extensions: ['.ts', '.js']
    },
    plugins: [
        // HTML ファイルの出力設定
        new HtmlWebpackPlugin({
            template: './src/index.html'
        })
    ]
}