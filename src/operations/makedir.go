package operations

import "errors"

type MakeDirOperation struct {
	Path string
}

func (o *MakeDirOperation) Execute() error {
	return errors.New("MakeDirOperation not implemented yet")
}
