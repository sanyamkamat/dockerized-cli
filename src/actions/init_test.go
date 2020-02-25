//go:generate $GOPATH/bin/mockgen -package mocks -destination mocks/dispatcher_generated.go benzaita/dockerized/operations Dispatcher

package actions

import (
	"benzaita/dockerized/actions/mocks"
	"benzaita/dockerized/operations"
	"testing"

	"github.com/golang/mock/gomock"
	"github.com/stretchr/testify/assert"
)

func TestCreatesDirectoryIfMissing(t *testing.T) {
	init := &Init{
		WithYarnCache: false,
	}
	mockCtrl := gomock.NewController(t)
	defer mockCtrl.Finish()

	m := mocks.NewMockDispatcher(mockCtrl)
	m.EXPECT().Dispatch(&operations.CheckIfPathExists{Path: ".dockerized"}).Return(nil, false)
	m.EXPECT().Dispatch(&operations.MakeDir{Path: ".dockerized"})

	assert.Nil(t, init.Execute(m))
}

// func TestFailsIfDirectoryExists(t *testing.T) {
// 	init := &Init{
// 		WithYarnCache: false,
// 	}
// 	mockCtrl := gomock.NewController(t)
// 	defer mockCtrl.Finish()

// 	m := mocks.NewMockDispatcher(mockCtrl)
// 	m.EXPECT().Dispatch(&operations.CheckIfPathExists{Path: ".dockerized"}).Return(nil, true)

// 	err := init.Execute(m)
// 	expectedError := errors.New("Refusing to override existing directory: .dockerized")
// 	assert.Equal(t, expectedError, err)
// }

func TestWritesDockerfile(t *testing.T) {
	init := &Init{
		WithYarnCache: false,
	}
	mockCtrl := gomock.NewController(t)
	defer mockCtrl.Finish()

	m := mocks.NewMockDispatcher(mockCtrl)
	m.EXPECT().Dispatch(&operations.CheckIfPathExists{Path: ".dockerized"}).Return(nil, false)
	m.EXPECT().Dispatch(&operations.MakeDir{Path: ".dockerized"})
	m.EXPECT().Dispatch(&operations.WriteFile{Path: ".dockerized/Dockerfile.dockerized", Content: "123"})

	err := init.Execute(m)
	assert.Nil(t, err)
}
