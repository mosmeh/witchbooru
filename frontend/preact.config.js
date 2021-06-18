import envVars from 'preact-cli-plugin-env-vars';

export default function (config, env, helpers) {
    envVars(config, env, helpers);

    const publicPath = process.env.GITHUB_PAGES
        ? `/${process.env.GITHUB_PAGES}/`
        : '/';
    const ghEnv =
        process.env.GITHUB_PAGES &&
        JSON.stringify(`${process.env.GITHUB_PAGES}`);

    config.output.publicPath = publicPath;
    const { plugin } = helpers.getPluginsByName(config, 'DefinePlugin')[0];
    Object.assign(plugin.definitions, { ['process.env.GITHUB_PAGES']: ghEnv });
}
