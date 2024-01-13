import * as webpack from "webpack";
import * as path from "path";

const config: webpack.Configuration = {
    entry: "./src/app.tsx",
    output: {
        filename: "bundle.js",
        path: path.resolve(process.cwd() + "/dist")
    },
    module: {
        rules: [
            {
                test: /\.tsx$/,
                use: "ts-loader",    
            }
        ]
    },
    resolve: {
        extensions: ['.ts', '.js', '.tsx', '.jsx'],
    }
};

export default config;