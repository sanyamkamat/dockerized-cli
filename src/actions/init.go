package actions

import "benzaita/dockerized/operations"

type Init struct {
	WithYarnCache bool
}

func (a *Init) Execute() []operations.Operation {
	return []operations.Operation{
		&operations.AssertPathDoesNotExist{Path: ".dockerized"},
		&operations.MakeDir{Path: ".dockerized"},
	}
}
