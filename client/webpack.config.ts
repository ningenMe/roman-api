import * as webpack from "webpack";
import * as path from "path";

const config: webpack.Configuration = {
    entry: "./src/app.ts",
    output: {
        filename: "bundle.js",
        path: path.resolve(process.cwd() + "/dist")
    },
    module: {
        rules: [
            {
                test: /\.ts$/,
                use: "ts-loader",    
            }
        ]
    }
};

export default config;