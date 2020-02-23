package actions

import (
	"benzaita/dockerized/operations"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

type MockDispatcher struct {
	mock.Mock
}

func (d *MockDispatcher) Dispatch(op operations.Operation) (error, interface{}) {
	args := d.Called(op)
	return args.Error(0), args.Get(1)
}

func TestCreatesDockerizedDirectoryIfMissing(t *testing.T) {
	init := &Init{WithYarnCache: false}

	dispatcher := new(MockDispatcher)
	dispatcher.On("Dispatch", &operations.CheckIfPathExists{Path: ".dockerized"}).Return(nil, false).Once()
	dispatcher.On("Dispatch", mock.Anything).Return(nil, nil)

	init.Execute(dispatcher)

	dispatcher.AssertCalled(t, "Dispatch", &operations.MakeDir{Path: ".dockerized"})
}

func TestFailsIfDockerizedDirectoryExists(t *testing.T) {
	init := &Init{WithYarnCache: false}

	dispatcher := new(MockDispatcher)
	dispatcher.On("Dispatch", &operations.CheckIfPathExists{Path: ".dockerized"}).Return(nil, true)

	err := init.Execute(dispatcher)

	assert.EqualError(t, err, "Refusing to override existing directory: .dockerized")
}

func TestCreatesDockerfile(t *testing.T) {
	init := &Init{WithYarnCache: false}

	dispatcher := new(MockDispatcher)
	dispatcher.On("Dispatch", &operations.CheckIfPathExists{Path: ".dockerized"}).Return(nil, false)
	dispatcher.On("Dispatch", mock.Anything).Return(nil, nil)

	init.Execute(dispatcher)

	dispatcher.AssertCalled(t, "Dispatch", &operations.WriteFile{
		Path:    ".dockerized/Dockerfile.dockerized",
		Content: "1322",
	},
	)
}
