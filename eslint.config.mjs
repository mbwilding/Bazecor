import { fixupConfigRules, fixupPluginRules } from "@eslint/compat";
import _import from "eslint-plugin-import";
import react from "eslint-plugin-react";
import prettier from "eslint-plugin-prettier";
import globals from "globals";
import path from "node:path";
import { fileURLToPath } from "node:url";
import js from "@eslint/js";
import { FlatCompat } from "@eslint/eslintrc";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const compat = new FlatCompat({
    baseDirectory: __dirname,
    recommendedConfig: js.configs.recommended,
    allConfig: js.configs.all
});

export default [{
    ignores: ["*", "!src"],
}, ...fixupConfigRules(compat.extends(
    "eslint:recommended",
    "plugin:@typescript-eslint/eslint-recommended",
    "plugin:@typescript-eslint/recommended",
    "plugin:import/recommended",
    "plugin:import/electron",
    "plugin:import/typescript",
    "plugin:react/recommended",
    "plugin:react-hooks/recommended",
    "airbnb",
    "airbnb/hooks",
    "airbnb-typescript",
    "prettier",
)), {
    plugins: {
        import: fixupPluginRules(_import),
        react: fixupPluginRules(react),
        prettier,
    },

    languageOptions: {
        globals: {
            ...globals.browser,
            ...globals.node,
        },

        ecmaVersion: 5,
        sourceType: "commonjs",

        parserOptions: {
            parser: "@typescript-eslint/parser",
            project: "./tsconfig.json",
            tsconfigRootDir: "/Users/anon/dev/bazecor",
        },
    },

    settings: {
        "import/parsers": {
            "@typescript-eslint/parser": [".ts", ".tsx"],
        },

        react: {
            version: "detect",
        },

        "import/resolver": {
            typescript: {
                project: ".",
            },

            alias: {
                map: [
                    ["@Assets", "./src/static"],
                    ["@Renderer", "./src/renderer"],
                    ["@Types", "./src/renderer/types"],
                ],

                extensions: [".ts", ".js", ".jsx", ".tsx", ".json"],
            },
        },
    },

    rules: {
        "prettier/prettier": ["error"],

        "import/no-extraneous-dependencies": ["error", {
            devDependencies: true,
        }],

        "no-underscore-dangle": "off",
        "react/require-default-props": "off",
        "react/function-component-definition": "off",
        "import/prefer-default-export": "off",
        "react/jsx-no-useless-fragment": "off",
        "no-restricted-syntax": "off",
        "@typescript-eslint/lines-between-class-members": "off",
        "max-classes-per-file": ["warn", 2],
        "prefer-regex-literals": "off",
        "no-control-regex": "off",
    },
}];
