import findBaseDir from '../utils/find-base-dir';
import createReadConfig from '../utils/read-config';
import prettifyErrors from '../utils/prettify-errors';
import runDockerCompose from '../utils/run-docker-compose';

const readConfig = createReadConfig();

function execDockerComposeCommand(command, baseDir, dockerComposeFile) {
    return runDockerCompose({
        baseDir,
        dockerComposeFile,
        dockerComposeArgs: command,
    });
}

export default {
    command: 'compose',
    desc: 'run a docker-compose command',
    builder: yargs => yargs,
    handler: prettifyErrors(async () => {
        const command = process.argv.slice(3);
        const baseDir = await findBaseDir();
        const config = readConfig(baseDir);
        const dockerComposeFile = config.composeFile;

        return execDockerComposeCommand(command, baseDir, dockerComposeFile);
    }),
};
