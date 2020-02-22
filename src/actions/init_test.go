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
	expected := operations.MakeDir{Path: ".dockerized"}
	if funk.Contains(ops, &expected) == false {
		t.Error(spew.Sprintf("\nhave %#v\nwant an array containing %#v", ops, expected))
	}
}

func TestChecksForExistingDirectory(t *testing.T) {
	init := &Init{
		WithYarnCache: false,
	}

	ops := init.Execute()
	expected := operations.AssertPathDoesNotExist{Path: ".dockerized"}
	if funk.Contains(ops, &expected) == false {
		t.Error(spew.Sprintf("\nhave %#v\nwant an array containing %#v", ops, expected))
	}
}
