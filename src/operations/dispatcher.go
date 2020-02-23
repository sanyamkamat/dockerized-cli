package operations

type Dispatcher interface {
	Dispatch(op Operation) (error, interface{})
}

type DefaultDispatcher struct{}

func (d *DefaultDispatcher) Dispatch(op Operation) (error, interface{}) {
	err, value := op.Execute()
	return err, value
}
