module.exports = {
    env: {
        browser: true,
        node: true,
    },
    parserOptions: {
        ecmaVersion: 2015,
        sourceType: 'module',
    },
    extends: ['eslint:recommended', 'prettier'],
    overrides: [
        {
            files: ['*.ts', '*.tsx'],
            env: {
                browser: true,
            },
            parser: '@typescript-eslint/parser',
            parserOptions: {
                project: 'tsconfig.json',
                tsconfigRootDir: __dirname,
                sourceType: 'module',
            },
            plugins: ['@typescript-eslint'],
            extends: [
                'preact',
                'plugin:@typescript-eslint/recommended',
                'plugin:@typescript-eslint/recommended-requiring-type-checking',
                'prettier',
            ],
        },
    ],
    ignorePatterns: ['build/'],
};
