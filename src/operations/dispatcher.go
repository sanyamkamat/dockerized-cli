//go:generate $GOPATH/bin/pegomock generate --package mocks --output mocks/dispatcher_generated.go Dispatcher

package operations

type Dispatcher interface {
	Dispatch(op Operation) (error, interface{})
}

type DefaultDispatcher struct{}

func (d *DefaultDispatcher) Dispatch(op Operation) (error, interface{}) {
	err, value := op.Execute()
	return err, value
}
