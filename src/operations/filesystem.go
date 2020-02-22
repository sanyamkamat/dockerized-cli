package operations

import "errors"

type MakeDir struct {
	Path string
}

func (operation *MakeDir) Execute() error {
	return errors.New("MakeDir not implemented yet")
}

type AssertPathDoesNotExist struct {
	Path string
}

func (operation *AssertPathDoesNotExist) Execute() error {
	return errors.New("AssertPathDoesNotExist not implemented yet")
}
