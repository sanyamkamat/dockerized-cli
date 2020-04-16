from pathlib import Path

import click

from core.commands.shell import ShellCommand
from core.commands.clean import CleanCommand
from core.commands.init import InitCommand, InitError
from core.commands.exec import ExecCommand, ExecError


@click.group()
def main():
    pass


@main.command()
def init():
    init_command = InitCommand(Path.cwd())
    try:
        init_command.run()
    except InitError as err:
        click.echo(err.message, err=True)
        click.get_current_context().exit(1)
    click.echo('created')


@main.command(context_settings=dict(
    ignore_unknown_options=True,
))
@click.argument('command', nargs=-1, type=click.UNPROCESSED)
def exec(command):
    exec_command = ExecCommand(' '.join(command))
    try:
        exit_code = exec_command.run()
    except ExecError as err:
        click.echo(err.message, err=True)
        click.get_current_context().exit(1)
    click.get_current_context().exit(exit_code)


@main.command()
def clean():
    clean_command = CleanCommand()
    clean_command.run()

@main.command()
def shell():
    shell_command = ShellCommand()
    shell_command.run()
