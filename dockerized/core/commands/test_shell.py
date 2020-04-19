import unittest
from pathlib import Path
from unittest.mock import MagicMock

from adapters.dockercompose import DockerCompose
from adapters.environment import Environment
from core.commands.shell import ShellCommand


class TestShellCommand(unittest.TestCase):
    def test_runs_docker_compose(self):
        project_dir = Path('/project-dir')
        working_dir = project_dir.joinpath('sub-dir')

        env = Environment()
        env.get_project_dir = MagicMock(return_value=project_dir)
        env.get_working_dir = MagicMock(return_value=working_dir)

        docker_compose = DockerCompose(Path(composefile='compose-file'), project_dir=project_dir)
        docker_compose.run = MagicMock()

        shell_command = ShellCommand(env=env, docker_compose=docker_compose)
        shell_command.run()

        docker_compose.run.assert_called_once_with(
            working_dir=working_dir,
            command='/bin/sh'
        )