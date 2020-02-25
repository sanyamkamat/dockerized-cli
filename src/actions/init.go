package actions

import (
	"benzaita/dockerized/operations"
	"errors"
)

type Init struct {
	WithYarnCache bool
}

func (a *Init) Execute(dispatcher operations.Dispatcher) error {
	_, exists := dispatcher.Dispatch(&operations.CheckIfPathExists{Path: ".dockerized"})
	if exists == true {
		return errors.New("Refusing to override existing directory: .dockerized")
	}
	dispatcher.Dispatch(&operations.MakeDir{Path: ".dockerized"})
	dispatcher.Dispatch(&operations.WriteFile{Path: ".dockerized/Dockerfile.dockerized", Content: "TODO"})
	return nil
}
