package actions

import "benzaita/dockerized/operations"

type Init struct {
	WithYarnCache bool
}

func (a *Init) Execute(dispatcher operations.Dispatcher) error {
	err, exists := dispatcher.Dispatch(&operations.CheckIfPathExists{Path: ".dockerized"})
	if exists == false {
		dispatcher.Dispatch(&operations.MakeDir{Path: ".dockerized"})
	}
	return err
}
