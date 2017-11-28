const child_process = require('child_process')
const path = require('path')
const debug = require('debug')('builder:exec')
const findBaseDir = require('../utils/find-base-dir')
const readConfig = require('../utils/read-config')
const prettifyErrors = require('../utils/prettify-errors')

function execInDockerCompose(command, baseDir, dockerComposeFile) {
  const args = [
    '-f', path.resolve(baseDir, dockerComposeFile),
    'run',
    '--rm',
    '-v', `${baseDir}:${baseDir}`,
    '-w', process.cwd(),
    'builder',
    command
  ]

  debug(`running: docker-compose ${args.join(' ')}`)
  const child = child_process.spawn('docker-compose', args, {
    stdio: 'inherit'
  })

  child.on('exit', code => {
    debug(`child process exited with code ${code}. propagating`)
    process.exit(code)
  })
}

module.exports = {
  command: 'exec',
  desc: 'execute a command inside the builder',
  builder: yargs => yargs,
  handler: prettifyErrors(async function exec(argv) {
    const command = process.argv.slice(3).join(' ')
    const baseDir = await findBaseDir()
    const config = readConfig(baseDir)
    const dockerComposeFile = config.composeFile

    execInDockerCompose(command, baseDir, dockerComposeFile)
  })
}