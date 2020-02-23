package actions

import (
	"benzaita/dockerized/mock_operations"
	"benzaita/dockerized/operations"
	"testing"

	"github.com/golang/mock/gomock"
)

// func TestCreatesDockerizedDirectory(t *testing.T) {
// 	init := &Init{
// 		WithYarnCache: false,
// 	}

// 	ops := init.Execute()
// 	expected := &operations.MakeDir{Path: ".dockerized"}
// 	if funk.Contains(ops, &expected) == false {
// 		t.Error(spew.Sprintf("\nhave %#v\nwant an array containing %#v", ops, expected))
// 	}
// }

func TestChecksForExistingDirectory(t *testing.T) {
	init := &Init{
		WithYarnCache: false,
	}
	mockCtrl := gomock.NewController(t)
	defer mockCtrl.Finish()

	m := mock_operations.NewMockDispatcher(mockCtrl)
	m.EXPECT().Dispatch(&operations.CheckIfPathExists{Path: ".dockerized"}).Return(nil, false)
	m.EXPECT().Dispatch(&operations.MakeDir{Path: ".dockerized"}).Return(nil, nil)

	init.Execute(m)
}
