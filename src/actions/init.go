package actions

import "benzaita/dockerized/operations"

type Init struct {
	WithYarnCache bool
}

func (a *Init) Execute() []operations.Operation {
	return []operations.Operation{
		&operations.MakeDirOperation{Path: ".dockerized"},
	}
}
