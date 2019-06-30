import { Command, flags } from '@oclif/command';
import * as fs from 'fs';
import * as path from 'path';
import * as yaml from 'js-yaml';
import prettifyErrors from '../utils/prettify-errors';

const composeConfig = {
    version: '2',
    services: {
        dockerized: {
            build: {
                context: '.',
                dockerfile: '',
            },
            entrypoint: ['sh', '-c'],
            environment: [],
            volumes: [],
            // eslint-disable-next-line @typescript-eslint/camelcase
            network_mode: '',
        },
    },
    volumes: {},
};

const dockerConfig = `
FROM busybox
# install your build dependencies here
`;

export default class Init extends Command {
    public static description = 'initialize dockerized in this directory (see also: init --help)';

    public static examples = [`$ dockerized init`];

    public static flags = {
        composeFile: flags.string({
            char: 'C',
            description: 'Docker-Compose file to create',
            default: '.dockerized/docker-compose.dockerized.yml',
        }),
        dockerFile: flags.string({
            char: 'D',
            description: 'Dockerfile to create',
            default: '.dockerized/Dockerfile.dockerized',
        }),
        withYarnCache: flags.boolean({
            char: 'y',
            description: 'Includes support for utilizing yarn cache',
            default: false,
            allowNo: true,
        }),
        withNestedDocker: flags.boolean({
            char: 'd',
            description: 'Includes support for running Docker inside Docker',
            default: true,
            allowNo: true,
        }),
    };

    public async run() {
        const { args, flags } = this.parse(Init);

        return prettifyErrors(() => {
            const config = {
                composeFile: flags.composeFile,
            };

            if (fs.existsSync('.dockerized')) {
                throw new Error('already initialized');
            }

            if (fs.existsSync(flags.composeFile)) {
                throw new Error(
                    `will not overwrite ${flags.composeFile}. Use --composeFile to choose a different name`,
                );
            }

            if (fs.existsSync(flags.dockerFile)) {
                throw new Error(`will not overwrite ${flags.dockerFile}. Use --dockerFile to choose a different name`);
            }

            fs.mkdirSync('.dockerized');
            fs.writeFileSync('.dockerized/config.json', JSON.stringify(config, null, 2));

            composeConfig.services.dockerized.build.dockerfile = path.relative(
                path.dirname(flags.composeFile),
                flags.dockerFile,
            );

            if (flags.withYarnCache) {
                composeConfig.services.dockerized.volumes.push('yarn-cache:/data/yarn-cache');
                composeConfig.services.dockerized.environment.push('YARN_CACHE_FOLDER=/data/yarn-cache');
                composeConfig.volumes['yarn-cache'] = {};
            }

            if (flags.withNestedDocker) {
                composeConfig.services.dockerized.volumes.push('/var/run/docker.sock:/var/run/docker.sock');
                // eslint-disable-next-line @typescript-eslint/camelcase
                composeConfig.services.dockerized.network_mode = 'host';
            }

            fs.writeFileSync(config.composeFile, yaml.safeDump(composeConfig));

            fs.writeFileSync(flags.dockerFile, dockerConfig);

            console.error(`created ${flags.composeFile}`);
            console.error(`created ${flags.dockerFile}`);
            console.error(`hint: edit ${flags.dockerFile} to set up your container`);
        })();
    }
}