package main

import (
	"fmt"
	"os"

	"github.com/urfave/cli/v2"

	"benzaita/dockerized/actions"
	"benzaita/dockerized/operations"
)

func toCliError(err error) error {
	if err == nil {
		return nil
	}
	return cli.Exit(fmt.Sprintf("Error executing command: %v", err), 1)
}

func main() {
	dispatcher := &operations.DefaultDispatcher{}

	app := &cli.App{
		Name: "Dockerized",
		Commands: []*cli.Command{
			&cli.Command{
				Name:            "exec",
				SkipFlagParsing: true,
				Action: func(c *cli.Context) error {
					args := c.Args().Slice()
					action := &actions.Exec{
						Args: args,
					}
					return action.Execute()
				},
			},
			&cli.Command{
				Name: "init",
				Flags: []cli.Flag{
					&cli.BoolFlag{Name: "withYarnCache"},
				},
				Action: func(c *cli.Context) error {
					action := &actions.Init{
						WithYarnCache: c.Bool("withYarnCache"),
					}
					err := action.Execute(dispatcher)
					return toCliError(err)
				},
			},
		},
	}

	app.Run(os.Args)
}
