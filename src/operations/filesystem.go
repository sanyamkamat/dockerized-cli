package operations

import "errors"

type MakeDir struct {
	Path string
}

func (operation *MakeDir) Execute() (error, interface{}) {
	return errors.New("MakeDir not implemented yet"), nil
}

type CheckIfPathExists struct {
	Path string
}

func (operation *CheckIfPathExists) Execute() (error, interface{}) {
	return errors.New("CheckIfPathExists not implemented yet"), nil
}

type WriteFile struct {
	Path    string
	Content string
}

func (Operation *WriteFile) Execute() (error, interface{}) {
	return errors.New("WriteFile not implemented yet"), nil
}
