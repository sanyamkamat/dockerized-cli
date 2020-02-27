package actions

import (
	"benzaita/dockerized/operations"
	"benzaita/dockerized/operations/mocks"
	"errors"
	"testing"

	. "github.com/petergtz/pegomock"
	"github.com/stretchr/testify/assert"
)

func NewInitAction() *Init {
	return &Init{WithYarnCache: false}
}

func TestCreatesDirectoryIfMissing(t *testing.T) {
	dispatcher := mocks.NewMockDispatcher(WithT(t))
	When(dispatcher.Dispatch(&operations.CheckIfPathExists{Path: ".dockerized"})).ThenReturn(nil, false)

	init := NewInitAction()

	assert.Nil(t, init.Execute(dispatcher))
	dispatcher.VerifyWasCalled(Once()).Dispatch(&operations.MakeDir{Path: ".dockerized"})
}

func TestFailsIfDirectoryExists(t *testing.T) {
	dispatcher := mocks.NewMockDispatcher(WithT(t))
	When(dispatcher.Dispatch(&operations.CheckIfPathExists{Path: ".dockerized"})).ThenReturn(nil, true)

	init := NewInitAction()

	err := init.Execute(dispatcher)
	expectedError := errors.New("Refusing to override existing directory: .dockerized")
	assert.Equal(t, expectedError, err)
}

func TestWritesDockerfile(t *testing.T) {
	dispatcher := mocks.NewMockDispatcher(WithT(t))
	When(dispatcher.Dispatch(&operations.CheckIfPathExists{Path: ".dockerized"})).ThenReturn(nil, false)

	init := NewInitAction()

	err := init.Execute(dispatcher)
	assert.Nil(t, err)
	dispatcher.VerifyWasCalled(Once()).Dispatch(&operations.WriteFile{Path: ".dockerized/Dockerfile.dockerized", Content: "123"})
}
