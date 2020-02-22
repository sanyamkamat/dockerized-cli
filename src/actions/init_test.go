package actions

import (
	"benzaita/dockerized/operations"
	"testing"

	"github.com/davecgh/go-spew/spew"
	"github.com/thoas/go-funk"
)

func TestCreatesDockerizedDirectory(t *testing.T) {
	init := &Init{
		WithYarnCache: false,
	}

	ops := init.Execute()
	expected := operations.MakeDirOperation{Path: ".dockerized"}
	if funk.Contains(ops, &expected) == false {
		t.Error(spew.Sprintf("got %#v; wanted an array containing %#v", ops, expected))
	}
}
